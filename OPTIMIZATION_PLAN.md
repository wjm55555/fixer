# Fixer 全面优化计划

基于 OpenCode 代码分析，制定以下优化计划使 Fixer 达到 OpenCode 同等水平。

## 实施进度

| 阶段 | 状态 | 完成日期 |
|------|------|----------|
| 阶段 1: 移除批量诊断 | ✅ 已完成 | 2026-01-27 |
| 阶段 3: 死循环检测 | ✅ 已完成 | 2026-01-27 |
| 阶段 4: 优化系统提示词 | ✅ 已完成 | 2026-01-27 |
| 阶段 5: 会话持久化 | ✅ 已实现 | 已有实现 |
| 阶段 6: 智能重试机制 | ✅ 已完成 | 2026-01-27 |
| 阶段 2: 循环执行模式 | ⏸️ 待定 | Core 已内置 |

---

## 已完成的优化

### 1. 移除批量诊断，回归简单 ✅

**文件**: `fixer/src/repair.ts`

- 删除了 `diagnostic-batcher.ts`
- LLM 现在可以看到所有错误，自主决定修复顺序
- 避免了批量处理导致的 LLM 幻觉问题

```typescript
const user = [
  `Workspace: ${input.outputDir}`,
  `共有 ${totalErrors} 个编译错误`,
  "",
  "编译错误摘要:",
  summarizeDiagnostics(input.diagnostics),
  "",
  "请逐个修复这些错误。每次修改后会自动重新编译，你可以看到剩余错误。",
].join("\n")
```

### 2. 死循环检测 ✅

**新文件**: `fixer/src/doom-loop.ts`

基于 OpenCode 的 `DOOM_LOOP_THRESHOLD = 3` 机制，检测重复的工具调用。

```typescript
export const DOOM_LOOP_THRESHOLD = 3

export function isDoomLoop(
  currentCall: ToolCall,
  history: ToolCall[]
): boolean {
  if (history.length < DOOM_LOOP_THRESHOLD) return false
  const lastN = history.slice(-DOOM_LOOP_THRESHOLD)
  return lastN.every(
    call =>
      call.tool === currentCall.tool &&
      JSON.stringify(call.input) === JSON.stringify(currentCall.input)
  )
}
```

### 3. 优化系统提示词 ✅

**文件**: `fixer/src/repair.ts`

新的系统提示词让 LLM 像 Claude Code 一样工作：

```typescript
const system = [
  "你是 Rust 编译错误修复专家。",
  "",
  "工作流程：",
  "1. 查看编译错误列表",
  "2. 选择一个你能修复的错误",
  "3. 用 Read 读取相关文件",
  "4. 用 Edit 修复错误（old_string 必须精确匹配文件内容）",
  "5. 继续修复下一个错误，直到没有你能修复的错误",
  "",
  "重要规则：",
  "- 每次 Edit 前必须先 Read 文件",
  "- 如果 Edit 失败，重新 Read 文件再试",
  "- 不要一次尝试修复太多地方，逐步进行",
  "- 完成后简要说明你做了什么",
  "",
  "当你认为已经修复了所有能修复的错误时，说\"修复完成\"并停止。",
].join("\n")
```

### 4. 智能重试机制 ✅

**新文件**: `fixer/src/retry.ts`

处理瞬态错误，支持指数退避重试：

```typescript
export const RETRYABLE_ERRORS = [
  "rate_limit", "timeout", "connection_error", "overloaded",
  "负载较高", "load", "AI_RetryError", "No response",
  "模型无返回结果", "ECONNRESET", "ETIMEDOUT", "503", "502", "500",
]

export async function withRetry<T>(
  fn: () => Promise<T>,
  options?: { maxRetries?: number; onRetry?: Function }
): Promise<T>
```

### 5. 会话持久化 ✅

**文件**: `fixer/src/repair.ts` + `fixer/src/core-manager.ts`

- 使用 CoreManager 复用 Core 实例
- 整个修复过程使用同一个会话
- 每次迭代发送新的错误信息，让 LLM 继续修复

---

## 文件清单

| 文件 | 状态 | 描述 |
|------|------|------|
| `src/repair.ts` | ✅ 已更新 | 优化提示词、移除批量诊断、集成重试 |
| `src/diagnostic-batcher.ts` | ❌ 已删除 | 不再需要批量诊断 |
| `src/doom-loop.ts` | ✅ 新增 | 死循环检测 |
| `src/retry.ts` | ✅ 新增 | 智能重试机制 |
| `src/core-manager.ts` | ✅ 已存在 | Core 实例复用 |
| `src/cache/file-cache.ts` | ✅ 已存在 | 文件内容缓存 |
| `src/cache/diagnostic-cache.ts` | ✅ 已存在 | 诊断摘要缓存 |

---

## 核心改进

### 之前的问题

| 问题 | 描述 | 影响 |
|------|------|------|
| **批量诊断混乱** | 尝试批量处理多个错误导致 LLM 幻觉 | Edit 频繁失败 |
| **无死循环检测** | LLM 可能重复尝试同样的修改 | 浪费 API 调用 |
| **错误恢复差** | Edit 失败后没有智能重试机制 | 修复成功率低 |

### 现在的解决方案

| 特性 | 实现 | 效果 |
|------|------|------|
| **简化输入** | 传递所有错误，让 LLM 自主决定 | LLM 会先 Read 再 Edit，避免幻觉 |
| **死循环检测** | `doom-loop.ts` 模块 | 防止重复失败操作 |
| **智能重试** | `retry.ts` 模块 + 指数退避 | 自动恢复瞬态错误 |
| **会话复用** | `core-manager.ts` | 保持上下文，减少初始化开销 |

---

## 预期效果

| 指标 | 优化前 | 优化后 |
|------|--------|--------|
| Edit 成功率 | ~30% | ~90% |
| 单个错误修复时间 | 不稳定 | 稳定 |
| API 调用浪费 | 高（重复失败） | 低（死循环检测） |
| 复杂错误处理 | 差 | 好（持续迭代） |

---

## 关于循环执行模式

阶段 2 的"循环执行模式"暂时搁置，原因是：

1. OpenCode 的 `Core.SessionPrompt.prompt` 已经内置了工具执行循环
2. LLM 可以在单次 prompt 调用中执行多轮工具调用
3. 当前架构通过外层的 `while` 循环 + cargo check 实现类似效果
4. 这种方式更符合 Rust 修复的场景：每次 LLM 修改后需要重新编译验证

如果未来需要，可以考虑添加更细粒度的循环控制，但目前的架构已经足够。
