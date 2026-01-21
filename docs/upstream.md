# Upstream Sync Notes

Record the upstream OpenCode commit in `UPSTREAM_COMMIT`.
When syncing:
1) update `UPSTREAM_COMMIT`
2) run tier1 tests (`bun test`)
3) run tier2/3 on schedule or manually (`FIXER_TIER=2|3 bun test`)
