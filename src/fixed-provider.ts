import fs from "fs/promises"

const API_FILE = "/Users/wujiaming/Desktop/rust修复/api的key以及平台.md"

export type FixedProviderConfig = {
  model: string
  config: Record<string, unknown>
}

function parseValue(pattern: RegExp, content: string, label: string) {
  const match = content.match(pattern)
  if (!match || !match[1]) {
    throw new Error(`Missing ${label} in ${API_FILE}`)
  }
  return match[1].trim()
}

export async function loadFixedProviderConfig(): Promise<FixedProviderConfig> {
  const content = await fs.readFile(API_FILE, "utf8")
  const baseUrl = parseValue(/base_url\s*=\s*"([^"]+)"/i, content, "base_url")
  const originalModel = parseValue(/模型为\s*([\w.-]+)/i, content, "model")
  const apiKey = parseValue(/API_KEY\s*:\s*([^\s]+)/i, content, "API_KEY")
  const providerID = "privatemode-ai"
  const model = originalModel
  const fallbackModels = ["qwen3-max"]
  const models: Record<string, any> = {}
  const registerModel = (name: string) => {
    if (models[name]) return
    models[name] = {
      id: name,
      name,
      temperature: true,
      reasoning: true,
      tool_call: true,
      attachment: false,
      modalities: { input: ["text"], output: ["text"] },
      limit: { context: 198000, output: 16000 },
    }
  }
  registerModel(model)
  for (const fallback of fallbackModels) {
    registerModel(fallback)
  }

  return {
    model,
    config: {
      enabled_providers: [providerID],
      model: `${providerID}/${model}`,
      provider: {
        [providerID]: {
          options: {
            apiKey,
            baseURL: baseUrl,
            extraBody: { enable_thinking: false },
          },
          models,
        },
      },
    },
  }
}
