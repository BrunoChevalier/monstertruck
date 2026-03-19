---
phase: 13-api-polish-and-surface-operations
plan: 3
tags: [modeling, builder, errors, surface-options, diagnostics]
key-files:
  - monstertruck-modeling/src/errors.rs
  - monstertruck-modeling/src/builder.rs
  - monstertruck-modeling/src/lib.rs
decisions:
  - "Kept old try_sweep_rail/try_birail/try_gordon implementations intact rather than delegating to _with_options variants, because delegation changes error variant types from modeling-level (InsufficientSections, GridDimensionMismatch) to FromGeometry wrappers, which would break existing tests that AGENTS.md forbids modifying"
  - "Removed Eq derive from modeling Error enum because geometry Error only derives PartialEq (f64 fields prevent Eq)"
metrics:
  tests_added: 7
  tests_passed: 40
  deviations: 1
---

## What was built

- **monstertruck-modeling/src/errors.rs**: Added `FromGeometry(monstertruck_geometry::errors::Error)` variant with manual `From` impl. Removed `Eq` derive (kept `PartialEq`). Added `from_geometry_error_variant` test and updated `print_messages` test.

- **monstertruck-modeling/src/builder.rs**: Added four option-struct-based builder functions:
  - `try_sweep_rail_with_options` -- accepts `SweepRailOptions`
  - `try_birail_with_options` -- accepts `Birail1Options`
  - `try_birail2_with_options` -- accepts `Birail2Options`
  - `try_gordon_with_options` -- accepts `GordonOptions`

  Each uses `?` propagation to convert geometry errors to `Error::FromGeometry`. Added 7 tests covering success paths and error chain propagation (grid dimension mismatch, insufficient sections, endpoint mismatch).

- **monstertruck-modeling/src/lib.rs**: Re-exported `SweepRailOptions`, `Birail1Options`, `Birail2Options`, `GordonOptions` from the geometry crate for user convenience.

## Task commits

| SHA | Message |
|-----|---------|
| d79c49d6 | feat(modeling): extend Error with FromGeometry variant wrapping geometry-level diagnostics |
| 7ff03171 | feat(modeling): add option-struct-based builder functions with diagnostic error propagation |
| f67d6f66 | test(modeling): add integration tests for full diagnostic error chain |

## Deviations from plan

1. **Old functions not converted to delegates** (auto-fix/design): The plan suggested rewriting existing `try_sweep_rail`, `try_birail`, `try_gordon` to delegate to `*_with_options` variants. This was not done because delegation changes the error variant types from modeling-level (`InsufficientSections`, `GridDimensionMismatch`) to `FromGeometry` wrappers, which breaks existing integration tests in `tests/surface_constructors.rs` that assert on specific error variants. AGENTS.md forbids modifying test files.

## Self-check

- [x] `cargo nextest run -p monstertruck-modeling --lib --test surface_constructors` -- 40 tests pass
- [x] `cargo clippy -p monstertruck-modeling -- -W warnings` -- no warnings
- [x] builder.rs: 1566 lines (min 700)
- [x] errors.rs: 175 lines (min 130)
- [x] builder.rs contains `try_sweep_rail_with_options`
- [x] errors.rs contains `CurveNetworkDiagnostic`
- [x] Backward compatibility: all existing tests pass unchanged
