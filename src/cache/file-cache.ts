/**
 * 文件内容缓存 - 减少重复文件读取
 */
import fs from "fs/promises"

type CacheEntry = {
  content: string
  mtime: number
}

export class FileContentCache {
  private cache: Map<string, CacheEntry> = new Map()

  /**
   * 读取文件，优先使用缓存
   * 如果文件已修改（mtime 变化）则重新读取
   */
  async readFile(filePath: string): Promise<string> {
    try {
      const stat = await fs.stat(filePath)
      const cached = this.cache.get(filePath)

      // 如果缓存存在且文件未修改，返回缓存内容
      if (cached && cached.mtime >= stat.mtimeMs) {
        return cached.content
      }

      // 读取文件并更新缓存
      const content = await fs.readFile(filePath, "utf8")
      this.cache.set(filePath, {
        content,
        mtime: stat.mtimeMs,
      })

      return content
    } catch (error) {
      // 文件不存在或读取失败，从缓存中移除
      this.cache.delete(filePath)
      throw error
    }
  }

  /**
   * 使指定文件的缓存失效
   */
  invalidate(filePath: string): void {
    this.cache.delete(filePath)
  }

  /**
   * 使匹配目录的所有缓存失效
   */
  invalidateDir(dirPath: string): void {
    const prefix = dirPath.endsWith("/") ? dirPath : `${dirPath}/`
    for (const key of this.cache.keys()) {
      if (key.startsWith(prefix) || key === dirPath) {
        this.cache.delete(key)
      }
    }
  }

  /**
   * 清空所有缓存
   */
  clear(): void {
    this.cache.clear()
  }

  /**
   * 获取缓存统计信息
   */
  getStats(): { entries: number; totalBytes: number } {
    let totalBytes = 0
    for (const entry of this.cache.values()) {
      totalBytes += Buffer.byteLength(entry.content, "utf8")
    }
    return {
      entries: this.cache.size,
      totalBytes,
    }
  }
}

// 全局缓存实例
let globalFileCache: FileContentCache | null = null

/**
 * 获取全局文件缓存实例
 */
export function getFileCache(): FileContentCache {
  if (!globalFileCache) {
    globalFileCache = new FileContentCache()
  }
  return globalFileCache
}

/**
 * 重置全局文件缓存
 */
export function resetFileCache(): void {
  if (globalFileCache) {
    globalFileCache.clear()
    globalFileCache = null
  }
}
