
import { Core, withCore } from "../src/core-runtime.ts"
import { loadFixedProviderConfig } from "../src/fixed-provider.ts"

const workspaceDir = process.cwd()

async function main() {
    console.log("Testing fallback model: qwen3-coder-plus...")
    const fixed = await loadFixedProviderConfig()

    // Force use of fallback model
    const config = JSON.parse(JSON.stringify(fixed.config))
    const providerID = "aliyun-dashscope"
    const modelID = "qwen3-coder-plus"

    // We don't need to change the config object because it contains ALL models in the provider definition
    // We just need to request the specific model ID in the prompt

    try {
        await withCore(
            {
                workspaceDir,
                config: config,
                permission: [
                    { permission: "list", action: "allow", pattern: "*" },
                    { permission: "read", action: "allow", pattern: "*" },
                    { permission: "global_cache_read", action: "allow", pattern: "*" },
                    { permission: "global_cache_write", action: "allow", pattern: "*" },
                ],
            },
            async () => {
                const session = await Core.Session.create({ title: `test-fallback` })
                const modelRef = { providerID, modelID }

                const result = await Core.SessionPrompt.prompt({
                    sessionID: session.id,
                    model: modelRef, // Explicitly request qwen3-coder-plus
                    agent: "build",
                    parts: [
                        {
                            type: "text",
                            text: "Hello, confirm you are qwen3-coder-plus.",
                        },
                    ],
                })

                if (result.parts) {
                    const text = result.parts
                        .filter(p => p.type === 'text')
                        .map(p => p.text)
                        .join('')
                    console.log(`✅ Success! Response: ${text}`)
                } else {
                    console.log("⚠️ No parts returned.")
                }
            },
        )
    } catch (e: any) {
        console.error(`❌ Failed: ${e.message}`)
        process.exit(1)
    }
}

main().catch(console.error)
