---
phase: 30-new-surface-constructors
plan: 1
tags: [ruled-surface, geometry, modeling, constructor]
key-files:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-modeling/src/builder.rs
  - monstertruck-modeling/src/lib.rs
  - monstertruck-modeling/tests/surface_constructors.rs
decisions: []
metrics:
  tests_added: 4
  tests_total: 253
  lines_added: ~70
---

## What was built

- **`RuledSurfaceOptions`** struct in `surface_options.rs` -- marker options struct following the existing `SkinOptions` pattern (`#[non_exhaustive]`, `Default`, `Debug`, `Clone`).
- **`BsplineSurface::try_ruled`** method in `bspline_surface.rs` -- fallible ruled surface construction with input validation before `syncro_degree`/`syncro_knots` to prevent panics on empty curves. Returns `Error::EmptyControlPoints` for degenerate input.
- **`builder::try_ruled_surface`** in `builder.rs` -- modeling-level wrapper that constructs a `Face` with 4-edge boundary wire using `face_from_bspline_surface` helper.
- **`RuledSurfaceOptions` re-export** in `lib.rs` for user convenience.
- **4 new tests** in `surface_constructors.rs`:
  - `ruled_surface_happy_path` -- verifies v=0/v=1/v=0.5 surface evaluation matches input curves and midpoint.
  - `ruled_surface_different_degrees` -- verifies syncro_degree normalization with mixed-degree curves.
  - `ruled_surface_empty_curve_error` -- verifies `Error::FromGeometry` on empty control points (no panic).
  - `ruled_surface_single_point_no_panic` -- verifies degree-0 single-point curve does not panic.

## Task commits

| Step | SHA | Message |
|------|-----|---------|
| RED | `b7f2a409` | `test(ruled-surface): add failing tests for ruled surface construction` |
| GREEN | `d0954178` | `feat(ruled-surface): implement try_ruled_surface constructor with input validation` |

## Deviations from plan

None. The plan's task ordering placed tests last (Task 3), but TDD strict mode required tests first. Implementation followed TDD RED-GREEN-COMPLETE cycle.

## Self-check

- `cargo nextest run -p monstertruck-geometry --lib`: 114 passed
- `cargo nextest run -p monstertruck-modeling`: 139 passed (including 4 new ruled surface tests)
- `cargo clippy`: no new warnings
- `cargo doc -p monstertruck-modeling --no-deps`: success
- All artifact min_lines constraints met (136/130, 1691/1100, 428/60)
