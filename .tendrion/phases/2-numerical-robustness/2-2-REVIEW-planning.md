---
target: "2-2"
type: "planning"
round: 1
max_rounds: 2
reviewer: "claude-opus-4-6"
stage: "planning"
date: "2026-03-09"
verdict: "PASS"
confidence_threshold: 80
---

# Review: planning - 2-2

**Reviewer:** claude-opus-4-6
**Round:** 1 of 2
**Stage:** planning
**Date:** 2026-03-09

## Verdict

**PASS**

**Rationale:** Plan 2-2 is well-structured, feasible, and correctly scoped to address ROBUST-05. The three tasks cover all required fuzz targets (NURBS evaluation, knot vector manipulation, STEP parsing) with appropriate seed corpora and validation. No blockers found. Two suggestions and two nits identified.

## Findings

### Blockers

None

### Suggestions

#### S1: STEP fuzz Cargo.toml uses wildcard version for ruststep [confidence: 88]
- **Confidence:** 88
- **File:** 2-2-PLAN.md, Task 2 action section (line ~153)
- **Issue:** The plan specifies `ruststep = version "*"` in the STEP fuzz Cargo.toml. While the plan includes a note to check the actual version, the template Cargo.toml shown uses `"*"`, which cargo considers bad practice and will produce a warning. Since fuzz crates use `[workspace] members = ["."]` (detached from the workspace), they cannot use `{ workspace = true }`. The specific version should be pinned in the template itself.
- **Impact:** Cargo will warn about wildcard dependencies. More importantly, if the implementer copies the template verbatim without reading the note, the fuzz target may resolve a different ruststep version than the one monstertruck-step uses, potentially masking or introducing false issues.
- **Suggested fix:** Replace `version = "*"` with `version = "0.4"` in the template Cargo.toml to match the workspace-level `ruststep = "0.4"` declaration.

#### S2: nurbs_eval fuzz target should use try_new instead of new [confidence: 86]
- **Confidence:** 86
- **File:** 2-2-PLAN.md, Task 1 action section (line ~114)
- **Issue:** The plan instructs the implementer to call `BsplineCurve::new(knot_vec, control_points)`. While the plan also specifies clamping degree to 1..=5 and n to degree+1..=20, and using `KnotVector::uniform_knot`, the `new()` constructor panics on invalid inputs (empty control points, too-short knot vector, zero range). A fuzzer may still find edge combinations where the structured input generation produces arguments that trip these checks (e.g., if `Arbitrary` generates degree=5 and n=6 but the control point vec ends up shorter due to deserialization limits). Using `try_new` and returning early on `Err` would be strictly more robust for a fuzz target -- the goal is to fuzz `subs`/`evaluate`, not the constructor validation.
- **Impact:** Potential false-positive crashes from the constructor rather than the evaluation code paths that are the actual fuzzing target.
- **Suggested fix:** Use `BsplineCurve::try_new(knot_vec, control_points)` and return early on `Err` to focus fuzzing on evaluation rather than constructor validation.

### Nits

#### N1: Plan references deprecated API names in must_haves [confidence: 82]
- **Confidence:** 82
- **File:** 2-2-PLAN.md, must_haves.truths (line ~19)
- **Issue:** The must_haves reference `BsplineCurve::subs` which is a deprecated compatibility method on the `ParametricCurve` trait. The modern API uses `evaluate`/`derivative`/`derivative_2`. The fuzz targets would ideally exercise the current API surface.

#### N2: Corpus files not listed in files_modified [confidence: 73]
- **Confidence:** 73
- **File:** 2-2-PLAN.md, frontmatter files_modified
- **Issue:** Task 3 creates corpus files (`.gitkeep`, `minimal.step`) that are not listed in the `files_modified` frontmatter. While these are auxiliary files and not critical for dependency tracking, including them would make the plan more complete.

## Summary

Plan 2-2 is a well-scoped, feasible plan for ROBUST-05 fuzzing infrastructure. The three-task breakdown (geometry targets, STEP target, validation/corpora) is logical and appropriately sized. Wave 1 placement with no dependencies is correct since fuzzing targets only depend on existing crate code. The two suggestions address minor robustness concerns in the fuzz target design but do not block execution.
