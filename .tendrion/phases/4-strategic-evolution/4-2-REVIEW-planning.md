---
target: "4-2"
type: planning
round: 2
max_rounds: 3
reviewer: claude
stage: planning
date: "2026-03-10"
verdict: PASS
---

# Plan Review: 4-2 (cgmath to nalgebra migration)

**Reviewer:** claude-sonnet-4-6
**Round:** 2 of 3
**Stage:** planning
**Date:** 2026-03-10

## Verdict

PASS — both previous blockers are resolved. Zero remaining blockers.

B1 (Task 3 files_modified incomplete) is closed: Task 3 now explicitly constrains itself to verify-only for downstream crates, with any compatibility fixes applied only to `monstertruck-math/src/traits.rs` and `monstertruck-math/src/conversions.rs`. The contradiction between the action description and the narrow files list is resolved by the added clarifying sentence: "This task is verify-only for downstream crates -- all fixes go into monstertruck-math's adapter layer, not downstream source files."

S1 (nalgebra not in workspace.dependencies) is closed: Task 1 step 1 now explicitly instructs adding nalgebra to `[workspace.dependencies]` in root `Cargo.toml` and referencing it as `nalgebra = { workspace = true }` in the crate Cargo.toml. The IMPORTANT note reinforces this as a hard requirement.

## Findings

### Blockers

None

### Suggestions

#### S1: matext4cgmath content not inventoried before removal [confidence: 79]
- **Confidence:** 79
- **File:** 4-2-PLAN.md, Task 2 action step 2 and Task 3 action step 3
- **Issue:** The plan removes `matext4cgmath` from `monstertruck-core` and states its "functionality is absorbed into monstertruck-math," but no task step explicitly inventories what `matext4cgmath` exports before designing the replacement. Task 3 step 3 acknowledges the risk but treats it as a fix-if-broken exercise.
- **Impact:** If any `matext4cgmath` API is missed in `monstertruck-math`, the result is a compilation failure discovered only during Task 3 verification, not during Task 1 design.
- **Suggested fix:** Add an explicit first step in Task 1 or Task 2: read `matext4cgmath` crate source and list every public item to ensure coverage in `monstertruck-math/src/conversions.rs` or `traits.rs`.

### Nits

#### N1: Task 1 Cargo.toml specifies edition = "2024" inline [confidence: 71]
- **Confidence:** 71
- **File:** 4-2-PLAN.md, Task 1 action step 2
- **Issue:** The proposed `monstertruck-math/Cargo.toml` sets `edition = "2024"` directly. Workspace does not centralize edition via `[workspace.package]`, so this is consistent with the rest of the project and not wrong. Minor style note only.

#### N2: monstertruck-gpu in Task 3 check list but 4-4 owns GPU work [confidence: 68]
- **Confidence:** 68
- **File:** 4-2-PLAN.md, Task 3 action step 1
- **Issue:** Task 3 includes `cargo check -p monstertruck-gpu` in its verification list. Plan 4-4 (wave 3) owns GPU tessellation work. Verifying GPU compilation here is reasonable but could surface issues that are 4-4's responsibility.

## Summary

Both round-1 findings that met the confidence threshold have been addressed. B1 is definitively closed by constraining Task 3 to monstertruck-math modifications only, eliminating the scope contradiction. S1 is closed by explicitly routing nalgebra through workspace.dependencies with a clear IMPORTANT annotation. The plan is coherent, well-scoped to EVOLVE-01, and structurally sound. Remaining suggestion (S1, confidence 79) is below the 80-point surfacing threshold and is carried forward for awareness only.
