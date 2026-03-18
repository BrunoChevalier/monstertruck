# Review Context: Plan 9-2 (Planning Review, Round 3 of 3)

## Plan Under Review

**Plan ID:** 9-2
**Plan Path:** `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-2-PLAN.md`
**Phase:** 9 - Boolean Repair and Tolerance Foundation
**Round:** 3 of 3 (FINAL ROUND)

## Sibling Plans

| Plan ID | Wave | Objective |
|---------|------|-----------|
| 9-1 | 1 | Establish a documented numeric tolerance policy in monstertruck-core and eliminate the hardcoded 1.0e-6 in monstertruck-solid's fillet edge_select. Scope is limited to monstertruck-core and monstertruck-solid fillet files. |
| 9-3 | 2 | Validate the boolean repairs and tolerance unification with end-to-end integration tests including topology assertions, volume checks, and a chained-boolean test. Run both unit tests and the boolean_edge_cases integration regression suite. |

Full sibling plans can be read from `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/{sibling_plan_id}-PLAN.md` if cross-plan analysis is needed.

## Roadmap (Phase 9)

### Phase 9: Boolean Repair and Tolerance Foundation
**Goal**: Boolean operations on complex faces produce correct topology and all crates share a consistent numeric tolerance policy
**Depends on**: None
**Requirements**: BOOL-01, TEST-02
**Success Criteria** (what must be TRUE):
  1. The v0.3.0 criteria 2 and 4 gaps (boolean result face handling) pass their original verification checks without manual workarounds
  2. A shared tolerance constants module exists and is imported by truck-shapeops, truck-modeling, and truck-meshalgo
  3. Running `cargo test -p truck-shapeops` passes with no boolean-related test failures
  4. Tolerance constants are documented with rationale for each value choice

## Previous Review (Round 2 of 3)

The following is the REVIEW.md from round 2. This is now round 3 (FINAL ROUND). Check whether the blockers identified below have been addressed in the updated plan.

**Round 2 Verdict:** FAIL
**Round 2 Rationale:** Structure validation passed, and Task 3's healing rewrite is much clearer than round 1, but Task 2 still has blocker-level execution problems. B1-B3 show that the coincident-face repair is still tied to the wrong classifier signal, still lacks a face-specific mapping, and still conflicts with the current optimizer model.

### Round 2 Blockers (must verify if addressed):

#### B1: Coincident fallback keys off the wrong failure signal [confidence: 96]
- **Confidence:** 96
- **File:** `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-2-PLAN.md:269`, `monstertruck-solid/src/transversal/integrate/mod.rs:151`
- **Issue:** The "final approach" only uses coincident data when `classify_unknown_face` returns `None`. In the current pipeline, successful ray casting always collapses to `Some(bool)`; `None` comes from missing sample points or missing polygon meshes, not from a close or ambiguous inside/outside vote. The plan therefore does not intercept the coincident-face cases it claims to repair.
- **Suggested fix:** Rewrite Task 2 so the repair observes an actual ambiguity signal in the classifier, or add an explicit coincident-face classification path on the same face objects that enter `unknown_faces`.

#### B2: Coincident fallback still has no face-to-pair mapping [confidence: 98]
- **Confidence:** 98
- **File:** `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-2-PLAN.md:289`
- **Issue:** The proposed fallback uses `has_coincident = !coincident_pairs.is_empty()` and `coincident_pairs.iter().any(|p| !p.normals_agree)` as shell-wide state. It never identifies whether the specific `unknown` face being classified belongs to any coincident pair, so any ambiguous face can inherit a default from unrelated coincident geometry.
- **Suggested fix:** Add a concrete per-face correlation strategy on the same identity layer as `unknown_faces` before allowing coincident normals to affect classification.

#### B3: Hard-assignment requirements still conflict with the optimizer model [confidence: 97]
- **Confidence:** 97
- **File:** `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-2-PLAN.md:20`, `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-2-PLAN.md:134`, `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-2-PLAN.md:289`, `monstertruck-solid/src/transversal/integrate/mod.rs:447`
- **Issue:** The plan still says coincident results propagate as hard assignments into `known_and` and `known_or`, but the final pseudocode pushes fallback booleans into `unknown_faces`. The current optimizer explicitly searches over `unknown_faces`, so those assignments are still overrideable.
- **Suggested fix:** Choose one model and align the whole plan to it. If hard assignments are required, matched faces must be removed from `unknown_faces` before optimization and appended directly to `known_and` or `known_or`.

### Round 2 Suggestions:

#### S1: Tangent handling scope is still not explicit [confidence: 84]
- **Issue:** Task 1 mentions `detect_tangent_faces` and `handle_degenerate_intersection` only in warning suppression context. Objective/success criteria never state whether tangent handling is intentionally out of scope.
- **Suggested fix:** Add one explicit sentence stating tangent helpers are only being kept compilable for future work.

#### S2: Clippy verification still omits `-W warnings` [confidence: 95]
- **Issue:** Verification steps use `cargo clippy` without `-- -W warnings`.
- **Suggested fix:** Change each clippy command to include `-- -W warnings`.

#### S3: Phase-level TEST-02 coverage still looks incomplete across sibling plans [confidence: 92]
- **Issue:** Roadmap requires shared tolerance module imported by truck-shapeops, truck-modeling, and truck-meshalgo, but sibling plans only cover monstertruck-core and monstertruck-solid.

## Review Parameters

- **Round:** 3 of 3 (FINAL ROUND)
- **Review Type:** planning
- **Embedded Mode:** false
