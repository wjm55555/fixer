/**
 * Deterministic Fixes - Mechanical fixes that don't need LLM
 *
 * These handle common C-to-Rust translation patterns:
 * 1. `type` keyword used as identifier → rename to `type_`
 * 2. Missing `struct` keyword: `pub TypeName {` → `pub struct TypeName {`
 * 3. Extra `struct` prefix: `struct TypeName` → `TypeName`
 */

import fs from "fs/promises"
import path from "path"

export interface FixResult {
  file: string
  fixes: number
  patterns: string[]
}

export interface DeterministicFixResult {
  totalFixes: number
  fileResults: FixResult[]
}

/**
 * Apply all deterministic fixes to a workspace
 */
export async function applyDeterministicFixes(
  workspaceDir: string,
  onProgress?: (message: string) => void
): Promise<DeterministicFixResult> {
  const srcDir = path.join(workspaceDir, "src")
  const results: FixResult[] = []

  // Get all .rs files
  const files = await findRustFiles(srcDir)
  onProgress?.(`找到 ${files.length} 个 Rust 源文件`)

  for (const file of files) {
    const result = await fixFile(file)
    if (result.fixes > 0) {
      results.push(result)
      onProgress?.(`  ✓ ${path.relative(workspaceDir, file)}: ${result.fixes} 处修复 (${result.patterns.join(", ")})`)
    }
  }

  return {
    totalFixes: results.reduce((sum, r) => sum + r.fixes, 0),
    fileResults: results,
  }
}

async function findRustFiles(dir: string): Promise<string[]> {
  const files: string[] = []

  async function walk(d: string) {
    const entries = await fs.readdir(d, { withFileTypes: true })
    for (const entry of entries) {
      const fullPath = path.join(d, entry.name)
      if (entry.isDirectory() && entry.name !== "target" && !entry.name.startsWith(".")) {
        await walk(fullPath)
      } else if (entry.isFile() && entry.name.endsWith(".rs")) {
        files.push(fullPath)
      }
    }
  }

  try {
    await walk(dir)
  } catch {
    // Directory doesn't exist
  }
  return files
}

async function fixFile(filePath: string): Promise<FixResult> {
  const result: FixResult = { file: filePath, fixes: 0, patterns: [] }
  const patternsUsed = new Set<string>()

  let content: string
  try {
    content = await fs.readFile(filePath, "utf8")
  } catch {
    return result
  }

  const original = content

  // Fix 1: Missing `struct` keyword
  // Pattern: `pub TypeName {` or `pub TypeName<'a> {` at start of line
  // Should be: `pub struct TypeName {`
  const missingStructPattern = /^(pub\s+)([A-Z][A-Za-z0-9_]*)(\s*(<[^>]+>)?\s*\{)/gm
  const missingStructMatches = content.match(missingStructPattern)
  if (missingStructMatches) {
    content = content.replace(missingStructPattern, "$1struct $2$3")
    result.fixes += missingStructMatches.length
    patternsUsed.add("缺少struct")
  }

  // Fix 2: C-style extra `struct TypeName` → `TypeName` in type position
  // Be conservative: only fix clear patterns where struct is used as a type
  const extraStructPatterns = [
    // In generic: `Vec<struct VALUE>` → `Vec<VALUE>`
    { pattern: /(<\s*)struct\s+([A-Z][A-Za-z0-9_]*)/g, replacement: "$1$2" },
    // After colon in type position: `: struct VALUE` → `: VALUE`
    { pattern: /(:\s*)struct\s+([A-Z][A-Za-z0-9_]*)/g, replacement: "$1$2" },
    // In comma list: `, struct VALUE` → `, VALUE`
    { pattern: /(,\s*)struct\s+([A-Z][A-Za-z0-9_]*)/g, replacement: "$1$2" },
    // After arrow: `-> struct VALUE` → `-> VALUE`
    { pattern: /(->\s*)struct\s+([A-Z][A-Za-z0-9_]*)/g, replacement: "$1$2" },
    // After *mut: `*mut struct VALUE` → `*mut VALUE`
    { pattern: /(\*mut\s+)struct\s+([A-Z][A-Za-z0-9_]*)/g, replacement: "$1$2" },
    // After *const: `*const struct VALUE` → `*const VALUE`
    { pattern: /(\*const\s+)struct\s+([A-Z][A-Za-z0-9_]*)/g, replacement: "$1$2" },
  ]

  for (const fix of extraStructPatterns) {
    const matches = content.match(fix.pattern)
    if (matches) {
      content = content.replace(fix.pattern, fix.replacement)
      result.fixes += matches.length
      patternsUsed.add("多余struct")
    }
  }

  // NOTE: We removed the `type` keyword fixes because they can cause more errors
  // if the struct fields aren't actually renamed. The LLM should handle these
  // along with the corresponding field definitions.

  result.patterns = Array.from(patternsUsed)

  // Write back if changed
  if (content !== original) {
    await fs.writeFile(filePath, content, "utf8")
  }

  return result
}

/**
 * Quick check if a workspace likely has C-to-Rust translation issues
 */
export async function detectCToRustPatterns(workspaceDir: string): Promise<boolean> {
  const srcDir = path.join(workspaceDir, "src")
  const files = await findRustFiles(srcDir)

  // Check first few files for C patterns
  for (const file of files.slice(0, 5)) {
    try {
      const content = await fs.readFile(file, "utf8")
      // Look for C-style patterns
      if (
        /^pub\s+[A-Z][A-Za-z0-9_]*\s*\{/m.test(content) || // Missing struct
        /\s+type\s*:\s*/.test(content) || // type as field name
        /\*mut\s/.test(content) || // raw pointers
        /extern\s+"C"/.test(content) // C FFI
      ) {
        return true
      }
    } catch {
      continue
    }
  }
  return false
}
