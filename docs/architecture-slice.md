# OpenCode → Rust Fixer 架构切片

## 目标
将 `opencode-dev` 中可复用的运行时能力抽为无 UI 的修复内核，并在 `fixer` 里构建 Rust 修复流水线与调用 API。

## 保留/复用清单（核心运行时）
- `packages/opencode/src/session`: 会话编排、消息/步骤、LLM 调用入口。
- `packages/opencode/src/provider`: 模型提供方与配置解析。
- `packages/opencode/src/tool`: 工具系统（bash/read/write/patch/grep/glob 等）。
- `packages/opencode/src/permission`: 权限与命令白名单（需改成无交互策略）。
- `packages/opencode/src/config`: 配置 schema/解析（后续改为“代码注入优先”）。
- `packages/opencode/src/project` + `worktree` + `snapshot`: 工作区、diff/回滚能力。
- `packages/opencode/src/storage`, `bus`, `util`, `id`, `format`: 运行时依赖的基础设施。

## 删除/剥离清单（强 UI 依赖）
- `packages/app`, `packages/desktop`, `packages/web`, `packages/console`, `packages/ui`, `packages/slack` 等 UI/客户端包。
- `packages/opencode/src/cli/**`, `packages/opencode/src/cli/cmd/tui/**`, `packages/opencode/src/cli/ui`。
- `packages/opencode/src/server/routes/tui`（仅 TUI API）。

## 可参考但不直接依赖
- `packages/opencode/src/cli/cmd/run.ts`: 非交互运行模式的实现逻辑参考。
- `packages/opencode/src/server/**`: headless 服务模式参考（但最终改为库内 API）。

## 最小可运行调用链（目标）
`repairRustProject()` → `opencode-core` → `Session`/`Agent` → `Provider` → `Tool` → `Permission` → `bash/read/write/patch`

## 风险点
- `Session` 强依赖 `Instance`/`Storage`/`Snapshot`，需要在库内可控初始化与可选禁用。
- `Permission` 目前含交互询问路径，需替换为策略化自动允许/拒绝。
- `Config` 依赖用户目录配置；库模式必须允许显式参数注入并支持默认安全配置。
