---
phase: 20-fixture-corpus-and-migration-documentation
plan: 2
tags: [documentation, migration, api]
key-files:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/lib.rs
  - monstertruck-geometry/tests/migration_docs_test.rs
decisions: []
metrics:
  tests_added: 8
  tests_passed: 292
  tests_skipped: 1
  deviations: 0
---

## What was built

- **monstertruck-geometry/src/nurbs/bspline_surface.rs**: Added `# Migration` doc sections with before/after code examples to all five `try_*` functions that have deprecated counterparts (`try_skin`, `try_sweep_rail`, `try_birail1`, `try_birail2`, `try_gordon`). Added `# Example` usage sections to `try_gordon_from_network` and `try_gordon_verified` (new functions with no deprecated counterpart).

- **monstertruck-geometry/src/lib.rs**: Added crate-level `# Migration Guide (v0.5.0+)` section with a quick reference table mapping deprecated functions to their `try_*` replacements and options types, documentation of new functions, a before/after code example, and links to error types.

- **monstertruck-geometry/tests/migration_docs_test.rs**: 8 tests verifying that all migration doc sections exist in the source with expected content markers (`# Migration`, `**Before**`, `**After**`, `# Example`).

## Verification

- `cargo doc --no-deps -p monstertruck-geometry` succeeds; no new warnings introduced (8 pre-existing broken links unrelated to migration docs).
- `cargo nextest run -p monstertruck-geometry` -- 292 tests pass, 1 skipped, 0 failures.

## Deviations

None.
