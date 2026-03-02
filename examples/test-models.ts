
import { Core, withCore } from "../src/core-runtime.ts"
import { loadFixedProviderConfig, FALLBACK_MODELS } from "../src/fixed-provider.ts"

const workspaceDir = process.cwd()

async function testModel(modelID: string, config: any) {
    console.log(`\nTesting model: ${modelID}...`)
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
                const session = await Core.Session.create({ title: `test-${modelID}` })
                // Override the model in the prompt call
                // The config puts the model under the provider, we need to construct the reference
                const providerID = "aliyun-dashscope"
                const modelRef = { providerID, modelID }

                const result = await Core.SessionPrompt.prompt({
                    sessionID: session.id,
                    model: modelRef,
                    agent: "build",
                    parts: [
                        {
                            type: "text",
                            text: "Hello, are you working?",
                        },
                    ],
                })

                if (result.parts) {
                    const text = result.parts
                        .filter(p => p.type === 'text')
                        .map(p => p.text)
                        .join('')
                    console.log(`✅ Success! Response: ${text.slice(0, 50)}...`)
                } else {
                    console.log("⚠️ No parts returned.")
                }
            },
        )
    } catch (e: any) {
        console.error(`❌ Failed: ${e.message}`)
    }
}

async function main() {
    const fixed = await loadFixedProviderConfig()
    const primaryModel = fixed.model

    // Test Primary
    await testModel(primaryModel, fixed.config)

    // Test Fallbacks
    for (const model of FALLBACK_MODELS) {
        await testModel(model, fixed.config)
    }
}

main().catch(console.error)
