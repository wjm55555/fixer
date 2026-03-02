import type { RepairResult } from "./types.ts"
import { repairRustProjectForPython, type PythonRepairInput } from "./python-bridge.ts"

const RESULT_MARKER = "__FIXER_RESULT__"

function toErrorMessage(error: unknown) {
  return error instanceof Error ? error.message : String(error)
}

function failedResult(outputDir: string, message: string): RepairResult {
  return {
    status: "failed",
    summary: message,
    diff: "",
    changedFiles: [],
    metrics: {
      iterations: 0,
      cargoCheckPass: false,
      cargoTestPass: false,
      clippyFixApplied: false,
    },
    artifacts: {
      outputDir,
    },
  }
}

async function readInput(): Promise<PythonRepairInput> {
  const argPayload = process.argv[2]?.trim()
  const raw = argPayload && argPayload.length > 0 ? argPayload : (await Bun.stdin.text()).trim()
  if (!raw) {
    throw new Error("Missing JSON payload. Provide as argv[2] or stdin.")
  }
  const parsed = JSON.parse(raw) as PythonRepairInput
  if (!parsed.rustProjectDir) {
    throw new Error("Missing rustProjectDir")
  }
  if (!parsed.outputRustProjectDir) {
    throw new Error("Missing outputRustProjectDir")
  }
  return parsed
}

async function main() {
  let payload: PythonRepairInput | null = null
  try {
    payload = await readInput()
  } catch (error) {
    const result = failedResult("", toErrorMessage(error))
    console.log(`${RESULT_MARKER}${JSON.stringify(result)}`)
    process.exit(2)
  }

  try {
    const result = await repairRustProjectForPython(payload)
    console.log(`${RESULT_MARKER}${JSON.stringify(result)}`)
    process.exit(result.status === "success" ? 0 : 2)
  } catch (error) {
    const result = failedResult(payload.outputRustProjectDir, toErrorMessage(error))
    console.log(`${RESULT_MARKER}${JSON.stringify(result)}`)
    process.exit(2)
  }
}

await main()
