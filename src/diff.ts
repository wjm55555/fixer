import path from "path"
import { runCommand } from "./runner.ts"
import { assertAllowed, type CommandPolicy } from "./security.ts"

export async function computeDiff(originalDir: string, outputDir: string, policy: CommandPolicy) {
  const left = path.resolve(originalDir)
  const right = path.resolve(outputDir)
  const command = ["git", "diff", "--no-index", "--", left, right]
  assertAllowed(command, policy)
  const result = await runCommand(command, {
    cwd: outputDir,
    timeoutMs: 120_000,
    maxOutputBytes: 2_000_000,
  })

  const rawDiff = result.stdout + result.stderr
  const changedFiles = new Set<string>()

  for (const line of rawDiff.split("\n")) {
    if (line.startsWith("diff --git")) {
      const parts = line.split(" ")
      const file = parts[3]?.replace(/^b\//, "")
      if (file) {
        changedFiles.add(file)
      }
    }
  }

  return { diff: rawDiff, changedFiles: Array.from(changedFiles) }
}
