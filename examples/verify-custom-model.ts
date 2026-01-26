
import { Core, withCore } from "@opencode-ai/core"
import { loadFixedProviderConfig } from "../src/fixed-provider.ts"

const workspaceDir = process.cwd()

async function main() {
    const fixed = await loadFixedProviderConfig()
    console.log("Loaded config:", JSON.stringify(fixed, null, 2))

    await withCore(
        {
            workspaceDir,
            config: fixed.config,
            permission: [
                { permission: "list", action: "allow", pattern: "*" },
                { permission: "read", action: "allow", pattern: "*" },
                { permission: "global_cache_read", action: "allow", pattern: "*" },
                { permission: "global_cache_write", action: "allow", pattern: "*" },
            ],
        },
        async () => {
            const session = await Core.Session.create({ title: "verify-custom-model" })
            console.log(`Session created: ${session.id}`)

            const result = await Core.SessionPrompt.prompt({
                sessionID: session.id,
                agent: "build",
                parts: [
                    {
                        type: "text",
                        text: "Hello! Please tell me which model you are and confirm you are capable of thinking/reasoning.",
                    },
                ],
            })

            console.log("\nResponse from model:")
            if (result.parts) {
                for (const part of result.parts) {
                    if (part.type === 'text') {
                        console.log(part.text)
                    } else if (part.type === 'reasoning') {
                        console.log("[REASONING]:", part.text)
                    }
                }
            } else {
                console.log(JSON.stringify(result, null, 2))
            }
        },
    )
}

main().catch(console.error)
