import path from "path"
import fs from "fs/promises"
import fsSync from "fs"
import { withCore, Core } from "./core-runtime.ts"
import { loadFixedProviderConfig } from "./fixed-provider.ts"
import { ensureEmptyDir, copyWorkspace, dirSizeBytes } from "./fs-utils.ts"
import { runCommand } from "./runner.ts"
import { computeDiff } from "./diff.ts"
import type { RepairInput, RepairResult } from "./types.ts"
import { DEFAULT_ALLOWED_COMMANDS, assertAllowed, bashPermissionPatterns, type CommandPolicy } from "./security.ts"
import { getFileCache, resetFileCache } from "./cache/file-cache.ts"
import { getDiagnosticCache, resetDiagnosticCache } from "./cache/diagnostic-cache.ts"
import { CoreManager, getCoreManager, resetCoreManager } from "./core-manager.ts"
import { getDoomLoopTracker, resetDoomLoopTracker, type ToolCall } from "./doom-loop.ts"
import { isRetryableError, getRetryDelay, sleep, withRetry } from "./retry.ts"
import { createRepairUI, RepairUI, Icons, Colors, bold, dim, success, error, warning, info } from "./ui/index.ts"
import { getBackupManager, resetBackupManager } from "./safety/index.ts"
import { applyDeterministicFixes } from "./deterministic-fixes.ts"

type Diagnostic = {
  kind: "fmt" | "check" | "clippy" | "test"
  code: number | null
  stdout: string
  stderr: string
  json?: CargoMessage[]
}

type CargoMessage = {
  reason?: string
  message?: {
    level?: string
    message?: string
    code?: { code?: string } | null
    spans?: Array<{
      file_name?: string
      line_start?: number
      column_start?: number
      line_end?: number
      column_end?: number
    }>
  }
}

function extractTextFromParts(parts: any[]): string {
  return parts
    .filter((part) => part && part.type === "text" && typeof part.text === "string")
    .map((part) => part.text)
    .join("")
    .trim()
}

function extractResponseText(parts: any[]): string | null {
  const textPart = [...parts].reverse().find((part) => part?.type === "text" && typeof part.text === "string")
  if (textPart) return textPart.text
  const reasoningPart = [...parts].reverse().find((part) => part?.type === "reasoning")
  if (reasoningPart) return null
  const toolParts = parts.filter((part) => part?.type === "tool" && part?.state?.status === "completed")
  if (toolParts.length > 0) return null
  return null
}

function describePartTypes(parts: any[]) {
  if (!Array.isArray(parts) || parts.length === 0) return "none"
  return parts.map((part) => (part && part.type ? part.type : "unknown")).join(", ")
}

function deriveFixPlan(diagnostics: Diagnostic[]) {
  const items = new Set<string>()
  const add = (text: string) => {
    if (text) items.add(text)
  }
  const addByCode = (code?: string | null) => {
    switch (code) {
      case "E0106":
        add("补齐生命周期标注（返回引用与参数绑定）")
        break
      case "E0308":
        add("修正类型不匹配（Box/Option/引用层级）")
        break
      case "E0499":
      case "E0502":
      case "E0506":
        add("重构借用范围，避免同时可变/不可变借用")
        break
      case "E0599":
        add("修正函数指针/字段调用语法")
        break
      case "E0133":
        add("为 unsafe 函数调用添加 unsafe 块")
        break
      case "E0061":
        add("修正函数参数数量/顺序")
        break
      default:
        break
    }
  }
  for (const diag of diagnostics) {
    for (const msg of diag.json ?? []) {
      const code = msg.message?.code?.code ?? null
      if (code) addByCode(code)
      const text = msg.message?.message ?? ""
      if (text.includes("expected identifier") && text.includes("keyword")) {
        add("重命名关键字冲突的标识符")
      }
    }
    const stderr = diag.stderr ?? ""
    if (stderr.includes("expected identifier") && stderr.includes("keyword")) {
      add("重命名关键字冲突的标识符")
    }
  }
  return Array.from(items)
}

function buildPermissionRules(
  outputDir: string,
  sourceDir?: string,
  allowBash = false,
  policy?: CommandPolicy,
) {
  const rules = [
    // File reads/lists use absolute paths.
    { permission: "read", action: "allow", pattern: `${outputDir}*` },
    { permission: "list", action: "allow", pattern: `${outputDir}*` },
    // Glob/grep permissions use the pattern string, not a filesystem path.
    { permission: "glob", action: "allow", pattern: "*" },
    { permission: "grep", action: "allow", pattern: "*" },
    // Edit/write/apply_patch permissions use paths relative to the workspace root.
    { permission: "edit", action: "allow", pattern: "*" },
    { permission: "write", action: "allow", pattern: "*" },
    { permission: "patch", action: "allow", pattern: "*" },
    // Default deny for external directory access (override below if sourceDir provided).
    { permission: "external_directory", action: "deny", pattern: "*" },
    // Disable tools that are not part of the headless repair flow.
    { permission: "task", action: "allow", pattern: "*" },
    { permission: "question", action: "allow", pattern: "*" },
    { permission: "webfetch", action: "allow", pattern: "*" },
    { permission: "websearch", action: "allow", pattern: "*" },
    { permission: "codesearch", action: "allow", pattern: "*" },
    { permission: "skill", action: "deny", pattern: "*" },
    { permission: "todoread", action: "allow", pattern: "*" },
    { permission: "todowrite", action: "allow", pattern: "*" },
    { permission: "lsp", action: "allow", pattern: "*" },
    { permission: "plan_enter", action: "allow", pattern: "*" },
    { permission: "plan_exit", action: "allow", pattern: "*" },
    { permission: "batch", action: "allow", pattern: "*" },
  ]

  if (sourceDir) {
    rules.push(
      { permission: "read", action: "allow", pattern: `${sourceDir}*` },
      { permission: "list", action: "allow", pattern: `${sourceDir}*` },
      { permission: "grep", action: "allow", pattern: `${sourceDir}*` },
      { permission: "glob", action: "allow", pattern: `${sourceDir}*` },
      { permission: "external_directory", action: "allow", pattern: `${sourceDir}*` },
    )
  }

  if (allowBash && policy) {
    for (const pattern of bashPermissionPatterns(policy)) {
      rules.push({ permission: "bash", action: "allow", pattern })
    }
  } else {
    rules.push({ permission: "bash", action: "deny", pattern: "*" })
  }

  return rules
}

function summarizeDiagnostics(diagnostics: Diagnostic[]) {
  // 尝试从缓存获取
  const cache = getDiagnosticCache()
  const cached = cache.get(diagnostics)
  if (cached) return cached

  // 计算摘要
  const summary = diagnostics
    .map((d) => {
      const header = `[${d.kind}] exit=${d.code ?? "null"}`
      if (d.json && d.json.length > 0) {
        const items = d.json.filter((item) => item.reason === "compiler-message" && item.message?.level)
        const errors = items.filter((item) => item.message?.level === "error")
        const selected = (errors.length > 0 ? errors : items).slice(0, 20)
        const lines = selected.map((item) => {
          const msg = item.message
          const span = msg?.spans?.[0]
          const loc = span?.file_name ? `${span.file_name}:${span.line_start ?? 0}` : "unknown"
          const code = msg?.code?.code ? ` (${msg.code.code})` : ""
          const raw = msg?.message ?? ""
          const text = raw.length > 200 ? raw.slice(0, 200) + "…" : raw
          const extras =
            msg?.children
              ?.map((child) => `${child.level ?? "note"}: ${child.message}`)
              .filter(Boolean)
              .slice(0, 2)
              .join(" | ") ?? ""
          return extras
            ? `${msg?.level ?? "error"}${code} @ ${loc}: ${text} | ${extras}`
            : `${msg?.level ?? "error"}${code} @ ${loc}: ${text}`
        })
        return `${header}\n${lines.join("\n")}`
      }
      const output = (d.stderr || d.stdout).trim()
      if (!output) return `${header}\n(no output)`
      return `${header}\n${output.slice(0, 2000)}`
    })
    .join("\n\n")

  // 存入缓存
  cache.set(diagnostics, summary)
  return summary
}

type ErrorSpan = {
  file: string
  line: number
  message?: string
  code?: string
}

function collectErrorSpans(diagnostics: Diagnostic[]) {
  const spans: ErrorSpan[] = []
  for (const diag of diagnostics) {
    if (!diag.json) continue
    for (const item of diag.json) {
      if (item.reason !== "compiler-message") continue
      if (item.message?.level !== "error") continue
      const span = item.message.spans?.[0]
      if (!span?.file_name) continue
      spans.push({
        file: span.file_name,
        line: span.line_start ?? 1,
        message: item.message.message,
        code: item.message.code?.code ?? undefined,
      })
    }
  }
  return spans
}

function countTotalErrors(diagnostics: Diagnostic[]): number {
  let count = 0
  for (const diag of diagnostics) {
    if (!diag.json) continue
    for (const item of diag.json) {
      if (item.reason !== "compiler-message") continue
      if (item.message?.level !== "error") continue
      count++
    }
  }
  return count
}

function getLatestDiagnosticByKind(diagnostics: Diagnostic[], kind: Diagnostic["kind"]) {
  for (let i = diagnostics.length - 1; i >= 0; i--) {
    if (diagnostics[i]?.kind === kind) return diagnostics[i]
  }
  return undefined
}

function countFailedTestRuns(diagnostics: Diagnostic[]) {
  return diagnostics.filter((diag) => diag.kind === "test" && diag.code !== 0).length
}

function summarizeTestFailures(diagnostics: Diagnostic[]) {
  const failed = diagnostics.filter((diag) => diag.kind === "test" && diag.code !== 0)
  if (failed.length === 0) return ""

  return failed
    .map((diag, index) => {
      const raw = `${diag.stdout ?? ""}\n${diag.stderr ?? ""}`.trim()
      if (!raw) return `[test#${index + 1}] exit=${diag.code ?? "null"}\n(no output)`
      const lines = raw
        .split("\n")
        .map((line) => line.trimEnd())
        .filter(Boolean)
      const selected = lines.filter((line) =>
        /FAILED|failures:|panicked at|assertion failed|thread '.*' panicked|test result:/i.test(line),
      )
      const body = (selected.length > 0 ? selected : lines.slice(-80)).slice(0, 120).join("\n")
      return `[test#${index + 1}] exit=${diag.code ?? "null"}\n${body.slice(0, 4000)}`
    })
    .join("\n\n")
}

/**
 * Group errors by file for batch processing
 * Returns files sorted by error count (most errors first)
 */
function groupErrorsByFile(diagnostics: Diagnostic[]): Map<string, ErrorSpan[]> {
  const byFile = new Map<string, ErrorSpan[]>()
  for (const span of collectErrorSpans(diagnostics)) {
    const existing = byFile.get(span.file) ?? []
    existing.push(span)
    byFile.set(span.file, existing)
  }
  // Sort by error count descending
  return new Map(
    [...byFile.entries()].sort((a, b) => b[1].length - a[1].length)
  )
}

/**
 * Get a summary of errors by file for the prompt
 */
function summarizeErrorsByFile(diagnostics: Diagnostic[]): string {
  const grouped = groupErrorsByFile(diagnostics)
  const lines: string[] = []
  for (const [file, spans] of grouped) {
    const shortFile = file.split("/").slice(-2).join("/")
    const errorCodes = [...new Set(spans.map(s => s.code).filter(Boolean))].join(", ")
    lines.push(`  • ${shortFile}: ${spans.length} 个错误${errorCodes ? ` (${errorCodes})` : ""}`)
  }
  return lines.join("\n")
}

async function collectErrorSnippets(
  outputDir: string,
  diagnostics: Diagnostic[],
  maxSnippets = 6,
  context = 4,
) {
  const seen = new Set<string>()
  const snippets: string[] = []
  const fileCache = getFileCache()

  for (const span of collectErrorSpans(diagnostics)) {
    const key = `${span.file}:${span.line}`
    if (seen.has(key)) continue
    seen.add(key)
    if (snippets.length >= maxSnippets) break
    const filePath = span.file.startsWith("/") ? span.file : path.join(outputDir, span.file)
    let content: string
    try {
      // 使用文件缓存减少 I/O
      content = await fileCache.readFile(filePath)
    } catch {
      continue
    }
    const lines = content.split("\n")
    const start = Math.max(1, span.line - context)
    const end = Math.min(lines.length, span.line + context)
    const block = lines.slice(start - 1, end)
    const numbered = block.map((line, idx) => `${start + idx}: ${line}`).join("\n")
    const header = `${span.file}:${span.line}${span.code ? ` (${span.code})` : ""}`
    snippets.push(`${header}\n${numbered}`)
  }
  return snippets
}

const SHOW_CHAT = process.env.FIXER_SHOW_CHAT !== "0"
const SHOW_CHAT_FULL = process.env.FIXER_SHOW_CHAT_FULL === "1"
const SHOW_CHAT_SUMMARY = process.env.FIXER_SHOW_CHAT_SUMMARY === "1"
const SHOW_TOOL_OUTPUT = process.env.FIXER_SHOW_TOOL_OUTPUT === "1"
const CORE_LOGS = process.env.FIXER_CORE_LOGS === "1"
const CORE_LOG_LEVEL =
  (process.env.FIXER_CORE_LOG_LEVEL as "DEBUG" | "INFO" | "WARN" | "ERROR" | undefined) ?? "INFO"

function truncateText(text: string, max = 1200) {
  if (text.length <= max) return text
  return text.slice(0, max) + "…"
}

function displayPath(value: string) {
  const rel = path.relative(process.cwd(), value)
  if (rel && !rel.startsWith("..") && rel !== value) return rel
  return path.basename(value)
}

function summarizeToolInput(input: Record<string, any>) {
  if (!input) return ""
  const offset = typeof input.offset === "number" ? input.offset : null
  const limit = typeof input.limit === "number" ? input.limit : null
  const range =
    offset !== null && limit !== null
      ? `行${offset + 1}-${offset + limit}`
      : offset !== null
        ? `行${offset + 1}+`
        : ""
  if (typeof input.path === "string") {
    return range ? `${displayPath(input.path)} ${range}` : displayPath(input.path)
  }
  if (typeof input.filePath === "string") {
    return range ? `${displayPath(input.filePath)} ${range}` : displayPath(input.filePath)
  }
  if (typeof input.file === "string") return displayPath(input.file)
  if (typeof input.pattern === "string") return `pattern=${truncateText(input.pattern, 120)}`
  if (Array.isArray(input.paths)) return `paths=${input.paths.map((p) => displayPath(p)).join(", ")}`
  if (range) return range
  const raw = JSON.stringify(input)
  return raw === "{}" ? "" : `input=${truncateText(raw, 200)}`
}

type SessionSummary = {
  reads: Set<string>
  writes: Set<string>
  edits: Set<string>
  patches: Set<string>
  lists: Set<string>
  greps: Set<string>
  globs: Set<string>
}

function emptySummary(): SessionSummary {
  return {
    reads: new Set<string>(),
    writes: new Set<string>(),
    edits: new Set<string>(),
    patches: new Set<string>(),
    lists: new Set<string>(),
    greps: new Set<string>(),
    globs: new Set<string>(),
  }
}

function recordSummary(summary: SessionSummary, tool: string, input: Record<string, any>) {
  const name = tool.toLowerCase()
  const paths: string[] = []
  if (typeof input.path === "string") paths.push(displayPath(input.path))
  if (typeof input.filePath === "string") paths.push(displayPath(input.filePath))
  if (typeof input.file === "string") paths.push(displayPath(input.file))
  if (Array.isArray(input.paths)) {
    paths.push(...input.paths.filter((p) => typeof p === "string").map((p) => displayPath(p)))
  }
  if (typeof input.pattern === "string") paths.push(truncateText(input.pattern, 120))

  if (name === "read") paths.forEach((p) => summary.reads.add(p))
  else if (name === "write") paths.forEach((p) => summary.writes.add(p))
  else if (name === "edit") paths.forEach((p) => summary.edits.add(p))
  else if (name === "patch" || name === "apply_patch") {
    if (paths.length === 0) summary.patches.add("<patch>")
    else paths.forEach((p) => summary.patches.add(p))
  } else if (name === "list") paths.forEach((p) => summary.lists.add(p))
  else if (name === "grep") paths.forEach((p) => summary.greps.add(p))
  else if (name === "glob") paths.forEach((p) => summary.globs.add(p))
}

function formatSummary(summary: SessionSummary) {
  const formatSet = (set: Set<string>) => {
    if (set.size === 0) return ""
    return Array.from(set).slice(0, 8).join(", ") + (set.size > 8 ? " …" : "")
  }
  const sections: Array<[string, string, Set<string>]> = [
    ["读取", "👀", summary.reads],
    ["写入", "💾", summary.writes],
    ["修改", "🔧", summary.edits],
    ["补丁", "🧩", summary.patches],
    ["列表", "📋", summary.lists],
    ["搜索", "🔍", summary.greps],
    ["匹配", "🧭", summary.globs],
  ]
  const active = sections.filter(([, , set]) => set.size > 0)
  if (active.length === 0) {
    console.log("🤔 本轮没有可展示的工具操作，我会继续处理。")
    return
  }
  const counts = active.map(([label, , set]) => `${label}${set.size}个`)
  console.log(`🔧 本轮完成：${counts.join("，")}`)
  const files = active.flatMap(([, , set]) => Array.from(set))
  const uniqueFiles = Array.from(new Set(files))
  const listing = uniqueFiles.slice(0, 8).join(", ") + (uniqueFiles.length > 8 ? " …" : "")
  if (listing) {
    console.log(`📄 涉及文件：${listing}`)
  }
}

function summaryHasChanges(summary: SessionSummary) {
  return summary.edits.size > 0 || summary.writes.size > 0 || summary.patches.size > 0
}

function summaryHasToolOps(summary: SessionSummary) {
  return (
    summary.reads.size > 0 ||
    summary.writes.size > 0 ||
    summary.edits.size > 0 ||
    summary.patches.size > 0 ||
    summary.lists.size > 0 ||
    summary.greps.size > 0 ||
    summary.globs.size > 0
  )
}

function formatChangedFilesForDisplay(files: string[], outputDir: string) {
  const ignoreTokens = ["/target/", "/.fixer/", "/.opencode/", "/.git/"]
  const ignoreNames = new Set([
    "Cargo.lock",
    "check.log",
    "fmt.log",
    "test.log",
    "opencode.json",
    "CACHEDIR.TAG",
    ".rustc_info.json",
    ".cargo-lock",
  ])
  const filtered = files
    .map((file) => {
      let value = file.replace(/^a\//, "").replace(/^b\//, "")
      value = value.replace(/^"+|"+$/g, "")
      if (value.startsWith(outputDir)) value = value.slice(outputDir.length + 1)
      return value
    })
    .filter((value) => {
      if (ignoreTokens.some((token) => value.includes(token))) return false
      const name = value.split("/").pop() ?? value
      if (ignoreNames.has(name)) return false
      if (name.startsWith("iter-") && name.endsWith(".diff")) return false
      if (name.startsWith("dep-")) return false
      if (name.endsWith(".rmeta") || name.endsWith(".d") || name.endsWith(".json")) return false
      if (name.endsWith(".timestamp")) return false
      return true
    })
    .map((value) => displayPath(value))
  if (filtered.length === 0) return ""
  const shown = filtered.slice(0, 6)
  const suffix = filtered.length > shown.length ? ` 等${filtered.length}个` : ""
  return `${shown.join(", ")}${suffix}`
}

type RootCauseHint = {
  enabled: boolean
  files: string[]
  symbols: string[]
  summary: string
}

function deriveRootCauseHints(diagnostics: Diagnostic[], outputDir: string): RootCauseHint {
  const errorCodes = new Set(["E0308", "E0599"])
  const files = new Set<string>()
  const symbols = new Set<string>()
  let errorCount = 0

  const symbolFromRendered = (text: string) => {
    const matches = [
      text.match(/expected `([^`]+)`, found `([^`]+)`/),
      text.match(/no variant or associated item named `([^`]+)` found for enum `([^`]+)`/),
    ]
    for (const m of matches) {
      if (!m) continue
      if (m[1]) symbols.add(m[1])
      if (m[2]) symbols.add(m[2])
    }
  }

  for (const diag of diagnostics) {
    if (diag.level !== "error") continue
    if (!diag.code || !errorCodes.has(diag.code)) continue
    errorCount += 1
    for (const span of diag.spans ?? []) {
      if (span.file && typeof span.file === "string") {
        files.add(span.file)
      }
    }
    if (diag.message?.rendered) symbolFromRendered(diag.message.rendered)
    else if (diag.message?.message) symbolFromRendered(diag.message.message)
  }

  if (errorCount === 0) {
    return { enabled: false, files: [], symbols: [], summary: "" }
  }

  const distinctFiles = files.size
  const enabled = errorCount >= 6 || distinctFiles >= 3

  if (!enabled) {
    return {
      enabled: false,
      files: Array.from(files),
      symbols: Array.from(symbols),
      summary: "",
    }
  }

  const resolved = Array.from(files)
    .map((f) => {
      if (path.isAbsolute(f)) return f
      return path.join(outputDir, f)
    })
    .filter((f) => {
      try {
        return fsSync.statSync(f).isFile()
      } catch {
        return false
      }
    })
    .map((f) => path.relative(outputDir, f))

  const summary = `Cross-module issues detected (${errorCount} errors across ${distinctFiles} files). Prioritize aligning definitions before fixing call sites.`

  return {
    enabled: true,
    files: resolved,
    symbols: Array.from(symbols).slice(0, 12),
    summary,
  }
}

async function collectRootCauseSnippets(
  outputDir: string,
  relFiles: string[],
  maxLines = 200,
) {
  const snippets: string[] = []
  const fileCache = getFileCache()

  for (const rel of relFiles) {
    const full = path.join(outputDir, rel)
    let content = ""
    try {
      // 使用文件缓存减少 I/O
      content = await fileCache.readFile(full)
    } catch {
      continue
    }
    const lines = content.split("\n").slice(0, maxLines)
    const numbered = lines.map((line, idx) => `${idx + 1}: ${line}`).join("\n")
    snippets.push(`${rel}\n${numbered}`)
  }
  return snippets
}

async function collectSessionSummaryRange(sessionID: string, startIndex = 0) {
  const messages = (await Core.Session.messages({ sessionID })) as Array<{
    info: { role: string }
    parts: Array<any>
  }>
  const summary = emptySummary()
  for (const msg of messages.slice(startIndex)) {
    if (msg.info.role !== "assistant") continue
    const toolParts = msg.parts.filter((part) => part.type === "tool")
    for (const tool of toolParts) {
      const state = tool.state ?? {}
      recordSummary(summary, tool.tool ?? "unknown", state.input ?? {})
    }
  }
  return { summary, messageCount: messages.length }
}

async function collectSessionSummary(sessionID: string) {
  const result = await collectSessionSummaryRange(sessionID, 0)
  return result.summary
}

function createStreamLogger(
  sessionRef: () => string | null,
  iteration: number,
  shouldLog?: () => boolean,
) {
  let started = false
  const lastToolStatus = new Map<string, string>()
  const textLengths = new Map<string, number>()
  const messageRoles = new Map<string, string>()
  const startedMessages = new Set<string>()
  const printedToolOutput = new Set<string>()
  let assistantReady = false
  let sawAssistantText = false
  const toolCounts = new Map<string, { label: string; detail: string; count: number }>()

  const ensureHeader = () => {
    if (started) return
    started = true
    console.log(`\n=== 修复进度（第 ${iteration} 轮） ===`)
  }

  const statusLabel = (status: string) => {
    switch (status) {
      case "pending":
        return "准备"
      case "running":
        return "进行中"
      case "completed":
        return "完成"
      case "error":
        return "失败"
      default:
        return "未知"
    }
  }

  const toolLabel = (tool: string) => {
    switch (tool.toLowerCase()) {
      case "read":
        return "读取"
      case "write":
        return "写入"
      case "edit":
        return "修改"
      case "patch":
      case "apply_patch":
        return "应用补丁"
      case "list":
        return "列出"
      case "grep":
        return "搜索"
      case "glob":
        return "匹配"
      default:
        return tool
    }
  }

  const toolIcon = (tool: string) => {
    switch (tool.toLowerCase()) {
      case "read":
        return "👀 "
      case "write":
        return "💾 "
      case "edit":
        return "🔧 "
      case "patch":
      case "apply_patch":
        return "🧩 "
      case "list":
        return "📋 "
      case "grep":
        return "🔍 "
      case "glob":
        return "🧭 "
      default:
        return ""
    }
  }

  const toolKey = (tool: string, input: Record<string, any>) => {
    const name = tool.toLowerCase()
    const path =
      (typeof input.filePath === "string" && input.filePath) ||
      (typeof input.path === "string" && input.path) ||
      (typeof input.file === "string" && input.file) ||
      ""
    if (name === "read" && path) return `${name}|${path}`
    if ((name === "edit" || name === "write" || name === "apply_patch" || name === "patch") && path)
      return `${name}|${path}`
    if (name === "grep" && typeof input.pattern === "string") return `${name}|${input.pattern}`
    if (name === "glob" && typeof input.pattern === "string") return `${name}|${input.pattern}`
    return ""
  }

  const shouldShowTool = (tool: string, input: Record<string, any>) => {
    const key = toolKey(tool, input)
    const label = toolLabel(tool)
    const detail = summarizeToolInput(input)
    if (!key) return { show: true, label, detail }
    const entry = toolCounts.get(key)
    if (entry) {
      entry.count += 1
      return { show: false, label: entry.label, detail: entry.detail }
    }
    toolCounts.set(key, { label, detail, count: 1 })
    return { show: true, label, detail }
  }

  const writeText = (part: any, delta?: string) => {
    const text = typeof part?.text === "string" ? part.text : ""
    const prev = textLengths.get(part.id) ?? 0
    if (text.length > prev) {
      sawAssistantText = true
      ensureHeader()
      if (!startedMessages.has(part.messageID)) {
        startedMessages.add(part.messageID)
        process.stdout.write("\n助手：")
      }
      process.stdout.write(text.slice(prev))
      textLengths.set(part.id, text.length)
      return
    }
    if (delta) {
      sawAssistantText = true
      ensureHeader()
      if (!startedMessages.has(part.messageID)) {
        startedMessages.add(part.messageID)
        process.stdout.write("\n助手：")
      }
      process.stdout.write(delta)
    }
  }

  const assistantAnnounced = new Set<string>()

  const writeTool = (part: any) => {
    const status = part?.state?.status ?? "unknown"
    const key = part?.callID ?? part?.id ?? "unknown"
    if (lastToolStatus.get(key) === status) return
    lastToolStatus.set(key, status)
    ensureHeader()
    if (status === "pending" || status === "running") return
    const input = part?.state?.input ?? {}
    const tool = part?.tool ?? "工具"
    const { show, label, detail } = shouldShowTool(tool, input)
    if (!show && status !== "error") return
    const icon = toolIcon(tool)
    const messageID = part?.messageID ?? "unknown"
    if (!startedMessages.has(messageID) && !assistantAnnounced.has(messageID)) {
      assistantAnnounced.add(messageID)
      const line = detail ? `${label} ${detail}` : label
      process.stdout.write(`\n助手：正在执行 ${line}\n`)
    }

    // 显示操作状态
    const statusIcon = status === "completed" ? "✅" : status === "error" ? "❌" : "⏳"
    process.stdout.write(
      `\n${statusIcon} ${icon}${label}${detail ? " " + detail : ""}\n`,
    )

    // 对于 Edit 操作，显示更多细节
    if (tool.toLowerCase() === "edit" && !printedToolOutput.has(key)) {
      printedToolOutput.add(key)
      const oldStr = input.old_string ?? input.oldString ?? ""
      const newStr = input.new_string ?? input.newString ?? ""
      if (status === "error") {
        const output = part?.state?.output
        // 尝试提取真正的错误信息
        let errorMsg = "未知错误"
        if (typeof output === "string") {
          errorMsg = output
        } else if (output?.error) {
          errorMsg = typeof output.error === "string" ? output.error : JSON.stringify(output.error)
        } else if (output?.message) {
          errorMsg = output.message
        } else if (output?.output) {
          errorMsg = output.output
        } else if (output) {
          // 打印原始输出以便调试
          errorMsg = JSON.stringify(output).slice(0, 300)
        }
        // 常见错误翻译
        if (errorMsg.includes("not found") || errorMsg.includes("Could not find")) {
          errorMsg = "找不到要替换的文本"
        } else if (errorMsg.includes("not unique")) {
          errorMsg = "匹配到多处文本，请提供更多上下文"
        }
        console.log(`   错误: ${truncateText(errorMsg, 300)}`)
        if (oldStr) {
          console.log(`   尝试匹配: "${truncateText(oldStr, 80)}"`)
        }
      } else if (status === "completed" && oldStr && newStr) {
        // 简要显示修改内容
        const oldPreview = oldStr.split("\n")[0]
        const newPreview = newStr.split("\n")[0]
        if (oldPreview !== newPreview) {
          console.log(`   - ${truncateText(oldPreview, 60)}`)
          console.log(`   + ${truncateText(newPreview, 60)}`)
        }
      }
    }

    // 显示其他工具的输出（如果有错误或重要信息）
    if (status === "completed" && !printedToolOutput.has(key + "_output")) {
      const output = part?.state?.output
      const outputText =
        typeof output === "string"
          ? output
          : typeof output?.output === "string"
            ? output.output
            : ""
      if (outputText && outputText.includes("LSP errors detected")) {
        printedToolOutput.add(key + "_output")
        process.stdout.write(`\n${outputText.trim()}\n`)
      }
    }
  }

  const writePatch = (part: any) => {
    const files = Array.isArray(part?.files) ? part.files.join(", ") : ""
    ensureHeader()
    if (!sawAssistantText) {
      process.stdout.write("\n助手：正在应用补丁\n")
      sawAssistantText = true
    }
    process.stdout.write(`\n操作：应用补丁 ${files}\n`)
  }

  const onEvent = (event: { type: string; properties: Record<string, any> }) => {
    if (shouldLog && !shouldLog()) return
    if (event.type === "session.compacted") return
    if (event.type === "message.updated") {
      const info = event.properties?.info
      if (info?.id && info?.role) {
        messageRoles.set(info.id, info.role)
        if (info.role === "assistant") assistantReady = true
      }
      return
    }
    if (event.type !== "message.part.updated") return
    const part = event.properties?.part
    const sessionID = sessionRef()
    if (!part || !sessionID || part.sessionID !== sessionID) return
    if (!assistantReady) return
    if (messageRoles.get(part.messageID) !== "assistant") return
    if (part.type === "text") {
      writeText(part, event.properties?.delta)
    } else if (part.type === "tool") {
      writeTool(part)
    } else if (part.type === "patch") {
      writePatch(part)
    }
  }

  const finish = () => {
    if (started) process.stdout.write("\n")
  }

  return { onEvent, finish, getHasText: () => sawAssistantText }
}

async function printSessionTranscript(sessionID: string, iteration: number, userPrompt: string) {
  const messages = (await Core.Session.messages({ sessionID })) as Array<{
    info: { role: string }
    parts: Array<any>
  }>

  const summary = emptySummary()

  console.log(`\n=== FIXER CHAT (iteration ${iteration}) ===`)
  console.log(`PROMPT:\n${truncateText(userPrompt, 1200)}\n`)

  for (const msg of messages) {
    if (msg.info.role !== "assistant") continue
    const textParts = msg.parts.filter((part) => part.type === "text").map((part) => part.text)
    if (textParts.length > 0) {
      console.log(`ASSISTANT:\n${truncateText(textParts.join("\n"), 2000)}\n`)
    }
    const toolParts = msg.parts.filter((part) => part.type === "tool")
    for (const tool of toolParts) {
      const state = tool.state ?? {}
      const input = summarizeToolInput(state.input ?? {})
      recordSummary(summary, tool.tool ?? "unknown", state.input ?? {})
      const status = state.status ?? "unknown"
      console.log(`TOOL ${tool.tool} (${status}) ${input}`)
      if (SHOW_TOOL_OUTPUT && typeof state.output === "string") {
        console.log(`OUTPUT:\n${truncateText(state.output, 800)}\n`)
      }
    }
  }
  formatSummary(summary)
}

async function resolveCargoWorkspaceRoot(root: string) {
  const resolved = path.resolve(root)
  try {
    const stat = await fs.stat(resolved)
    if (!stat.isDirectory()) {
      throw new Error(`Workspace path is not a directory: ${resolved}`)
    }
  } catch (error: any) {
    if (error?.code === "ENOENT") {
      throw new Error(`Workspace directory not found: ${resolved}`)
    }
    throw error
  }

  const manifest = path.join(resolved, "Cargo.toml")
  try {
    const stat = await fs.stat(manifest)
    if (stat.isFile()) return resolved
  } catch { }

  const entries = await fs.readdir(resolved, { withFileTypes: true })
  const candidates: string[] = []
  for (const entry of entries) {
    if (!entry.isDirectory()) continue
    if (entry.name === ".git" || entry.name === "target" || entry.name === ".fixer") continue
    const candidate = path.join(resolved, entry.name, "Cargo.toml")
    try {
      const stat = await fs.stat(candidate)
      if (stat.isFile()) {
        candidates.push(path.dirname(candidate))
      }
    } catch { }
  }
  if (candidates.length === 1) return candidates[0]
  return resolved
}

async function writeLog(logsDir: string, name: string, content: string, maxBytes?: number) {
  await fs.mkdir(logsDir, { recursive: true })
  let output = content
  if (maxBytes && Buffer.byteLength(output, "utf8") > maxBytes) {
    output = output.slice(0, maxBytes) + "\n\n(log truncated)"
  }
  await fs.writeFile(path.join(logsDir, name), output)
}

async function writePatch(patchesDir: string, name: string, diff: string, patchList: string[]) {
  await fs.mkdir(patchesDir, { recursive: true })
  const filename = `${name}.diff`
  const full = path.join(patchesDir, filename)
  await fs.writeFile(full, diff)
  patchList.push(full)
}

async function runCargo(
  outputDir: string,
  args: string[],
  logsDir: string,
  kind: Diagnostic["kind"],
  policy: CommandPolicy,
  maxLogBytes?: number,
  jsonOutput?: boolean,
) {
  if (SHOW_CHAT) {
    console.log(`⏳ 正在运行 cargo ${args.join(" ")}`)
  }
  const command = ["cargo", ...args]
  if (jsonOutput && !command.includes("--message-format=json")) {
    command.push("--message-format=json")
  }
  if (command[1] === "clippy" && !command.includes("--allow-no-vcs")) {
    command.push("--allow-no-vcs")
  }
  assertAllowed(command, policy)
  const result = await runCommand(command, {
    cwd: outputDir,
    timeoutMs: 15 * 60 * 1000,
    maxOutputBytes: 2_000_000,
  })
  if (SHOW_CHAT) {
    const icon = result.code === 0 ? "✅" : "❌"
    console.log(`${icon} cargo ${args.join(" ")} 结束（exit=${result.code}）`)
  }
  const combined = result.stdout + "\n" + result.stderr
  await writeLog(logsDir, `${kind}.log`, combined, maxLogBytes)
  const json = parseCargoMessages(result.stdout)
  return {
    kind,
    code: result.code,
    stdout: result.stdout,
    stderr: result.stderr,
    json,
  } satisfies Diagnostic
}

async function fixReservedConstIdentifiers(root: string) {
  const entries = await fs.readdir(root, { withFileTypes: true })
  for (const entry of entries) {
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "target" || entry.name === ".git" || entry.name === ".fixer") continue
      await fixReservedConstIdentifiers(full)
      continue
    }
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue
    const content = await fs.readFile(full, "utf8")
    const updated = content
      .replace(/\bconst\s+true\s*:/g, "const r#true:")
      .replace(/\bconst\s+false\s*:/g, "const r#false:")
    if (updated !== content) {
      await fs.writeFile(full, updated)
    }
  }
}

async function fixPreallocatedBorrow(root: string) {
  const entries = await fs.readdir(root, { withFileTypes: true })
  for (const entry of entries) {
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "target" || entry.name === ".git" || entry.name === ".fixer") continue
      await fixPreallocatedBorrow(full)
      continue
    }
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue
    const content = await fs.readFile(full, "utf8")
    const lines = content.split("\n")
    let changed = false
    const out: string[] = []
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i]
      const match = line.match(
        /(.*)cJSON_PrintPreallocated\([^,]+,\s*&mut\s+(\w+),\s*\2\.len\(\)\s*as\s*i32/,
      )
      if (match) {
        const indent = match[1]
        const name = match[2]
        const prev = out[out.length - 1] ?? ""
        if (!prev.includes(`let ${name}_len`)) {
          out.push(`${indent}let ${name}_len = ${name}.len() as i32;`)
        }
        out.push(line.replace(`${name}.len() as i32`, `${name}_len`))
        changed = true
        continue
      }
      out.push(line)
    }
    if (changed) {
      await fs.writeFile(full, out.join("\n"))
    }
  }
}

async function fixTestFile(root: string) {
  const testPath = path.join(root, "src", "test.rs")
  const exists = await fs
    .stat(testPath)
    .then((s) => s.isFile())
    .catch(() => false)
  if (!exists) return

  const content = await fs.readFile(testPath, "utf8")
  const lines = content.split("\n")
  const out: string[] = []
  let changed = false
  let seenProcessUse = false
  let inPreallocIf = false
  let preallocIndent = ""

  for (let i = 0; i < lines.length; i++) {
    let line = lines[i]

    if (line.trim() === "use std::process;") {
      if (seenProcessUse) {
        changed = true
        continue
      }
      seenProcessUse = true
    }

    if (line.trim().startsWith("if let buf_fail_len =")) {
      line = line.replace("if let buf_fail_len =", "let buf_fail_len =")
      changed = true
    }

    if (line.trim().startsWith("if cJSON_PrintPreallocated(") && !line.includes("{")) {
      preallocIndent = line.match(/^\s*/)?.[0] ?? ""
      line = `${line} {`
      inPreallocIf = true
      out.push(line)
      changed = true
      continue
    }

    // Fix missing length arg in cJSON_PrintPreallocated
    const callMatch = line.match(/(.*)cJSON_PrintPreallocated\(([^,]+),\s*&mut\s+(\w+),\s*true\)/)
    if (callMatch) {
      const indent = callMatch[1]
      const arg1 = callMatch[2]
      const name = callMatch[3]
      const prev = out[out.length - 1] ?? ""
      if (!prev.includes(`let ${name}_len`)) {
        out.push(`${indent}let ${name}_len = ${name}.len() as i32;`)
      }
      line = `${indent}cJSON_PrintPreallocated(${arg1}, &mut ${name}, ${name}_len, true)`
      out.push(line)
      changed = true
      continue
    }

    // Fix borrow in existing calls with inline len()
    const inlineLen = line.match(/(.*)cJSON_PrintPreallocated\(([^,]+),\s*&mut\s+(\w+),\s*\3\.len\(\)\s*as\s*i32,\s*true\)/)
    if (inlineLen) {
      const indent = inlineLen[1]
      const arg1 = inlineLen[2]
      const name = inlineLen[3]
      const prev = out[out.length - 1] ?? ""
      if (!prev.includes(`let ${name}_len`)) {
        out.push(`${indent}let ${name}_len = ${name}.len() as i32;`)
      }
      line = `${indent}cJSON_PrintPreallocated(${arg1}, &mut ${name}, ${name}_len, true)`
      out.push(line)
      changed = true
      continue
    }

    // Replace Box::new(cJSON_CreateObject/Array()) with direct call
    line = line
      .replace(/Box::new\(cJSON_CreateObject\(\)\)/g, "cJSON_CreateObject()")
      .replace(/Box::new\(cJSON_CreateArray\(\)\)/g, "cJSON_CreateArray()")
      .replace(/Box::new\(cJSON_CreateStringArray\(&strings,\s*\d+\)\)/g, "cJSON_CreateStringArray(&strings).unwrap()")

    // Fix cJSON_CreateIntArray call shape
    line = line.replace(
      /cJSON_CreateIntArray\(&numbers\[i as usize\],\s*\d+\)/g,
      "cJSON_CreateIntArray(&numbers[i as usize..i as usize + 3]).unwrap()",
    )
    line = line.replace(
      /cJSON_CreateIntArray\(&numbers\[i as usize\.\.i as usize \+ 3\]\)\.unwrap\(\)/g,
      "cJSON_CreateIntArray(&numbers[i as usize]).unwrap()",
    )

    out.push(line)
    if (line !== lines[i]) changed = true

    if (inPreallocIf && line.trim() === "return -1;") {
      const nextNonEmpty = lines.slice(i + 1).find((l) => l.trim().length > 0)
      if (!nextNonEmpty || !nextNonEmpty.trim().startsWith("}")) {
        out.push(`${preallocIndent}}`)
      }
      inPreallocIf = false
      preallocIndent = ""
      changed = true
    }
  }

  if (changed) {
    await fs.writeFile(testPath, out.join("\n"))
  }
}

async function fixCjsonPrevLink(root: string) {
  const entries = await fs.readdir(root, { withFileTypes: true })
  for (const entry of entries) {
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "target" || entry.name === ".git" || entry.name === ".fixer") continue
      await fixCjsonPrevLink(full)
      continue
    }
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue
    const content = await fs.readFile(full, "utf8")
    if (!content.includes("current.prev = Some")) continue
    const lines = content.split("\n")
    const out: string[] = []
    let changed = false
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i]
      const next = lines[i + 1]
      const next2 = lines[i + 2]
      if (
        line.includes("if let Some") &&
        line.includes("first.child") &&
        next?.includes("current.prev = Some(") &&
        next2?.trim() === "}"
      ) {
        changed = true
        i += 2
        continue
      }
      out.push(line)
    }
    if (changed) {
      await fs.writeFile(full, out.join("\n"))
    }
  }
}

async function fixCjsonObjectBorrow(root: string) {
  const entries = await fs.readdir(root, { withFileTypes: true })
  const detachMarker = "pub fn cJSON_DetachItemFromObject("
  const detachCaseMarker = "pub fn cJSON_DetachItemFromObjectCaseSensitive("
  const replaceMarker = "fn replace_item_in_object("
  const detachReplacement = [
    "pub fn cJSON_DetachItemFromObject(object: &mut cJSON, string: &str) -> Option<Box<cJSON>> {",
    "    let item_ptr = {",
    "        let item = get_object_item(object, string, false)?;",
    "        item as *const cJSON",
    "    };",
    "    unsafe { cJSON_DetachItemViaPointer(object, &*item_ptr) }",
    "}",
  ].join("\n")
  const detachCaseReplacement = [
    "pub fn cJSON_DetachItemFromObjectCaseSensitive(",
    "    object: &mut cJSON,",
    "    string: &str,",
    ") -> Option<Box<cJSON>> {",
    "    let item_ptr = {",
    "        let item = get_object_item(object, string, true)?;",
    "        item as *const cJSON",
    "    };",
    "    unsafe { cJSON_DetachItemViaPointer(object, &*item_ptr) }",
    "}",
  ].join("\n")
  const replaceReplacement = [
    "fn replace_item_in_object(",
    "    object: &mut cJSON,",
    "    string: &str,",
    "    replacement: Box<cJSON>,",
    "    case_sensitive: bool,",
    ") -> cJSON_bool {",
    "    let item_ptr = {",
    "        let item = get_object_item(object, string, case_sensitive);",
    "        if item.is_none() {",
    "            return 0;",
    "        }",
    "        item.unwrap() as *const cJSON",
    "    };",
    "    let mut new_item = replacement;",
    "    new_item.string = Some(string.to_string());",
    "    unsafe { cJSON_ReplaceItemViaPointer(object, &*item_ptr, new_item) }",
    "}",
  ].join("\n")

  const replaceFunction = (content: string, marker: string, replacement: string) => {
    const start = content.indexOf(marker)
    if (start === -1) return content
    const braceStart = content.indexOf("{", start)
    if (braceStart === -1) return content
    let depth = 0
    let end = -1
    for (let i = braceStart; i < content.length; i++) {
      const ch = content[i]
      if (ch === "{") depth += 1
      else if (ch === "}") {
        depth -= 1
        if (depth === 0) {
          end = i + 1
          break
        }
      }
    }
    if (end === -1) return content
    return `${content.slice(0, start)}${replacement}${content.slice(end)}`
  }

  for (const entry of entries) {
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "target" || entry.name === ".git" || entry.name === ".fixer") continue
      await fixCjsonObjectBorrow(full)
      continue
    }
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue
    const content = await fs.readFile(full, "utf8")
    let updated = content
    updated = replaceFunction(updated, detachMarker, detachReplacement)
    updated = replaceFunction(updated, detachCaseMarker, detachCaseReplacement)
    updated = replaceFunction(updated, replaceMarker, replaceReplacement)
    if (updated !== content) {
      await fs.writeFile(full, updated)
    }
  }
}

async function fixCjsonLifetimes(root: string) {
  const entries = await fs.readdir(root, { withFileTypes: true })
  const getObjectSig =
    /fn get_object_item\(object: &cJSON, name: &str, case_sensitive: bool\) -> Option<&cJSON>/g
  const getObjectReplacement =
    "fn get_object_item<'a>(object: &'a cJSON, name: &str, case_sensitive: bool) -> Option<&'a cJSON>"
  const getItemSig =
    /pub fn cJSON_GetObjectItem\(object: &cJSON, string: &str\) -> Option<&cJSON>/g
  const getItemReplacement =
    "pub fn cJSON_GetObjectItem<'a>(object: &'a cJSON, string: &str) -> Option<&'a cJSON>"
  const getItemCaseSig =
    /pub fn cJSON_GetObjectItemCaseSensitive\(object: &cJSON, string: &str\) -> Option<&cJSON>/g
  const getItemCaseReplacement =
    "pub fn cJSON_GetObjectItemCaseSensitive<'a>(object: &'a cJSON, string: &str) -> Option<&'a cJSON>"

  for (const entry of entries) {
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "target" || entry.name === ".git" || entry.name === ".fixer") continue
      await fixCjsonLifetimes(full)
      continue
    }
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue
    const content = await fs.readFile(full, "utf8")
    let updated = content
    updated = updated.replace(getObjectSig, getObjectReplacement)
    updated = updated.replace(getItemSig, getItemReplacement)
    updated = updated.replace(getItemCaseSig, getItemCaseReplacement)
    if (updated !== content) {
      await fs.writeFile(full, updated)
    }
  }
}

async function fixCjsonUnsafeAlloc(root: string) {
  const entries = await fs.readdir(root, { withFileTypes: true })
  const mallocPattern = new RegExp(
    String.raw`fn default_malloc\(size: usize\) -> \*mut u8\s*\{[\s\S]*?\n\}`,
    "g",
  )
  const freePattern = new RegExp(
    String.raw`fn default_free\(ptr: \*mut u8\)\s*\{[\s\S]*?\n\}`,
    "g",
  )
  const reallocPattern = new RegExp(
    String.raw`fn default_realloc\(ptr: \*mut u8, size: usize\) -> \*mut u8\s*\{[\s\S]*?\n\}`,
    "g",
  )
  const mallocReplacement = [
    "fn default_malloc(size: usize) -> *mut u8 {",
    "    unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(size, 1).unwrap()) }",
    "}",
  ].join("\n")
  const freeReplacement = [
    "fn default_free(ptr: *mut u8) {",
    "    unsafe { std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align(1, 1).unwrap()) };",
    "}",
  ].join("\n")
  const reallocReplacement = [
    "fn default_realloc(ptr: *mut u8, size: usize) -> *mut u8 {",
    "    unsafe {",
    "        std::alloc::realloc(",
    "            ptr,",
    "            std::alloc::Layout::from_size_align(1, 1).unwrap(),",
    "            size,",
    "        )",
    "    }",
    "}",
  ].join("\n")

  for (const entry of entries) {
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "target" || entry.name === ".git" || entry.name === ".fixer") continue
      await fixCjsonUnsafeAlloc(full)
      continue
    }
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue
    const content = await fs.readFile(full, "utf8")
    let updated = content
    updated = updated.replace(mallocPattern, mallocReplacement)
    updated = updated.replace(freePattern, freeReplacement)
    updated = updated.replace(reallocPattern, reallocReplacement)
    if (updated !== content) {
      await fs.writeFile(full, updated)
    }
  }
}

async function fixCjsonDetachBorrow(root: string) {
  const entries = await fs.readdir(root, { withFileTypes: true })
  const detachReplacement = [
    "pub fn cJSON_DetachItemViaPointer(parent: &mut cJSON, item: &cJSON) -> Option<Box<cJSON>> {",
    "    if parent.child.is_none() {",
    "        return None;",
    "    }",
    "",
    "    let mut current = &mut parent.child;",
    "    while current.is_some() {",
    "        let is_target = {",
    "            let c = current.as_ref().unwrap();",
    "            std::ptr::eq(c.as_ref(), item)",
    "        };",
    "        if is_target {",
    "            let mut detached = current.take();",
    "            if let Some(ref mut d) = detached {",
    "                if let Some(next) = d.next.take() {",
    "                    *current = Some(next);",
    "                }",
    "            }",
    "            return detached;",
    "        }",
    "        current = &mut current.as_mut().unwrap().next;",
    "    }",
    "",
    "    None",
    "}",
  ].join("\n")

  for (const entry of entries) {
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "target" || entry.name === ".git" || entry.name === ".fixer") continue
      await fixCjsonDetachBorrow(full)
      continue
    }
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue
    const content = await fs.readFile(full, "utf8")
    const marker = "pub fn cJSON_DetachItemViaPointer"
    const start = content.indexOf(marker)
    if (start === -1) continue
    const braceStart = content.indexOf("{", start)
    if (braceStart === -1) continue
    let depth = 0
    let end = -1
    for (let i = braceStart; i < content.length; i++) {
      const ch = content[i]
      if (ch === "{") depth += 1
      else if (ch === "}") {
        depth -= 1
        if (depth === 0) {
          end = i + 1
          break
        }
      }
    }
    if (end === -1) continue
    const before = content.slice(0, start)
    const after = content.slice(end)
    const updated = `${before}${detachReplacement}${after}`
    if (updated !== content) await fs.writeFile(full, updated)
  }
}

async function fixCjsonHookCalls(root: string) {
  const entries = await fs.readdir(root, { withFileTypes: true })
  const mallocCall = /GLOBAL_HOOKS\.lock\(\)\.unwrap\(\)\.allocate\(([^)]+)\)/g
  const freeCall = /GLOBAL_HOOKS\.lock\(\)\.unwrap\(\)\.deallocate\(([^)]+)\)/g
  const reallocCall = /GLOBAL_HOOKS\.lock\(\)\.unwrap\(\)\.reallocate\(([^)]+)\)/g

  for (const entry of entries) {
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      if (entry.name === "target" || entry.name === ".git" || entry.name === ".fixer") continue
      await fixCjsonHookCalls(full)
      continue
    }
    if (!entry.isFile() || !entry.name.endsWith(".rs")) continue
    const content = await fs.readFile(full, "utf8")
    let updated = content
    updated = updated.replace(mallocCall, "(GLOBAL_HOOKS.lock().unwrap().allocate)($1)")
    updated = updated.replace(freeCall, "(GLOBAL_HOOKS.lock().unwrap().deallocate)($1)")
    updated = updated.replace(reallocCall, "(GLOBAL_HOOKS.lock().unwrap().reallocate)($1)")
    if (updated !== content) {
      await fs.writeFile(full, updated)
    }
  }
}

function parseCargoMessages(stdout: string) {
  const messages: CargoMessage[] = []
  for (const line of stdout.split("\n")) {
    if (!line.trim().startsWith("{")) continue
    try {
      const parsed = JSON.parse(line)
      messages.push(parsed)
    } catch {
      // ignore non-JSON lines
    }
  }
  return messages
}

async function getWorkspaceOrder(outputDir: string, policy: CommandPolicy) {
  const command = ["cargo", "metadata", "--format-version=1", "--no-deps"]
  assertAllowed(command, policy)
  const result = await runCommand(command, {
    cwd: outputDir,
    timeoutMs: 60_000,
    maxOutputBytes: 2_000_000,
  })
  if (result.code !== 0) return []
  try {
    const data = JSON.parse(result.stdout)
    const members = new Set<string>(data.workspace_members ?? [])
    const packages = (data.packages ?? []).filter((p: any) => members.has(p.id))
    const graph = new Map<string, string[]>()
    for (const pkg of packages) {
      const deps = (pkg.dependencies ?? [])
        .map((d: any) => d.name)
        .filter((name: string) => packages.some((p: any) => p.name === name))
      graph.set(pkg.name, deps)
    }
    const ordered: string[] = []
    const temp = new Set<string>()
    const perm = new Set<string>()

    const visit = (name: string) => {
      if (perm.has(name) || temp.has(name)) return
      temp.add(name)
      for (const dep of graph.get(name) ?? []) {
        visit(dep)
      }
      temp.delete(name)
      perm.add(name)
      ordered.push(name)
    }

    for (const name of graph.keys()) {
      visit(name)
    }
    return ordered
  } catch {
    return []
  }
}

async function runWorkspaceChecks(
  outputDir: string,
  policy: CommandPolicy,
  logsDir: string,
  maxLogBytes?: number,
) {
  const order = await getWorkspaceOrder(outputDir, policy)
  if (order.length <= 1) return []

  // 并行执行所有 crate 的 check
  const results = await Promise.all(
    order.map((name) =>
      runCargo(
        outputDir,
        ["check", "--all-targets", "-p", name],
        logsDir,
        "check",
        policy,
        maxLogBytes,
        true,
      ),
    ),
  )
  return results
}

async function runTestSuite(
  outputDir: string,
  policy: CommandPolicy,
  logsDir: string,
  maxLogBytes: number | undefined,
  testCases: string[],
  ui?: RepairUI,
) {
  const normalized = testCases.map((item) => item.trim()).filter(Boolean)
  const suites = normalized.length > 0 ? normalized : [""]
  const results: Diagnostic[] = []

  for (const testCase of suites) {
    const args = testCase ? ["test", testCase] : ["test"]
    if (SHOW_CHAT && ui) {
      ui.printCargoCommand(args.join(" "), "running")
    }
    const result = await runCargo(outputDir, args, logsDir, "test", policy, maxLogBytes, true)
    if (SHOW_CHAT && ui) {
      ui.printCargoCommand(args.join(" "), result.code === 0 ? "success" : "error")
    }
    results.push(result)
    if (result.code !== 0) break
  }

  return results
}

async function llmFixStep(input: {
  outputDir: string
  sourceDir?: string
  diagnostics: Diagnostic[]
  logsDir: string
  iteration: number
  policy: CommandPolicy
  lastIterationNoChange?: boolean
  sessionID?: string | null
  lastChangedFiles?: string[]
  lastDiffSnippet?: string | null
  lastAgentError?: string | null
  coreManager?: CoreManager // 新增：Core 实例管理器
  coreConfig?: {
    workspaceDir: string
    config: Record<string, unknown>
    permission: Array<{ permission: string; action: string; pattern: string }>
    printLogs?: boolean
    logLevel?: "DEBUG" | "INFO" | "WARN" | "ERROR"
    onEvent?: (event: { type: string; properties: Record<string, any> }) => void
  }
  ui?: RepairUI // 新增：UI 实例
}) {
  const fixed = await loadFixedProviderConfig()
  const modelRef = (() => {
    const raw = typeof fixed.config?.model === "string" ? fixed.config.model : undefined
    if (!raw || !raw.includes("/")) return undefined
    const [providerID, ...rest] = raw.split("/")
    const modelID = rest.join("/")
    if (!providerID || !modelID) return undefined
    return { providerID, modelID }
  })()
  const fallbackModelIDs = fixed.fallbackModels?.length ? fixed.fallbackModels : []
  const fallbackModels = modelRef
    ? fallbackModelIDs.map((modelID) => ({ providerID: modelRef.providerID, modelID }))
    : []

  // 计算错误统计信息
  const totalErrors = countTotalErrors(input.diagnostics)
  const failedTestRuns = countFailedTestRuns(input.diagnostics)
  const testFailureSummary = summarizeTestFailures(input.diagnostics)
  const snippets = await collectErrorSnippets(input.outputDir, input.diagnostics, 10)

  // 显示 LLM 开始工作
  if (SHOW_CHAT && input.ui) {
    input.ui.printLLMThinking()
  }

  const system = [
    "你是 Rust 编译错误修复专家。你的目标是高效修复多个错误。",
    "",
    "工作流程：",
    "1. 查看错误列表，找出错误最多的文件",
    "2. 优先修复同一文件中的多个错误（效率更高）",
    "3. 用 Read 读取文件，然后用多次 Edit 修复该文件中的所有错误",
    "4. 完成一个文件后，继续处理下一个文件",
    "5. 重复直到没有你能修复的错误",
    "6. 如果编译通过但测试失败，必须根据测试失败信息继续修复，直到测试通过",
    "",
    "重要规则：",
    "- 每次 Edit 前必须先 Read 文件",
    "- 如果 Edit 失败，重新 Read 文件确认内容后再试",
    "- 同类型错误（如所有 E0308）可以批量修复",
    "- 完成后简要说明你修复了多少个错误",
    "",
    "禁止事项（非常重要）：",
    "- 禁止使用 sed、awk、perl 等命令修改文件",
    "- 禁止使用 echo/cat 重定向修改文件",
    "- 只能使用 Edit 工具修改文件",
    "- bash 只能用于 cargo build、grep 搜索等只读操作",
    "",
    "当且仅当编译错误为 0 且测试失败数也为 0 时，说\"修复完成\"并停止。",
  ].join("\n")

  // Generate error summary by file
  const errorsByFileSummary = summarizeErrorsByFile(input.diagnostics)

  const user = [
    `Workspace: ${input.outputDir}`,
    input.sourceDir ? `C源码参考: ${input.sourceDir}` : undefined,
    "",
    `编译错误: ${totalErrors} 个`,
    `失败测试轮次: ${failedTestRuns} 个`,
    "",
    "按文件分组的错误:",
    errorsByFileSummary,
    "",
    "编译错误详情:",
    summarizeDiagnostics(input.diagnostics),
    testFailureSummary ? "\n测试失败详情:\n" + testFailureSummary : undefined,
    snippets.length > 0 ? "\n错误代码上下文:\n" + snippets.join("\n\n") : undefined,
    "",
    "提示：优先修复错误最多的文件，每个文件可以一次修复多个错误。",
    failedTestRuns > 0 ? "提示：当前必须以测试通过为目标，优先处理失败测试涉及的函数。": undefined,
  ]
    .filter(Boolean)
    .join("\n")

  let activeSessionID: string | null = input.sessionID ?? null
  let streamPaused = false
  const stream = createStreamLogger(() => activeSessionID, input.iteration, () => !streamPaused)
  const doomLoopTracker = getDoomLoopTracker()

  const promptWithRetry = async (
    promptInput: Parameters<typeof Core.SessionPrompt.prompt>[0],
    purpose: string,
  ) => {
    const tryPrompt = async (modelOverride?: { providerID: string; modelID: string }) => {
      // Add a small delay and retry for "No output generated" errors specific to some providers
      let lastError: any
      for (let i = 0; i < 3; i++) {
        try {
          if (i > 0) await new Promise(resolve => setTimeout(resolve, 1000 * i))
          const result = await Core.SessionPrompt.prompt({
            ...promptInput,
            model: modelOverride ?? promptInput.model,
          })
          if (result?.info?.role === "assistant" && result?.info?.error) {
            console.error("Agent error:", result.info.error)
            const err = result.info.error as {
              name?: string
              message?: string
              data?: { message?: string }
            }
            const detail = err?.message ?? err?.data?.message ?? JSON.stringify(err)
            throw new Error(`${err?.name ?? "AgentError"}: ${detail ?? ""}`)
          }
          return result
        } catch (e: any) {
          lastError = e
          if (e.message?.includes("NoOutputGeneratedError") || e.message?.includes("ConnectTimeout")) {
            console.log(`⚠️ Stream error (attempt ${i + 1}/3), retrying...`)
            continue
          }
          throw e
        }
      }
      throw lastError
    }

    // Use withRetry for main model, then try fallbacks
    try {
      return await withRetry(
        () => tryPrompt(promptInput.model),
        {
          maxRetries: 2,
          onRetry: (error, attempt, delay) => {
            console.log(`⚠️ 第 ${attempt} 次重试${purpose}（${delay / 1000}s 后）: ${error instanceof Error ? error.message : error}`)
          },
        }
      )
    } catch (error) {
      if (!isRetryableError(error) || fallbackModels.length === 0) throw error
      for (const fallback of fallbackModels) {
        console.log(`⚠️ 原模型无响应，临时切换到 ${fallback.modelID} 继续${purpose}…`)
        try {
          const result = await tryPrompt(fallback)
          console.log("✅ 临时模型成功，已切回原模型。")
          return result
        } catch (fallbackError) {
          if (!isRetryableError(fallbackError)) throw fallbackError
        }
      }
      throw error
    }
  }

  // 内部执行函数
  const executePromptSession = async () => {
    if (!activeSessionID) {
      const session = await Core.Session.create({ title: `rust-fix` })
      activeSessionID = session.id
    }
    const beforeMessages = await Core.Session.messages({ sessionID: activeSessionID! })
    const beforeCount = Array.isArray(beforeMessages) ? beforeMessages.length : 0
    const result = await promptWithRetry(
      {
        sessionID: activeSessionID!,
        model: modelRef,
        agent: "build",
        system,
        parts: [
          {
            type: "text",
            text: user,
          },
        ],
      },
      "修复",
    )
    const responseText = extractResponseText(result?.parts ?? [])
    let requestedSummary = false
    if (!responseText) requestedSummary = true
    if (requestedSummary) {
      console.log("Requesting summary from agent...")
      streamPaused = true
      const summaryMessage = await promptWithRetry(
        {
          sessionID: activeSessionID!,
          model: modelRef,
          tools: { "*": false },
          parts: [
            {
              type: "text",
              text: "请用1-2句简要总结你刚刚做了哪些修复，以及下一步要做什么。",
            },
          ],
        },
        "生成总结",
      )
      streamPaused = false
      const summaryParts = summaryMessage?.parts ?? []
      const summaryText = extractResponseText(summaryParts)
      if (summaryText) console.log(summaryText)
      else {
        throw new Error(
          `Failed to get summary from agent (parts: ${describePartTypes(summaryParts)})`,
        )
      }
    }
    stream.finish()
    const summaryResult = await collectSessionSummaryRange(activeSessionID!, beforeCount)

    // 显示本轮修复结果摘要
    if (SHOW_CHAT) {
      const editCount = summaryResult.summary.edits.size
      const writeCount = summaryResult.summary.writes.size
      const patchCount = summaryResult.summary.patches.size
      const totalChanges = editCount + writeCount + patchCount
      if (totalChanges > 0) {
        console.log(`\n✅ LLM 本轮完成: ${totalChanges} 个文件修改`)
        if (editCount > 0) console.log(`   🔧 编辑: ${Array.from(summaryResult.summary.edits).join(", ")}`)
        if (writeCount > 0) console.log(`   💾 写入: ${Array.from(summaryResult.summary.writes).join(", ")}`)
      } else {
        console.log(`\n⚠️ LLM 本轮未产生实际修改`)
      }
    }

    if (SHOW_CHAT_FULL) {
      await printSessionTranscript(activeSessionID!, input.iteration, user)
    } else if (SHOW_CHAT_SUMMARY) {
      formatSummary(summaryResult.summary)
      if (!summaryHasToolOps(summaryResult.summary)) {
        console.log("🤔 模型本轮未调用任何工具，因此没有实际修改。")
      }
      if (!summaryHasChanges(summaryResult.summary)) {
        console.log("🤔 本轮没有实际修改，我会继续尝试。")
      }
    }
  }

  // 如果提供了 CoreManager，使用它来执行；否则使用 withCore
  if (input.coreManager && input.coreConfig) {
    await input.coreManager.initialize(input.coreConfig)
    // 使用 configOverride 传入当前迭代的 stream.onEvent
    await input.coreManager.withContext(executePromptSession, {
      onEvent: SHOW_CHAT ? stream.onEvent : undefined,
    })
  } else {
    // 回退到原有的 withCore 方式
    await withCore(
      {
        workspaceDir: input.outputDir,
        config: fixed.config,
        permission: buildPermissionRules(input.outputDir, input.sourceDir, true, input.policy),
        printLogs: CORE_LOGS,
        logLevel: CORE_LOG_LEVEL,
        onEvent: SHOW_CHAT ? stream.onEvent : undefined,
      },
      executePromptSession,
    )
  }

  return activeSessionID
}

export async function repairRustProject(input: RepairInput): Promise<RepairResult> {
  const constraints = input.constraints ?? {}
  const policy: CommandPolicy = {
    allowed: constraints.allowedCommands ?? DEFAULT_ALLOWED_COMMANDS,
  }
  const maxLogBytes = constraints.maxLogBytes
  const maxWorkspaceBytes = constraints.maxWorkspaceBytes
  const maxIterations = constraints.maxIterations ?? 3
  const runTestsWhenCheckPass = constraints.runTestsWhenCheckPass ?? constraints.requireCargoTest ?? true
  const testCases = (constraints.testCases ?? []).map((item) => item.trim()).filter(Boolean)
  const startedAt = Date.now()

  const workspaceDir = await resolveCargoWorkspaceRoot(input.workspaceDir)

  // 提取项目名称用于 UI 显示
  const projectName = path.basename(workspaceDir)

  // 初始化 UI
  const fixed = await loadFixedProviderConfig()
  const ui = createRepairUI({
    projectName,
    model: fixed.model,
    maxIterations,
  })

  // 显示欢迎信息
  if (SHOW_CHAT) {
    ui.printWelcome()
  }

  await ensureEmptyDir(input.outputDir)
  await copyWorkspace(workspaceDir, input.outputDir)

  // 初始化备份管理器
  const backupManager = getBackupManager(input.outputDir)
  await backupManager.init()

  const artifacts = {
    outputDir: input.outputDir,
    logsDir: path.join(input.outputDir, ".fixer", "logs"),
    patchesDir: path.join(input.outputDir, ".fixer", "patches"),
    patches: [] as string[],
  }
  await fs.mkdir(artifacts.logsDir, { recursive: true })
  await fs.mkdir(artifacts.patchesDir, { recursive: true })

  let diagnostics: Diagnostic[] = []
  const metrics = {
    iterations: 0,
    cargoCheckPass: false,
    cargoTestPass: false,
    clippyFixApplied: false,
  }
  let noProgressCount = 0
  let consecutiveApiErrors = 0
  let sessionID: string | null = null
  let lastAgentError: string | null = null
  let lastChangedFiles: string[] = []
  let lastDiffSnippet: string | null = null
  let initialErrorCount = 0

  // 初始化 CoreManager 和配置，以便在多次迭代中复用
  const coreManager = getCoreManager()
  const coreConfig = {
    workspaceDir: input.outputDir,
    config: fixed.config,
    permission: buildPermissionRules(input.outputDir, input.sourceDir, true, policy),
    printLogs: CORE_LOGS,
    logLevel: CORE_LOG_LEVEL,
    // onEvent 在 llmFixStep 中根据当前迭代的 stream logger 动态设置
  }

  // fmt 必须先执行
  if (SHOW_CHAT) ui.printCargoCommand("fmt", "running")
  diagnostics.push(await runCargo(input.outputDir, ["fmt"], artifacts.logsDir!, "fmt", policy, maxLogBytes))
  if (SHOW_CHAT) ui.printCargoCommand("fmt", diagnostics.at(-1)?.code === 0 ? "success" : "error")

  // workspaceChecks 和全局 check 可并行执行
  if (SHOW_CHAT) ui.printCargoCommand("check --all-targets", "running")
  const [wsResults, checkResult] = await Promise.all([
    runWorkspaceChecks(input.outputDir, policy, artifacts.logsDir!, maxLogBytes),
    runCargo(input.outputDir, ["check", "--all-targets"], artifacts.logsDir!, "check", policy, maxLogBytes, true),
  ])
  diagnostics.push(...wsResults, checkResult)
  if (SHOW_CHAT) ui.printCargoCommand("check", checkResult.code === 0 ? "success" : "error")
  if (checkResult.code === 0 && runTestsWhenCheckPass) {
    diagnostics.push(...(await runTestSuite(input.outputDir, policy, artifacts.logsDir!, maxLogBytes, testCases, ui)))
  }

  // 显示初始分析结果
  initialErrorCount = countTotalErrors(diagnostics)
  const initialCheck = getLatestDiagnosticByKind(diagnostics, "check")
  const hasInitialTestFailures = countFailedTestRuns(diagnostics) > 0
  const hasCheckFailureWithoutDiagnostics = initialCheck?.code !== 0 && initialErrorCount === 0
  if (SHOW_CHAT) {
    if (hasCheckFailureWithoutDiagnostics) {
      ui.printInfo("初始检查失败，且未解析到可定位的 Rust 编译错误。")
    } else if (initialErrorCount === 0 && hasInitialTestFailures) {
      ui.printInfo("编译已通过，但测试失败；将根据测试失败信息继续修复。")
    } else {
      ui.printAnalysis(initialErrorCount)
    }
  }
  if (hasCheckFailureWithoutDiagnostics && SHOW_CHAT) {
    const stderr = initialCheck?.stderr ?? ""
    const isLikelyNonRustWorkspace =
      stderr.includes("could not find `Cargo.toml`") || stderr.includes("failed to parse manifest")
    ui.printError(
      "无法开始修复",
      isLikelyNonRustWorkspace
        ? "当前目录不是可构建的 Rust workspace（缺少有效 Cargo.toml），已跳过 LLM 修复循环。"
        : "cargo check 失败，但没有可定位的 Rust 编译诊断，已跳过 LLM 修复循环。",
    )
  }

  if (diagnostics.at(-1)?.code !== 0) {
    // NOTE: 模式化修复暂时禁用
    // C→Rust 翻译的问题是相互关联的（如 struct 前缀和 type 关键字），
    // 单独修复一个会暴露另一个的错误，导致错误数反而增加。
    // LLM 能看到完整上下文，更适合处理这类问题。
  }

  while (!hasCheckFailureWithoutDiagnostics && diagnostics.at(-1)?.code !== 0 && metrics.iterations < maxIterations) {
    if (constraints.timeBudgetMs && Date.now() - startedAt > constraints.timeBudgetMs) break

    // 记录修复前的错误数量
    const errorCountBefore = countTotalErrors(diagnostics)

    // 创建备份点
    await backupManager.createBackup(input.outputDir, metrics.iterations, errorCountBefore)

    metrics.iterations += 1

    // 显示迭代开始
    if (SHOW_CHAT) {
      ui.printIterationStart(metrics.iterations, errorCountBefore)
    }

    try {
      sessionID = await llmFixStep({
        outputDir: input.outputDir,
        sourceDir: input.sourceDir,
        diagnostics,
        logsDir: artifacts.logsDir!,
        iteration: metrics.iterations,
        policy,
        lastIterationNoChange: noProgressCount > 0,
        sessionID,
        lastChangedFiles,
        lastDiffSnippet,
        lastAgentError,
        coreManager, // 传入 CoreManager 以复用实例
        coreConfig,  // 传入配置
        ui,          // 传入 UI 实例
      })
      lastAgentError = null
      consecutiveApiErrors = 0
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err)
      if (SHOW_CHAT) {
        ui.printError("LLM 调用出错", message)
      }
      lastAgentError = message
      consecutiveApiErrors += 1

      // 如果连续 3 次 API 错误（非重试类型），提前退出
      if (consecutiveApiErrors >= 3) {
        if (SHOW_CHAT) {
          ui.printError("连续 API 错误", `已连续 ${consecutiveApiErrors} 次 API 调用失败，停止修复。请检查 API 配置和账户余额。`)
        }
        break
      }
    }

    // 并行执行 workspaceChecks 和全局 check
    if (SHOW_CHAT) ui.printCargoCommand("check --all-targets", "running")
    const [loopWsResults, loopCheckResult] = await Promise.all([
      runWorkspaceChecks(input.outputDir, policy, artifacts.logsDir!, maxLogBytes),
      runCargo(input.outputDir, ["check", "--all-targets"], artifacts.logsDir!, "check", policy, maxLogBytes, true),
    ])
    diagnostics = [...loopWsResults, loopCheckResult]
    if (SHOW_CHAT) ui.printCargoCommand("check", loopCheckResult.code === 0 ? "success" : "error")
    if (loopCheckResult.code === 0 && runTestsWhenCheckPass) {
      diagnostics.push(...(await runTestSuite(input.outputDir, policy, artifacts.logsDir!, maxLogBytes, testCases, ui)))
    }

    // 检测回归：如果错误数量增加，发出警告
    const errorCountAfter = countTotalErrors(diagnostics)
    const iterDiff = await computeDiff(workspaceDir, input.outputDir, policy)
    await writePatch(artifacts.patchesDir!, `iter-${metrics.iterations}`, iterDiff.diff, artifacts.patches)
    lastChangedFiles = iterDiff.changedFiles
    lastDiffSnippet = iterDiff.diff.length > 2000 ? iterDiff.diff.slice(0, 2000) + "\n...(truncated)" : iterDiff.diff

    if (SHOW_CHAT) {
      // 显示迭代结果
      ui.printIterationResult({
        errorsBefore: errorCountBefore,
        errorsAfter: errorCountAfter,
        filesChanged: iterDiff.changedFiles,
        editsApplied: iterDiff.changedFiles.length,
        duration: Date.now() - startedAt,
      })

      // 如果检测到回归，自动回滚到最佳备份
      if (errorCountAfter > errorCountBefore) {
        ui.printRegression(errorCountBefore, errorCountAfter)

        // 找到最佳备份（错误数最少的）
        const bestBackup = backupManager.getBestBackup()
        if (bestBackup && bestBackup.errorCount < errorCountAfter) {
          try {
            await backupManager.restore(bestBackup, input.outputDir)
            ui.printRollback(bestBackup.iteration, bestBackup.errorCount, true)

            // 重新检查，替换 diagnostics 为回滚后的状态
            const [rollbackWsResults, rollbackCheckResult] = await Promise.all([
              runWorkspaceChecks(input.outputDir, policy, artifacts.logsDir!, maxLogBytes),
              runCargo(input.outputDir, ["check", "--all-targets"], artifacts.logsDir!, "check-rollback", policy, maxLogBytes, true),
            ])
            diagnostics = [...rollbackWsResults, rollbackCheckResult]
            if (rollbackCheckResult.code === 0 && runTestsWhenCheckPass) {
              diagnostics.push(...(await runTestSuite(input.outputDir, policy, artifacts.logsDir!, maxLogBytes, testCases, ui)))
            }
          } catch (rollbackError) {
            ui.printRollback(bestBackup.iteration, bestBackup.errorCount, false)
          }
        }
      }
    }

    if (iterDiff.changedFiles.length === 0) {
      noProgressCount += 1
    } else {
      noProgressCount = 0
    }

    if (maxWorkspaceBytes) {
      const size = await dirSizeBytes(input.outputDir, new Set([".fixer"]))
      if (size > maxWorkspaceBytes) {
        throw new Error(`Workspace size exceeded limit: ${size} > ${maxWorkspaceBytes} bytes`)
      }
    }
  }

  const finalCheck = getLatestDiagnosticByKind(diagnostics, "check")
  const finalTest = getLatestDiagnosticByKind(diagnostics, "test")
  metrics.cargoCheckPass = finalCheck?.code === 0
  metrics.cargoTestPass = runTestsWhenCheckPass ? finalTest?.code === 0 : true

  const diffResult = await computeDiff(workspaceDir, input.outputDir, policy)

  const { diff, changedFiles } = diffResult

  const finalStatus = metrics.cargoCheckPass && metrics.cargoTestPass ? "success" : metrics.cargoCheckPass ? "partial" : "failed"

  // 显示最终报告
  if (SHOW_CHAT) {
    const finalErrorCount = countTotalErrors(diagnostics)
    ui.printSummary({
      status: finalStatus,
      iterations: metrics.iterations,
      duration: Date.now() - startedAt,
      errorsBefore: initialErrorCount,
      errorsAfter: finalErrorCount,
      filesChanged: changedFiles,
    })
  }

  // 清理缓存和资源
  resetFileCache()
  resetDiagnosticCache()
  resetDoomLoopTracker()
  await resetCoreManager()
  await resetBackupManager()

  return {
    status: finalStatus,
    summary:
      metrics.cargoCheckPass && metrics.cargoTestPass
        ? "Repair completed; cargo check and tests passed."
        : metrics.cargoCheckPass
          ? "Repair completed; cargo check passed but tests are still failing."
          : "Repair completed but cargo check still failing.",
    diff,
    changedFiles,
    metrics,
    artifacts,
  }
}
