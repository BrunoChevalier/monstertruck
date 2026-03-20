---
phase: 18
name: gordon-surface-variants
status: complete
plans_total: 2
plans_complete: 2
tdd_compliant: false
tdd_violations: ["18-1: missing REFACTOR commit (strict mode)", "18-2: missing REFACTOR commit (strict mode)"]
requirements_covered: [GORDON-01, GORDON-02]
deviations: {auto_fix: 44, approval_needed: 0}
---

## What Was Built

### Plan 18-1: Geometry-level variant APIs
- `GordonOptions.grid_tolerance` field added with default from `SNAP_TOLERANCE`.
- New `CurveNetworkDiagnostic` variants: `IntersectionCountMismatch` and `GridPointNotOnCurve`, both with `Display` impls.
- `BsplineSurface::try_gordon_from_network` -- calls `curve_intersect::find_intersections` on each (u, v) curve pair **before** compatibility normalization, expects exactly 1 hit, delegates to `try_gordon`.
- `BsplineSurface::try_gordon_verified` -- validates caller-supplied grid via `search_nearest_parameter`, snaps near-miss points to midpoint of two nearest curve positions, rejects points exceeding tolerance.
- 13 new geometry-level tests; 262 total passing.

### Plan 18-2: Builder wrappers and additional fixtures
- `builder::try_gordon_from_network` and `builder::try_gordon_verified` added to `monstertruck-modeling`, wrapping geometry methods and returning `Face` topology.
- 6 new tests (2 geometry, 4 builder-level); 366 total passing.
- Nonuniform spacing fixture adjusted from 3x2 to 3x3 due to pre-existing `try_gordon` asymmetric-grid bug (logged to DEVIATIONS.md).

## Requirement Coverage

| Requirement | Status | Evidence |
|-------------|--------|----------|
| GORDON-01 | Covered | `try_gordon_from_network` calls `curve_intersect::find_intersections` before normalization (bspline_surface.rs:2338) |
| GORDON-02 | Covered | `try_gordon_verified` uses `search_nearest_parameter` + snap logic (bspline_surface.rs:2396-2448) |

## Test Results

- **monstertruck-geometry:** 262 passed, 1 skipped, 0 failed (cargo nextest)
- **monstertruck-modeling + geometry:** 366 passed, 0 failed (cargo nextest)
- **New tests:** 19 total (15 geometry-level, 4 builder-level)
- **Key fixtures:** `try_gordon_from_network_nonuniform_spacing`, `try_gordon_verified_snaps_near_miss_points`, `try_gordon_verified_equivalence_with_from_network`
- **TDD compliance:** 0/2 cycles compliant (both missing REFACTOR commit in strict mode)

## Deviations

- **auto-fix:** 44 (cumulative project total; phase 18 contributed 1: asymmetric-grid test adjusted to 3x3)
- **approval-needed:** 0

## Decisions Made

- Adjusted nonuniform spacing test from 3x2 to 3x3 grid to avoid pre-existing `try_gordon` panic with asymmetric grids while still exercising nonuniform spacing behavior.

## TDD Compliance

- Level: strict
- Compliant: false
- Violation: both plans are missing a REFACTOR commit; RED and GREEN commits are present in each cycle
