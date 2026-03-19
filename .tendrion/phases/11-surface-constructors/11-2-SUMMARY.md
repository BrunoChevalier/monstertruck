---
phase: 11-surface-constructors
plan: 2
tags: [builder, surface-constructors, error-handling, topology]
key-files:
  - monstertruck-modeling/src/builder.rs
  - monstertruck-modeling/src/errors.rs
  - monstertruck-modeling/tests/surface_constructors.rs
  - monstertruck-modeling/src/geom_impls.rs
decisions:
  - "Used free functions (builder::try_*) matching codebase convention, not SweepBuilder type"
  - "try_sweep_periodic returns Result<Shell> for closed-surface topology"
  - "Shared face_from_bspline_surface helper extracts boundary curves from control-point grid"
metrics:
  tests_added: 13
  tests_passed: 13
  deviations: 1
---

## What Was Built

**monstertruck-modeling/src/errors.rs** -- Added 4 error variants: `InsufficientRails`, `InsufficientSections`, `SurfaceConstructionFailed`, `GridDimensionMismatch`. Updated `print_messages` test.

**monstertruck-modeling/src/builder.rs** -- Added `face_from_bspline_surface` private helper and 5 public builder wrappers:
- `try_sweep_rail` -- sweeps profile along single rail, returns `Result<Face>`.
- `try_birail` -- sweeps profile along two rails, returns `Result<Face>`.
- `try_gordon` -- constructs Gordon surface from curve network, returns `Result<Face>`.
- `try_sweep_multi_rail` -- sweeps profile along 2+ rails with affine fitting, returns `Result<Face>`.
- `try_sweep_periodic` -- sweeps profile along closed rail, returns `Result<Shell>`.

**monstertruck-modeling/tests/surface_constructors.rs** -- 13 integration tests covering all wrappers, error paths, seam continuity, vertex positions, and Euler-Poincare consistency.

**monstertruck-modeling/src/geom_impls.rs** -- Fixed pre-existing compilation errors in proptest: `.angle()` method requires pass-by-reference and returns `f64` (not `Rad<f64>`).

## Task Commits

| SHA | Message |
|-----|---------|
| `3f537cfe` | test(errors): add failing tests for surface construction error variants |
| `3d520261` | feat(errors): add surface construction error variants |
| `a6a90471` | test(builder): add failing tests for try_sweep_rail, try_birail, try_gordon |
| `6838a75b` | feat(builder): implement try_sweep_rail, try_birail, try_gordon with face topology |
| `691afbd0` | test(builder): add failing tests for try_sweep_multi_rail, try_sweep_periodic |
| `f5c077b7` | feat(builder): implement try_sweep_multi_rail and try_sweep_periodic |
| `70c5104a` | test(builder): add Euler-Poincare and vertex position integration tests |

## Deviations

1. **auto-fix/bug**: Pre-existing compilation errors in `geom_impls.rs` proptest -- `.angle()` method requires `&` argument and returns `f64` not `Rad<f64>`. Fixed in first commit.

## Self-Check

- [x] All 5 builder wrappers exist and return typed `Result` errors.
- [x] `try_sweep_periodic` returns `Result<Shell>` (not `Face`).
- [x] `is_geometric_consistent()` passes on periodic sweep shell.
- [x] Seam continuity verified: `subs(u, 0) == subs(u, 1)`.
- [x] 19 lib tests + 13 integration tests pass.
- [x] `cargo clippy` clean on both lib and test targets.
