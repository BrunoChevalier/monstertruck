---
phase: 21-edge-identity-and-topology-repair
verified: true
tdd_compliance: 100%
requirements_covered: ETOPO-01, ETOPO-02
---

# Phase 21 Summary: Edge Identity and Topology Repair

## What Was Built

Three targeted changes to the fillet conversion pipeline:

1. **`monstertruck-solid/src/fillet/topology.rs`** -- `ensure_cuttable_edge` refactored to use `edge.set_curve()` for in-place curve mutation instead of `Edge::new()`. Changed visibility to `pub(super)`. This preserves `EdgeId` so `is_same()` returns true before and after IntersectionCurve-to-NURBS conversion, enabling `cut_face_by_bezier` boundary replacement (lines 92-93) to locate converted edges correctly.

2. **`monstertruck-solid/src/fillet/convert.rs`** -- `convert_shell_in` endpoint matching widened from `near()` (TOLERANCE=1e-6) to `abs_diff_eq(..., SNAP_TOLERANCE)` (1e-5). Added `SNAP_TOLERANCE` import; removed unused `Tolerance` import.

3. **`monstertruck-solid/src/fillet/tests.rs`** -- Two new tests:
   - `ensure_cuttable_edge_preserves_identity`: Builds an IntersectionCurve edge, calls `ensure_cuttable_edge`, asserts `edge.is_same(&converted)` is true and curve is NurbsCurve.
   - `convert_shell_in_tolerant_endpoint_matching`: Verifies endpoint matching succeeds with 5e-6 offset (above TOLERANCE, within SNAP_TOLERANCE).

## Requirement Coverage

| Requirement | Status | Evidence |
|-------------|--------|----------|
| ETOPO-01 | Covered | `set_curve()` in `ensure_cuttable_edge` (topology.rs:38); `is_same()` assertion in test (tests.rs:3639) |
| ETOPO-02 | Covered | `abs_diff_eq(..., SNAP_TOLERANCE)` in `convert_shell_in` (convert.rs:156-159); tolerant matching test (tests.rs:3648) |

## Test Results

- Plans total: 1 / Plans with summary: 1 (100% complete)
- Tests added: 2
- Tests passing: 110
- Pre-existing failures: 6 (unrelated: test_unit_circle, generic_fillet_* tests)
- TDD cycle: RED (aa8eb8f5) -> GREEN (58b78600) -> REFACTOR (7faebbfa)

## TDD Compliance

100% compliant (strict mode). 1 cycle, 0 violations.

## Deviations

- Auto-fix deviations in project: 49 (all pre-existing, none from this phase)
- Approval-needed: 0
- Phase-specific deviations: 6 pre-existing test failures confirmed not caused by this plan's changes.

## Decisions Made

No architectural decisions required. The fix was a targeted in-place mutation change with a tolerance constant substitution.
