---
phase: 9
name: Boolean Repair and Tolerance Foundation
status: FAIL
plans_total: 5
plans_complete: 5
tdd_compliance: N/A (not checked)
deviations_auto_fix: 26
deviations_approval_needed: 0
---

## What Was Built

### Plan 9-1: Tolerance Documentation and Regression Tests
- Added `//! # Numeric Tolerance Policy` module doc to `monstertruck-core/src/tolerance.rs` (32-line doc block covering all constants, traits, OperationTolerance, and local-constant guidance)
- Replaced hardcoded `1.0e-6` in `monstertruck-solid/src/fillet/edge_select.rs` with imported `TOLERANCE`
- Created `monstertruck-core/tests/tolerance_policy.rs` with 4 regression tests (4/4 pass)

### Plan 9-2: Boolean Face Classification Hardening
- Replaced `integrate_by_component` with robust majority-edge scoring using FxHashSet
- Added resilient fallback (false / outside-OR) for unknown face classification failures
- Implemented 3-stage shell healing fallback (healed > unhealed > original; never returns None)
- Added diagnostic logging behind env vars (`MT_BOOL_DEBUG_COINCIDENT`, etc.)
- Added 6 new passing unit tests (heal resilience, classify, cap early-return)

### Plan 9-3: Boolean Pipeline Tests and Tolerance Documentation
- Added 4 topology-validating boolean unit tests and strengthened `adjacent_cubes_or`
- Added 5 inline doc comments in `integrate/mod.rs` and `loops_store/mod.rs` explaining tolerance rationale

### Plan 9-4: monstertruck-meshing Tolerance Imports
- Added explicit `use monstertruck_core::tolerance::{TOLERANCE, TOLERANCE2}` imports to 4 meshing files (tessellation/mod.rs, tessellation/triangulation.rs, tessellation/boundary_stitching.rs, analyzers/collision.rs)

### Plan 9-5: MissingPolygon Root Cause Fix and Shell Welding
- Fixed `Matrix4::from_translation` column placement bug (wrong column caused corrupted extrusion geometry)
- Added `weld_compressed_shell()` to merge coincident vertices/edges in boolean results
- Fixed `detect_coincident_faces` self-comparison bug
- Result: 5 previously-failing boolean tests now pass; `boolean_edge_cases` integration suite 7/7 pass

## Requirement Coverage

| Requirement | Plans | Status |
|-------------|-------|--------|
| BOOL-01 | 9-2, 9-3, 9-5 | PARTIAL -- boolean_edge_cases integration tests 7/7 pass; but `adjacent_cubes_or` and `crossing_edges` lib tests still fail; `punched_cube` times out |
| TEST-02 | 9-1, 9-3, 9-4 | PASS -- monstertruck-core/src/tolerance.rs documented; monstertruck-solid (9 files), monstertruck-meshing (8 files), monstertruck-modeling (1 file) all import from monstertruck-core::tolerance |

## Test Results

- `monstertruck-core` lib: 10/10 pass
- `monstertruck-core` tolerance_policy: 4/4 pass
- `monstertruck-solid` `--test boolean_edge_cases`: 7/7 PASS
- `monstertruck-solid` `--lib -E 'test(transversal)'`: 24 passed, 2 failed, 1 timed out
  - FAIL: `transversal::integrate::tests::adjacent_cubes_or` (assertion: 2 shells != 1)
  - FAIL: `transversal::loops_store::tests::crossing_edges` (assertion: 1 loop != 2)
  - TIMEOUT: `transversal::integrate::tests::punched_cube` (>180s)

## TDD Compliance

Not re-checked; prior verification recorded 0% compliance under strict mode (missing REFACTOR commits).

## Deviations

- 26 auto-fix deviations logged across project lifetime (pre-existing compilation errors, TDD cycle adaptations, nalgebra migration fixes)
- 0 approval-needed deviations

## Decisions Made

- Conservative default `false` (outside/OR) for failed unknown-face classification
- FxHashSet for majority-edge scoring (O(1) lookups)
- 3-stage healing: healed > unhealed > original; never returns None
- `weld_compressed_shell` applied as stage 0.5 of `heal_shell_if_needed` on CompressedShell format
- boundary_stitching.rs TOLERANCE import placed inside `#[cfg(test)]` to avoid unused-import warning

## Gaps (Blocking Pass)

1. **Criterion 3**: `cargo test -p monstertruck-solid` has boolean-related test failures -- `adjacent_cubes_or` (coplanar face topology: loops_store cannot place all 4 intersection vertices when solids share a face boundary), `crossing_edges` (loops_store assertion failure), `punched_cube` (timeout on revolution surface boolean). ROADMAP requires zero boolean-related test failures.
2. **Criterion 1**: `adjacent_cubes_or` still fails, which was one of the original v0.3.0 gap tests. The boolean_edge_cases integration suite (7/7) passes, but the lib-level coplanar case is not resolved.
