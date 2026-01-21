# Rust Fixer Module

This package is a headless repair module specialized for C→Rust translation projects. It accepts a source Rust workspace, an optional original C source directory for context, and an output directory. It runs deterministic checks (`cargo fmt/check/clippy`) and uses an LLM loop when needed to repair compile errors.

## Quickstart
```ts
import { repairRustProject } from "opencode-rust-fix"

const result = await repairRustProject({
  workspaceDir: "/path/to/rust-project",
  sourceDir: "/path/to/c-source",
  outputDir: "/path/to/output",
  constraints: {
    maxIterations: 3,
    requireCargoTest: false,
  },
})
```

See:
- `docs/quickstart.md`
- `docs/api.md`
- `docs/security.md`
- `docs/troubleshooting.md`

## Inputs
- `workspaceDir`: translated Rust project root (crate/workspace).
- `sourceDir`: optional C source root for context.
- `outputDir`: where fixes are applied; the module copies the workspace into this directory.
- `constraints`: optional limits (iterations, time budget, allowed commands, size caps).

## Outputs
`RepairResult` includes:
- `status`: success/partial/failed
- `diff`: unified diff against the original workspace
- `changedFiles`: touched files
- `metrics`: checks/tests and iteration counts
- `artifacts`: logs and per-iteration patches under `.fixer/`

## Security Model
- All write/patch operations are restricted to `outputDir`.
- Shell commands are executed via a strict allowlist (default: cargo/rg/git diff).
- Command output and log sizes can be capped via `constraints.maxLogBytes`.
- Workspace size can be capped via `constraints.maxWorkspaceBytes`.

## Fixed Provider
Model configuration is read from:
`/Users/wujiaming/Desktop/rust修复/api的key以及平台.md`
