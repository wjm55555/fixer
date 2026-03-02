
import { Core, withCore } from "../src/core-runtime.ts"
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
            console.log("\nResponse from model:")
            if (result.parts) {
                const text = result.parts
                    .filter(p => p.type === 'text')
                    .map(p => p.text)
                    .join('')
                console.log(text)
            } else {
                console.log(JSON.stringify(result, null, 2))
            }
        },
    )
}

main().catch(console.error)
