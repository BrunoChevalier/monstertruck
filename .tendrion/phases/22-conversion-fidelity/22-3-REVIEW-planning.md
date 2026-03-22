---
target: "22-3"
type: planning
round: 2
max_rounds: 3
reviewer: opus-4-6
stage: planning
date: 2026-03-22
verdict: FAIL
---

# Planning Review: 22-3 (Endpoint Snapping)

**Reviewer:** opus-4-6
**Round:** 2 of 3
**Stage:** Planning
**Date:** 2026-03-22

## Verdict

**FAIL** -- due to B1. The plan's code examples use `edge.curve_mut()` which does not exist on `Edge<P, C>`. Edge stores its curve behind `Arc<RwLock<C>>` and exposes `curve()` (returns clone) and `set_curve(c)` (replaces). This affects both `convert_shell_in` and `convert_shell_out` integration code and is not hedged anywhere in the plan, unlike the `edge_iter_mut` uncertainty which is properly acknowledged.

## Previous Round Status

All three R1 findings have been addressed:
- **B1 (non-existent test fixture):** Now correctly references `build_face_with_intersection_curve_edge` at line 3046 of tests.rs, which exists.
- **B2 (omitted convert_shell_in):** Objective and must_haves now explicitly cover both conversion directions.
- **S1 (three options without decision):** Plan now commits to a specific approach with inline code.

## Findings

### Blockers

#### B1: Code examples use non-existent Edge::curve_mut() method [confidence: 97]
- **Confidence:** 97
- **File:** 22-3-PLAN.md, Task 1 lines 110 and 135
- **Issue:** The plan's inline code uses `edge.curve_mut()` to mutate curve control points in place. This method does not exist on `Edge<P, C>`. The Edge type stores its curve in `Arc<RwLock<C>>` and provides: `curve() -> C` (clones via read lock) and `set_curve(c: C)` (replaces via write lock). There is no mutable borrow accessor. The pattern `if let Curve::NurbsCurve(ref mut nc) = edge.curve_mut()` will not compile.
- **Impact:** Both `convert_shell_in` and `convert_shell_out` integration code blocks are affected. Unlike the `edge_iter_mut` uncertainty (which the plan explicitly hedges on lines 116-118), this API mismatch has no fallback mentioned. An autonomous executor encountering a compile error on a non-existent method with no hedging guidance may stall or produce an incorrect workaround.
- **Suggested fix:** Replace the `edge.curve_mut()` pattern with the clone-modify-set pattern: `let mut c = edge.curve(); if let Curve::NurbsCurve(ref mut nc) = c { snap_curve_endpoints(nc, front, back); } edge.set_curve(c);`. Also add a note that this is necessary because Edge uses interior mutability via `Arc<RwLock<C>>`.

### Suggestions

#### S1: Shell::edge_iter_mut does not exist -- clarify the iteration approach [confidence: 91]
- **Confidence:** 91
- **File:** 22-3-PLAN.md, Task 1 lines 107 and 132
- **Issue:** The primary code examples call `internal_shell.edge_iter_mut()`, which does not exist on Shell. Shell only has `edge_iter()` (immutable). The plan acknowledges this uncertainty on line 116 and provides fallback ideas, which is good. However, the fallback ideas are vague ("use the face-based iteration approach" or "collect edge IDs and use indexed access"). Since Shell derefs to `Vec<Face>`, the correct approach is: `for face in shell.iter() { for wire in face.boundaries() { for edge in wire.edge_iter() { ... } } }` -- and since Edge uses RwLock, mutation through immutable references is actually possible via `set_curve()`. The plan should make this the primary approach rather than burying it as a fallback.
- **Impact:** Reduced clarity for autonomous execution. The executor must discover the correct iteration pattern through API exploration.
- **Suggested fix:** Replace the `edge_iter_mut()` code with face/wire/edge iteration using `set_curve()`. Since Edge uses interior mutability, immutable iteration is sufficient.

#### S2: Duplicate closing tag still present [confidence: 98]
- **Confidence:** 98
- **File:** 22-3-PLAN.md, lines 236-237
- **Issue:** The file ends with `</output>\n</output>` -- a duplicate closing XML tag. This was noted as S2 in round 1 and has not been fixed.
- **Impact:** Minor structural defect in the plan document.
- **Suggested fix:** Remove the extra `</output>` tag on line 237.

### Nits

None

## Summary

All three round 1 findings (B1, B2, S1) have been properly addressed. The plan now correctly covers both conversion directions, references real test fixtures, and commits to a specific implementation approach. However, the committed approach introduces a new blocker: the inline code uses `Edge::curve_mut()` which does not exist in the topology API. Edge uses `Arc<RwLock<C>>` with `curve()`/`set_curve()` accessors. This must be corrected since autonomous execution cannot succeed with non-compiling primary code examples that lack hedging or fallback guidance.
