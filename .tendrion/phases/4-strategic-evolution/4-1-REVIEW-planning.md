---
target: "4-1"
type: planning
round: 1
max_rounds: 3
reviewer: claude
stage: planning
date: "2026-03-10"
verdict: PASS
---

# Review: 4-1-PLAN.md

**Reviewer:** claude (claude-sonnet-4-6)
**Round:** 1 of 3
**Stage:** planning
**Date:** 2026-03-10

---

## Verdict

PASS

No blockers found. The plan is feasible, structurally valid, and maps cleanly to FEAT-04 requirement SC-4 from ROADMAP.md. Two suggestions and one nit are noted.

---

## Findings

### Blockers

None

### Suggestions

#### S1: Task 2 branch condition for parity may resolve as no-op without verification proof [confidence: 81]
- **Confidence:** 81
- **File:** 4-1-PLAN.md, Task 2 action step 1
- **Issue:** Task 2 instructs the implementer to verify the L/R parity mapping against the paper and then either confirm it is correct (remove TODO + add comment) or swap the arrays. With no reference implementation or worked example provided in the plan, an implementer could rationalize the existing mapping as correct without genuinely verifying it, leaving the mathematical question open. The plan should supply a concrete test oracle -- e.g., a hand-computed alpha value for a specific mesh -- so the parity decision is falsifiable.
- **Impact:** The TODO at L390/L403 exists precisely because the correctness of LeftAcw/LeftCw for a_od is uncertain. Removing it without a falsifiable oracle risks silencing a real bug.
- **Suggested fix:** In Task 1, require the test to include at least one hard-coded alpha value computed from the paper's Equation 14 so the parity mapping is forced to match a known answer rather than pass by coincidence.

#### S2: Zero knot interval "does not panic" verification is underspecified [confidence: 78]
- **Confidence:** 78
- **File:** 4-1-PLAN.md, Task 3 action step 2
- **Issue:** The plan says to "verify that the operation succeeds (does not panic or error)". Zero knot intervals produce repeated knots, which can make knot vectors non-invertible. The plan should explicitly specify the expected output topology and knot vector for the zero-interval case (reference Figure 9 content), not just absence of a panic.
- **Impact:** Tests that assert "no panic" without asserting correct topology can mask incorrect behavior that happens to not crash.
- **Suggested fix:** Expand the Task 1 zero knot interval test to assert the resulting mesh's knot vector contains the expected repeated knot entry, cross-referenced against Figure 9 of Sederberg et al. 2003.

### Nits

#### N1: `<output>` XML tag duplicated in plan XML structure [confidence: 92]
- **Confidence:** 92
- **File:** 4-1-PLAN.md, line 154-157
- **Issue:** The plan has two closing `</output>` tags at the end (line 156 closes the inner `<output>` content block, line 157 is a stray duplicate). This is cosmetic but may cause XML parsers to reject the document.

---

## Summary

The plan is well-scoped and directly addresses FEAT-04's success criterion (SC-4: no TODOs remaining in t_spline/t_nurcc.rs and t_spline/t_mesh.rs, with passing tests). All four TODO locations identified in the plan match their actual positions in source (t_nurcc.rs L390, L403, L451; t_mesh.rs L345). The TDD ordering is correct: Task 1 writes failing tests, Tasks 2 and 3 resolve the code. Wave 1 placement with no dependencies is appropriate since 4-2/4-3/4-4 operate on unrelated subsystems. The two suggestions target test oracle strength for the parity decision, which is the plan's highest-risk judgment call.
