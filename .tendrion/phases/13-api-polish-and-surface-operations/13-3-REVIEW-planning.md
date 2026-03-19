---
target: "13-3"
type: "planning"
round: 1
max_rounds: 3
reviewer: "claude-opus-4-6"
stage: "planning"
date: "2026-03-19"
verdict: "pass"
confidence_threshold: 80
---

# Review: planning - 13-3

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** planning
**Date:** 2026-03-19

## Verdict

**PASS**

**Rationale:** No blockers found. The plan is well-structured, feasible, and correctly depends on 13-1 for the geometry-level option structs and diagnostic types it needs to wire into the modeling layer. The three tasks are appropriately sized and cover error wrapping, builder functions, and integration testing. Requirements API-01 and API-02 are addressed at the modeling layer. The `skin` function coverage gap is noted as a suggestion since `try_skin_wires` has a fundamentally different architecture (Wire-based, not BsplineCurve-based) and may not benefit from the same option struct pattern.

## Findings

### Blockers

None

### Suggestions

#### S1: Roadmap success criterion 1 mentions `skin` but no plan covers it [confidence: 72]
- **Confidence:** 72
- **File:** ROADMAP.md, Phase 13 Success Criteria item 1
- **Issue:** The roadmap states "All surface constructor functions (sweep_rail, birail, gordon, skin) accept dedicated option structs" but neither plan 13-1 nor 13-3 introduces option structs for `try_skin_wires`. The existing `try_skin_wires` takes `Wire` slices rather than `BsplineCurve` inputs, making it architecturally different from the other surface constructors.
- **Impact:** If the roadmap criterion is taken literally, Phase 13 would fail success criteria verification. However, `skin` may reasonably be out of scope for the option-struct pattern since it operates on topological wires rather than parametric curves.
- **Suggested fix:** Either add a `SkinOptions` struct (even if it is a marker struct like `GordonOptions`) and a `try_skin_wires_with_options` wrapper, or update the roadmap to clarify that `skin` is excluded from the option-struct pattern due to its different input signature.

#### S2: lib.rs listed in files_modified but no task explicitly modifies it [confidence: 86]
- **Confidence:** 86
- **File:** 13-3-PLAN.md, frontmatter `files_modified` and Task 2
- **Issue:** The plan lists `monstertruck-modeling/src/lib.rs` in `files_modified` and Task 2 mentions "Check monstertruck-modeling/src/lib.rs for the appropriate re-export location" for option struct re-exports. However, no task has `lib.rs` in its `<files>` element. The re-export work is described as an afterthought in Task 2 rather than a tracked deliverable.
- **Impact:** The implementer may forget to add re-exports, or the re-export work won't be verified independently. The `files_modified` metadata suggests the plan intends to modify it but doesn't formally assign it.
- **Suggested fix:** Add `monstertruck-modeling/src/lib.rs` to Task 2's `<files>` element and add a specific verify step checking that `SweepRailOptions` etc. are accessible from `monstertruck_modeling::prelude` or the crate root.

#### S3: Deprecation of existing APIs may cause downstream warnings [confidence: 81]
- **Confidence:** 81
- **File:** 13-3-PLAN.md, Task 2
- **Issue:** Task 2 says to mark existing `try_sweep_rail`, `try_birail`, `try_gordon` with `#[deprecated]`. Since `cargo clippy -p monstertruck-modeling -- -W warnings` is used as a verification step and existing tests call these functions, the deprecation warnings may cause clippy to fail. The plan does not address how to suppress `#[allow(deprecated)]` in tests or downstream callers within the same crate.
- **Impact:** The verification step in Task 2 may fail due to deprecation warnings in existing test code that calls these functions, creating a false negative that blocks implementation progress.
- **Suggested fix:** Note that test modules calling deprecated functions will need `#[allow(deprecated)]` attributes, and that the existing clippy invocation should either be run with `--allow deprecated` or the test callsites should be annotated.

### Nits

#### N1: Integration tests could use a separate test file [confidence: 63]
- **Confidence:** 63
- **File:** 13-3-PLAN.md, Task 3
- **Issue:** builder.rs is already 1242 lines. Task 3 adds integration tests directly to builder.rs (or mentions "a separate test file" as an alternative). A dedicated test file would improve maintainability.

#### N2: Duplicate closing `</output>` tag in plan [confidence: 95]
- **Confidence:** 95
- **File:** 13-3-PLAN.md, line 178
- **Issue:** The plan has two `</output>` closing tags at the end of the file (lines 177-178). This is a minor formatting issue that does not affect plan structure parsing.

## Summary

Plan 13-3 is a solid, well-scoped plan that correctly wires the geometry-level option structs and diagnostic errors (created by 13-1) into the modeling builder layer. The three tasks are appropriately sized (15-45 minutes each), the dependency on 13-1 is correctly declared, and the must-haves cover both the happy path and error propagation. The main areas for improvement are ensuring `lib.rs` re-exports are formally tracked in a task and handling deprecation warnings in test code. The `skin` coverage gap is a lower-confidence concern given the architectural differences.
