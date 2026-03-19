---
phase: 14-profile-solid-pipeline
plan: 3
type: tdd
wave: 3
depends_on: ["14-1", "14-2"]
files_modified:
  - monstertruck-modeling/src/profile.rs
  - monstertruck-modeling/src/errors.rs
  - monstertruck-modeling/tests/profile_test.rs
  - monstertruck-modeling/tests/font_pipeline.rs
autonomous: true
must_haves:
  truths:
    - "User calls profile::validate_solid on an extruded profile solid and gets Ok(()) confirming Euler-Poincare invariant and orientation"
    - "User calls profile::validate_solid on a revolved profile solid and gets Ok(()) confirming topology and geometric consistency"
    - "User calls profile::validate_solid on a swept profile solid and gets Ok(()) confirming topology and geometric consistency"
    - "User calls profile::validate_solid on a mixed glyph+custom extruded solid and gets Ok(()) confirming consistency"
    - "User calls profile::validate_solid on a deliberately broken solid and gets an Err describing the topology violation"
    - "User calls profile::validate_solid and the tessellation smoke test confirms the solid can be meshed without panics"
  artifacts:
    - path: "monstertruck-modeling/src/profile.rs"
      provides: "validate_solid function with Euler-Poincare and orientation checks"
      min_lines: 400
      contains: "validate_solid"
    - path: "monstertruck-modeling/tests/profile_test.rs"
      provides: "Validation tests for all profile-generated solid types including negative and tessellation tests"
      min_lines: 280
      contains: "validate_solid"
  key_links:
    - from: "monstertruck-modeling/src/profile.rs"
      to: "monstertruck-topology/src/shell.rs"
      via: "uses shell_condition() and ShellCondition for orientation checks"
      pattern: "shell_condition"
    - from: "monstertruck-modeling/src/profile.rs"
      to: "monstertruck-topology/src/solid.rs"
      via: "uses is_geometric_consistent() for edge-surface consistency"
      pattern: "is_geometric_consistent"
---

<objective>
Add comprehensive consistency and tessellation validation for all profile-generated solids. The validate_solid function will check Euler-Poincare invariants, orientation consistency, geometric consistency, and optional tessellation soundness, providing clear error diagnostics when any check fails. Includes negative tests for broken solids and a tessellation smoke test.
</objective>

<execution_context>
</execution_context>

<context>
@monstertruck-modeling/src/profile.rs
@monstertruck-solid/src/fillet/validate.rs
@monstertruck-topology/src/shell.rs
@monstertruck-topology/src/solid.rs
@monstertruck-modeling/tests/profile_test.rs
@monstertruck-modeling/tests/font_pipeline.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write tests for profile solid validation including negative and tessellation tests</name>
  <files>monstertruck-modeling/tests/profile_test.rs, monstertruck-modeling/tests/font_pipeline.rs</files>
  <action>
Add validation tests to both test files.

**In `profile_test.rs`, add:**

1. `validate_extruded_box` -- Create a simple extruded box via `solid_from_planar_profile`. Call `profile::validate_solid(&solid)`. Assert Ok(()).

2. `validate_extruded_with_hole` -- Extrude a profile with a hole. Validate. Assert Ok(()).

3. `validate_revolved_solid` -- Create a revolved solid via `profile::revolve_from_planar_profile` (full 360-degree revolve). Validate. Assert Ok(()).

4. `validate_swept_solid` -- Create a swept solid via `profile::sweep_from_planar_profile`. Validate. Assert Ok(()).

5. `validate_triangle_extrusion` -- Extrude a triangular profile. Validate. Assert Ok(()).

6. `validate_diagonal_extrusion` -- Extrude with a diagonal direction vector. Validate. Assert Ok(()).

7. `validate_reports_details` -- Call `profile::validate_solid` on a valid solid and verify the returned `ValidationReport` contains expected metrics (vertex count > 0, edge count > 0, face count > 0, euler_characteristic == 2).

8. `validate_broken_solid_returns_error` -- **Negative test.** Construct a deliberately broken solid by manually building a Shell with inconsistent topology (e.g., create 3 faces that share edges but leave one face missing so the shell is not closed, or create a shell with an edge shared by 3 faces). Wrap in `Solid::new(vec![shell])`. Call `profile::validate_solid(&solid)`. Assert `result.is_err()` and that the error message contains a descriptive reason (e.g., mentioning "euler" or "orientation" or "geometric").

   Construction approach for broken solid:
   ```rust
   // Build a partial box: take an extruded box and remove one face from the shell.
   let outer = rect_wire(-1.0, -1.0, 1.0, 1.0);
   let solid = profile::solid_from_planar_profile::<Curve, Surface>(
       vec![outer], Vector3::new(0.0, 0.0, 1.0),
   ).unwrap();
   let shell = &solid.boundaries()[0];
   // Create a new shell with one face removed (making it non-closed).
   let mut faces: Vec<_> = shell.iter().cloned().collect();
   faces.pop(); // Remove last face
   let broken_shell: Shell = faces.into_iter().collect();
   let broken_solid = Solid::new(vec![broken_shell]);
   let result = profile::validate_solid(&broken_solid);
   assert!(result.is_err());
   ```

9. `validate_tessellation_smoke` -- **Tessellation test.** Create an extruded box solid, validate it, then call `solid.triangulation()` (or the meshing API from `monstertruck-meshing` if available as a dev-dependency). Assert that the triangulation produces a non-empty mesh (non-zero triangles). If `triangulation()` is not directly available on Solid, use `shell.triangulation()` on each boundary shell. This confirms that validated solids can actually be tessellated without panics.

   If the meshing API requires features or is in a separate crate not available as a dev-dependency, implement this as a simpler check: iterate over all faces in the solid, call `face.oriented_surface()`, and verify the surface can be sampled at `(0.5, 0.5)` without panicking. This confirms geometric data is intact for downstream tessellation.

**In `font_pipeline.rs`, add (gated behind `#[cfg(feature = "font")]`):**

10. `validate_glyph_o_extruded` -- Extrude glyph 'O' profile. Validate. Assert Ok(()).

11. `validate_glyph_b_extruded` -- Extrude glyph 'B' profile. Validate. Assert Ok(()).

12. `validate_mixed_glyph_custom_extruded` -- Create mixed glyph+custom solid. Validate. Assert Ok(()).

All tests should use `profile::validate_solid` as the API entry point.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --test profile_test` -- tests should fail to compile since `validate_solid` doesn't exist yet.</verify>
  <done>Validation tests written for all profile-generated solid types, including negative test for broken solid error path and tessellation smoke test.</done>
</task>

<task type="auto">
  <name>Task 2: Implement validate_solid with Euler-Poincare and consistency checks</name>
  <files>monstertruck-modeling/src/profile.rs, monstertruck-modeling/src/errors.rs</files>
  <action>
Add validation types and the `validate_solid` function to `monstertruck-modeling/src/profile.rs`.

**Add a ValidationReport struct:**

```rust
/// Report from solid topology validation.
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Number of unique vertices.
    pub vertices: usize,
    /// Number of unique edges.
    pub edges: usize,
    /// Number of faces.
    pub faces: usize,
    /// Euler characteristic (V - E + F).
    pub euler_characteristic: isize,
    /// Whether the shell is oriented (faces have compatible normals).
    pub is_oriented: bool,
    /// Whether the shell is closed (watertight).
    pub is_closed: bool,
    /// Whether all edges are geometrically consistent with their vertices.
    pub is_geometric_consistent: bool,
}
```

**Add to `errors.rs`:**
```rust
/// Profile solid validation failure.
#[error("profile solid validation failed: {reason}")]
ProfileValidationFailed {
    /// Description of the validation failure.
    reason: String,
},
```

**Implement validate_solid:**

```rust
/// Validates that a profile-generated solid passes topological and geometric
/// consistency checks.
///
/// Performs the following checks on each shell boundary:
/// 1. **Euler-Poincare**: For closed shells, V - E + F must equal 2.
/// 2. **Orientation**: Shell must be oriented or closed (faces have compatible normals).
/// 3. **Geometric consistency**: All edge endpoints must match their curve endpoints.
///
/// Returns a [`ValidationReport`] with detailed metrics on success, or an
/// error describing the first violation found.
pub fn validate_solid<C, S>(
    solid: &monstertruck_topology::Solid<Point3, C, S>,
) -> Result<ValidationReport>
where
    C: ParametricCurve3D + BoundedCurve + Clone,
    S: IncludeCurve<C>,
{
```

Implementation:
1. Iterate over `solid.boundaries()` (shells).
2. For each shell:
   a. Count unique vertices (by ID via HashSet), edges (by ID), and faces.
   b. Compute Euler characteristic: V - E + F.
   c. Check `shell.shell_condition()` for orientation (must be `Oriented` or `Closed`).
   d. For closed shells, verify euler_characteristic == 2.
   e. Check `shell.is_geometric_consistent()`.
3. If any check fails, return `Err(Error::ProfileValidationFailed { reason: ... })` with a descriptive message including the actual V, E, F values and which check failed.
4. On success, return `Ok(ValidationReport { ... })` with the aggregated metrics from the first (primary) shell.

Import `std::collections::HashSet` and `monstertruck_topology::shell::ShellCondition` at the top of the file. The `IncludeCurve` trait is needed for `is_geometric_consistent()` on faces.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --test profile_test -E 'test(validate)' ` and `cargo nextest run -p monstertruck-modeling --features font --test font_pipeline -E 'test(validate)'` -- all validation tests should pass.</verify>
  <done>validate_solid implemented with Euler-Poincare, orientation, and geometric consistency checks. All validation tests pass including negative error-path test.</done>
</task>

<task type="auto">
  <name>Task 3: Add comprehensive cross-cutting validation tests</name>
  <files>monstertruck-modeling/tests/profile_test.rs</files>
  <action>
Add cross-cutting tests that exercise the full pipeline from profile creation through validation, confirming that all three profile operations (extrude, revolve, sweep) produce solids that consistently pass all checks.

**Tests to add:**

1. `all_profile_ops_euler_poincare` -- For each of extrude, revolve, and sweep: create a solid from the same rectangular profile, validate, and assert `report.euler_characteristic == 2`.

2. `validation_report_metrics_consistent` -- Create an extruded box. Get the validation report. Assert:
   - `report.vertices == 8` (box has 8 vertices)
   - `report.edges == 12` (box has 12 edges)
   - `report.faces == 6` (box has 6 faces)
   - `report.euler_characteristic == 2`
   - `report.is_oriented == true`
   - `report.is_closed == true`
   - `report.is_geometric_consistent == true`

3. `validation_report_tube_metrics` -- Create an extruded tube (rect with hole). Get report. Assert:
   - `report.faces == 10` (2 caps with holes + 4 outer sides + 4 inner sides)
   - `report.is_geometric_consistent == true`

4. `profile_revolved_torus_closed` -- Create a torus via full revolve. Validate. Assert `report.is_closed == true`.

These tests provide the final verification that all profile-generated solids pass Euler-Poincare invariant checks and tessellate without topological issues (success criterion 4).
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --test profile_test` -- all tests pass, including the new cross-cutting validation tests.</verify>
  <done>Cross-cutting validation tests written and passing, confirming Euler-Poincare invariants hold for all profile-generated solid types.</done>
</task>

</tasks>

<verification>
1. All profile tests pass: `cargo nextest run -p monstertruck-modeling --test profile_test`
2. All font pipeline tests pass: `cargo nextest run -p monstertruck-modeling --features font --test font_pipeline`
3. Validation catches topology violations: `validate_broken_solid_returns_error` confirms error paths work
4. ValidationReport metrics match expected values for known shapes (box: V=8, E=12, F=6)
5. Every profile-generated solid type (extrude, revolve, sweep, mixed glyph) passes validate_solid
6. Tessellation smoke test confirms validated solids can be meshed
7. No regressions in existing test suites: `cargo nextest run -p monstertruck-modeling`
</verification>

<success_criteria>
- All profile-generated solids pass Euler-Poincare invariant checks (V - E + F = 2 for closed shells)
- All solids pass orientation checks (shell_condition is Oriented or Closed)
- All solids pass geometric consistency checks (edge endpoints match curve endpoints)
- ValidationReport provides useful diagnostic metrics
- Invalid/broken solids produce descriptive error messages via validate_solid
- Tessellation smoke test passes for validated solids
- Extruded, revolved, swept, and mixed glyph+custom solids all validate successfully
</success_criteria>

<output>
After completion, create `.tendrion/phases/14-profile-solid-pipeline/14-3-SUMMARY.md`
</output>
