---
target: "21-1"
type: planning
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-22
verdict: PASS
---

# Planning Review: Plan 21-1

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** Planning
**Date:** 2026-03-22

## Verdict

**PASS**

No blockers found. The plan accurately reflects the current codebase state, covers both ETOPO-01 and ETOPO-02 requirements, and proposes technically sound fixes with appropriate verification steps.

## Findings

### Blockers

None

### Suggestions

#### S1: Test 2 lacks concrete implementation guidance [confidence: 78]
- **Confidence:** 78
- **File:** 21-1-PLAN.md, Task 3 action (lines 190-198)
- **Issue:** Test 2 (`convert_shell_in_tolerant_endpoint_matching`) is described with two alternative approaches ("extend or add a companion test" vs. "verify the existing test passes with a comment") but neither provides a concrete implementation. The plan even says "If constructing offset endpoints is complex, a simpler approach is..." which delegates the design decision to the implementer. For a test that validates ETOPO-02, the plan should provide clearer guidance on constructing the test scenario.
- **Impact:** The implementer may take the easy path (just adding a comment to an existing test) which would not actually validate the tolerance widening in isolation.
- **Suggested fix:** Provide a concrete test skeleton that creates two Point3 values differing by ~5e-6 and verifies they match under SNAP_TOLERANCE but not under TOLERANCE. Even a unit-level test of the matching predicate (extracted or inline) would be more robust than relying on an existing integration test.

#### S2: Visibility change for ensure_cuttable_edge described in wrong task [confidence: 82]
- **Confidence:** 82
- **File:** 21-1-PLAN.md, Task 3 action (lines 174-178) vs Task 1
- **Issue:** Task 3 describes the need to change `ensure_cuttable_edge` from `fn` to `pub(super) fn` in topology.rs, but this file modification is logically part of Task 1's scope (which modifies topology.rs). The `files` field for Task 3 lists only `tests.rs`, not `topology.rs`. This means Task 3's action section describes modifying a file not in its `files` list.
- **Impact:** Minor inconsistency. The implementer will likely handle it correctly regardless, but the task boundary is blurred.
- **Suggested fix:** Either add `monstertruck-solid/src/fillet/topology.rs` to Task 3's `files` field, or move the visibility change to Task 1.

### Nits

#### N1: Duplicate closing output tag [confidence: 95]
- **Confidence:** 95
- **File:** 21-1-PLAN.md, line 230
- **Issue:** There are two consecutive `</output>` closing tags at lines 229-230. The second appears to be a typo.

#### N2: Line references may drift [confidence: 72]
- **Confidence:** 72
- **File:** 21-1-PLAN.md, throughout
- **Issue:** The plan references specific line numbers (e.g., "lines 34-45", "lines 148-162", "lines 96-97") which are accurate today but could drift if any upstream changes land before this plan executes. The function name references are sufficient for navigation.

## Summary

Plan 21-1 is well-structured and technically accurate. The proposed `set_curve()` in-place mutation approach for ETOPO-01 is validated by the existing `set_curve` usage at topology.rs:176 and the `Edge::set_curve` API at edge.rs:288. The SNAP_TOLERANCE widening for ETOPO-02 correctly targets the `near()` calls in the endpoint matching closure and the `abs_diff_eq` method is available on nalgebra Point3 types. Both requirements are covered with appropriate verification steps. The two suggestions are minor and neither blocks execution.
