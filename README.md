# Rust Fixer Module

This package is a headless repair module specialized for C→Rust translation projects. It accepts a source Rust workspace, an optional original C source directory for context, and an output directory. It runs deterministic checks (`cargo fmt/check/clippy`) and uses an LLM loop when needed to repair compile errors and failing tests.

## Quickstart
```ts
import { repairRustProject } from "opencode-rust-fix"

const result = await repairRustProject({
  workspaceDir: "/path/to/rust-project",
  sourceDir: "/path/to/c-source",
  outputDir: "/path/to/output",
  constraints: {
    maxIterations: 3,
    runTestsWhenCheckPass: true,
    testCases: [], // optional: ["module::case_name"]
  },
})
```

See:
- `docs/quickstart.md`
- `docs/api.md`
- `docs/security.md`
- `docs/troubleshooting.md`
- `docs/python-integration-zh.md` (Python 调用说明，含函数/CLI/HTTP 三种方式)

## Inputs
- `workspaceDir`: translated Rust project root (crate/workspace).
- `sourceDir`: optional C source root for context.
- `outputDir`: where fixes are applied; the module copies the workspace into this directory.
- `constraints`: optional limits (iterations, time budget, allowed commands, size caps).
  - `runTestsWhenCheckPass` defaults to `true`
  - `testCases` optionally restricts `cargo test` to specific test names

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
Provider configuration loading order:
1. Environment variables: `FIXER_BASE_URL`, `FIXER_API_KEY` (optional: `FIXER_MODEL`, `FIXER_PROVIDER_ID`, `FIXER_FALLBACK_MODELS`)
2. `fixer/config/provider.local.json`
3. `fixer/config/provider.json`

Template:
`fixer/config/provider.example.json`

## Dependency note
`fixer` can run standalone and no longer requires local `opencode-dev` runtime.
The default runtime is vendored in `fixer/vendor/packages/*`.
Set `FIXER_ENGINE=shim` only for fallback troubleshooting.

## Python Bridge

### TS bridge entry
- CLI: `bun run src/python-bridge-cli.ts '<json_payload>'`
- Output includes a marker line: `__FIXER_RESULT__{...json...}`

### Python helper
- File: `python/fixer_bridge.py`
- Function: `repair_rust_project(...)`

### Minimal HTTP service (for Python projects)
- File: `python/fixer_service.py`
- Start: `python3 python/fixer_service.py --host 127.0.0.1 --port 8787`
- Health: `GET /health`
- Repair: `POST /repair` (JSON body)

Example request:
```bash
curl -sS http://127.0.0.1:8787/repair \
  -H "Content-Type: application/json" \
  -d '{
    "rustProjectDir": "/path/to/rust-project",
    "outputRustProjectDir": "/path/to/output",
    "sourceProjectDir": "/path/to/c-source",
    "maxIterations": 0,
    "runTestsWhenCheckPass": true
  }'
```
