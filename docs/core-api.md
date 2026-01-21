# opencode-core（最小 API 草案）

## withCore(options, fn)
在指定 `workspaceDir` 上初始化 OpenCode 核心运行时，并在该上下文内执行 `fn`。

### CoreOptions
- `workspaceDir`：目标工作区路径（必填）。
- `dataDir`：运行时数据目录（默认 `${workspaceDir}/.opencode-core`）。
- `config`：内联配置对象（会写入 `OPENCODE_CONFIG_CONTENT`）。
- `permission`：权限规则集（等同 `OPENCODE_PERMISSION`）。
- `onEvent`：订阅事件回调（所有 Bus 事件）。
- `abort`：外部取消信号（建议与 `promptWithAbort` 结合使用）。
- `logLevel`：日志级别。

### CoreContext
仅包含 `workspaceDir`（后续可扩展为 session/metrics）。

## Core 导出
`Core.Session`、`Core.SessionPrompt`、`Core.Provider`、`Core.PermissionNext`、`Core.Config`、`Core.Instance`、`Core.Bus`

## Abort 相关
- `bindAbortToSession(sessionID, abortSignal)`：将 AbortSignal 绑定到 session，触发时自动 cancel。
- `promptWithAbort({ prompt, abort })`：带取消支持的 prompt 包装。

## 示例
见 `examples/core-smoke/index.ts`。
