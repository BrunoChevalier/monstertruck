---
phase: 22-conversion-fidelity
plan: 2
type: execute
wave: 2
depends_on: ["22-1"]
files_modified:
  - monstertruck-modeling/src/fillet_impl.rs
  - monstertruck-geometry/src/decorators/revolved_curve.rs
autonomous: true
must_haves:
  truths:
    - "RevolutedCurve surfaces convert to NurbsSurface via exact rational circle arc tensor product"
    - "TryFrom<Surface> for NurbsSurface<Vector4> succeeds for Surface::RevolutedCurve instead of returning Err(())"
    - "The converted NURBS surface evaluates to the same points as the original RevolutedCurve within machine epsilon"
    - "Full 2*PI revolution and partial arcs are both handled correctly"
    - "The sampling fallback path is no longer reached for RevolutedCurve surfaces"
  artifacts:
    - path: "monstertruck-modeling/src/fillet_impl.rs"
      provides: "TryFrom<Surface> for NurbsSurface<Vector4> handles RevolutedCurve variant"
      min_lines: 50
      contains: "RevolutedCurve"
    - path: "monstertruck-geometry/src/decorators/revolved_curve.rs"
      provides: "Exact NURBS conversion method for RevolutedCurve"
      min_lines: 500
      contains: "to_nurbs_surface"
  key_links:
    - from: "monstertruck-geometry/src/decorators/revolved_curve.rs"
      to: "monstertruck-modeling/src/fillet_impl.rs"
      via: "RevolutedCurve::to_nurbs_surface called from TryFrom<Surface>"
      pattern: "to_nurbs_surface"
    - from: "monstertruck-geometry/src/decorators/revolved_curve.rs"
      to: "monstertruck-solid/src/fillet/convert.rs"
      via: "FilletableSurface::to_nurbs_surface tries TryInto first, avoiding sampling fallback"
      pattern: "try_into"
---

<objective>
Implement exact RevolutedCurve to NurbsSurface conversion via rational circle arc tensor product, eliminating the sampling fallback for this common surface type. A RevolutedCurve is the tensor product of its profile curve (u-direction) and a rational circle arc (v-direction). This conversion produces an exact NURBS representation.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/decorators/revolved_curve.rs
@monstertruck-geometry/src/decorators/mod.rs
@monstertruck-modeling/src/fillet_impl.rs
@monstertruck-modeling/src/geometry.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Implement RevolutedCurve::to_nurbs_surface for exact conversion</name>
  <files>monstertruck-geometry/src/decorators/revolved_curve.rs</files>
  <action>
Add a method `to_nurbs_surface` to `RevolutedCurve<NurbsCurve<Vector4>>` (and a convenience version for `BsplineCurve<Point3>`) that converts the revolved surface to an exact NURBS representation.

The algorithm for revolving a NURBS curve around an axis:

**V-direction (revolution/circle arc):**
A full circle of revolution (0 to 2*PI) is represented as a rational NURBS curve using 9 control points with degree 2. For a point P at radius r from the axis, the circle arc control points and weights follow the standard rational Bezier circle construction:
- Split the full circle into 4 quarter arcs
- Each quarter uses 3 control points: on-circle, off-circle (weight = cos(pi/4) = 1/sqrt(2)), on-circle
- Knot vector: [0,0,0, pi/2,pi/2, pi,pi, 3pi/2,3pi/2, 2pi,2pi,2pi]

For a point P on the profile curve at parameter u:
1. Project P onto the revolution axis to get the axis point A
2. Compute the radius vector R = P - A (perpendicular to axis)
3. Compute the tangent direction T = axis x R (normalized, perpendicular to both)
4. The 9 control points for the v-circle at this u are computed using R and T

**Tensor product construction:**
For each control point of the profile NurbsCurve (u-direction), compute the 9 circle-arc control points (v-direction). The result is an (n_u x 9) control point grid in homogeneous coordinates (Vector4), with:
- u_knot = profile curve's knot vector
- v_knot = circle arc knot vector [0,0,0, pi/2,pi/2, pi,pi, 3pi/2,3pi/2, 2pi,2pi,2pi]

**Handling the profile curve weights:**
If the profile NurbsCurve control point is `w_i * [x_i, y_i, z_i, 1]` (homogeneous Vector4), then the tensor product control point weight is `w_i * w_circle_j`. The 3D position of each tensor product control point is computed from the profile control point's 3D position (after dividing by w_i).

**Implementation steps:**
1. Extract the profile curve's control points and knot vector
2. For each profile control point (in 3D after dehomogenization):
   a. Project onto axis: `a = origin + ((cp - origin).dot(axis)) * axis`
   b. Radius vector: `r_vec = cp - a`
   c. Radius: `r = r_vec.magnitude()`
   d. If r is near zero (point on axis), all 9 circle points collapse to the same point (A)
   e. Otherwise: unit_r = r_vec / r, unit_t = axis.cross(unit_r).normalize()
   f. Generate 9 control points using the standard rational circle representation
3. Combine profile weights with circle weights
4. Build the NurbsSurface

**Circle arc control points for point at A with radius r, radial direction R, tangent T:**
Using the standard 9-point rational circle:
```
cos: [1, 1, 0, -1, -1, -1, 0, 1, 1]
sin: [0, 1, 1, 1, 0, -1, -1, -1, 0]
weights: [1, 1/sqrt(2), 1, 1/sqrt(2), 1, 1/sqrt(2), 1, 1/sqrt(2), 1]
```
cp_3d[j] = A + r * (cos[j] * unit_r + sin[j] * unit_t)
w_total[j] = w_profile * w_circle[j]
homogeneous[j] = Vector4::new(cp_3d[j].x * w_total[j], cp_3d[j].y * w_total[j], cp_3d[j].z * w_total[j], w_total[j])

Add the method to the impl block:
```rust
impl RevolutedCurve<NurbsCurve<Vector4>> {
    pub fn to_nurbs_surface(&self) -> NurbsSurface<Vector4> { ... }
}
impl RevolutedCurve<BsplineCurve<Point3>> {
    pub fn to_nurbs_surface(&self) -> NurbsSurface<Vector4> {
        let nurbs_curve = NurbsCurve::from(self.curve.clone());
        RevolutedCurve::by_revolution(nurbs_curve, self.origin(), self.axis())
            .to_nurbs_surface()
    }
}
```

Note: This method is on the `RevolutedCurve` struct in the geometry crate since it's a pure geometric operation.
  </action>
  <verify>
Add a unit test in the same file that:
1. Creates a RevolutedCurve from a known NurbsCurve (e.g., a line segment) around the Y axis
2. Converts to NurbsSurface
3. Evaluates both surfaces at a grid of (u,v) points and asserts they match within 1e-10
4. Run `cargo test -p monstertruck-geometry -- revolved` to verify

Also test with a half-circle profile (rational Bezier) to exercise weighted profile curves.
  </verify>
  <done>RevolutedCurve has an exact to_nurbs_surface method that produces rational NURBS via circle arc tensor product.</done>
</task>

<task type="auto">
  <name>Task 2: Wire RevolutedCurve conversion into TryFrom<Surface> for NurbsSurface</name>
  <files>monstertruck-modeling/src/fillet_impl.rs</files>
  <action>
Update the `TryFrom<Surface> for NurbsSurface<Vector4>` implementation to handle `Surface::RevolutedCurve` instead of returning `Err(())`.

Current code:
```rust
Surface::RevolutedCurve(_) | Surface::TSplineSurface(_) => Err(()),
```

Change to handle `RevolutedCurve` by unwrapping the `Processor` and calling the new conversion:
```rust
Surface::RevolutedCurve(proc) => {
    // The Processor wraps RevolutedCurve<Curve> with a Matrix4 transform.
    // First convert the inner RevolutedCurve to NURBS, then apply the transform.
    // 1. Extract the inner Curve from RevolutedCurve
    // 2. Convert it to NurbsCurve<Vector4> (TryFrom<Curve> for NurbsCurve<Vector4> exists)
    // 3. Build a RevolutedCurve<NurbsCurve<Vector4>> with same origin/axis
    // 4. Call to_nurbs_surface() for exact NURBS
    // 5. Apply Processor's transform matrix to resulting surface control points
    // 6. Handle orientation (if Processor is inverted, invert the surface)
    Ok(result)
}
```

Check how `Processor` exposes its internals -- it likely has `entity()`, `transform()`, `orientation()` getters or can be destructured. The `RevolutedCurve` has `entity_curve()`, `origin()`, and `axis()` accessors.

This eliminates the sampling fallback for RevolutedCurve in the `FilletableSurface::to_nurbs_surface` trait method (convert.rs line 27-31), since `try_into()` will now succeed.
  </action>
  <verify>
Run `cargo build -p monstertruck-modeling` and `cargo test -p monstertruck-modeling` to verify compilation and no regressions. Verify that `Surface::RevolutedCurve` no longer falls through to `Err(())`.
  </verify>
  <done>TryFrom<Surface> for NurbsSurface<Vector4> handles RevolutedCurve via exact conversion, eliminating sampling fallback.</done>
</task>

</tasks>

<verification>
1. `cargo build --workspace` compiles without errors
2. `cargo test -p monstertruck-geometry -- revolved` passes the new unit test for exact conversion
3. `cargo test -p monstertruck-modeling` passes all existing tests
4. `cargo test -p monstertruck-solid -- fillet` passes all fillet tests (RevolutedCurve surfaces now convert exactly)
5. Grep for `RevolutedCurve(_) |` in fillet_impl.rs shows it's no longer in the Err(()) arm
</verification>

<success_criteria>
- RevolutedCurve surfaces convert to NurbsSurface via rational circle arc tensor product without falling back to the sampling path (FCONV-03)
- The exact conversion is mathematically correct (evaluated points match within machine epsilon)
- Existing tests pass without regression
</success_criteria>

<output>
After completion, create `.tendrion/phases/22-conversion-fidelity/22-2-SUMMARY.md`
</output>
