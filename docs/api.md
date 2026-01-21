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
  - `requireCargoTest`
  - `allowedCommands`
  - `maxLogBytes`
  - `maxWorkspaceBytes`

### RepairResult
- `status`: success | partial | failed
- `summary`
- `diff`: unified diff from original to output
- `changedFiles`
- `metrics`: cargo check/test, iteration count, clippy usage
- `artifacts`: logs and patch paths
