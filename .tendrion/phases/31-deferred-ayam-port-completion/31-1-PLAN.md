---
phase: 31-deferred-ayam-port-completion
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/tests/gordon_intersection_grid_test.rs
  - monstertruck-modeling/tests/gordon_brep_validation_test.rs
autonomous: true
must_haves:
  truths:
    - "User calls try_gordon_from_network with curved (non-linear) u/v curve families and gets a valid Gordon surface"
    - "User builds a shell from intersection-grid Gordon faces and shell_condition() returns Regular or better"
    - "User supplies a 3x3 curve grid with curved intersections and the Gordon surface interpolates all grid intersection points within tolerance"
    - "User calls try_gordon_from_network with near-tangent curve pairs and gets a meaningful error, not a panic"
  artifacts:
    - path: "monstertruck-geometry/tests/gordon_intersection_grid_test.rs"
      provides: "Integration tests for intersection-grid Gordon surface with curved networks and B-rep validation"
      min_lines: 80
      contains: "try_gordon_from_network"
    - path: "monstertruck-modeling/tests/gordon_brep_validation_test.rs"
      provides: "Shell-level validation tests proving Gordon surfaces produce valid B-rep topology"
      min_lines: 60
      contains: "shell_condition"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-modeling/tests/gordon_brep_validation_test.rs"
      via: "try_gordon_from_network builds Face, assembled into Shell for validation"
      pattern: "try_gordon_from_network"
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/tests/gordon_intersection_grid_test.rs"
      via: "Geometry-level tests for curved intersection grid computation"
      pattern: "try_gordon_from_network"
---

<objective>
Validate and harden intersection-grid driven Gordon surface construction with curved curve networks and prove B-rep topology correctness through shell validation tests. This extends the Phase 18 implementation (which focused on API creation) with rigorous curved-network integration testing and shell-level topology validation required by PORT-01 success criteria.
</objective>

<execution_context>
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs
@monstertruck-modeling/src/builder.rs
@monstertruck-topology/src/shell.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: RED - Write failing tests for curved intersection-grid Gordon surfaces and B-rep validation</name>
  <files>monstertruck-geometry/tests/gordon_intersection_grid_test.rs, monstertruck-modeling/tests/gordon_brep_validation_test.rs</files>
  <action>
Create two new test files:

**monstertruck-geometry/tests/gordon_intersection_grid_test.rs:**
Tests exercising `try_gordon_from_network` with non-trivial curved curve families:

1. `curved_3x3_network_interpolates_intersections` - Build a 3x3 grid of quadratic curves (e.g., parabolic arcs in u-direction, circular-like arcs in v-direction). Call `try_gordon_from_network`. Independently compute intersection points using `curve_intersect::find_intersections` and verify the resulting surface evaluates to within `SNAP_TOLERANCE` at those parameter locations.

2. `curved_network_with_near_tangent_curves_error` - Create curve pairs that are nearly tangent (close to touching but not cleanly intersecting) and verify `try_gordon_from_network` returns an `IntersectionCountMismatch` error rather than panicking.

3. `large_5x4_curved_grid_success` - Build a 5x4 network of degree-2 curves with genuine curved intersections. Verify `try_gordon_from_network` succeeds and the surface has expected control point grid dimensions.

4. `gordon_from_network_surface_corners_match_curve_endpoints` - For a 2x2 curved grid, verify that the Gordon surface corner evaluations `subs(0,0)`, `subs(1,0)`, `subs(0,1)`, `subs(1,1)` match the expected curve intersection points.

**monstertruck-modeling/tests/gordon_brep_validation_test.rs:**
Tests proving B-rep topology validity:

1. `gordon_face_has_valid_boundary_topology` - Build a Gordon face via `builder::try_gordon_from_network`. Verify `face.boundaries().len() == 1` and `face.boundaries()[0].len() == 4` (quad boundary).

2. `gordon_shell_passes_shell_condition` - Assemble two adjacent Gordon faces sharing an edge into a `Shell`. Call `shell.shell_condition()` and assert it is at least `ShellCondition::Regular`.

3. `gordon_face_tessellation_produces_valid_mesh` - Build a Gordon face, tessellate it with `robust_triangulation`, verify the output polygon mesh has non-zero positions and normals.

Use `use monstertruck_topology::shell::ShellCondition;` for shell validation.
Use `cargo nextest run` verification.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry --test gordon_intersection_grid_test` and `cargo nextest run -p monstertruck-modeling --test gordon_brep_validation_test`. Tests should compile but some may fail if the current implementation cannot handle certain curved networks.</verify>
  <done>Failing test files created for both geometry-level curved intersection-grid tests and modeling-level B-rep validation tests.</done>
</task>

<task type="auto">
  <name>Task 2: GREEN - Fix any intersection-grid issues and ensure all tests pass</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs, monstertruck-geometry/src/nurbs/surface_options.rs</files>
  <action>
Run the RED tests. Analyze failures:

1. If `curved_3x3_network_interpolates_intersections` fails because the surface doesn't interpolate intersection points accurately enough, investigate the `try_gordon` internal skinning + tensor product assembly. The issue may be in how the compatibility normalization reparameterizes curves relative to intersection points. Add a `GordonOptions` field `intersection_tolerance: f64` (defaulting to `SNAP_TOLERANCE`) that controls how much the intersection-point-to-surface deviation is tolerated.

2. If `curved_network_with_near_tangent_curves_error` panics instead of returning an error, add guards in `try_gordon_from_network` around the `find_intersections` call to catch edge cases (zero or multiple intersections) and convert them to proper `CurveNetworkDiagnostic` errors.

3. If B-rep tests fail because boundary topology is wrong, investigate `face_from_bspline_surface` in `monstertruck-modeling/src/builder.rs` to ensure it correctly extracts 4 boundary edges from the Gordon surface.

4. Adjust test expectations if the implementation is already correct (tests may pass as-is if Phase 18 implementation handles curved networks well).

Key areas to modify if needed:
- `bspline_surface.rs`: `try_gordon_from_network` intersection handling robustness
- `surface_options.rs`: Any new tolerance fields on `GordonOptions`
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry --test gordon_intersection_grid_test` and `cargo nextest run -p monstertruck-modeling --test gordon_brep_validation_test` -- all tests pass. Also run `cargo nextest run -p monstertruck-geometry -p monstertruck-modeling` to verify no regressions.</verify>
  <done>All intersection-grid Gordon surface tests and B-rep validation tests pass. No regressions in existing test suites.</done>
</task>

<task type="auto">
  <name>Task 3: REFACTOR - Clean up and add documentation</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs, monstertruck-geometry/tests/gordon_intersection_grid_test.rs</files>
  <action>
1. Review `try_gordon_from_network` implementation and add/improve doc comments explaining the intersection-grid algorithm: how curves are intersected before compatibility normalization, why this ordering matters for numerical accuracy.

2. Extract any repeated test helper functions (curve construction) into a shared helper module within the test file.

3. Run `cargo clippy -p monstertruck-geometry -p monstertruck-modeling --all-targets -- -W warnings` and fix any warnings.

4. Ensure all new public types/methods have complete doc comments with `# Errors`, `# Examples`, and `# Panics` sections as appropriate.
  </action>
  <verify>Run `cargo clippy -p monstertruck-geometry -p monstertruck-modeling --all-targets -- -W warnings` with zero warnings. Run full test suite `cargo nextest run -p monstertruck-geometry -p monstertruck-modeling` with all tests passing.</verify>
  <done>Code cleaned up, doc comments complete, clippy clean, all tests passing.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-geometry --test gordon_intersection_grid_test` -- all tests pass
2. `cargo nextest run -p monstertruck-modeling --test gordon_brep_validation_test` -- all tests pass
3. `cargo nextest run -p monstertruck-geometry -p monstertruck-modeling` -- no regressions
4. `cargo clippy -p monstertruck-geometry -p monstertruck-modeling --all-targets -- -W warnings` -- clean
5. Gordon surfaces built from curved networks produce faces with 4-edge boundaries
6. Shell assembled from Gordon faces has ShellCondition::Regular or better
</verification>

<success_criteria>
- Gordon surface constructor accepts curved curve families and automatically computes intersection grid points (PORT-01)
- Intersection-grid Gordon surfaces produce valid B-rep topology that passes shell validation (success criterion 2)
- Near-tangent or degenerate curve networks produce diagnostic errors, not panics
- All existing Gordon surface tests continue to pass
</success_criteria>

<output>
After completion, create `.tendrion/phases/31-deferred-ayam-port-completion/31-1-SUMMARY.md`
</output>
