---
target: "5-2"
type: "planning"
round: 1
max_rounds: 3
reviewer: "claude-opus-4-6"
stage: "planning"
date: "2026-03-16"
verdict: "pass"
confidence_threshold: 80
---

# Review: planning - 5-2

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** planning
**Date:** 2026-03-16

## Verdict

**PASS**

**Rationale:** No blockers found. The plan correctly identifies the two call sites that need updating (hyperbola.rs:82, parabola.rs:84), the re-export chain mechanism is sound (adding `pub use monstertruck_math::polynomial;` to cgmath64.rs will propagate through `base::*` and `prelude::*`), the dependency on Plan 5-1 is correct, and the verification steps are concrete and automatable. The plan covers all requirements assigned to it (wiring portion of BUILD-01) and the must_haves truths align with Phase 5 success criteria.

## Findings

### Blockers

None

### Suggestions

#### S1: Workspace dependency consolidation for num-complex may conflict with Plan 5-1 [confidence: 82]
- **Confidence:** 82
- **File:** 5-2-PLAN.md, Task 1 steps 1-2
- **Issue:** Task 1 steps 1-2 add `num-complex = "0.4"` to `[workspace.dependencies]` in root Cargo.toml and then modify `monstertruck-math/Cargo.toml` to use `num-complex = { workspace = true }`. However, Plan 5-1 Task 1 adds `num-complex = "0.4"` directly to `monstertruck-math/Cargo.toml`. If Plan 5-1 executes first (wave 1 before wave 2), the direct version spec will already be present. The Plan 5-2 Task 1 would then need to both add the workspace entry AND change the monstertruck-math entry. This is workable but adds coupling that could cause confusion if Plan 5-1 evolves.
- **Impact:** Minor friction during execution -- the implementer must be aware of exactly what state Plan 5-1 left `monstertruck-math/Cargo.toml` in. Not a correctness issue since the plan's steps handle both sides of the change.
- **Suggested fix:** Clarify in Task 1 that the direct `num-complex = "0.4"` line from Plan 5-1 must be replaced with `num-complex = { workspace = true }`, or note this is a refactoring step that modifies what Plan 5-1 created.

#### S2: Verify re-export chain with glob actually propagates modules [confidence: 81]
- **Confidence:** 81
- **File:** 5-2-PLAN.md, Task 1 action step 3
- **Issue:** The plan states that adding `pub use monstertruck_math::polynomial;` to cgmath64.rs will make `polynomial::solve_quartic` available via `use crate::prelude::*` in specifieds/mod.rs. This relies on Rust's glob re-export (`pub use cgmath64::*` in base, then `pub use base::*` in prelude) propagating re-exported modules. While this is correct Rust behavior (and it is exactly how the old `solver::` module was available via `matext4cgmath`), the plan's verify step only checks `cargo check -p monstertruck-core` -- it should also verify the symbol is accessible from monstertruck-geometry to catch re-export chain issues early.
- **Impact:** If the re-export chain doesn't work as expected, the issue would only surface in Task 2, requiring backtracking to Task 1.
- **Suggested fix:** Add `cargo check -p monstertruck-geometry` to Task 1's verify step (after the cgmath64 re-export is added but before the call sites are changed -- this would still fail due to the `solver::` references, but the error messages would confirm `polynomial` is in scope or reveal if it's missing).

### Nits

#### N1: Duplicate closing output tag [confidence: 91]
- **Confidence:** 91
- **File:** 5-2-PLAN.md, line 160-161
- **Issue:** There are two `</output>` closing tags at the end of the file. The inner one closes the output section content, but the outer one appears to be a duplicate.

#### N2: Line numbers in Task 2 may shift after Plan 5-1 [confidence: 74]
- **Confidence:** 74
- **File:** 5-2-PLAN.md, Task 2 action steps 1-2
- **Issue:** Task 2 references "line 82" in hyperbola.rs and "line 84" in parabola.rs. These line numbers are correct for the current codebase, and since Plan 5-1 does not modify these files, the references should remain valid. However, pinning exact line numbers in plans can be fragile if any other change touches these files.

## Summary

Plan 5-2 is well-structured and correctly scoped for its wave-2 role of wiring the polynomial module (created by Plan 5-1) into the existing geometry call sites. The re-export chain approach follows the established pattern used by the original `matext4cgmath::solver` module. Task sizing is appropriate -- three tasks covering re-export setup, call-site updates, and verification, each completable in 15-30 minutes. The must_haves truths and artifacts are comprehensive and directly verifiable. The two suggestions are minor coordination and verification improvements that would reduce execution friction but are not correctness issues.
