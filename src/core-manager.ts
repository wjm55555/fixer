/**
 * Core 实例管理器 - 复用 Core 实例以减少初始化开销
 */
import { withCore, Core } from "./core-runtime.ts"

type CoreConfig = {
  workspaceDir: string
  config: Record<string, unknown>
  permission: Array<{ permission: string; action: string; pattern: string }>
  printLogs?: boolean
  logLevel?: "DEBUG" | "INFO" | "WARN" | "ERROR"
  onEvent?: (event: { type: string; properties: Record<string, any> }) => void
}

type SessionInfo = {
  id: string
  messageCount: number
}

export class CoreManager {
  private initialized = false
  private currentConfig: CoreConfig | null = null
  private sessionInfo: SessionInfo | null = null
  private cleanupFn: (() => Promise<void>) | null = null

  /**
   * 初始化 Core（如果尚未初始化或配置已更改）
   */
  async initialize(config: CoreConfig): Promise<void> {
    // 如果已初始化且配置相同，跳过
    if (this.initialized && this.configMatches(config)) {
      return
    }

    // 如果已初始化但配置不同，先清理
    if (this.initialized) {
      await this.cleanup()
    }

    this.currentConfig = config
    this.initialized = true
  }

  /**
   * 在 Core 上下文中执行操作
   * @param fn 要执行的函数
   * @param configOverride 可选的配置覆盖（如 onEvent）
   */
  async withContext<T>(
    fn: () => Promise<T>,
    configOverride?: Partial<CoreConfig>,
  ): Promise<T> {
    if (!this.currentConfig) {
      throw new Error("CoreManager not initialized. Call initialize() first.")
    }

    // 合并配置覆盖
    const config = configOverride
      ? { ...this.currentConfig, ...configOverride }
      : this.currentConfig

    return withCore(config, fn)
  }

  /**
   * 获取或创建会话
   */
  async ensureSession(title = "rust-fix"): Promise<string> {
    if (this.sessionInfo) {
      return this.sessionInfo.id
    }

    const session = await Core.Session.create({ title })
    this.sessionInfo = {
      id: session.id,
      messageCount: 0,
    }
    return session.id
  }

  /**
   * 获取当前会话 ID
   */
  getSessionID(): string | null {
    return this.sessionInfo?.id ?? null
  }

  /**
   * 获取当前会话消息数量
   */
  getMessageCount(): number {
    return this.sessionInfo?.messageCount ?? 0
  }

  /**
   * 更新消息数量
   */
  updateMessageCount(count: number): void {
    if (this.sessionInfo) {
      this.sessionInfo.messageCount = count
    }
  }

  /**
   * 发送提示并获取响应
   */
  async prompt(input: Parameters<typeof Core.SessionPrompt.prompt>[0]) {
    return Core.SessionPrompt.prompt(input)
  }

  /**
   * 获取会话消息
   */
  async getMessages(sessionID: string) {
    return Core.Session.messages({ sessionID })
  }

  /**
   * 清理资源
   */
  async cleanup(): Promise<void> {
    if (this.cleanupFn) {
      await this.cleanupFn()
      this.cleanupFn = null
    }
    this.initialized = false
    this.currentConfig = null
    this.sessionInfo = null
  }

  /**
   * 检查是否已初始化
   */
  isInitialized(): boolean {
    return this.initialized
  }

  /**
   * 检查配置是否匹配（简单比较）
   */
  private configMatches(config: CoreConfig): boolean {
    if (!this.currentConfig) return false
    return this.currentConfig.workspaceDir === config.workspaceDir
  }
}

// 全局 CoreManager 实例
let globalCoreManager: CoreManager | null = null

/**
 * 获取全局 CoreManager 实例
 */
export function getCoreManager(): CoreManager {
  if (!globalCoreManager) {
    globalCoreManager = new CoreManager()
  }
  return globalCoreManager
}

/**
 * 重置全局 CoreManager
 */
export async function resetCoreManager(): Promise<void> {
  if (globalCoreManager) {
    await globalCoreManager.cleanup()
    globalCoreManager = null
  }
}
