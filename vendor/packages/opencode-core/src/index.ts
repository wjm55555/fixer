import path from "path"
import fs from "fs/promises"
import { Log } from "../../opencode/src/util/log"
import { Instance } from "../../opencode/src/project/instance"
import { InstanceBootstrap } from "../../opencode/src/project/bootstrap"
import { Bus } from "../../opencode/src/bus"
import { Session } from "../../opencode/src/session"
import { SessionPrompt } from "../../opencode/src/session/prompt"
import { Provider } from "../../opencode/src/provider/provider"
import { PermissionNext } from "../../opencode/src/permission/next"
import { Config } from "../../opencode/src/config/config"

export type CoreEvent = {
  type: string
  properties: Record<string, unknown>
}

export type CoreOptions = {
  workspaceDir: string
  dataDir?: string
  config?: Config.Info | Record<string, unknown>
  permission?: PermissionNext.Ruleset
  onEvent?: (event: CoreEvent) => void
  abort?: AbortSignal
  logLevel?: "DEBUG" | "INFO" | "WARN" | "ERROR"
  printLogs?: boolean
}

export type CoreContext = {
  workspaceDir: string
  abort?: AbortSignal
}

function applyCoreEnv(options: CoreOptions) {
  const dataDir = options.dataDir ?? path.join(options.workspaceDir, ".opencode-core")
  process.env.OPENCODE_TEST_HOME = dataDir
  const cfg = options.config as { lsp?: boolean | Record<string, unknown> } | undefined
  if (cfg?.lsp === false) {
    process.env.OPENCODE_DISABLE_LSP_DOWNLOAD = "1"
  }
  process.env.OPENCODE_EXPERIMENTAL_DISABLE_FILEWATCHER = "1"
  process.env.OPENCODE_DISABLE_AUTOCOMPACT = "1"
  process.env.OPENCODE_DISABLE_PRUNE = "1"
  process.env.OPENCODE_DISABLE_MODELS_FETCH = "1"
  process.env.OPENCODE_DISABLE_DEFAULT_PLUGINS = "1"
  process.env.OPENCODE_DISABLE_TITLE = "1"
  process.env.MODELS_DEV_API_JSON =
    process.env.MODELS_DEV_API_JSON ?? path.join(dataDir, "models.dev.json")
  if (options.config) {
    process.env.OPENCODE_CONFIG_CONTENT = JSON.stringify(options.config)
  }
  if (options.permission) {
    process.env.OPENCODE_PERMISSION = JSON.stringify(options.permission)
  }
}

async function ensureModelsDevFile(options: CoreOptions) {
  const dataDir = options.dataDir ?? path.join(options.workspaceDir, ".opencode-core")
  const target = process.env.MODELS_DEV_API_JSON ?? path.join(dataDir, "models.dev.json")
  await fs.mkdir(path.dirname(target), { recursive: true })
  try {
    await fs.stat(target)
  } catch (error: any) {
    if (error?.code !== "ENOENT") throw error
    await fs.writeFile(target, "{}", "utf8")
  }
}

function rulesetToConfigPermission(ruleset: PermissionNext.Ruleset): Config.Permission {
  const actionOnly = new Set([
    "todoread",
    "todowrite",
    "question",
    "webfetch",
    "websearch",
    "codesearch",
    "doom_loop",
  ])
  const result: Record<string, PermissionNext.Action | Record<string, PermissionNext.Action>> = {}
  for (const rule of ruleset) {
    if (actionOnly.has(rule.permission) && rule.pattern === "*") {
      result[rule.permission] = rule.action
      continue
    }
    if (!result[rule.permission] || typeof result[rule.permission] === "string") {
      result[rule.permission] = {}
    }
    ;(result[rule.permission] as Record<string, PermissionNext.Action>)[rule.pattern] = rule.action
  }
  return result as Config.Permission
}

function normalizeConfigPermission(permission?: Config.Permission) {
  const result: Record<string, PermissionNext.Action | Record<string, PermissionNext.Action>> = {}
  if (!permission) return result
  if (typeof permission === "string") {
    result["*"] = { "*": permission }
    return result
  }
  for (const [key, value] of Object.entries(permission)) {
    if (typeof value === "string") {
      result[key] = value
    } else {
      result[key] = { ...value }
    }
  }
  return result
}

async function ensureInlineConfig(options: CoreOptions) {
  if (!options.config && !options.permission) return
  const configDir = path.join(options.workspaceDir, ".opencode")
  const configPath = path.join(configDir, "opencode.json")
  const mergedConfig = {
    ...(options.config ?? {}),
  }
  if (options.permission) {
    const existing = normalizeConfigPermission(
      (mergedConfig as { permission?: Config.Permission }).permission,
    )
    const incoming = rulesetToConfigPermission(options.permission)
    for (const [key, value] of Object.entries(
      incoming as Record<string, PermissionNext.Action | Record<string, PermissionNext.Action>>,
    )) {
      if (typeof value === "string") {
        existing[key] = value
        continue
      }
      if (!existing[key] || typeof existing[key] === "string") existing[key] = {}
      Object.assign(existing[key] as Record<string, PermissionNext.Action>, value)
    }
    ;(mergedConfig as { permission?: Config.Permission }).permission = existing as Config.Permission
  }
  await fs.mkdir(configDir, { recursive: true })
  await fs.writeFile(configPath, JSON.stringify(mergedConfig, null, 2))
}

export async function withCore<T>(options: CoreOptions, fn: (ctx: CoreContext) => Promise<T>): Promise<T> {
  applyCoreEnv(options)
  if (options.abort?.aborted) {
    throw new Error("Aborted before core initialization")
  }
  await ensureModelsDevFile(options)
  await ensureInlineConfig(options)
  const printLogs =
    typeof options.printLogs === "boolean" ? options.printLogs : process.env.OPENCODE_CORE_PRINT_LOGS === "1"
  await Log.init({
    print: printLogs,
    dev: true,
    level: options.logLevel ?? "INFO",
  })
  return await Instance.provide({
    directory: options.workspaceDir,
    init: InstanceBootstrap,
    fn: async () => {
      const unsub = options.onEvent ? Bus.subscribeAll(options.onEvent) : undefined
      try {
        return await fn({ workspaceDir: options.workspaceDir, abort: options.abort })
      } finally {
        if (unsub) unsub()
      }
    },
  })
}

export function bindAbortToSession(sessionID: string, abort?: AbortSignal) {
  if (!abort) return () => {}
  const cancel = () => {
    try {
      SessionPrompt.cancel(sessionID)
    } catch {
      // ignore cancellation errors
    }
  }
  if (abort.aborted) {
    cancel()
    return () => {}
  }
  abort.addEventListener("abort", cancel, { once: true })
  return () => abort.removeEventListener("abort", cancel)
}

export async function promptWithAbort(input: { prompt: SessionPrompt.PromptInput; abort?: AbortSignal }) {
  const cleanup = bindAbortToSession(input.prompt.sessionID, input.abort)
  try {
    return await SessionPrompt.prompt(input.prompt)
  } finally {
    cleanup()
  }
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
