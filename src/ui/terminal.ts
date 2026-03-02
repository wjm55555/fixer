/**
 * Terminal UI - Main controller for beautiful CLI output
 *
 * Design goals:
 * - Human-readable: Even non-programmers can understand what's happening
 * - Visual: Clear progress and status indicators
 * - Friendly: Like a real assistant explaining their work
 */

import { Colors, success, error, warning, info, bold, dim, highlight, Icons } from "./colors.ts"
import { drawBox, tree } from "./box.ts"
import {
  progressBar,
  status,
  formatDuration,
  formatErrorDelta,
  stepIndicator,
  changeSummary,
} from "./progress.ts"

export interface RepairUIOptions {
  projectName: string
  model: string
  maxIterations: number
  showDetails?: boolean
}

export class RepairUI {
  private startTime: number
  private options: RepairUIOptions
  private currentIteration = 0

  constructor(options: RepairUIOptions) {
    this.options = options
    this.startTime = Date.now()
  }

  /**
   * Print the welcome header
   */
  printWelcome() {
    console.log("")
    console.log(
      drawBox(
        [
          `${Icons.fixing} ${bold("Rust 修复助手")} v2.0`,
          "",
          `${dim("项目:")} ${highlight(this.options.projectName)}`,
          `${dim("模型:")} ${this.options.model}`,
          `${dim("最大迭代:")} ${this.options.maxIterations} 轮`,
        ],
        { width: 50, style: "rounded", color: Colors.cyan }
      )
    )
    console.log("")
  }

  /**
   * Print initial analysis result
   */
  printAnalysis(errorCount: number) {
    console.log(`${Icons.searching} ${bold("正在分析项目...")}`)
    console.log("")

    if (errorCount === 0) {
      console.log(`   ${status("success", "太棒了！项目已经编译通过，无需修复。")}`)
    } else {
      console.log(`   发现 ${highlight(String(errorCount))} 个编译错误`)
      console.log(`   ${dim("让我来逐步修复它们...")}`)
    }
    console.log("")
  }

  /**
   * Print deterministic fixes phase
   */
  printDeterministicFixStart() {
    console.log(`${Icons.fixing} ${bold("应用模式化修复...")}`)
    console.log(`   ${dim("自动修复常见的 C→Rust 翻译问题")}`)
    console.log("")
  }

  printDeterministicFixProgress(message: string) {
    console.log(`   ${message}`)
  }

  printDeterministicFixResult(totalFixes: number, errorsBefore: number, errorsAfter: number) {
    if (totalFixes > 0) {
      console.log("")
      console.log(`   ${Icons.success} ${success(`自动修复了 ${totalFixes} 处问题`)}`)
      console.log(`   ${dim(`错误: ${errorsBefore} → ${errorsAfter} (减少 ${errorsBefore - errorsAfter} 个)`)}`)
    } else {
      console.log(`   ${dim("没有发现可自动修复的模式")}`)
    }
    console.log("")
  }

  /**
   * Print iteration header
   */
  printIterationStart(iteration: number, errorCount: number) {
    this.currentIteration = iteration
    const elapsed = formatDuration(Date.now() - this.startTime)

    console.log(`${Icons.reading} ${bold(`第 ${iteration} 轮修复`)} ${dim(`[${elapsed}]`)}`)
    console.log(`   ${dim(`当前错误数: ${errorCount}`)}`)
    console.log("")
  }

  /**
   * Print what the assistant is thinking/doing
   */
  printThinking(message: string) {
    console.log(`   ${Icons.thinking} ${message}`)
  }

  /**
   * Print a file read action
   */
  printReading(filePath: string, lineCount?: number) {
    const shortPath = this.shortenPath(filePath)
    const lineInfo = lineCount ? ` (${lineCount} 行)` : ""
    console.log(`   ${Icons.branch} 阅读 ${info(shortPath)}${dim(lineInfo)}`)
  }

  /**
   * Print a file edit action with explanation
   */
  printEditing(options: {
    filePath: string
    problem: string
    solution: string
    before?: string
    after?: string
    success: boolean
  }) {
    const shortPath = this.shortenPath(options.filePath)

    console.log(`   ${Icons.branch} ${Icons.fixing} 修改 ${info(shortPath)}`)
    console.log(`   ${Icons.line}`)
    console.log(`   ${Icons.line}  ${dim("问题：")}${options.problem}`)
    console.log(`   ${Icons.line}  ${dim("解决：")}${options.solution}`)

    if (options.before && options.after) {
      console.log(`   ${Icons.line}`)
      console.log(`   ${Icons.line}  ${error(`- ${this.truncate(options.before, 50)}`)}`)
      console.log(`   ${Icons.line}  ${success(`+ ${this.truncate(options.after, 50)}`)}`)
    }

    console.log(
      `   ${Icons.corner} ${options.success ? status("success", "修改成功") : status("error", "修改失败")}`
    )
    console.log("")
  }

  /**
   * Print a simple edit action (compact mode)
   */
  printEditCompact(filePath: string, description: string, success: boolean) {
    const shortPath = this.shortenPath(filePath)
    const icon = success ? Icons.success : Icons.error
    console.log(`   ${Icons.branch} ${icon} ${info(shortPath)}: ${description}`)
  }

  /**
   * Print search action
   */
  printSearching(pattern: string, path?: string) {
    const pathInfo = path ? ` in ${this.shortenPath(path)}` : ""
    console.log(`   ${Icons.branch} ${Icons.searching} 搜索 ${dim(`"${this.truncate(pattern, 30)}"`)}${dim(pathInfo)}`)
  }

  /**
   * Print cargo command execution
   */
  printCargoCommand(command: string, status: "running" | "success" | "error") {
    const icons = {
      running: Icons.working,
      success: Icons.success,
      error: Icons.error,
    }
    const colors = {
      running: (t: string) => t,
      success,
      error,
    }

    console.log(`${icons[status]} ${colors[status](`cargo ${command}`)}`)
  }

  /**
   * Print iteration result
   */
  printIterationResult(options: {
    errorsBefore: number
    errorsAfter: number
    filesChanged: string[]
    editsApplied: number
    duration: number
  }) {
    console.log("")
    console.log(`${Icons.bullet} ${bold("本轮结果")}`)

    // Error count change
    console.log(`   ${Icons.branch} 错误: ${formatErrorDelta(options.errorsBefore, options.errorsAfter)}`)

    // Files changed
    if (options.filesChanged.length > 0) {
      const fileList = options.filesChanged.slice(0, 3).map(f => this.shortenPath(f)).join(", ")
      const more = options.filesChanged.length > 3 ? ` 等 ${options.filesChanged.length} 个文件` : ""
      console.log(`   ${Icons.branch} 修改: ${fileList}${more}`)
    } else {
      console.log(`   ${Icons.branch} 修改: ${dim("无文件变化")}`)
    }

    // Progress bar
    const progress =
      options.errorsBefore > 0
        ? Math.max(0, 100 - Math.round((options.errorsAfter / options.errorsBefore) * 100))
        : options.errorsAfter === 0
          ? 100
          : 0
    console.log(`   ${Icons.corner} 进度: ${progressBar(progress, 100, { width: 20 })}`)

    console.log("")
  }

  /**
   * Print regression warning
   */
  printRegression(errorsBefore: number, errorsAfter: number) {
    console.log("")
    console.log(
      drawBox(
        [
          `${Icons.warning} 检测到回归`,
          "",
          `错误数量从 ${errorsBefore} 增加到 ${errorsAfter}`,
          "正在自动回滚到最佳状态...",
        ],
        { width: 50, style: "rounded", color: Colors.yellow }
      )
    )
    console.log("")
  }

  /**
   * Print rollback result
   */
  printRollback(targetIteration: number, errorCount: number, success: boolean) {
    if (success) {
      console.log(`   ${Icons.success} 已回滚到第 ${targetIteration} 轮的状态 (${errorCount} 个错误)`)
    } else {
      console.log(`   ${Icons.error} 回滚失败`)
    }
    console.log("")
  }

  /**
   * Print final summary report
   */
  printSummary(options: {
    status: "success" | "partial" | "failed"
    iterations: number
    duration: number
    errorsBefore: number
    errorsAfter: number
    filesChanged: string[]
    fixesByType?: Map<string, number>
  }) {
    const statusInfo = {
      success: { icon: Icons.success, label: "成功", color: Colors.green },
      partial: { icon: Icons.warning, label: "部分成功", color: Colors.yellow },
      failed: { icon: Icons.error, label: "失败", color: Colors.red },
    }

    const { icon, label, color } = statusInfo[options.status]

    console.log("")
    console.log(
      drawBox(
        [
          `${bold("📋 修复报告")}`,
        ],
        { width: 60, style: "double", color }
      )
    )

    // Status section
    console.log("")
    console.log(`  ${bold("状态:")} ${icon} ${label}`)
    console.log(`  ${bold("耗时:")} ${formatDuration(options.duration)}`)
    console.log(`  ${bold("迭代:")} ${options.iterations} 轮`)
    console.log(`  ${bold("错误:")} ${formatErrorDelta(options.errorsBefore, options.errorsAfter)}`)

    // Files changed section
    if (options.filesChanged.length > 0) {
      console.log("")
      console.log(`  ${bold("修改的文件:")}`)
      for (const file of options.filesChanged.slice(0, 10)) {
        console.log(`    ${Icons.bullet} ${this.shortenPath(file)}`)
      }
      if (options.filesChanged.length > 10) {
        console.log(`    ${dim(`... 等共 ${options.filesChanged.length} 个文件`)}`)
      }
    }

    // Fixes by type section
    if (options.fixesByType && options.fixesByType.size > 0) {
      console.log("")
      console.log(`  ${bold("修复的问题类型:")}`)
      for (const [type, count] of options.fixesByType) {
        console.log(`    ${Icons.bullet} ${type}: ${count} 个`)
      }
    }

    console.log("")
  }

  /**
   * Print an error message
   */
  printError(message: string, details?: string) {
    console.log(`${Icons.error} ${error(message)}`)
    if (details) {
      console.log(`   ${dim(details)}`)
    }
  }

  /**
   * Print a warning message
   */
  printWarning(message: string) {
    console.log(`${Icons.warning} ${warning(message)}`)
  }

  /**
   * Print an info message
   */
  printInfo(message: string) {
    console.log(`${Icons.info} ${info(message)}`)
  }

  /**
   * Print LLM thinking message
   */
  printLLMThinking() {
    console.log(`   ${Icons.working} ${dim("LLM 正在分析错误...")}`)
  }

  /**
   * Print LLM response summary
   */
  printLLMResponse(summary: string) {
    console.log(`   ${Icons.thinking} ${summary}`)
  }

  // Helper methods

  private shortenPath(filePath: string): string {
    // Keep only the last 2-3 path segments
    const parts = filePath.split("/")
    if (parts.length <= 3) return filePath
    return "..." + parts.slice(-3).join("/")
  }

  private truncate(text: string, maxLen: number): string {
    if (text.length <= maxLen) return text
    return text.slice(0, maxLen - 3) + "..."
  }
}

// Export a simple API for quick usage
export function createRepairUI(options: RepairUIOptions): RepairUI {
  return new RepairUI(options)
}
