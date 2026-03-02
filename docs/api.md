# API

## repairRustProject(input)
Executes the repair pipeline on a copied workspace.

### RepairInput
- `workspaceDir`: translated Rust project root.
- `sourceDir?`: optional C source root for context.
- `outputDir`: destination directory (must be empty).
- `constraints?`:
  - `maxIterations`
  - `timeBudgetMs`
  - `runTestsWhenCheckPass` (default `true`)
  - `testCases` (optional list of test filters)
  - `requireCargoTest` (legacy alias)
  - `allowedCommands`
  - `maxLogBytes`
  - `maxWorkspaceBytes`

### Python-friendly API
- `repairRustProjectForPython(input)` in `src/python-bridge.ts`
- CLI bridge: `src/python-bridge-cli.ts` (prints `__FIXER_RESULT__` marker JSON)

### Provider configuration
Load order:
1. `FIXER_BASE_URL` + `FIXER_API_KEY` (env)
2. `fixer/config/provider.local.json`
3. `fixer/config/provider.json`

Template: `fixer/config/provider.example.json`

### Runtime engine
- Default: vendored full runtime (`fixer/vendor/packages/opencode*`)
- Fallback: set `FIXER_ENGINE=shim` to use lightweight shim runtime

### RepairResult
- `status`: success | partial | failed
- `summary`
- `diff`: unified diff from original to output
- `changedFiles`
- `metrics`: cargo check/test, iteration count, clippy usage
- `artifacts`: logs and patch paths
