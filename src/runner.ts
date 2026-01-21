import { spawn } from "child_process"

export type RunResult = {
  command: string[]
  code: number | null
  stdout: string
  stderr: string
  durationMs: number
  truncated: boolean
}

export type RunOptions = {
  cwd: string
  timeoutMs?: number
  maxOutputBytes?: number
  env?: Record<string, string | undefined>
}

export async function runCommand(command: string[], options: RunOptions): Promise<RunResult> {
  const start = Date.now()
  const maxBytes = options.maxOutputBytes ?? 1_000_000
  let stdout = ""
  let stderr = ""
  let truncated = false

  const proc = spawn(command[0], command.slice(1), {
    cwd: options.cwd,
    env: { ...process.env, ...(options.env ?? {}) },
  })

  const onChunk = (chunk: Buffer, target: "stdout" | "stderr") => {
    const text = chunk.toString("utf8")
    if (target === "stdout") stdout += text
    else stderr += text
    if (stdout.length + stderr.length > maxBytes && !truncated) {
      truncated = true
    }
  }

  proc.stdout?.on("data", (chunk) => onChunk(chunk as Buffer, "stdout"))
  proc.stderr?.on("data", (chunk) => onChunk(chunk as Buffer, "stderr"))

  let timeout: NodeJS.Timeout | undefined
  if (options.timeoutMs && options.timeoutMs > 0) {
    timeout = setTimeout(() => {
      proc.kill("SIGKILL")
    }, options.timeoutMs)
  }

  const code = await new Promise<number | null>((resolve) => {
    proc.on("close", (exitCode) => resolve(exitCode))
  })

  if (timeout) clearTimeout(timeout)

  return {
    command,
    code,
    stdout,
    stderr,
    durationMs: Date.now() - start,
    truncated,
  }
}
