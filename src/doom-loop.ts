/**
 * Doom Loop Detection Module
 *
 * Detects when the LLM is stuck in a loop making the same tool calls repeatedly.
 * Based on OpenCode's DOOM_LOOP_THRESHOLD detection mechanism.
 */

export const DOOM_LOOP_THRESHOLD = 3

export type ToolCall = {
  tool: string
  input: Record<string, unknown>
}

/**
 * Check if the current tool call is part of a doom loop
 * (i.e., the same call has been made DOOM_LOOP_THRESHOLD times in a row)
 */
export function isDoomLoop(
  currentCall: ToolCall,
  history: ToolCall[]
): boolean {
  // Need at least THRESHOLD - 1 previous calls to have THRESHOLD total
  if (history.length < DOOM_LOOP_THRESHOLD - 1) return false

  const lastN = history.slice(-(DOOM_LOOP_THRESHOLD - 1))
  const currentJson = JSON.stringify(currentCall.input)
  return lastN.every(
    call =>
      call.tool === currentCall.tool &&
      JSON.stringify(call.input) === currentJson
  )
}

/**
 * Track tool call history for doom loop detection
 */
export class DoomLoopTracker {
  private history: ToolCall[] = []
  private doomLoopCount = 0

  /**
   * Record a tool call and check if it's part of a doom loop
   * @returns true if doom loop detected
   */
  recordCall(call: ToolCall): boolean {
    const isDoom = isDoomLoop(call, this.history)
    if (isDoom) {
      this.doomLoopCount++
    }
    this.history.push(call)

    // Keep history bounded
    if (this.history.length > 100) {
      this.history = this.history.slice(-50)
    }

    return isDoom
  }

  /**
   * Get the number of doom loops detected
   */
  getDoomLoopCount(): number {
    return this.doomLoopCount
  }

  /**
   * Reset the tracker
   */
  reset(): void {
    this.history = []
    this.doomLoopCount = 0
  }

  /**
   * Get a summary of recent calls for debugging
   */
  getRecentCallsSummary(n = 5): string {
    const recent = this.history.slice(-n)
    return recent
      .map(call => `${call.tool}(${JSON.stringify(call.input).slice(0, 100)})`)
      .join("\n")
  }
}

// Global tracker instance
let globalTracker: DoomLoopTracker | null = null

export function getDoomLoopTracker(): DoomLoopTracker {
  if (!globalTracker) {
    globalTracker = new DoomLoopTracker()
  }
  return globalTracker
}

export function resetDoomLoopTracker(): void {
  if (globalTracker) {
    globalTracker.reset()
    globalTracker = null
  }
}
