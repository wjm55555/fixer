import path from "path"
import { withCore, Core } from "../src/core-runtime.ts"
import { loadFixedProviderConfig } from "../src/fixed-provider.ts"

const workspaceDir = process.cwd()
const dataDir = path.join(workspaceDir, ".llm-health")

const fixed = await loadFixedProviderConfig()
console.log(JSON.stringify({ fixedModel: fixed.model, fixedConfigKeys: Object.keys(fixed.config) }, null, 2))
const providerID = "privatemode-ai"
const modelID = fixed.model

await withCore(
  {
    workspaceDir,
    dataDir,
    config: fixed.config,
    permission: [],
  },
  async () => {
    const config = await Core.Config.get()
    const providers = await Core.Provider.list()
    console.log(
      JSON.stringify(
        {
          enabled_providers: config.enabled_providers,
          provider_keys: config.provider ? Object.keys(config.provider) : [],
          providers: Object.keys(providers),
        },
        null,
        2,
      ),
    )

    const session = await Core.Session.create({ title: "llm-health" })
    try {
      const result = await Core.SessionPrompt.prompt({
        sessionID: session.id,
        agent: "build",
        model: {
          providerID,
          modelID,
        },
        parts: [{ type: "text", text: "ping" }],
      })
      const text = result.parts.find((part) => part.type === "text")?.text ?? ""
      console.log(JSON.stringify({ ok: true, model: `${providerID}/${modelID}`, text }, null, 2))
    } catch (error) {
      console.error(
        JSON.stringify(
          {
            ok: false,
            model: `${providerID}/${modelID}`,
            error: error && typeof error === "object" && "message" in error ? (error as any).message : String(error),
          },
          null,
          2,
        ),
      )
      throw error
    }
  },
)
