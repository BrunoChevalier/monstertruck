---
phase: 4-strategic-evolution
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/t_spline/t_nurcc.rs
  - monstertruck-geometry/src/t_spline/t_mesh.rs
  - monstertruck-geometry/src/t_spline/t_mesh_control_point.rs
  - monstertruck-geometry/tests/t_spline_validation.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo test on monstertruck-geometry and all T-spline validation tests pass"
    - "No TODO comments remain in t_nurcc.rs related to connection parity checks"
    - "No TODO comments remain in t_mesh.rs related to zero knot intervals"
    - "T-NURCC subdivision produces correct limit surface points for meshes with L/R parity edges"
    - "T-mesh insertion correctly handles knot intervals of zero per figure 9 of Sederberg et al. 2003"
  artifacts:
    - path: "monstertruck-geometry/src/t_spline/t_nurcc.rs"
      provides: "Connection parity (L/R) validation for TnurccConnection in subdivision"
      min_lines: 400
      contains: "a_od"
    - path: "monstertruck-geometry/src/t_spline/t_mesh.rs"
      provides: "Zero knot interval handling for T-mesh point insertion"
      min_lines: 300
      contains: "knot_interval"
    - path: "monstertruck-geometry/tests/t_spline_validation.rs"
      provides: "Validation tests for connection parity and zero knot intervals"
      min_lines: 80
      contains: "parity"
  key_links:
    - from: "monstertruck-geometry/tests/t_spline_validation.rs"
      to: "monstertruck-geometry/src/t_spline/t_nurcc.rs"
      via: "test imports and exercises subdivision with parity-sensitive edges"
      pattern: "Tnurcc"
    - from: "monstertruck-geometry/tests/t_spline_validation.rs"
      to: "monstertruck-geometry/src/t_spline/t_mesh.rs"
      via: "test imports and exercises T-mesh insertion with zero knot intervals"
      pattern: "Tmesh"
---

<objective>
Complete T-spline validation by resolving all TODO items in t_nurcc.rs (connection parity checks at L390, L403, L451) and t_mesh.rs (zero knot interval support at L345), backed by comprehensive tests that exercise the corrected logic.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/t_spline/mod.rs
@monstertruck-geometry/src/t_spline/t_nurcc.rs
@monstertruck-geometry/src/t_spline/t_mesh.rs
@monstertruck-geometry/src/t_spline/t_nurcc_edge.rs
@monstertruck-geometry/src/t_spline/t_mesh_control_point.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write failing tests for T-spline validation scenarios</name>
  <files>monstertruck-geometry/tests/t_spline_validation.rs</files>
  <action>
Create a new test file `monstertruck-geometry/tests/t_spline_validation.rs` with tests covering:

1. **Connection parity tests for T-NURCC subdivision** (addresses TODOs at t_nurcc.rs L390, L403):
   - Build a minimal T-NURCC mesh (e.g., 2x2 quad grid) where edges have asymmetric knot intervals on left vs right sides.
   - Call `subdivide()` and verify the alpha values (a_od, a_do from Equation 14 of Sederberg et al. 1998) are computed correctly by checking the resulting subdivided point positions.
   - Test that LeftAcw/LeftCw vs RightAcw/RightCw connections produce the correct parity-dependent results.
   - Include an assertion that verifies the subdivided mesh's points match reference values computed by hand from the paper's equations.

2. **Zero knot interval tests for T-mesh** (addresses TODO at t_mesh.rs L345):
   - Build a T-mesh with edges that have knot intervals of 0.0 (as described in Figure 9 of Sederberg et al. 2003).
   - Attempt `add_point_on_edge` where the resulting split would create zero knot interval connections.
   - Verify that the operation succeeds (does not panic or error) and the resulting mesh topology is correct.
   - Test knot vector computation with zero intervals present.

3. **Malformed face error test** (addresses TODO at t_nurcc.rs L451):
   - Test that `subdivide()` returns an appropriate error when a control point has no radial edges.

These tests should initially fail because the TODOs represent unimplemented/unverified logic.
  </action>
  <verify>Run `cargo test -p monstertruck-geometry t_spline_validation` and confirm the tests exist. Some tests may fail initially, which is expected for TDD.</verify>
  <done>Failing test suite for T-spline validation scenarios was created with tests covering connection parity, zero knot intervals, and error handling.</done>
</task>

<task type="auto">
  <name>Task 2: Resolve connection parity TODOs in t_nurcc.rs</name>
  <files>monstertruck-geometry/src/t_spline/t_nurcc.rs, monstertruck-geometry/src/t_spline/t_nurcc_edge.rs</files>
  <action>
Address the three TODOs in t_nurcc.rs:

1. **Lines ~390 and ~403 -- Connection parity check (L/R for alpha values)**:
   - The current code computes `a_od` using `[LeftAcw, LeftCw]` and `a_do` using `[RightAcw, RightCw]`.
   - Verify this mapping against Equation 14 in Sederberg et al. 1998: alpha_{ij} should use the knot intervals on the LEFT side of edge (i,j), and alpha_{ji} should use the RIGHT side.
   - The direction convention is: when traversing from origin to dest, "left" connections are LeftAcw/LeftCw and "right" connections are RightAcw/RightCw.
   - If the current mapping is correct, add a comment explaining why and remove the TODO. If incorrect, swap the connection arrays and add the explanatory comment.
   - Add debug assertions that validate the knot interval values are non-negative.

2. **Line ~451 -- Malformed face error**:
   - The current code returns `Error::TnurccMalformedFace` when radial_edges is empty.
   - Review whether a more specific error type is needed (e.g., `TnurccIsolatedVertex`).
   - If the existing error is appropriate, remove the TODO and add a clarifying comment about when this condition occurs. If a new error variant is warranted, add it to the errors module.

Remove all TODO comments and replace with documentation comments explaining the mathematical basis for each decision.
  </action>
  <verify>Run `cargo test -p monstertruck-geometry t_spline_validation` and confirm all connection parity tests pass. Run `cargo check -p monstertruck-geometry` to ensure no compilation errors.</verify>
  <done>Connection parity TODOs in t_nurcc.rs were resolved with verified mathematical basis and passing tests.</done>
</task>

<task type="auto">
  <name>Task 3: Resolve zero knot interval TODO in t_mesh.rs</name>
  <files>monstertruck-geometry/src/t_spline/t_mesh.rs, monstertruck-geometry/src/t_spline/t_mesh_control_point.rs</files>
  <action>
Address the TODO at t_mesh.rs line ~345 regarding zero knot intervals:

1. **Understand the issue**: The current `add_point_on_edge` method does not handle knot intervals of 0. Per Figure 9 in Sederberg et al. 2003, zero knot intervals are valid and represent degenerate cases where control points share the same parametric position.

2. **Implement zero knot interval support**:
   - Modify the `add_point_on_edge` logic to handle the case where `knot_interval * knot_ratio` or `knot_interval * (1.0 - knot_ratio)` equals 0.0.
   - When a zero knot interval results from a split, the new connection should still be created but with weight 0.0.
   - Ensure the perpendicular edge conditions (clockwise/anti-clockwise) are correctly evaluated even when adjacent edges have zero intervals.
   - Update the inferred connection logic (`find_inferred_connection`) to handle zero-interval boundaries correctly.

3. **Update knot vector computation**:
   - Review `compute_knot_vectors` to ensure it handles zero intervals in the knot vector assembly (zero intervals should appear as repeated knots).
   - Zero intervals between control points means those points have identical parametric coordinates, which is valid for multiplicity.

4. Remove the TODO comment and add documentation explaining the zero knot interval behavior per the paper.
  </action>
  <verify>Run `cargo test -p monstertruck-geometry t_spline_validation` and confirm all zero knot interval tests pass. Verify existing T-mesh tests still pass with `cargo test -p monstertruck-geometry`.</verify>
  <done>Zero knot interval handling was implemented in t_mesh.rs with documentation referencing the paper, and all tests pass.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-geometry` passes with all existing and new tests
2. `grep -r "TODO" monstertruck-geometry/src/t_spline/t_nurcc.rs monstertruck-geometry/src/t_spline/t_mesh.rs` returns no results
3. New test file `monstertruck-geometry/tests/t_spline_validation.rs` exists with tests for parity, zero intervals, and error handling
4. `cargo clippy -p monstertruck-geometry` produces no new warnings
</verification>

<success_criteria>
- All 4 TODOs in t_spline/ are resolved with tested implementations
- Connection parity (L/R) in T-NURCC subdivision is verified against the paper's equations
- Zero knot intervals are supported in T-mesh point insertion
- No regressions in existing monstertruck-geometry tests
</success_criteria>

<output>
After completion, create `.tendrion/phases/4-strategic-evolution/4-1-SUMMARY.md`
</output>
