# Troubleshooting

## Provider errors
Ensure provider config is set by either:
- environment variables: `FIXER_BASE_URL` and `FIXER_API_KEY`
- or local file: `fixer/config/provider.local.json` (see `fixer/config/provider.example.json`)

## Cargo failures
Inspect `.fixer/logs/check.log` and `.fixer/logs/clippy.log` in the output directory.

## Permission issues
Review `constraints.allowedCommands` if cargo or git commands are blocked.
