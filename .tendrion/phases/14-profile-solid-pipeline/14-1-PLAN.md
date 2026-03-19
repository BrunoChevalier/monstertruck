---
phase: 14-profile-solid-pipeline
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-modeling/src/profile.rs
  - monstertruck-modeling/tests/profile_test.rs
  - monstertruck-modeling/src/errors.rs
autonomous: true
must_haves:
  truths:
    - "User calls profile::revolve_from_planar_profile with a closed rectangular wire and a Y-axis, producing a valid Solid with correct topology"
    - "User calls profile::revolve_from_planar_profile with a circle-like profile revolved 360 degrees around a distant axis, producing a torus-like solid"
    - "User calls profile::sweep_from_planar_profile with a closed profile and a 3D guide curve, producing a valid solid whose cross-sections match the profile"
    - "User passes a profile with holes to revolve_from_planar_profile and gets a solid with correct inner/outer surfaces"
    - "User passes invalid input (open wire, empty wires) and gets descriptive errors"
  artifacts:
    - path: "monstertruck-modeling/src/profile.rs"
      provides: "revolve_from_planar_profile and sweep_from_planar_profile public functions"
      min_lines: 300
      contains: "revolve_from_planar_profile"
    - path: "monstertruck-modeling/tests/profile_test.rs"
      provides: "Integration tests for revolve and sweep profile operations"
      min_lines: 200
      contains: "revolve_from_planar_profile"
  key_links:
    - from: "monstertruck-modeling/src/profile.rs"
      to: "monstertruck-modeling/src/builder.rs"
      via: "calls builder::revolve on face for revolve, and builder::try_sweep_rail for sweep"
      pattern: "builder::revolve"
    - from: "monstertruck-modeling/tests/profile_test.rs"
      to: "monstertruck-modeling/src/profile.rs"
      via: "imports and tests profile::revolve_from_planar_profile and profile::sweep_from_planar_profile"
      pattern: "profile::revolve_from_planar_profile"
---

<objective>
Extend the profile module to support revolve and sweep solid creation from planar profiles, complementing the existing extrusion path. Users will be able to revolve a closed planar profile around an axis to produce solids (including torus-like shapes) and sweep a profile along a 3D guide curve.
</objective>

<execution_context>
</execution_context>

<context>
@monstertruck-modeling/src/profile.rs
@monstertruck-modeling/src/builder.rs
@monstertruck-modeling/src/errors.rs
@monstertruck-modeling/tests/profile_test.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write tests for profile revolve and sweep operations</name>
  <files>monstertruck-modeling/tests/profile_test.rs</files>
  <action>
Add integration tests to `monstertruck-modeling/tests/profile_test.rs` for the new revolve and sweep functions. Tests should cover:

**Revolve tests:**
1. `revolve_simple_rect` -- revolve a rectangular profile (wire in XZ plane) 360 degrees around the Y-axis. The result should be a valid solid. Assert `is_geometric_consistent()` and that the shell has the expected number of faces (4 wire edges * division sections for the sides, plus potential degenerate-edge handling).
2. `revolve_torus_topology` -- revolve a small circle-approximation profile (square offset from the axis) 360 degrees around a parallel axis. Produces a torus-like solid. Assert `is_geometric_consistent()`.
3. `revolve_partial_angle` -- revolve a rectangular profile 90 degrees (PI/2). Should produce an open shell wrapped in a solid with capping faces. Assert the solid has boundaries and is geometrically consistent.
4. `revolve_with_hole` -- revolve a profile with an outer rectangle and inner hole around an axis. The solid should have more faces than without the hole. Assert `is_geometric_consistent()`.
5. `revolve_open_wire_rejected` -- passing an open wire should return an appropriate error.

**Sweep tests:**
6. `sweep_rect_along_line` -- sweep a rectangular profile along a straight-line BsplineCurve. Should produce a solid similar to extrusion. Assert `is_geometric_consistent()`.
7. `sweep_rect_along_curve` -- sweep a rectangular profile along a curved BsplineCurve (e.g., a quarter circle arc as a spline). Assert `is_geometric_consistent()`.
8. `sweep_open_wire_rejected` -- passing an open wire for sweep should return an error.

Each test should use the existing `rect_wire` helper or create appropriate wire geometry. Use `profile::revolve_from_planar_profile` and `profile::sweep_from_planar_profile` as the API names.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --test profile_test` -- tests should fail to compile since the functions don't exist yet.</verify>
  <done>Integration tests for revolve and sweep profile operations written and confirmed to reference the correct API signatures.</done>
</task>

<task type="auto">
  <name>Task 2: Implement revolve_from_planar_profile in profile.rs</name>
  <files>monstertruck-modeling/src/profile.rs</files>
  <action>
Add a public function `revolve_from_planar_profile` to `monstertruck-modeling/src/profile.rs`:

```rust
/// Constructs a [`Solid`] by revolving a planar profile around an axis.
///
/// Takes a set of wires (possibly with holes), normalizes orientation,
/// creates a planar face, and revolves it around the specified axis.
///
/// For 360-degree revolves, produces a closed solid (e.g., torus).
/// For partial revolves, attaches planar caps at both ends.
///
/// # Arguments
/// * `wires` - closed planar profile wires (outer + holes)
/// * `origin` - point on the revolve axis
/// * `axis` - unit vector defining the revolve axis direction
/// * `angle` - revolve angle (>=2pi means full revolution)
/// * `division` - number of angular divisions (>=2 for full, >=1 for partial)
///
/// # Errors
/// Returns errors from [`attach_plane_normalized`] if the profile is invalid.
pub fn revolve_from_planar_profile<C, S, R>(
    wires: Vec<Wire<C>>,
    origin: Point3,
    axis: Vector3,
    angle: R,
    division: usize,
) -> Result<monstertruck_topology::Solid<Point3, C, S>>
```

Implementation approach -- use face-level revolve via `builder::revolve`:
1. Normalize wires via `classify_and_normalize`.
2. Create the planar face via `crate::builder::try_attach_plane(normalized_wires)`.
3. Call `crate::builder::revolve(&face, origin, axis, angle, division)`.
   - For a `Face` input, `builder::revolve` returns a `Shell` (the `ClosedSweep`/`MultiSweep` trait implementation for Face produces Shell).
4. Wrap the resulting shell in `Solid::new(vec![shell])`.

The trait bounds should match what `builder::revolve` requires for Face:
```rust
where
    C: ParametricCurve3D + BoundedCurve + Cut + Invertible + Transformed<Matrix4>,
    S: Invertible + Transformed<Matrix4>,
    R: Into<Rad<f64>>,
    Processor<TrimmedCurve<UnitCircle<Point3>>, Matrix4>: ToSameGeometry<C>,
    RevolutedCurve<C>: ToSameGeometry<S>,
    Plane: IncludeCurve<C> + ToSameGeometry<S>,
    Line<Point3>: ToSameGeometry<C>,
```

This approach is cleaner than revolving individual wires because `builder::revolve` on a Face handles caps and topology correctly for both full and partial revolves.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --test profile_test revolve` -- revolve tests should pass.</verify>
  <done>revolve_from_planar_profile implemented and passing all revolve integration tests.</done>
</task>

<task type="auto">
  <name>Task 3: Implement sweep_from_planar_profile in profile.rs</name>
  <files>monstertruck-modeling/src/profile.rs, monstertruck-modeling/src/errors.rs</files>
  <action>
Add a public function `sweep_from_planar_profile` to `monstertruck-modeling/src/profile.rs`:

```rust
/// Constructs a [`Solid`] by sweeping a planar profile along a 3D guide curve.
///
/// Takes a set of wires (possibly with holes), normalizes orientation,
/// and sweeps each outer wire edge along the guide rail to produce swept faces,
/// then combines them with cap faces into a solid.
///
/// # Arguments
/// * `wires` - closed planar profile wires (outer + holes)
/// * `guide` - 3D BsplineCurve defining the sweep path
/// * `n_sections` - number of cross-sections along the guide (>=2)
///
/// # Errors
/// Returns errors from profile normalization or sweep construction.
pub fn sweep_from_planar_profile(
    wires: Vec<Wire<Curve>>,
    guide: &BsplineCurve<Point3>,
    n_sections: usize,
) -> Result<monstertruck_topology::Solid<Point3, Curve, Surface>>
```

**Use ONE clear approach: per-edge sweep via `builder::try_sweep_rail`.**

Implementation:
1. Normalize wires via `classify_and_normalize`.
2. Create start cap face via `crate::builder::try_attach_plane(normalized_wires.clone())`.
3. For each edge of the outer wire, extract its curve as a `BsplineCurve<Point3>`:
   - If `Curve::BsplineCurve(bsp)` -- use directly.
   - If `Curve::Line(line)` -- convert to a degree-1 BsplineCurve with `BsplineCurve::new(KnotVector::bezier_knot(1), vec![line.0, line.1])`.
   - Add a helper `edge_curve_to_bspline(edge: &Edge) -> Result<BsplineCurve<Point3>>` that handles this conversion, returning `Error::UnsupportedCurveType` for other curve variants.
4. For each extracted BsplineCurve profile, call `builder::try_sweep_rail(&profile, guide, n_sections)` to get a `Face`.
5. Create end cap face: sample the guide at its end parameter, compute the transform from start to end (translation + rotation from guide tangent), apply to the normalized wires, then call `builder::try_attach_plane` on the transformed wires.
6. Collect all swept side faces + start cap + end cap into a Shell, wrap in `Solid::new(vec![shell])`.

If the `UnsupportedCurveType` error variant does not exist in errors.rs, add it:
```rust
#[error("unsupported curve type for sweep: only Line and BsplineCurve edges are supported.")]
UnsupportedCurveType,
```

This approach keeps the implementation focused on the well-tested `try_sweep_rail` API and avoids the complexity of face-level sweep or homotopy-based approaches.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --test profile_test sweep` -- sweep tests should pass.</verify>
  <done>sweep_from_planar_profile implemented and passing all sweep integration tests.</done>
</task>

</tasks>

<verification>
1. All existing profile tests still pass: `cargo nextest run -p monstertruck-modeling --test profile_test`
2. All existing font pipeline tests still pass: `cargo nextest run -p monstertruck-modeling --features font --test font_pipeline`
3. New revolve tests pass: `cargo nextest run -p monstertruck-modeling --test profile_test -E 'test(revolve)'`
4. New sweep tests pass: `cargo nextest run -p monstertruck-modeling --test profile_test -E 'test(sweep)'`
5. No compiler warnings: `cargo clippy -p monstertruck-modeling --all-targets -- -W warnings`
6. Doc tests pass: `cargo nextest run -p monstertruck-modeling --doc`
</verification>

<success_criteria>
- A closed planar profile can be revolved around an axis to produce a valid solid with correct topology
- A circle-like profile revolved 360 degrees around a distant axis produces a torus-like solid
- A closed planar profile can be swept along a 3D guide curve to produce a valid solid
- Profiles with holes work correctly with revolve (inner surfaces generated)
- Invalid inputs (open wires, empty wires) produce descriptive errors
- All solids pass is_geometric_consistent() checks
</success_criteria>

<output>
After completion, create `.tendrion/phases/14-profile-solid-pipeline/14-1-SUMMARY.md`
</output>
