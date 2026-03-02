import { Core, withCore } from "../../src/core-runtime.ts"

const workspaceDir = process.argv[2] ?? process.cwd()

await withCore(
  {
    workspaceDir,
    permission: [
      { permission: "list", action: "allow", pattern: "*" },
      { permission: "read", action: "allow", pattern: "*" },
      { permission: "grep", action: "allow", pattern: "*" },
      { permission: "glob", action: "allow", pattern: "*" },
      { permission: "bash", action: "allow", pattern: "*" },
    ],
  },
  async () => {
    const session = await Core.Session.create({ title: "core-smoke" })
    const result = await Core.SessionPrompt.prompt({
      sessionID: session.id,
      agent: "build",
      parts: [
        {
          type: "text",
          text: "List Rust files under the current workspace. Use list and glob tools only. Return a short list.",
        },
      ],
    })
    console.log(result)
  },
)
