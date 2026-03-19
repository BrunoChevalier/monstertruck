---
phase: 11-surface-constructors
plan: 1
tags: [geometry, sweep, surface-constructor, nurbs]
key-files:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/tests/bspsurface.rs
decisions:
  - Used SVD-based pseudoinverse in affine_fit_3x3 instead of direct matrix inverse to handle coplanar reference points (rank 2 covariance matrix)
metrics:
  tests_added: 6
  tests_passing: 21
  deviations: 1
  tdd_violations: 0
---

## What was built

### Files modified

- **monstertruck-geometry/src/nurbs/bspline_surface.rs**: Added `sweep_multi_rail` and `sweep_periodic` methods on `impl BsplineSurface<Point3>`, plus private helper `affine_fit_3x3` for least-squares affine fitting with SVD-based pseudoinverse.
- **monstertruck-geometry/tests/bspsurface.rs**: Added 6 integration tests covering both new sweep variants.

### Methods implemented

- `BsplineSurface::sweep_multi_rail(profile, rails, n_sections)` -- Sweeps a profile along 2+ rails with affine fitting. 2-rail case uses birail1-style scale+rotate+translate. 3+ rails use least-squares SVD-based affine fit.
- `BsplineSurface::sweep_periodic(profile, rail, n_sections)` -- Sweeps a profile along a rail with tangent-aligned framing, duplicating the first section as the last to guarantee C0 seam continuity.
- `affine_fit_3x3(ref_pts, target_pts)` -- Private helper computing least-squares affine transform using SVD pseudoinverse. Returns Err for rank < 2 (collinear/coincident points).

## Task commits

| Step | SHA | Message |
|------|-----|---------|
| RED | 576a6c36 | test(geometry): add failing tests for sweep_multi_rail and sweep_periodic |
| GREEN | 12f76333 | feat(geometry): implement sweep_multi_rail and sweep_periodic on BsplineSurface |
| REFACTOR | 37197fd7 | refactor(geometry): hoist invariant computations out of sweep_multi_rail section loop |

## Decisions made

1. **SVD pseudoinverse for affine fitting**: The plan specified direct matrix inverse (`(ref * ref^T)^(-1)`), but 3D reference points lying in a plane produce a rank-2 covariance matrix (singular). Changed to SVD-based pseudoinverse which handles rank-deficient cases while still detecting truly degenerate (rank < 2) configurations. This is a standard numerical approach.

## Deviations from plan

- **Design**: SVD pseudoinverse instead of direct inverse in `affine_fit_3x3` (auto-fixed, no plan change needed).

## Self-check

- [x] `sweep_multi_rail` method exists with documented `# Errors` and `# Examples` sections
- [x] `sweep_periodic` method exists with documented `# Errors` and `# Examples` sections
- [x] `affine_fit_3x3` helper handles rank-deficient cases
- [x] 6 new tests all pass
- [x] All 21 bspsurface integration tests pass (15 existing + 6 new)
- [x] Clippy clean on lib target
- [x] Both methods return `Err` instead of panicking on invalid input
