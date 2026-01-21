# Case Library

This directory holds regression cases for the Rust fixer.

## Tiers
- `tier1`: tiny crates that should finish in seconds; required on every PR.
- `tier2`: medium crates with multiple modules; run nightly.
- `tier3`: workspace-style crates; run on a schedule.

## Adding a Case
1) Place the crate under the correct tier folder.
2) Keep the case minimal and deterministic.
3) Ensure it can run without network access.

## Reproducing Failures
Case runs write artifacts under `.fixer/` in the output directory passed to `repairRustProject`.
