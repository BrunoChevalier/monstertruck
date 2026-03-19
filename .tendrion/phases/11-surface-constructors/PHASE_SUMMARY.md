---
phase: 11
name: Surface Constructors
status: complete
plans_total: 2
plans_complete: 2
requirements: [SURF-01, SURF-02]
tdd: compliant
---

## What Was Built

**Plan 11-1 (monstertruck-geometry):**
- `BsplineSurface::sweep_multi_rail(profile, rails, n_sections)` -- SVD-based affine fitting for 2+ rails; 3+ rails use least-squares pseudoinverse to handle coplanar reference points.
- `BsplineSurface::sweep_periodic(profile, rail, n_sections)` -- Tangent-aligned framing with duplicated first section ensuring C0 seam continuity.
- Private `affine_fit_3x3` helper using SVD pseudoinverse (handles rank-deficient/coplanar inputs).
- 6 integration tests added; all 21 bspsurface tests pass.

**Plan 11-2 (monstertruck-modeling):**
- 4 error variants added to `monstertruck_modeling::Error`: `InsufficientRails`, `InsufficientSections`, `SurfaceConstructionFailed`, `GridDimensionMismatch`.
- 5 public builder wrappers: `try_sweep_rail`, `try_birail`, `try_gordon`, `try_sweep_multi_rail`, `try_sweep_periodic` -- all return typed `Result` errors.
- `try_sweep_periodic` returns `Result<Shell>` for closed-surface topology.
- 13 integration tests covering all wrappers, error paths, seam continuity, and Euler-Poincare checks.

## Requirement Coverage

| Requirement | Plan | Status |
|-------------|------|--------|
| SURF-01 | 11-1 | Covered -- `sweep_multi_rail` and `sweep_periodic` on `BsplineSurface<Point3>` |
| SURF-02 | 11-2 | Covered -- 5 typed builder wrappers returning `Result<_, Error>` |

## Test Results

- monstertruck-geometry bspsurface: 21/21 passed
- monstertruck-modeling surface_constructors: 13/13 passed
- TDD: compliant (RED/GREEN/REFACTOR commits in both plans)

## Deviations

- Plan 11-1: 1 auto-fix -- SVD pseudoinverse instead of direct inverse in `affine_fit_3x3` (handles rank-deficient geometry; no approval needed).
- Plan 11-2: 1 auto-fix -- Fixed pre-existing compilation errors in `geom_impls.rs` proptest (`.angle()` method signature).
- Global DEVIATIONS.md: 30 auto-fix entries (all phases), 0 approval-needed.

## Decisions Made

1. Free functions (`builder::try_*`) used instead of `SweepBuilder` type -- matches existing codebase convention; documented as intentional in plans.
2. SVD pseudoinverse in `affine_fit_3x3` -- plan specified direct inverse, but coplanar reference points produce singular matrix; SVD is the standard numerical fix.
3. `try_sweep_periodic` returns `Result<Shell>` (not `Face`) -- periodic sweep produces a closed cylindrical surface requiring shell topology.

## TDD Compliance

Both plans followed RED/GREEN/REFACTOR cycle with separate commits at each stage. No TDD violations.
