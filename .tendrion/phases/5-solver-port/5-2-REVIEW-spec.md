---
target: 5-2
type: impl-review
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: spec-compliance
date: 2026-03-16
verdict: FAIL
---

# Implementation Review: Plan 5-2 (Spec Compliance)

**Reviewer:** claude-opus-4-6 | **Round:** 1/3 | **Stage:** spec-compliance | **Date:** 2026-03-16

## Verdict

**FAIL** -- due to B1. The plan explicitly requires "cargo build --workspace succeeds with zero errors" and "Existing parabola snp_test and sp_test pass without modification." Neither condition is met. While the workspace build failures are pre-existing (monstertruck-modeling, monstertruck-meshing, monstertruck-render were not modified by this plan), and the test compilation failures are also pre-existing (202 errors in t_spline/decorators modules), the plan's must-have truths are stated unconditionally. The implementation correctly wires the polynomial solver but cannot satisfy the plan's literal verification criteria due to the broader codebase state. The summary acknowledges this but frames it as acceptable. A pragmatic interpretation would pass this plan since the solver-related work is correct and complete; however, spec compliance review must check against what the plan actually says.

## Findings

### Blockers

#### B1: Plan must-have criteria not literally satisfiable [confidence: 82]
- **Confidence:** 82
- **File:** 5-2-PLAN.md, must_haves.truths
- **Issue:** Two of the five must-have truths cannot be verified as stated:
  1. "cargo build --workspace succeeds with zero errors" -- fails due to 31 pre-existing errors in monstertruck-modeling (15), monstertruck-render (15), and monstertruck-meshing (1). None of these crates were modified by this plan.
  2. "Existing parabola snp_test and sp_test pass without modification" -- `cargo test -p monstertruck-geometry -- parabola` fails to compile due to 202+ pre-existing errors in t_spline and decorators modules. The parabola module itself compiles and the test logic is correct, but the tests cannot be executed.
- **Impact:** Verification criteria cannot be confirmed as written. The solver wiring itself is correct -- `cargo build -p monstertruck-geometry` succeeds, and the polynomial re-export integration tests in monstertruck-core pass (2 tests). The 7 monstertruck-math polynomial unit tests also pass. But the plan's literal must-haves are not met.
- **Suggested fix:** Either (a) acknowledge in the plan that "workspace" build refers to solver-related crates only (monstertruck-math, monstertruck-core, monstertruck-traits, monstertruck-geometry), or (b) fix the pre-existing errors to make the literal criteria satisfiable. Given these are pre-existing issues, option (a) -- updating the plan's verification criteria to match the realistic scope -- is the pragmatic path. Alternatively, the reviewer could be overruled if the orchestrator considers pre-existing failures outside the plan's responsibility.

### Suggestions

#### S1: Extra scope -- monstertruck-traits/src/polynomial.rs bug fixes [confidence: 88]
- **Confidence:** 88
- **File:** monstertruck-traits/src/polynomial.rs
- **Issue:** The plan does not mention modifying `monstertruck-traits/src/polynomial.rs`. The implementation fixed three pre-existing bugs (ElementWise -> MulElementWise in three locations, .cross() missing borrow). While these were necessary to unblock the workspace build, they constitute scope beyond what the plan specified. The summary correctly logs these as deviations.
- **Impact:** Low risk -- the fixes are mechanical and correct (confirmed pre-existing at base commit 833ab6c4). The deviation was properly documented. However, for spec compliance, the plan's `files_modified` list does not include this file.
- **Suggested fix:** No code change needed. Accept the deviation as a necessary auto-fix and note it in the plan or DEVIATIONS.md (already done).

#### S2: Extra scope -- namespace disambiguation in geometry lib.rs [confidence: 86]
- **Confidence:** 86
- **File:** monstertruck-geometry/src/lib.rs:33
- **Issue:** The plan does not mention adding `pub use monstertruck_core::cgmath64::polynomial;` to the `base` module in `monstertruck-geometry/src/lib.rs`. This was necessary because `monstertruck_traits::polynomial` (PolynomialCurve/Surface) would shadow `cgmath64::polynomial` (solver) in the wildcard re-export. The fix is correct and well-commented but is not in the plan's `files_modified` list.
- **Impact:** Without this disambiguation, `polynomial::solve_quartic` would not resolve in hyperbola.rs and parabola.rs. This was a genuine discovery during implementation that the plan missed.
- **Suggested fix:** Accept the deviation. The plan's re-export chain analysis was incomplete -- it didn't account for the `pub use monstertruck_traits::*` in the base module creating a namespace collision.

### Nits

None.

## Summary

The core solver wiring work is correct and complete. All three primary artifacts exist with the expected content: cgmath64.rs has the polynomial re-export, hyperbola.rs calls `polynomial::solve_quartic`, parabola.rs calls `polynomial::pre_solve_cubic`, and zero `solver::` references remain in source code. The implementation discovered and resolved two issues the plan didn't anticipate (namespace collision, pre-existing trait bugs). The only spec compliance concern is that two of the plan's must-have verification criteria are not literally satisfiable due to pre-existing codebase issues outside this plan's scope.
