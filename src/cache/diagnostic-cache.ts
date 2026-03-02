/**
 * 诊断摘要缓存 - 避免重复计算诊断摘要
 */

type DiagnosticKey = string

type CacheEntry = {
  summary: string
  hash: string
}

/**
 * 计算诊断的哈希值（用于缓存键）
 */
function hashDiagnostic(diag: {
  kind: string
  code: number | null
  stdout: string
  stderr: string
  json?: unknown[]
}): string {
  // 使用简单的字符串哈希
  const content = JSON.stringify({
    kind: diag.kind,
    code: diag.code,
    // 只取前 1000 字符以提高哈希性能
    stdout: diag.stdout?.slice(0, 1000),
    stderr: diag.stderr?.slice(0, 1000),
    jsonLen: diag.json?.length ?? 0,
  })

  let hash = 0
  for (let i = 0; i < content.length; i++) {
    const char = content.charCodeAt(i)
    hash = ((hash << 5) - hash) + char
    hash = hash & hash // Convert to 32bit integer
  }
  return hash.toString(16)
}

export class DiagnosticSummaryCache {
  private cache: Map<DiagnosticKey, CacheEntry> = new Map()
  private maxEntries: number

  constructor(maxEntries = 100) {
    this.maxEntries = maxEntries
  }

  /**
   * 获取缓存的摘要，如果不存在则返回 null
   */
  get(
    diagnostics: Array<{
      kind: string
      code: number | null
      stdout: string
      stderr: string
      json?: unknown[]
    }>,
  ): string | null {
    const key = this.computeKey(diagnostics)
    const entry = this.cache.get(key)
    if (!entry) return null

    // 验证哈希是否匹配
    const hash = this.computeHash(diagnostics)
    if (entry.hash !== hash) {
      this.cache.delete(key)
      return null
    }

    return entry.summary
  }

  /**
   * 缓存摘要
   */
  set(
    diagnostics: Array<{
      kind: string
      code: number | null
      stdout: string
      stderr: string
      json?: unknown[]
    }>,
    summary: string,
  ): void {
    // 如果缓存满了，删除最旧的条目
    if (this.cache.size >= this.maxEntries) {
      const firstKey = this.cache.keys().next().value
      if (firstKey) {
        this.cache.delete(firstKey)
      }
    }

    const key = this.computeKey(diagnostics)
    const hash = this.computeHash(diagnostics)
    this.cache.set(key, { summary, hash })
  }

  /**
   * 清空缓存
   */
  clear(): void {
    this.cache.clear()
  }

  /**
   * 获取缓存统计
   */
  getStats(): { entries: number; maxEntries: number } {
    return {
      entries: this.cache.size,
      maxEntries: this.maxEntries,
    }
  }

  private computeKey(
    diagnostics: Array<{
      kind: string
      code: number | null
      stdout: string
      stderr: string
      json?: unknown[]
    }>,
  ): DiagnosticKey {
    // 使用诊断数量和类型作为键
    const kinds = diagnostics.map((d) => d.kind).join(",")
    const codes = diagnostics.map((d) => d.code ?? "null").join(",")
    return `${diagnostics.length}:${kinds}:${codes}`
  }

  private computeHash(
    diagnostics: Array<{
      kind: string
      code: number | null
      stdout: string
      stderr: string
      json?: unknown[]
    }>,
  ): string {
    return diagnostics.map(hashDiagnostic).join("|")
  }
}

// 全局缓存实例
let globalDiagnosticCache: DiagnosticSummaryCache | null = null

/**
 * 获取全局诊断摘要缓存
 */
export function getDiagnosticCache(): DiagnosticSummaryCache {
  if (!globalDiagnosticCache) {
    globalDiagnosticCache = new DiagnosticSummaryCache()
  }
  return globalDiagnosticCache
}

/**
 * 重置全局诊断摘要缓存
 */
export function resetDiagnosticCache(): void {
  if (globalDiagnosticCache) {
    globalDiagnosticCache.clear()
    globalDiagnosticCache = null
  }
}
