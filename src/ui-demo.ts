/**
 * UI Demo - Preview the new terminal UI
 */

import { createRepairUI } from "./ui/index.ts"

const ui = createRepairUI({
  projectName: "translate_gci_1",
  model: "glm-4.7",
  maxIterations: 5,
})

// Demo welcome
ui.printWelcome()

// Demo analysis
ui.printAnalysis(328)

// Demo iteration
ui.printIterationStart(1, 328)

// Demo thinking
ui.printThinking("让我分析一下这些错误...")

// Demo reading
ui.printReading("/outputs/translate_gci_1/src/parser.rs", 2814)
ui.printReading("/outputs/translate_gci_1/src/lexer.rs", 1500)

// Demo searching
ui.printSearching("struct VALUE", "/outputs/translate_gci_1/src")

// Demo editing
ui.printEditing({
  filePath: "/outputs/translate_gci_1/src/virtual_machine.rs",
  problem: "Rust 不允许直接写 `struct VALUE` 作为类型",
  solution: "把 `struct VALUE` 改成 `VALUE`",
  before: "pub stack: Vec<struct VALUE>,",
  after: "pub stack: Vec<VALUE>,",
  success: true,
})

// Demo compact edit
ui.printEditCompact("/src/parser.rs", "删除重复的函数定义", true)
ui.printEditCompact("/src/lexer.rs", "修复类型不匹配", true)
ui.printEditCompact("/src/tests.rs", "添加缺失的 trait", false)

// Demo iteration result
ui.printIterationResult({
  errorsBefore: 328,
  errorsAfter: 280,
  filesChanged: [
    "/src/parser.rs",
    "/src/lexer.rs",
    "/src/virtual_machine.rs",
    "/src/tests.rs",
  ],
  editsApplied: 15,
  duration: 45000,
})

// Demo another iteration
ui.printIterationStart(2, 280)
ui.printLLMThinking()
ui.printLLMResponse("我发现了更多可以批量修复的问题...")

// Demo regression warning
ui.printRegression(280, 295)

// Demo final summary - success
ui.printSummary({
  status: "success",
  iterations: 3,
  duration: 120000,
  errorsBefore: 328,
  errorsAfter: 0,
  filesChanged: [
    "/src/parser.rs",
    "/src/lexer.rs",
    "/src/virtual_machine.rs",
    "/src/tests.rs",
    "/src/data_types.rs",
  ],
})

// Demo final summary - failed
console.log("\n--- Demo: Failed Case ---\n")
ui.printSummary({
  status: "failed",
  iterations: 5,
  duration: 300000,
  errorsBefore: 328,
  errorsAfter: 45,
  filesChanged: [
    "/src/parser.rs",
    "/src/lexer.rs",
  ],
})
