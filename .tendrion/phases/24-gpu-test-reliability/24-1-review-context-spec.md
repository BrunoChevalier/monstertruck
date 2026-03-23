# Review Context: Spec Compliance - Plan 24-1

## Review Metadata
- **Plan ID:** 24-1
- **Review Type:** spec-compliance
- **Round:** 1 of 3
- **Commit Range:** 82d47113123e2cb802612726cac66a56164f118a..8988da4d3100f9c235d9821a7287c2012cad12be

## Plan Content

---
phase: 24-gpu-test-reliability
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-gpu/src/camera.rs
  - monstertruck-gpu/tests/camera.rs
autonomous: true
must_haves:
  truths:
    - "User runs `cargo nextest run -p monstertruck-gpu -E 'binary(camera)'` and both proptests pass on non-degenerate 3D point clouds (existing prop_assume guards filter coplanar/degenerate inputs which are tested by dedicated unit tests)"
    - "Dedicated unit tests verify that parallel_view_fitting and perspective_view_fitting produce valid Camera structs for degenerate inputs (all-zero, all-identical point clouds) with finite values, near_clip < far_clip, and positive screen_size"
    - "User provides 32 points that are all [0,0,0] and parallel_view_fitting returns a Camera with finite non-NaN values, strictly positive screen_size, and near_clip < far_clip"
    - "User provides 32 identical points like [5,5,5] and perspective_view_fitting returns a Camera with near_clip > 0 and near_clip < far_clip"
    - "No proptest input coverage is weakened -- existing prop_assume guards are kept as-is, no new broad exclusion filters are added"
  artifacts:
    - path: "monstertruck-gpu/src/camera.rs"
      provides: "Fixed parallel_view_fitting and perspective_view_fitting that handle degenerate point clouds with zero-span bounding boxes"
      min_lines: 400
      contains: "parallel_view_fitting"
    - path: "monstertruck-gpu/tests/camera.rs"
      provides: "Proptests plus dedicated unit tests for degenerate edge cases"
      min_lines: 90
      contains: "proptest"
  key_links:
    - from: "monstertruck-gpu/tests/camera.rs"
      to: "monstertruck-gpu/src/camera.rs"
      via: "Camera::parallel_view_fitting and Camera::perspective_view_fitting calls"
      pattern: "Camera::parallel_view_fitting"
---

Objective: Fix the two GPU camera proptest failures caused by degenerate all-zero and all-identical point cloud inputs. Both `parallel_view_fitting` and `perspective_view_fitting` must produce valid Camera structs (finite values, strictly positive clip intervals and screen sizes) for any non-empty point cloud, including cases where all points are identical.

Tasks:
1. Write failing unit tests for degenerate point cloud edge cases
2. Fix parallel_view_fitting and perspective_view_fitting for degenerate inputs
3. Verify proptest robustness with existing guards intact

## Summary Content (DO NOT TRUST -- verify independently)

### Files modified
- monstertruck-gpu/tests/camera.rs: Added 4 dedicated unit tests for degenerate point cloud edge cases
- monstertruck-gpu/src/camera.rs: Added epsilon guards in parallel_view_fitting and perspective_view_fitting
- monstertruck-math/src/lib.rs: Fixed transposed ortho(), perspective(), and frustum() projection matrix functions

### Deviations from plan
1. Pre-existing build errors in msaa.rs and bindgroup.rs -- used --test camera flag
2. Transposed projection matrices in monstertruck-math/src/lib.rs -- fixed row/column-major ordering

## Must-Haves Verification Checklist

### Truths (verify each by reading code)
1. `cargo nextest run -p monstertruck-gpu -E 'binary(camera)'` proptests pass on non-degenerate 3D point clouds
2. Dedicated unit tests verify parallel_view_fitting and perspective_view_fitting for degenerate inputs (all-zero, all-identical)
3. 32 all-[0,0,0] points -> parallel_view_fitting returns Camera with finite non-NaN, positive screen_size, near_clip < far_clip
4. 32 identical [5,5,5] points -> perspective_view_fitting returns Camera with near_clip > 0 and near_clip < far_clip
5. No proptest coverage weakened -- existing prop_assume guards kept, no new broad exclusion filters

### Artifacts (verify existence, line count, content)
1. monstertruck-gpu/src/camera.rs: min 400 lines, contains "parallel_view_fitting"
2. monstertruck-gpu/tests/camera.rs: min 90 lines, contains "proptest"

### Key Links (verify import/call patterns)
1. monstertruck-gpu/tests/camera.rs -> monstertruck-gpu/src/camera.rs via Camera::parallel_view_fitting

## Confidence Rules
- Every finding MUST include a confidence score (0-100)
- Blockers SHOULD have confidence >= 85
- Confidence threshold for surfacing: 80
- Report ALL findings with honest confidence scores
