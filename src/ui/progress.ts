/**
 * Progress - Progress bars and status indicators
 */

import { Colors, success, error, warning, info } from "./colors.ts"

/**
 * Create a progress bar
 */
export function progressBar(
  current: number,
  total: number,
  options: {
    width?: number
    showPercentage?: boolean
    showCounts?: boolean
    filledChar?: string
    emptyChar?: string
    color?: string
  } = {}
): string {
  const {
    width = 20,
    showPercentage = true,
    showCounts = false,
    filledChar = "█",
    emptyChar = "░",
    color = Colors.green,
  } = options

  const percentage = total > 0 ? Math.round((current / total) * 100) : 0
  const filled = total > 0 ? Math.round((current / total) * width) : 0
  const empty = width - filled

  let bar = `${color}${filledChar.repeat(filled)}${Colors.gray}${emptyChar.repeat(empty)}${Colors.reset}`

  if (showPercentage) {
    bar += ` ${percentage}%`
  }

  if (showCounts) {
    bar += ` (${current}/${total})`
  }

  return bar
}

/**
 * Create a spinner animation frame
 */
const SPINNER_FRAMES = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
let spinnerIndex = 0

export function spinner(): string {
  const frame = SPINNER_FRAMES[spinnerIndex % SPINNER_FRAMES.length]
  spinnerIndex++
  return `${Colors.cyan}${frame}${Colors.reset}`
}

/**
 * Format a status with icon
 */
export function status(
  type: "success" | "error" | "warning" | "info" | "working" | "pending",
  message: string
): string {
  const icons = {
    success: "✅",
    error: "❌",
    warning: "⚠️ ",
    info: "ℹ️ ",
    working: "⏳",
    pending: "○",
  }

  const colorFns = {
    success,
    error,
    warning,
    info,
    working: (t: string) => t,
    pending: (t: string) => `${Colors.dim}${t}${Colors.reset}`,
  }

  return `${icons[type]} ${colorFns[type](message)}`
}

/**
 * Format duration in human-readable format
 */
export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`
  if (ms < 3600000) {
    const mins = Math.floor(ms / 60000)
    const secs = Math.round((ms % 60000) / 1000)
    return secs > 0 ? `${mins}分${secs}秒` : `${mins}分`
  }
  const hours = Math.floor(ms / 3600000)
  const mins = Math.round((ms % 3600000) / 60000)
  return mins > 0 ? `${hours}小时${mins}分` : `${hours}小时`
}

/**
 * Format error count delta
 */
export function formatErrorDelta(before: number, after: number): string {
  const delta = before - after
  if (delta > 0) {
    return success(`${before} → ${after} (减少 ${delta} 个)`)
  } else if (delta < 0) {
    return error(`${before} → ${after} (增加 ${Math.abs(delta)} 个)`)
  }
  return warning(`${before} → ${after} (无变化)`)
}

/**
 * Create a step indicator
 */
export function stepIndicator(
  current: number,
  total: number,
  label: string
): string {
  const filled = "●"
  const empty = "○"

  const dots = Array(total)
    .fill(null)
    .map((_, i) => (i < current ? success(filled) : `${Colors.dim}${empty}${Colors.reset}`))
    .join(" ")

  return `${dots}  ${label}`
}

/**
 * Format a change summary
 */
export function changeSummary(changes: {
  edits?: number
  writes?: number
  deletes?: number
}): string {
  const parts: string[] = []

  if (changes.edits && changes.edits > 0) {
    parts.push(`${changes.edits} 处编辑`)
  }
  if (changes.writes && changes.writes > 0) {
    parts.push(`${changes.writes} 个新文件`)
  }
  if (changes.deletes && changes.deletes > 0) {
    parts.push(`${changes.deletes} 处删除`)
  }

  return parts.length > 0 ? parts.join("，") : "无变化"
}

/**
 * Create a countdown timer display
 */
export function countdown(remainingMs: number, totalMs: number): string {
  const remaining = formatDuration(remainingMs)
  const bar = progressBar(totalMs - remainingMs, totalMs, { width: 10 })
  return `${bar} (剩余 ${remaining})`
}
