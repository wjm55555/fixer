import { repairRustProject } from "../src/repair.ts"

const [workspaceDir, outputDir, sourceDir] = process.argv.slice(2)
if (!workspaceDir || !outputDir) {
  console.error("Usage: bun run examples/repair-demo.ts <workspaceDir> <outputDir> [sourceDir]")
  process.exit(1)
}

const maxIterations = Number(process.env.FIXER_MAX_ITERATIONS ?? "35")
const requireCargoTest = process.env.FIXER_REQUIRE_TESTS === "1"

const result = await repairRustProject({
  workspaceDir,
  outputDir,
  sourceDir,
  constraints: {
    maxIterations: Number.isFinite(maxIterations) && maxIterations > 0 ? maxIterations : 20,
    requireCargoTest,
  },
})

const printResult = process.env.FIXER_PRINT_RESULT === "1"
if (printResult) {
  console.log(JSON.stringify(result, null, 2))
} else {
  console.log(
    JSON.stringify(
      {
        status: result.status,
        metrics: result.metrics,
        outputDir: result.artifacts.outputDir,
      },
      null,
      2,
    ),
  )
}

// Ensure the CLI exits even if background handles remain.
process.exit(result.status === "success" ? 0 : 2)
