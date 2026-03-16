---
phase: 6-topology-surgery-hardening
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/ops.rs
  - monstertruck-solid/src/fillet/tests.rs
autonomous: true
must_haves:
  truths:
    - "Seam control points in fillet_along_wire are dehomogenized before averaging, producing correct 3D midpoints"
    - "Averaging two Vector4 control points with different weights no longer produces weight-biased positions"
    - "Fillet along a wire with non-uniform-weight control points produces geometrically correct seam transitions"
    - "All existing fillet tests continue to pass unchanged"
  artifacts:
    - path: "monstertruck-solid/src/fillet/ops.rs"
      provides: "Fixed seam averaging logic using dehomogenize-average-rehomogenize pattern"
      min_lines: 600
      contains: "to_point"
    - path: "monstertruck-solid/src/fillet/tests.rs"
      provides: "Test verifying dehomogenized seam averaging produces correct 3D midpoints"
      min_lines: 1800
      contains: "seam_averaging_dehomogenizes"
  key_links:
    - from: "monstertruck-solid/src/fillet/ops.rs"
      to: "monstertruck-core/src/cgmath_extend_traits.rs"
      via: "Homogeneous::to_point() and Homogeneous::from_point_weight()"
      pattern: "to_point"
---

<objective>
Fix the homogeneous coordinate seam averaging bug in `fillet_along_wire` so that Vector4 control points are dehomogenized before averaging, producing correct 3D midpoints instead of weight-biased positions. This addresses TOPO-02.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@monstertruck-solid/src/fillet/ops.rs
@monstertruck-core/src/cgmath_extend_traits.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write test for dehomogenized seam averaging</name>
  <files>monstertruck-solid/src/fillet/tests.rs</files>
  <action>
Add a test `seam_averaging_dehomogenizes` that directly validates the seam averaging logic. The test should:

1. Create two adjacent `NurbsSurface<Vector4>` control point grids where boundary control points have differing weights (e.g., weight=1.0 on one surface, weight=2.0 on the other).
2. Compute the naive average `(p + q) / 2.0` of the homogeneous Vector4 control points and show that the resulting 3D point (via `.to_point()`) is NOT the midpoint of the two dehomogenized 3D points.
3. Compute the correct average: dehomogenize both points via `.to_point()`, average the 3D points, then rehomogenize via `Vector4::from_point_weight(midpoint, avg_weight)`. Assert this produces the correct 3D midpoint.

Also add a test `fillet_wire_seam_continuity` that:
1. Builds a 4-face open box (similar to existing `fillet_semi_cube` test topology).
2. Creates a 2-edge wire along the box ridge.
3. Applies `fillet_along_wire` with a constant radius.
4. Verifies the resulting shell has the expected face count (original 4 + 2 fillet faces = 6).
5. Samples the seam between adjacent fillet surfaces and verifies C0 continuity (points at the shared boundary match within tolerance).

These tests should FAIL before the fix is applied (the first one will demonstrate incorrect midpoints, the second may or may not fail depending on weight uniformity in the test geometry -- but the unit test for the averaging math itself is the critical one).
  </action>
  <verify>Run `cargo test -p monstertruck-solid seam_averaging_dehomogenizes` -- expect it to fail (demonstrating the bug). Run `cargo test -p monstertruck-solid fillet_wire_seam_continuity` -- may pass or fail depending on test geometry weights.</verify>
  <done>Tests written that expose the homogeneous averaging bug.</done>
</task>

<task type="auto">
  <name>Task 2: Fix seam averaging in fillet_along_wire</name>
  <files>monstertruck-solid/src/fillet/ops.rs</files>
  <action>
Fix the two seam averaging blocks in `fillet_along_wire` (lines ~234-257 in ops.rs):

**Interior seam averaging (lines 234-244):**
Replace:
```rust
let p = *fillet_surfaces[i - 1].control_point(j, len - 1);
let q = *fillet_surfaces[i].control_point(j, 0);
let c = (p + q) / 2.0;
```

With dehomogenize-average-rehomogenize:
```rust
let p = *fillet_surfaces[i - 1].control_point(j, len - 1);
let q = *fillet_surfaces[i].control_point(j, 0);
let p3 = p.to_point();
let q3 = q.to_point();
let mid = p3.midpoint(q3);
let avg_w = (p.w + q.w) / 2.0;
let c = Vector4::from_point_weight(mid, avg_w);
```

**Wrap-around seam averaging for closed wires (lines 247-257):**
Apply the same fix:
```rust
let p = *fillet_surfaces[last].control_point(j, len - 1);
let q = *fillet_surfaces[0].control_point(j, 0);
let p3 = p.to_point();
let q3 = q.to_point();
let mid = p3.midpoint(q3);
let avg_w = (p.w + q.w) / 2.0;
let c = Vector4::from_point_weight(mid, avg_w);
```

Ensure `Homogeneous` trait is imported at the top of ops.rs. The `Homogeneous` trait provides `to_point()` and `from_point_weight()` and should be available via `monstertruck_geometry::prelude::*`.

Note: `Point3::midpoint` may not exist -- if so, use `Point3::from_vec((p3.to_vec() + q3.to_vec()) / 2.0)` instead.
  </action>
  <verify>Run `cargo test -p monstertruck-solid seam_averaging_dehomogenizes` -- should pass. Run full fillet test suite: `cargo test -p monstertruck-solid --lib fillet` -- all tests pass.</verify>
  <done>Seam averaging dehomogenizes before averaging and rehomogenizes afterward, producing correct 3D midpoints.</done>
</task>

<task type="auto">
  <name>Task 3: Verify no regressions in full test suite</name>
  <files>monstertruck-solid/src/fillet/ops.rs, monstertruck-solid/src/fillet/tests.rs</files>
  <action>
Run the complete fillet test suite and verify all tests pass:
1. `cargo test -p monstertruck-solid --lib fillet` -- all existing tests plus new tests pass.
2. If any test fails, diagnose whether the failure is due to the seam averaging change or a pre-existing issue. The fix should be purely corrective -- for uniform-weight control points (weight=1.0), the dehomogenize-average-rehomogenize path produces the same result as naive averaging, so no existing tests should break.
3. If a test does break, it means the test was relying on the buggy behavior. Investigate and update the test expectation if the new behavior is geometrically correct.
  </action>
  <verify>Run `cargo test -p monstertruck-solid --lib fillet` -- all tests pass with zero failures.</verify>
  <done>Full fillet test suite passes with the seam averaging fix applied.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-solid seam_averaging_dehomogenizes` passes
2. `cargo test -p monstertruck-solid fillet_wire_seam_continuity` passes
3. `cargo test -p monstertruck-solid --lib fillet` passes with no regressions
4. Manual inspection confirms dehomogenize-average-rehomogenize pattern in both seam averaging blocks
</verification>

<success_criteria>
- Seam control points in `fillet_along_wire` are dehomogenized before averaging (TOPO-02)
- Correct 3D midpoints are produced instead of weight-biased positions
- Both interior and wrap-around seam averaging use the same fix
- All existing fillet tests continue to pass
</success_criteria>

<output>
After completion, create `.tendrion/phases/6-topology-surgery-hardening/6-1-SUMMARY.md`
</output>
