import fs from "fs/promises"
import path from "path"

type CoreEvent = {
  type: string
  properties: Record<string, unknown>
}

type CoreOptions = {
  workspaceDir: string
  dataDir?: string
  config?: Record<string, unknown>
  permission?: Array<{ permission: string; action: string; pattern: string }>
  onEvent?: (event: CoreEvent) => void
  abort?: AbortSignal
  logLevel?: "DEBUG" | "INFO" | "WARN" | "ERROR"
  printLogs?: boolean
}

type CoreContext = {
  workspaceDir: string
  abort?: AbortSignal
}

type MessagePart = {
  type: string
  [key: string]: any
}

type SessionMessage = {
  info: { role: string; error?: unknown }
  parts: MessagePart[]
}

type SessionRecord = {
  id: string
  title?: string
  messages: SessionMessage[]
  lastSummary?: string
}

type PromptInput = {
  sessionID: string
  model?: { providerID: string; modelID: string }
  agent?: string
  system?: string
  tools?: Record<string, boolean>
  parts: MessagePart[]
}

type RepairEdit = {
  file: string
  find: string
  replace: string
  replaceAll?: boolean
}

type RuntimeState = {
  workspaceDir: string
  config: Record<string, any>
  onEvent?: (event: CoreEvent) => void
}

const stateStack: RuntimeState[] = []
const sessionStore = new Map<string, SessionRecord>()

function currentState(): RuntimeState {
  const state = stateStack[stateStack.length - 1]
  if (!state) throw new Error("Core runtime is not initialized. Wrap calls with withCore().")
  return state
}

function newID(prefix: string) {
  const random = Math.random().toString(36).slice(2, 10)
  return `${prefix}_${Date.now().toString(36)}${random}`
}

function extractTextFromParts(parts: MessagePart[]) {
  return parts
    .filter((part) => part?.type === "text" && typeof part?.text === "string")
    .map((part) => String(part.text))
    .join("\n")
}

function extractJsonObject(text: string): any | null {
  const direct = text.trim()
  if (!direct) return null
  try {
    return JSON.parse(direct)
  } catch {
    // ignore
  }

  const fenced = direct.match(/```json\s*([\s\S]*?)```/i)?.[1] ?? direct.match(/```\s*([\s\S]*?)```/)?.[1]
  if (fenced) {
    try {
      return JSON.parse(fenced.trim())
    } catch {
      // ignore
    }
  }

  const start = direct.indexOf("{")
  const end = direct.lastIndexOf("}")
  if (start >= 0 && end > start) {
    const snippet = direct.slice(start, end + 1)
    try {
      return JSON.parse(snippet)
    } catch {
      return null
    }
  }
  return null
}

function ensureArrayEdits(value: unknown): RepairEdit[] {
  if (!Array.isArray(value)) return []
  const edits: RepairEdit[] = []
  for (const item of value) {
    if (!item || typeof item !== "object") continue
    const file = typeof (item as any).file === "string" ? (item as any).file.trim() : ""
    const find = typeof (item as any).find === "string" ? (item as any).find : ""
    const replace = typeof (item as any).replace === "string" ? (item as any).replace : ""
    const replaceAll = Boolean((item as any).replaceAll)
    if (!file || !find) continue
    edits.push({ file, find, replace, replaceAll })
  }
  return edits
}

function parseWorkspaceFromPrompt(text: string, fallback: string) {
  const matched = text.match(/^Workspace:\s*(.+)$/m)?.[1]?.trim()
  return matched && matched.length > 0 ? matched : fallback
}

function resolveModelConfig(config: Record<string, any>, override?: { providerID: string; modelID: string }) {
  if (override) return override
  const raw = typeof config.model === "string" ? config.model : ""
  if (!raw.includes("/")) throw new Error("Invalid model config; expected provider/model.")
  const [providerID, ...rest] = raw.split("/")
  const modelID = rest.join("/")
  if (!providerID || !modelID) throw new Error("Invalid model config; provider/model missing.")
  return { providerID, modelID }
}

function resolveEndpoint(baseURL: string) {
  const trimmed = baseURL.trim().replace(/\/+$/, "")
  if (trimmed.endsWith("/chat/completions")) return trimmed
  return `${trimmed}/chat/completions`
}

async function callLLM(config: Record<string, any>, modelOverride: { providerID: string; modelID: string } | undefined, system: string, user: string) {
  const modelRef = resolveModelConfig(config, modelOverride)
  const provider = config.provider?.[modelRef.providerID]
  const options = provider?.options ?? {}
  const apiKey = options.apiKey
  const baseURL = options.baseURL
  if (!apiKey || !baseURL) {
    throw new Error(`Missing provider credentials for ${modelRef.providerID}.`)
  }

  const endpoint = resolveEndpoint(String(baseURL))
  const response = await fetch(endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${apiKey}`,
    },
    body: JSON.stringify({
      model: modelRef.modelID,
      messages: [
        { role: "system", content: system },
        { role: "user", content: user },
      ],
      temperature: 0.1,
    }),
  })

  const text = await response.text()
  if (!response.ok) {
    throw new Error(`LLM request failed (${response.status}): ${text.slice(0, 500)}`)
  }
  try {
    const json = JSON.parse(text)
    const content = json?.choices?.[0]?.message?.content
    if (typeof content === "string") return content
    if (Array.isArray(content)) {
      return content.map((item: any) => (typeof item?.text === "string" ? item.text : "")).join("")
    }
    return ""
  } catch {
    return text
  }
}

async function applyEdits(workspaceDir: string, edits: RepairEdit[]) {
  const workspace = path.resolve(workspaceDir)
  const changed = new Set<string>()
  const failed: string[] = []
  let attempted = 0

  for (const edit of edits) {
    attempted++
    const absolutePath = path.resolve(workspace, edit.file)
    if (!(absolutePath === workspace || absolutePath.startsWith(workspace + path.sep))) {
      failed.push(`${edit.file}: outside workspace`)
      continue
    }
    let content: string
    try {
      content = await fs.readFile(absolutePath, "utf8")
    } catch {
      failed.push(`${edit.file}: file not found`)
      continue
    }
    if (!content.includes(edit.find)) {
      failed.push(`${edit.file}: find text not found`)
      continue
    }
    const updated = edit.replaceAll
      ? content.split(edit.find).join(edit.replace)
      : content.replace(edit.find, edit.replace)
    if (updated === content) continue
    await fs.writeFile(absolutePath, updated)
    changed.add(path.relative(workspace, absolutePath))
  }

  return {
    attempted,
    changedFiles: Array.from(changed),
    failed,
  }
}

function buildRepairInstruction(existingSystem: string | undefined, userPrompt: string) {
  const system = [
    "你是 Rust 修复助手。",
    "你只能输出 JSON，不要输出其他内容。",
    "返回格式:",
    "{",
    '  "summary": "一句话说明本轮做了什么",',
    '  "edits": [',
    '    {"file":"相对路径","find":"原文片段","replace":"替换内容","replaceAll":false}',
    "  ]",
    "}",
    "要求：",
    "- file 必须是工作区内相对路径",
    "- find 必须是可唯一匹配的原文片段",
    "- 每轮最多输出 12 个 edits",
    "- 如果暂时无法修复，edits 返回空数组并在 summary 说明原因",
    "",
    "参考上下文（可选）：",
    existingSystem ?? "",
  ].join("\n")
  return { system, user: userPrompt }
}

async function generateAssistantMessage(session: SessionRecord, input: PromptInput) {
  const state = currentState()
  const userPrompt = extractTextFromParts(input.parts ?? [])

  const summaryOnly =
    input.tools?.["*"] === false ||
    userPrompt.includes("请用1-2句简要总结") ||
    userPrompt.includes("简要总结你刚刚做了哪些修复")

  if (summaryOnly) {
    const text = session.lastSummary ?? "本轮未应用修改。"
    return {
      info: { role: "assistant" },
      parts: [{ type: "text", text }],
    } satisfies SessionMessage
  }

  const workspace = parseWorkspaceFromPrompt(userPrompt, state.workspaceDir)
  const prompt = buildRepairInstruction(input.system, userPrompt)
  const llmText = await callLLM(state.config, input.model, prompt.system, prompt.user)
  const parsed = extractJsonObject(llmText)
  const edits = ensureArrayEdits(parsed?.edits)
  const applyResult = await applyEdits(workspace, edits)

  const summary =
    (typeof parsed?.summary === "string" && parsed.summary.trim()) ||
    `本轮尝试 ${applyResult.attempted} 处修改，成功修改 ${applyResult.changedFiles.length} 个文件。`
  const failedSummary =
    applyResult.failed.length > 0
      ? ` 失败项：${applyResult.failed.slice(0, 3).join("; ")}${applyResult.failed.length > 3 ? "..." : ""}`
      : ""
  const text = `${summary}${failedSummary}`
  session.lastSummary = text

  const toolParts = applyResult.changedFiles.map((filePath) => ({
    type: "tool",
    tool: "edit",
    state: {
      status: "completed",
      input: { filePath },
      output: "applied",
    },
  }))

  if (state.onEvent) {
    for (const filePath of applyResult.changedFiles) {
      state.onEvent({
        type: "message.part.updated",
        properties: {
          part: {
            type: "tool",
            tool: "edit",
            state: { status: "completed", input: { filePath } },
          },
        },
      })
    }
  }

  return {
    info: { role: "assistant" },
    parts: [
      { type: "text", text },
      ...toolParts,
    ],
  } satisfies SessionMessage
}

const Session = {
  async create(input: { title?: string }) {
    const id = newID("ses")
    sessionStore.set(id, {
      id,
      title: input.title,
      messages: [],
    })
    return { id }
  },
  async messages(input: { sessionID: string }) {
    return sessionStore.get(input.sessionID)?.messages ?? []
  },
}

const SessionPrompt = {
  async prompt(input: PromptInput) {
    const session = sessionStore.get(input.sessionID)
    if (!session) throw new Error(`Session not found: ${input.sessionID}`)
    session.messages.push({ info: { role: "user" }, parts: input.parts ?? [] })
    const assistant = await generateAssistantMessage(session, input)
    session.messages.push(assistant)
    return assistant
  },
  cancel(_sessionID: string) {
    // no-op in standalone mode
  },
}

const Provider = {
  async list() {
    const state = currentState()
    return state.config?.provider ?? {}
  },
}

const Config = {
  async get() {
    return currentState().config ?? {}
  },
}

const PermissionNext = {}

const Instance = {
  async provide<T>(input: { directory: string; fn: () => Promise<T> }) {
    return input.fn()
  },
}

const Bus = {
  subscribeAll(_fn: (event: CoreEvent) => void) {
    return () => {}
  },
}

export async function withCore<T>(options: CoreOptions, fn: (ctx: CoreContext) => Promise<T>): Promise<T> {
  stateStack.push({
    workspaceDir: options.workspaceDir,
    config: (options.config ?? {}) as Record<string, unknown>,
    onEvent: options.onEvent,
  })
  try {
    if (options.abort?.aborted) {
      throw new Error("Aborted before core initialization")
    }
    return await fn({ workspaceDir: options.workspaceDir, abort: options.abort })
  } finally {
    stateStack.pop()
  }
}

export function bindAbortToSession(_sessionID: string, _abort?: AbortSignal) {
  return () => {}
}

export async function promptWithAbort(input: { prompt: PromptInput; abort?: AbortSignal }) {
  if (input.abort?.aborted) {
    throw new Error("Aborted")
  }
  return SessionPrompt.prompt(input.prompt)
}

export const Core = {
  Session,
  SessionPrompt,
  Provider,
  PermissionNext,
  Config,
  Instance,
  Bus,
}
