# Troubleshooting

## Provider errors
Ensure `/Users/wujiaming/Desktop/rust修复/api的key以及平台.md` exists and contains:
- `base_url=...`
- `模型为 ...`
- `API_KEY:...`

## Cargo failures
Inspect `.fixer/logs/check.log` and `.fixer/logs/clippy.log` in the output directory.

## Permission issues
Review `constraints.allowedCommands` if cargo or git commands are blocked.
