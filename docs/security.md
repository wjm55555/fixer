# Security Model

- The module runs in a copied `outputDir`, never mutating the input workspace.
- Writes/patches are restricted to `outputDir`.
- Shell commands are restricted to an allowlist (default cargo/rg/git diff).
- Command output and log files can be capped via `maxLogBytes`.
- Workspace size can be capped via `maxWorkspaceBytes`.
