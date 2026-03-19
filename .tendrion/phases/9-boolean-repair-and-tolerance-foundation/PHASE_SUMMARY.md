---
phase: 9
name: Boolean Repair and Tolerance Foundation
status: PASS
plans_total: 5
plans_complete: 5
tdd_compliance: standard
deviations_auto_fix: 26
deviations_approval_needed: 0
verified_at: 2026-03-19
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

### Plan 9-5: Boolean Debug Fixes (adjacent_cubes_or, crossing_edges, punched_cube)
- Added `split_edges_at_intermediate_vertices()` to detect and split edges passing through merged intermediate vertices after welding
- Added midpoint verification to `has_edge_between_points()` to distinguish boundary segments from intersection curve duplicates
- Pre-convert known faces before hill-climbing; removed triangulation from capping; reduced Newton trials in edge splitting
- Result: all 27 transversal lib tests pass; boolean_edge_cases integration suite 7/7 pass

## Requirement Coverage

| Requirement | Plans | Status |
|-------------|-------|--------|
| BOOL-01 | 9-2, 9-3, 9-5 | COVERED -- all transversal lib tests 27/27 pass including adjacent_cubes_or, crossing_edges, punched_cube |
| TEST-02 | 9-1, 9-4 | COVERED -- tolerance.rs documented with rationale; monstertruck-solid, monstertruck-meshing, and monstertruck-modeling (via `pub use base::*` re-export of `tolerance::*`) all import from monstertruck_core::tolerance |

## Test Results

- `monstertruck-solid` `--lib -E 'test(transversal)'`: 27/27 PASS (0 failures, 83 skipped)
  - `transversal::integrate::tests::adjacent_cubes_or`: PASS
  - `transversal::loops_store::tests::crossing_edges`: PASS
  - `transversal::integrate::tests::punched_cube`: PASS
- `monstertruck-solid` `--test boolean_edge_cases`: 7/7 PASS
- CLI pre-checks: all passed (5/5 plans with summaries, 0 structural errors)

## TDD Compliance

Standard TDD: test additions accompanied each implementation change. All tests added in plans 9-2, 9-3, and 9-5 verify actual behavior (topology assertion counts, face classification correctness, edge loop construction).

## Deviations

- 26 auto-fix deviations logged across project lifetime (pre-existing compilation errors, TDD cycle adaptations, nalgebra migration fixes -- none introduced by phase 9)
- 0 approval-needed deviations

## Decisions Made

- Conservative default `false` (outside/OR) for failed unknown-face classification
- FxHashSet for majority-edge scoring (O(1) lookups)
- 3-stage healing: healed > unhealed > original; never returns None
- `boundary_stitching.rs` TOLERANCE import placed inside `#[cfg(test)]` to avoid unused-import warning
- `split_edges_at_intermediate_vertices()` added to repair welded shells with edges passing through merged vertices
- Midpoint verification in `has_edge_between_points()` to correctly distinguish boundary vs. intersection curve edges
- Pre-convert known faces before hill-climbing in punched_cube path; remove triangulation from capping
