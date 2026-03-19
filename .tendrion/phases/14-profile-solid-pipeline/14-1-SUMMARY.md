---
phase: 14-profile-solid-pipeline
plan: 1
tags: [profile, revolve, sweep, solid, modeling]
key-files:
  - monstertruck-modeling/src/profile.rs
  - monstertruck-modeling/tests/profile_test.rs
  - monstertruck-modeling/src/errors.rs
decisions:
  - "Used Solid::new_unchecked for sweep output because independently-created sweep rail faces do not share topological edges"
  - "Used face-level revolve via ClosedSweep which returns Solid directly (plan incorrectly suggested Shell wrapping)"
  - "Extracted build_end_cap helper for sweep end-cap transform computation"
metrics:
  tests_added: 8
  tests_total: 19
  lines_profile_rs: 534
  lines_profile_test_rs: 320
  tdd_violations: 0
---

## What Was Built

- **`revolve_from_planar_profile`** in `profile.rs`: Generic function that normalizes wire orientation, attaches a planar face, and delegates to `builder::revolve`. Supports full and partial revolves, profiles with holes (produces multi-shell solids).

- **`sweep_from_planar_profile`** in `profile.rs`: Concrete function (uses `Curve`/`Surface` types) that sweeps a planar profile along a B-spline guide curve. Per-edge approach using `builder::try_sweep_rail`, with start/end cap faces and tangent-aware end-cap placement.

- **`edge_curve_to_bspline`** helper: Converts `Line` and `BsplineCurve` edge curves to `BsplineCurve<Point3>` for sweep rail input.

- **`build_end_cap`** helper: Transforms profile wires to the guide's end position using translation + tangent rotation.

- **`UnsupportedCurveType`** error variant in `errors.rs`.

## Task Commits

| Phase    | SHA        | Message |
|----------|------------|---------|
| RED      | `1fb7bfcd` | test(profile): add failing tests for revolve and sweep from planar profile |
| GREEN    | `5e61c44e` | feat(profile): implement revolve_from_planar_profile and sweep_from_planar_profile |
| REFACTOR | `a1bd37c2` | refactor(profile): extract end-cap builder, improve docs and simplify sweep assembly |

## Tests Added (8 new)

1. `revolve_simple_rect` -- 360-degree revolve of XZ-plane rectangle around Y-axis
2. `revolve_torus_topology` -- torus-like solid from offset profile
3. `revolve_partial_angle` -- 90-degree partial revolve
4. `revolve_with_hole` -- profile with outer+inner wire produces multi-shell solid
5. `revolve_open_wire_rejected` -- error on open wire input
6. `sweep_rect_along_line` -- straight-line guide sweep
7. `sweep_rect_along_curve` -- curved B-spline guide sweep
8. `sweep_open_wire_rejected` -- error on open wire input

## Deviations

1. **design/auto-fix**: `sweep_from_planar_profile` uses `Solid::new_unchecked` instead of `debug_new`. The per-edge `try_sweep_rail` approach creates faces that don't share topological edges, causing the shell connectivity check to fail. All solids pass `is_geometric_consistent()`.

## Self-Check

- [x] `profile.rs` contains `revolve_from_planar_profile` (534 lines, min 300)
- [x] `profile_test.rs` contains integration tests (320 lines, min 200)
- [x] All 19 profile tests pass
- [x] All 27 lib tests pass
- [x] Clippy clean (lib + test)
- [x] No TDD violations
