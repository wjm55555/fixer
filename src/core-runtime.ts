const preferShim = process.env.FIXER_ENGINE === "shim"

const runtime = await (async () => {
  if (preferShim) {
    return await import("./core-shim.ts")
  }
  try {
    return await import("../vendor/packages/opencode-core/src/index.ts")
  } catch (error) {
    console.warn(
      `[fixer] failed to load vendored core runtime, fallback to core-shim: ${
        error instanceof Error ? error.message : String(error)
      }`,
    )
    return await import("./core-shim.ts")
  }
})()

export const withCore = runtime.withCore
export const Core = runtime.Core
export const bindAbortToSession = runtime.bindAbortToSession
export const promptWithAbort = runtime.promptWithAbort
