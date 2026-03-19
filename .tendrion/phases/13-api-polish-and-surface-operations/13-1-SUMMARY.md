---
phase: 13-api-polish-and-surface-operations
plan: 1
tags: [api, surface-constructors, error-handling, options-structs, diagnostics, blocker-fix]
key-files:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/src/nurbs/surface_diagnostics.rs
  - monstertruck-geometry/src/errors.rs
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/nurbs/mod.rs
  - monstertruck-geometry/src/nurbs/offset.rs
  - monstertruck-geometry/tests/try_surface_constructors_test.rs
decisions: []
metrics:
  tests_added: 36
  files_created: 5
  files_modified: 5
---

## What Was Built

### New Files
- **`surface_options.rs`**: Typed option structs (`SweepRailOptions`, `Birail1Options`, `Birail2Options`, `GordonOptions`, `SkinOptions`) with `Default` impls and `FrameRule` enum.
- **`surface_diagnostics.rs`**: `CurveNetworkDiagnostic` enum with 7 variants (`InsufficientCurves`, `InsufficientSections`, `EndpointMismatch`, `DomainMismatch`, `GridDimensionMismatch`, `CompatNormalizationFailed`, `DegenerateGeometry`) plus `Display` and `Error` trait impls.
- **`tests/surface_types_test.rs`**: 12 tests covering option struct defaults and diagnostic Display/PartialEq.
- **`tests/error_variants_test.rs`**: 5 tests covering new Error enum variants.
- **`tests/try_surface_constructors_test.rs`**: 10 tests covering try_sweep_rail, try_birail1, try_birail2 success and error paths.
- **`tests/try_gordon_skin_test.rs`**: 9 tests covering try_gordon, try_skin success and error paths plus deprecated API backward compat.

### Modified Files
- **`errors.rs`**: Added `CurveNetworkIncompatible(CurveNetworkDiagnostic)`, `InsufficientSections`, `SurfaceConstructionFailed` variants and `From<CurveNetworkDiagnostic>` impl. Updated `print_messages` test.
- **`bspline_surface.rs`**: Added `try_sweep_rail`, `try_birail1`, `try_birail2`, `try_skin`, `try_gordon` methods with full input validation and `Result` returns. Old positional-parameter methods (`sweep_rail`, `birail1`, `birail2`, `gordon`, `skin`) deprecated with `#[deprecated]`.
- **`mod.rs`**: Added `pub mod surface_options` and `pub mod surface_diagnostics`.
- **`offset.rs`**: Updated `surface_offset` to use `try_skin` instead of deprecated `skin`.

### Blocker Fixes (spec compliance review)
- **B1 (EndpointMismatch):** Added proximity check in `try_birail1` between profile start and rail1 start. Returns `CurveNetworkDiagnostic::EndpointMismatch` with coordinates and distance when they differ beyond tolerance.
- **B2 (DegenerateGeometry):** Changed silent fallback (scale=1.0, identity rotation) in `try_birail1` to return `CurveNetworkDiagnostic::DegenerateGeometry` when profile chord is zero-length.
- **S1 (surface_options.rs min_lines):** Accepted as plan self-contradiction per review guidance -- file is functionally complete at 67 lines.

## Task Commits
1. `a99d0c8e` -- test(surface-types): add failing tests for option structs and diagnostic types
2. `e85aa77e` -- feat(surface-types): implement option structs and diagnostic types
3. `55649da8` -- test(errors): add failing tests for curve network error variants
4. `3b493178` -- feat(errors): add CurveNetworkIncompatible, InsufficientSections, SurfaceConstructionFailed variants
5. `a8d68ddc` -- test(surface): add failing tests for try_sweep_rail, try_birail1, try_birail2
6. `5ff39b9d` -- feat(surface): add try_sweep_rail, try_birail1, try_birail2 with option structs
7. `53dec964` -- test(surface): add failing tests for try_gordon and try_skin
8. `4e6dbb48` -- feat(surface): add try_gordon and try_skin with option structs, deprecate old APIs
9. `9981518e` -- test(birail1): add failing tests for endpoint mismatch and degenerate chord validation
10. `1c087977` -- fix(birail1): add endpoint mismatch and degenerate chord validation per spec

## Deviations
- Pre-existing compile errors in `approx_fillet_surface.rs`, `t_nurcc_edge.rs`, `t_mesh.rs`, `t_spline_validation.rs` test code prevent running `--lib` test target. Used `--test` binaries for verification.

## Self-Check
- All 36 tests pass across 4 test files
- Deprecated wrappers compile and function correctly
- All `try_*` methods return `Result` with diagnostic errors instead of panicking
- `try_birail1` now validates endpoint mismatch and degenerate chord per plan spec
