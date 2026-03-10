---
target: "3-4"
type: "implementation"
round: 1
max_rounds: 3
reviewer: "claude"
stage: "spec-compliance"
date: "2026-03-10"
verdict: "PASS"
confidence_threshold: 80
---

# Spec Compliance Review: 3-4 Draft/Taper Operations

**Reviewer:** claude-sonnet-4-6
**Round:** 1 of 3
**Stage:** spec-compliance
**Date:** 2026-03-10

---

## Verdict

PASS. All must-have artifacts satisfy line-count and content constraints. All must-have truths have test coverage and all 8 tests pass. All key links are present. One suggestion regarding the neutral-plane test methodology deviating from the plan's specification.

---

## Findings

### Blockers

None

### Suggestions

#### S1: `draft_cube_neutral_plane_fixed` tests vertex z-coordinates rather than face-surface sampling at neutral plane height [confidence: 81]
- **Confidence:** 81
- **File:** monstertruck-solid/src/draft/tests.rs:185-215
- **Issue:** The plan (Task 3) specifies for this test: "Sample the face surface at the neutral plane height and check distance to neutral plane is near zero." The implementation instead checks that all vertices remain near z=0 or z=1 (the top/bottom planes, not the neutral plane at z=0.5). This verifies a side-effect of undrafted top/bottom faces rather than directly asserting that points on the drafted side faces at the neutral plane height remain on the neutral plane.
- **Impact:** The neutral-plane fixed-point invariant -- a core geometric property of draft operations -- is not directly verified at the face-surface level. The `draft_cube_angle_verification` test confirms geometric correctness indirectly (correct angle implies correct hinge computation), but the specific neutral-plane preservation property as described in the plan remains untested directly.
- **Suggested fix:** In the `draft_cube_neutral_plane_fixed` test, after running draft_faces, iterate over the drafted side faces. For each face, evaluate the surface at the parameter value corresponding to the neutral plane height (z=0.5) and assert that the resulting 3D point's z-coordinate is within tolerance of neutral_z. This can supplement the existing vertex check.

### Nits

None

---

## Artifact Verification

| Artifact | Min Lines | Actual Lines | Contains Pattern | Status |
|---|---|---|---|---|
| monstertruck-solid/src/draft/mod.rs | 15 | 20 | `pub fn draft_faces` (via `pub use`) | PASS |
| monstertruck-solid/src/draft/draft_op.rs | 100 | 300 | `draft_faces` | PASS |
| monstertruck-solid/src/draft/tests.rs | 100 | 300 | `draft_cube_faces` | PASS |

Note on mod.rs pattern: the plan artifact specifies `contains: "pub fn draft_faces"`. The mod.rs contains `pub use draft_op::{DraftError, DraftOptions, draft_faces};` which re-exports `draft_faces` but does not define it with `pub fn`. The function definition is in draft_op.rs. The key link check (`pub mod draft` in lib.rs) is satisfied. Given the plan's Task 1 code template for mod.rs explicitly uses `pub use draft_op::{draft_faces, ...}` as the intended pattern, and the plan's `draft_op.rs` artifact contains the `pub fn draft_faces` definition, this artifact check is satisfied by the plan's own intended structure.

## Key Link Verification

| Link | Pattern | Status |
|---|---|---|
| draft/mod.rs -> lib.rs | `pub mod draft` | PASS (lib.rs line 20) |
| draft/draft_op.rs -> topology/face.rs | `Face` | PASS (`CompressedFace` via `monstertruck_topology::compress::*`) |

## Must-Have Truth Coverage

| Truth | Test(s) | Status |
|---|---|---|
| 5-degree draft on cube produces valid solid with tapered faces | `draft_cube_faces_valid_topology`, `draft_cube_angle_verification` | PASS |
| Specifies pull direction and neutral plane; tilts faces relative to that plane | `draft_cube_faces_valid_topology`, `draft_cube_angle_verification` | PASS |
| Drafted solid passes topological validity checks (closed shell, no singular vertices) | `draft_cube_faces_valid_topology`, `draft_10_degree_larger_angle`, `draft_box_non_unit` | PASS |
| Draft produces valid B-rep output that can be serialized and deserialized | `draft_serialization_round_trip` | PASS |
| Draft angle of 0 degrees returns the original solid unchanged | `draft_zero_angle_returns_original` | PASS |

## Test Run Results

All 8 tests pass (`cargo test -p monstertruck-solid -- draft::tests`):
- `draft_cube_faces_valid_topology` - ok
- `draft_cube_angle_verification` - ok
- `draft_cube_neutral_plane_fixed` - ok
- `draft_zero_angle_returns_original` - ok
- `draft_invalid_angle_error` - ok
- `draft_serialization_round_trip` - ok
- `draft_10_degree_larger_angle` - ok
- `draft_box_non_unit` - ok

---

## Summary

The implementation fully satisfies all must-have artifacts, key links, and must-have truths from plan 3-4. The `draft_faces` function is correctly implemented for planar-face solids, handles zero-angle identity and invalid-angle/direction errors, and a hinge-point computation bug (zero denominator when face normal is perpendicular to neutral plane normal) was correctly identified and fixed. All 8 tests pass including topological validity, geometric angle verification, serialization round-trip, and non-unit box cases. One suggestion remains: the `draft_cube_neutral_plane_fixed` test verifies a proxy property (vertex z-positions at top/bottom) rather than directly sampling drafted face surfaces at the neutral plane height as the plan description specifies, leaving the core neutral-plane invariant less directly exercised.
