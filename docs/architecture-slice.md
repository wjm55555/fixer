# Rust Fixer 独立架构

## 目标
`fixer` 作为独立模块运行，不再依赖本地 `opencode-dev` 代码树。

## 当前调用链
`repairRustProject()` → `core-shim`（会话/模型调用/文件编辑）→ DeepSeek API → 迭代检查（fmt/check/test）

## 核心模块
- `src/repair.ts`: 修复状态机与迭代逻辑
- `src/core-shim.ts`: 独立运行时（替代原本对 opencode core 的依赖）
- `src/fixed-provider.ts`: Provider 配置加载（env + 本地文件）
- `src/python-bridge.ts` / `src/python-bridge-cli.ts`: Python 调用桥接
- `python/fixer_service.py`: HTTP 服务入口

## 配置策略
- 优先环境变量（`FIXER_BASE_URL`、`FIXER_API_KEY`）
- 回退到 `fixer/config/provider.local.json`
- 模板：`fixer/config/provider.example.json`
