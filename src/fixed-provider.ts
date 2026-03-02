import fs from "fs/promises"
import path from "path"
import { fileURLToPath } from "url"

const ROOT_DIR = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..")
const DEFAULT_PROVIDER_ID = "deepseek"
const DEFAULT_MODEL = "deepseek-chat"

function parseEnvFallbackModels(): string[] {
  const raw = process.env.FIXER_FALLBACK_MODELS
  if (!raw) return ["deepseek-reasoner"]
  const parsed = raw
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean)
  return parsed.length > 0 ? parsed : ["deepseek-reasoner"]
}

// 备用模型列表，当主模型报错时可以切换使用
export const FALLBACK_MODELS = parseEnvFallbackModels()

export type FixedProviderConfig = {
  model: string
  fallbackModels: string[]
  config: Record<string, unknown>
}

type ProviderFileConfig = {
  providerID?: string
  model?: string
  baseURL?: string
  apiKey?: string
  fallbackModels?: string[]
}

function normalizeValue(value: unknown): string | undefined {
  if (typeof value !== "string") return undefined
  const trimmed = value.trim()
  return trimmed.length > 0 ? trimmed : undefined
}

function parseTextConfig(content: string): ProviderFileConfig {
  const pick = (pattern: RegExp) => {
    const match = content.match(pattern)
    return match?.[1]?.trim()
  }
  const rawFallback = pick(/fallback[_-]?models?\s*[:=]\s*([^\n\r]+)/i)
  const fallbackModels = rawFallback
    ?.split(",")
    .map((item) => item.trim())
    .filter(Boolean)
  return {
    providerID: pick(/provider[_-]?id\s*[:=]\s*["']?([^\s"']+)["']?/i),
    model: pick(/model\s*[:=]\s*["']?([^\s"']+)["']?/i),
    baseURL: pick(/base[_-]?url\s*[:=]\s*["']?([^\s"']+)["']?/i),
    apiKey: pick(/api[_-]?key\s*[:=]\s*([^\s]+)/i),
    fallbackModels,
  }
}

async function loadConfigFile(configPath: string): Promise<ProviderFileConfig | null> {
  try {
    const content = await fs.readFile(configPath, "utf8")
    if (!content.trim()) return null
    if (configPath.endsWith(".json")) {
      const parsed = JSON.parse(content) as Record<string, unknown>
      const fallbackRaw = parsed.fallbackModels
      const fallbackModels =
        Array.isArray(fallbackRaw)
          ? fallbackRaw.map((item) => String(item).trim()).filter(Boolean)
          : undefined
      return {
        providerID: normalizeValue(parsed.providerID ?? parsed.provider_id),
        model: normalizeValue(parsed.model),
        baseURL: normalizeValue(parsed.baseURL ?? parsed.base_url),
        apiKey: normalizeValue(parsed.apiKey ?? parsed.api_key ?? parsed.API_KEY),
        fallbackModels,
      }
    }
    return parseTextConfig(content)
  } catch (error: any) {
    if (error?.code === "ENOENT") return null
    throw new Error(`Failed to read provider config file: ${configPath}\n${error instanceof Error ? error.message : String(error)}`)
  }
}

function resolveProviderConfigCandidates() {
  const explicitPath = process.env.FIXER_PROVIDER_CONFIG?.trim()
  const candidates = [
    explicitPath,
    path.join(ROOT_DIR, "config", "provider.local.json"),
    path.join(ROOT_DIR, "config", "provider.json"),
  ].filter((item): item is string => Boolean(item))
  return Array.from(new Set(candidates))
}

async function loadProviderRuntimeConfig() {
  const envProviderID = normalizeValue(process.env.FIXER_PROVIDER_ID)
  const envModel = normalizeValue(process.env.FIXER_MODEL)
  const envBaseURL = normalizeValue(process.env.FIXER_BASE_URL)
  const envApiKey = normalizeValue(process.env.FIXER_API_KEY)

  if ((envBaseURL && !envApiKey) || (!envBaseURL && envApiKey)) {
    throw new Error("Both FIXER_BASE_URL and FIXER_API_KEY must be set together.")
  }

  const envFallbackModels = parseEnvFallbackModels()
  if (envBaseURL && envApiKey) {
    return {
      providerID: envProviderID ?? DEFAULT_PROVIDER_ID,
      model: envModel ?? DEFAULT_MODEL,
      baseURL: envBaseURL,
      apiKey: envApiKey,
      fallbackModels: envFallbackModels,
    }
  }

  const candidates = resolveProviderConfigCandidates()
  for (const candidate of candidates) {
    const fileConfig = await loadConfigFile(candidate)
    if (!fileConfig) continue
    const baseURL = fileConfig.baseURL
    const apiKey = fileConfig.apiKey
    if (baseURL && apiKey) {
      return {
        providerID: fileConfig.providerID ?? envProviderID ?? DEFAULT_PROVIDER_ID,
        model: fileConfig.model ?? envModel ?? DEFAULT_MODEL,
        baseURL,
        apiKey,
        fallbackModels: fileConfig.fallbackModels && fileConfig.fallbackModels.length > 0
          ? fileConfig.fallbackModels
          : envFallbackModels,
      }
    }
  }

  throw new Error(
    [
      "Provider config not found.",
      "Set FIXER_BASE_URL and FIXER_API_KEY, or create fixer/config/provider.local.json.",
      "Template file: fixer/config/provider.example.json",
    ].join(" "),
  )
}

// 缓存 Provider 配置，避免每次迭代重复加载
let cachedConfig: FixedProviderConfig | null = null

export async function loadFixedProviderConfig(): Promise<FixedProviderConfig> {
  // 返回缓存的配置的深拷贝（防止外部修改导致污染）
  if (cachedConfig) {
    return JSON.parse(JSON.stringify(cachedConfig))
  }
  const runtime = await loadProviderRuntimeConfig()
  const models: Record<string, any> = {}

  // 添加主模型
  models[runtime.model] = {
    id: runtime.model,
    name: runtime.model,
    temperature: false,
    reasoning: false,
    tool_call: true,
    attachment: false,
    modalities: { input: ["text"], output: ["text"] },
    limit: { context: 128000, output: 4096 },
  }

  // 添加备用模型
  for (const fallbackModel of runtime.fallbackModels) {
    models[fallbackModel] = {
      id: fallbackModel,
      name: fallbackModel,
      temperature: false,
      reasoning: false,
      tool_call: true,
      attachment: false,
      modalities: { input: ["text"], output: ["text"] },
      limit: { context: 128000, output: 4096 },
    }
  }

  const result = {
    model: runtime.model,
    fallbackModels: runtime.fallbackModels,
    config: {
      enabled_providers: [runtime.providerID],
      model: `${runtime.providerID}/${runtime.model}`,
      experimental: {
        batch_tool: true,
      },
      provider: {
        [runtime.providerID]: {
          name: "DeepSeek",
          npm: "@ai-sdk/openai-compatible",
          env: [],
          options: {
            apiKey: runtime.apiKey,
            baseURL: runtime.baseURL,
          },
          models,
        },
      },
    },
  }

  // 缓存配置以供后续迭代复用
  cachedConfig = result
  // 返回深拷贝
  return JSON.parse(JSON.stringify(result))
}

// 清除缓存（用于测试或需要重新加载配置时）
export function clearProviderConfigCache(): void {
  cachedConfig = null
}
