# Review Context: Plan 24-1 (Planning Review, Round 3)

## Plan Under Review

File: `.tendrion/phases/24-gpu-test-reliability/24-1-PLAN.md`

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

<objective>
Fix the two GPU camera proptest failures caused by degenerate all-zero and all-identical point cloud inputs. Both `parallel_view_fitting` and `perspective_view_fitting` must produce valid Camera structs (finite values, strictly positive clip intervals and screen sizes) for any non-empty point cloud, including cases where all points are identical.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@monstertruck-gpu/src/camera.rs
@monstertruck-gpu/tests/camera.rs
@monstertruck-core/src/bounding_box.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write failing unit tests for degenerate point cloud edge cases</name>
  <files>monstertruck-gpu/tests/camera.rs</files>
  <action>
Add dedicated unit tests to `monstertruck-gpu/tests/camera.rs` BELOW the existing `proptest! { ... }` block. These tests exercise the degenerate cases that currently cause failures. Do NOT modify the existing proptests or add proptest input filters that would weaken coverage.

Add these test functions after the `proptest! { ... }` block:

```rust
#[test]
fn parallel_view_fitting_all_zeros() {
    let rot = Matrix3::identity();
    let aspect = 1.0;
    let near_clip = 0.1;
    let points: Vec<Point3> = (0..32).map(|_| Point3::origin()).collect();
    let camera = Camera::parallel_view_fitting(rot, aspect, near_clip, &points);

    // Camera must have finite, non-NaN values
    assert!(camera.near_clip.is_finite(), "near_clip must be finite");
    assert!(camera.far_clip.is_finite(), "far_clip must be finite");
    assert!(camera.near_clip < camera.far_clip,
        "near_clip ({}) must be < far_clip ({})", camera.near_clip, camera.far_clip);

    // screen_size must be strictly positive for a valid ortho projection
    match camera.method {
        ProjectionMethod::Parallel { screen_size } => {
            assert!(screen_size > 0.0, "screen_size must be positive, got {}", screen_size);
        }
        _ => panic!("Expected Parallel projection method"),
    }
}

#[test]
fn parallel_view_fitting_all_identical() {
    let rot = Matrix3::identity();
    let aspect = 16.0 / 9.0;
    let near_clip = 0.5;
    let pt = Point3::new(5.0, 3.0, -2.0);
    let points: Vec<Point3> = (0..32).map(|_| pt).collect();
    let camera = Camera::parallel_view_fitting(rot, aspect, near_clip, &points);

    assert!(camera.near_clip.is_finite());
    assert!(camera.far_clip.is_finite());
    assert!(camera.near_clip < camera.far_clip);
    match camera.method {
        ProjectionMethod::Parallel { screen_size } => {
            assert!(screen_size > 0.0, "screen_size must be positive, got {}", screen_size);
        }
        _ => panic!("Expected Parallel projection method"),
    }
}

#[test]
fn perspective_view_fitting_all_zeros() {
    let rot = Matrix3::identity();
    let aspect = 1.0;
    let fov = Rad(PI / 4.0);
    let points: Vec<Point3> = (0..32).map(|_| Point3::origin()).collect();
    let camera = Camera::perspective_view_fitting(rot, aspect, fov, &points);

    assert!(camera.near_clip.is_finite(), "near_clip must be finite");
    assert!(camera.far_clip.is_finite(), "far_clip must be finite");
    assert!(camera.near_clip > 0.0, "near_clip ({}) must be > 0", camera.near_clip);
    assert!(camera.near_clip < camera.far_clip,
        "near_clip ({}) must be < far_clip ({})", camera.near_clip, camera.far_clip);
}

#[test]
fn perspective_view_fitting_all_identical() {
    let rot = Matrix3::identity();
    let aspect = 16.0 / 9.0;
    let fov = Rad(PI / 4.0);
    let pt = Point3::new(5.0, 3.0, -2.0);
    let points: Vec<Point3> = (0..32).map(|_| pt).collect();
    let camera = Camera::perspective_view_fitting(rot, aspect, fov, &points);

    assert!(camera.near_clip.is_finite(), "near_clip must be finite");
    assert!(camera.far_clip.is_finite(), "far_clip must be finite");
    assert!(camera.near_clip > 0.0, "near_clip ({}) must be > 0", camera.near_clip);
    assert!(camera.near_clip < camera.far_clip,
        "near_clip ({}) must be < far_clip ({})", camera.near_clip, camera.far_clip);
}
```

Confirm these tests fail before proceeding to Task 2.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-gpu -E 'test(=parallel_view_fitting_all_zeros) | test(=parallel_view_fitting_all_identical) | test(=perspective_view_fitting_all_zeros) | test(=perspective_view_fitting_all_identical)'` and confirm the new tests fail with assertion errors (zero screen_size, near_clip >= far_clip, etc.).</verify>
  <done>Four dedicated unit tests for degenerate point clouds were added and confirmed to fail on the unfixed implementation.</done>
</task>

<task type="auto">
  <name>Task 2: Fix parallel_view_fitting and perspective_view_fitting for degenerate inputs</name>
  <files>monstertruck-gpu/src/camera.rs</files>
  <action>
Fix both camera fitting functions in `monstertruck-gpu/src/camera.rs` to handle degenerate point clouds where the bounding box has zero span in one or more dimensions. The fix must NOT change behavior for non-degenerate inputs.

**Fix for `parallel_view_fitting` (lines 360-381):**

After computing `diag` from the bounding box, clamp each dimension to a minimum epsilon value. This ensures `screen_size > 0` and `far_clip > near_clip` even when all points are identical.

Replace the section after `let (center, diag) = (bbox.center(), bbox.diagonal());` (lines 372-381) with:

```rust
let (center, diag) = (bbox.center(), bbox.diagonal());
// Guard against degenerate point clouds where bbox has zero span.
// Use a small fraction of the cloud's distance from origin as epsilon,
// falling back to 1.0 when the cloud is at the origin.
let scale = center.to_vec().magnitude().max(1.0);
let eps = scale * 1e-10;
let dx = diag[0].max(eps);
let dy = diag[1].max(eps);
let dz = diag[2].max(eps);
let screen_size = f64::max(dx / aspect, dy);
let position = Vector3::new(center[0], center[1], center[2] + dz / 2.0 + near_clip);
Self {
    matrix: Matrix4::from(direction) * Matrix4::from_translation(position),
    method: ProjectionMethod::Parallel { screen_size },
    near_clip,
    far_clip: near_clip + dz,
}
```

**Fix for `perspective_view_fitting` (lines 433-469):**

The core issue: when all points are identical, `x_min == x_max`, `y_min == y_max`, `z_min == z_max`, producing `z_x = 0`, `z_y = 0`, `position[2] = 0`. This means the camera sits at the same z as the cloud, yielding `near_clip = far_clip = 0`.

Replace the section after the min/max for-loop (lines 460-468) with:

```rust
let z_x = (x_max - x_min) / (2.0 * tan) / aspect;
let z_y = (y_max - y_min) / (2.0 * tan);
let mut cam_z = z_x.max(z_y);

// Guard against degenerate point clouds where all points project to
// the same x/y/z in camera space. Ensure the camera is placed behind
// the cloud with a minimum separation so near_clip > 0 and
// near_clip < far_clip.
let scale = z_max.abs().max(z_min.abs()).max(1.0);
let min_separation = scale * 1e-10;
if cam_z <= z_max + min_separation {
    cam_z = z_max + min_separation;
}
let near_clip = cam_z - z_max;
let far_clip_raw = cam_z - z_min;
// Ensure far_clip > near_clip even when z_span == 0
let far_clip = if far_clip_raw <= near_clip {
    near_clip + min_separation
} else {
    far_clip_raw
};

let position = Vector3::new((x_min + x_max) / 2.0, (y_min + y_max) / 2.0, cam_z);
Camera {
    matrix: Matrix4::from(direction) * Matrix4::from_translation(position),
    method: ProjectionMethod::Perspective { fov },
    near_clip,
    far_clip,
}
```

This replaces the old `z_x`/`z_y`/`position`/`Camera` construction block (lines 460-468). Do NOT change the min/max for-loop (lines 443-458).

**Important:** The epsilon values are small enough (1e-10 relative to scale) that they will not affect non-degenerate inputs. Verify that existing doc-tests and proptests still pass.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-gpu -E 'binary(camera)'` and confirm all tests pass (both the new degenerate edge-case tests and existing proptests).</verify>
  <done>Both `parallel_view_fitting` and `perspective_view_fitting` were fixed to handle degenerate point clouds. All unit tests and proptests pass.</done>
</task>

<task type="auto">
  <name>Task 3: Verify proptest robustness and evaluate the perspective near_clip assumption</name>
  <files>monstertruck-gpu/tests/camera.rs</files>
  <action>
Now that `perspective_view_fitting` guarantees `near_clip > 0` for all inputs, the `prop_assume!(camera.near_clip > TOLERANCE)` guard on line 65 of the perspective proptest may be unnecessary. Evaluate whether it can be removed.

**Step 1:** Remove line 65 (`prop_assume!(camera.near_clip > TOLERANCE);`) from the `perspective_view_fitting` proptest and run:

```
PROPTEST_CASES=1000 cargo nextest run -p monstertruck-gpu -E 'test(=perspective_view_fitting)'
```

If any failures occur:
- Analyze the failing seed. If the failure is due to near_clip being positive but very small (below TOLERANCE), that is a valid geometric condition unrelated to the degenerate-input fix. In that case, restore the `prop_assume!` guard and document why it is necessary (e.g., "near_clip can be geometrically very small for certain point arrangements but is still positive").
- If the failure is a true bug (near_clip <= 0), refine the epsilon logic in Task 2.

If all tests pass with the guard removed, leave it removed -- it means the fix is robust.

**Step 2:** Keep the `prop_assume!(!same_plane)` guards in both proptests. These are legitimate geometric constraints (coplanar points produce degenerate views that cannot map to a proper 3D normalized view volume), NOT the degenerate-input issue this phase addresses.

**Step 3:** Do NOT add any additional `prop_assume!` guards that filter degenerate bounding boxes. The implementation must handle all inputs; proptest coverage should not be weakened.

**Step 4:** Run the full camera test suite with extra cases to stress-test:

```
PROPTEST_CASES=1000 cargo nextest run -p monstertruck-gpu -E 'binary(camera)'
```

Confirm consistent passes across multiple runs.
  </action>
  <verify>Run `PROPTEST_CASES=1000 cargo nextest run -p monstertruck-gpu -E 'binary(camera)'` and confirm all proptests pass consistently.</verify>
  <done>Proptest robustness was verified with 1000 cases. The `near_clip > TOLERANCE` assumption was either successfully removed (proving the fix handles all inputs) or retained with documented justification.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-gpu -E 'binary(camera)'` passes all camera tests (proptests + degenerate edge-case unit tests)
2. `PROPTEST_CASES=1000 cargo nextest run -p monstertruck-gpu -E 'binary(camera)'` passes with no failures
3. No NaN, Infinity, or zero-division in Camera output for any input including all-zeros and all-identical points
4. Existing proptest coverage is not weakened -- no broad input exclusion filters were added, existing prop_assume guards kept as-is
</verification>

<success_criteria>
- RELY-01 is fully addressed: both `parallel_view_fitting` and `perspective_view_fitting` handle degenerate all-zero and all-identical point clouds
- Camera struct invariants (finite values, near_clip < far_clip, positive screen_size) hold for all inputs
- Proptest coverage is maintained or improved (no weakening of input ranges)
- All existing tests continue to pass
</success_criteria>

<output>
After completion, create `.tendrion/phases/24-gpu-test-reliability/24-1-SUMMARY.md`
</output>

---

## Sibling Plans

| Plan ID | Wave | Objective |
|---------|------|-----------|
| 24-2 | 1 | Make GPU render tests skip gracefully without GPU hardware using try_init_device with .await.ok()? pattern, scoped to monstertruck-gpu |

Full sibling plans can be read from .tendrion/phases/24-gpu-test-reliability/{sibling_plan_id}-PLAN.md if cross-plan analysis is needed.

---

## Roadmap: Phase 24

### Phase 24: GPU Test Reliability
**Goal**: GPU camera proptests pass on all inputs and render tests skip gracefully when no GPU hardware is available
**Depends on**: None
**Requirements**: RELY-01, RELY-02
**Success Criteria** (what must be TRUE):
  1. Running `cargo test` in monstertruck-rendimpl produces zero proptest failures for camera perspective and parallel view fitting
  2. Degenerate all-zero-point inputs are handled with explicit early returns or clamped defaults instead of NaN propagation
  3. GPU/render tests on a headless CI machine (no GPU) exit with skip status rather than failure
  4. `cargo test --workspace` shows no GPU-related test failures regardless of hardware availability

### Requirement Details:
- **RELY-01**: Fix 2 GPU camera proptest failures caused by degenerate all-zero-point input handling in perspective and parallel view fitting
- **RELY-02**: Make GPU/render tests gracefully skip when no GPU hardware is available instead of failing

---

## Review Round Info

- **Round:** 3 of 3
- **Previous Review:** Round 2 resulted in FAIL

---

## Previous Review (Round 2)

---
target: "24-1"
type: "planning"
round: 2
max_rounds: 3
reviewer: "codex"
stage: "planning"
date: "2026-03-23"
verdict: "FAIL"
confidence_threshold: 80
---

## Verdict

FAIL

FAIL due to B1 and B3.

## Findings

### Blockers

#### B1: The plan still requires `cargo test --doc`, which violates repository test-command rules [confidence: 98]
- File: `.tendrion/phases/24-gpu-test-reliability/24-1-PLAN.md:214,256`
- Issue: Task 2 verification and the top-level verification checklist still instruct `cargo test -p monstertruck-gpu --doc`. The repository instructions require `cargo nextest run` for test execution and explicitly say never to use plain `cargo test`.
- Impact: Round 1 blocker B1 is not fully addressed. The plan still does not provide a verification path that is executable under repository rules.
- Suggested fix: Remove the doc-test command from this plan, or document an explicit repository-approved exception with the exact command that should be used.

#### B3: The plan is internally inconsistent about degenerate-input proptest coverage [confidence: 95]
- File: `.tendrion/phases/24-gpu-test-reliability/24-1-PLAN.md:13-17,222-239`
- Supporting code: `monstertruck-gpu/tests/camera.rs:23-29`, `monstertruck-gpu/tests/camera.rs:57-65`
- Issue: The must-haves still say the `parallel_view_fitting` and `perspective_view_fitting` proptests pass on all inputs including all-zero and all-identical point clouds. But Task 3 explicitly says to keep the existing `prop_assume!(!same_plane)` guards, and those guards exclude all-zero and all-identical clouds because they are coplanar. The current proptest property also asserts `min.z == 0` and `max.z == 1`, which a single-point or zero-depth cloud cannot satisfy.
- Impact: An implementer cannot both follow Task 3 and satisfy the plan's stated truths. As written, the plan is not self-consistent.
- Suggested fix: Reword the must-haves so degenerate inputs are covered by the dedicated unit tests, or redesign the proptests/property if degenerate clouds truly need to remain inside the proptest corpus.

### Suggestions

#### S1: Replace `test(camera)` with explicit nextest filters [confidence: 84]
- File: `.tendrion/phases/24-gpu-test-reliability/24-1-PLAN.md:214,248,255`
- Issue: The current test names in `monstertruck-gpu/tests/camera.rs` are `parallel_view_fitting` and `perspective_view_fitting`; none contain `camera`. The `test(camera)` filter is therefore ambiguous and may not select the intended tests.
- Impact: Even after B1 is fixed, the verification steps may not actually run the full camera test set they claim to cover.
- Suggested fix: Use explicit predicates such as `test(parallel_view_fitting) | test(perspective_view_fitting) | test(all_zeros) | test(all_identical)`, or a `binary(camera)` predicate if the intent is to target the integration-test binary.

### Nits

None

## Summary

Structural validation passed with `passed: true` and `task_count: 3`.

Round 1 blocker B2 is addressed. Task 2 now fixes `perspective_view_fitting` by enforcing a positive camera-to-cloud separation and ordered clip interval, which covers the identical-points failure mode in the current `monstertruck-gpu/src/camera.rs` implementation. Round 1 suggestion S1 is addressed because `files_modified` no longer claims the regression file, and Round 1 suggestion S2 is addressed because Task 3 no longer proposes adding broad degenerate-input filters.

The plan still fails because B1 remains unresolved and B3 introduces a new contradiction between the kept `same_plane` assumptions and the must-have claim that the proptests themselves cover all-zero and all-identical inputs. Review is based on static inspection. Rust test enumeration in this sandbox is blocked by `snap-confine` capability errors.
