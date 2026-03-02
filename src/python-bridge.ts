import { repairRustProject } from "./repair.ts"
import type { RepairResult } from "./types.ts"

export type PythonRepairInput = {
  rustProjectDir: string
  outputRustProjectDir: string
  sourceProjectDir?: string
  testCases?: string[]
  maxIterations?: number
  timeBudgetMs?: number
  runTestsWhenCheckPass?: boolean
}

export async function repairRustProjectForPython(input: PythonRepairInput): Promise<RepairResult> {
  const testCases = (input.testCases ?? []).map((item) => item.trim()).filter(Boolean)
  return repairRustProject({
    workspaceDir: input.rustProjectDir,
    outputDir: input.outputRustProjectDir,
    sourceDir: input.sourceProjectDir,
    constraints: {
      maxIterations: input.maxIterations,
      timeBudgetMs: input.timeBudgetMs,
      runTestsWhenCheckPass: input.runTestsWhenCheckPass ?? true,
      testCases,
    },
  })
}
