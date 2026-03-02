import { applyDeterministicFixes } from "./src/deterministic-fixes.ts"

const targetDir = process.argv[2] || "/Users/wujiaming/Desktop/rust修复/fixer/outputs/translate_gci_1_test"

console.log(`应用模式化修复到: ${targetDir}`)
console.log("")

const result = await applyDeterministicFixes(targetDir, (msg) => console.log(msg))

console.log("")
console.log("=== 修复统计 ===")
console.log("总修复数:", result.totalFixes)
console.log("修改文件数:", result.fileResults.length)

for (const fr of result.fileResults) {
  console.log(`  ${fr.file}: ${fr.fixes} fixes (${fr.patterns.join(", ")})`)
}
