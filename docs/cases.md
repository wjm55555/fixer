# Case Runs & Failure Snapshots

When a case fails, the output directory contains `.fixer/`:
- `logs/`: cargo and tool logs.
- `patches/`: per-iteration diffs.

To reproduce a failure, rerun the case with the same input crate and a fresh output directory, then inspect `.fixer/logs`.
