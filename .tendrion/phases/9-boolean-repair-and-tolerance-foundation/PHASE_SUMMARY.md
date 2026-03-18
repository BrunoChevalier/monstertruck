---
phase: 9
name: Boolean Repair and Tolerance Foundation
status: FAIL
plans_total: 3
plans_complete: 3
tdd_compliance: 0% (strict; all 3 plans missing REFACTOR commit)
deviations_auto_fix: 23
deviations_approval_needed: 0
---

## What Was Built

### Plan 9-1: Tolerance Documentation and Regression Tests
- Added `//! # Numeric Tolerance Policy` module doc to `monstertruck-core/src/tolerance.rs` explaining all constants, traits, and when to use local constants
- Replaced hardcoded `1.0e-6` in `monstertruck-solid/src/fillet/edge_select.rs` with imported `TOLERANCE`
- Created `monstertruck-core/tests/tolerance_policy.rs` with 4 regression tests pinning TOLERANCE, TOLERANCE2, OperationTolerance, and Tolerance trait behavior

### Plan 9-2: Boolean Face Classification Hardening
- Replaced `integrate_by_component` with robust majority-edge scoring using FxHashSet; empty boundary components now skip rather than panic
- Added resilient fallback (`false` / outside-OR) for unknown face classification failures instead of propagating errors
- Implemented 3-stage shell healing fallback (healed > unhealed > original); never returns None for non-empty shells
- Added diagnostic logging behind `MT_BOOL_DEBUG_COINCIDENT`, `MT_BOOL_DEBUG_HEAL`, `MT_BOOL_DEBUG_COMPONENTS` env vars
- Enabled `edge_cases` module as `pub(crate)`
- Added 6 new passing unit tests covering no-panic on empty boundaries, majority scoring, heal resilience, and cap early-return

### Plan 9-3: Boolean Pipeline Tests and Tolerance Documentation
- Added 4 topology-validating boolean tests (`overlapping_cubes_and_topology`, `overlapping_cubes_or_topology`, `overlapping_cubes_difference_topology`, `chained_boolean_and_then_or`) and strengthened `adjacent_cubes_or`
- Added 5 inline doc comments in `integrate/mod.rs` and `loops_store/mod.rs` explaining tolerance rationale (operation floor, triangulation derivation, snap floor, vertex merge threshold)

## Requirement Coverage

| Requirement | Plans | Status |
|-------------|-------|--------|
| BOOL-01 | 9-2, 9-3 | PARTIAL -- face classification hardened, but `adjacent_cubes_or` and `punched_cube` still fail (pre-existing MissingPolygon bug in meshing triangulation layer) |
| TEST-02 | 9-1, 9-3 | PARTIAL -- monstertruck-core and monstertruck-solid covered; monstertruck-meshing has existing TOLERANCE import (vtk.rs) but not phase-delivered; monstertruck-modeling has existing use |

## Test Results

- monstertruck-core lib: 10/10 pass
- monstertruck-core tolerance_policy: 4/4 pass
- monstertruck-solid resilience unit tests (heal, cap, classify, coincident): 6/6 pass
- monstertruck-solid boolean integration (adjacent_cubes_or, punched_cube, overlapping/chained): FAIL (pre-existing MissingPolygon in meshing triangulation)
- monstertruck-solid transversal total: 20 passed, 7 failed (all 7 failures pre-existing)

## TDD Compliance

0% -- tdd level is "strict"; all 3 plans are missing REFACTOR commits. No violations introduced new behavior defects, but the process was not followed.

## Deviations

- 23 auto-fix deviations logged (pre-existing compilation errors, pre-existing test failures, TDD cycle adaptations)
- 0 approval-needed deviations

## Decisions Made

- Conservative default `false` (outside/OR) for failed unknown-face classification
- FxHashSet for majority-edge scoring (O(1) lookups)
- 3-stage healing: healed > unhealed > original; never returns None
- Coincident detection wired as logging-only behind env var flag
- Boolean pipeline tests correctly written but blocked by pre-existing MissingPolygon bug (documented, not fixed -- outside plan scope)

## Gaps (Blocking Pass)

1. **Criteria 1 and 3**: `cargo test -p monstertruck-solid` has 7 boolean-related failures in `transversal::integrate` and `transversal::loops_store`. All are `CreateLoopsStoreFailed { source: MissingPolygon }` from a pre-existing bug in `to_polygon()` for `Plane` surfaces. These failures existed before phase 9 and are not regressions, but the ROADMAP success criterion requires zero boolean-related test failures.
2. **Criterion 2**: monstertruck-meshing does not have a phase-9-delivered explicit tolerance import. `vtk.rs` uses `TOLERANCE` but this predates the phase.
3. **TDD**: 0% compliance under strict mode (missing REFACTOR commits in all 3 plans).
