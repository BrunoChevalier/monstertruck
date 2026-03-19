---
phase: 13-api-polish-and-surface-operations
plan: 1
tags: [api, surface-constructors, error-handling, options-structs, diagnostics]
key-files:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/src/nurbs/surface_diagnostics.rs
  - monstertruck-geometry/src/errors.rs
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/nurbs/mod.rs
  - monstertruck-geometry/src/nurbs/offset.rs
decisions: []
metrics:
  tests_added: 34
  files_created: 5
  files_modified: 4
---

## What Was Built

### New Files
- **`surface_options.rs`**: Typed option structs (`SweepRailOptions`, `Birail1Options`, `Birail2Options`, `GordonOptions`, `SkinOptions`) with `Default` impls and `FrameRule` enum.
- **`surface_diagnostics.rs`**: `CurveNetworkDiagnostic` enum with 7 variants (`InsufficientCurves`, `InsufficientSections`, `EndpointMismatch`, `DomainMismatch`, `GridDimensionMismatch`, `CompatNormalizationFailed`, `DegenerateGeometry`) plus `Display` and `Error` trait impls.
- **`tests/surface_types_test.rs`**: 12 tests covering option struct defaults and diagnostic Display/PartialEq.
- **`tests/error_variants_test.rs`**: 5 tests covering new Error enum variants.
- **`tests/try_surface_constructors_test.rs`**: 8 tests covering try_sweep_rail, try_birail1, try_birail2 success and error paths.
- **`tests/try_gordon_skin_test.rs`**: 9 tests covering try_gordon, try_skin success and error paths plus deprecated API backward compat.

### Modified Files
- **`errors.rs`**: Added `CurveNetworkIncompatible(CurveNetworkDiagnostic)`, `InsufficientSections`, `SurfaceConstructionFailed` variants and `From<CurveNetworkDiagnostic>` impl. Updated `print_messages` test.
- **`bspline_surface.rs`**: Added `try_sweep_rail`, `try_birail1`, `try_birail2`, `try_skin`, `try_gordon` methods with full input validation and `Result` returns. Old positional-parameter methods (`sweep_rail`, `birail1`, `birail2`, `gordon`, `skin`) deprecated with `#[deprecated]`.
- **`mod.rs`**: Added `pub mod surface_options` and `pub mod surface_diagnostics`.
- **`offset.rs`**: Updated `surface_offset` to use `try_skin` instead of deprecated `skin`.

## Task Commits
1. `a99d0c8e` -- test(surface-types): add failing tests for option structs and diagnostic types
2. `e85aa77e` -- feat(surface-types): implement option structs and diagnostic types
3. `55649da8` -- test(errors): add failing tests for curve network error variants
4. `3b493178` -- feat(errors): add CurveNetworkIncompatible, InsufficientSections, SurfaceConstructionFailed variants
5. `a8d68ddc` -- test(surface): add failing tests for try_sweep_rail, try_birail1, try_birail2
6. `5ff39b9d` -- feat(surface): add try_sweep_rail, try_birail1, try_birail2 with option structs
7. `53dec964` -- test(surface): add failing tests for try_gordon and try_skin
8. `4e6dbb48` -- feat(surface): add try_gordon and try_skin with option structs, deprecate old APIs

## Deviations
- Pre-existing compile errors in `approx_fillet_surface.rs` and `t_nurcc_edge.rs` test code prevent running `--lib` test target. Used `--test` binaries for verification.

## Self-Check
- `cargo clippy -p monstertruck-geometry --lib -- -W warnings`: 0 warnings
- All 34 new tests pass
- Deprecated wrappers compile and function correctly
- All `try_*` methods return `Result` with diagnostic errors instead of panicking
