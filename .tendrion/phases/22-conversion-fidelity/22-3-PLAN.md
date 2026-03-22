---
phase: 22-conversion-fidelity
plan: 3
type: execute
wave: 3
depends_on: ["22-1", "22-2"]
files_modified:
  - monstertruck-solid/src/fillet/convert.rs
  - monstertruck-solid/src/fillet/tests.rs
autonomous: true
must_haves:
  truths:
    - "Converted curve endpoints exactly match their source vertex positions within machine epsilon"
    - "Endpoint snapping is applied in both convert_shell_in and convert_shell_out after curve conversion"
    - "ShellCondition::Closed is preserved through the conversion round-trip"
    - "Snapping corrects both first and last control points of the NURBS curve"
    - "sample_curve_to_nurbs snaps endpoints to exact boundary values after try_interpolate"
  artifacts:
    - path: "monstertruck-solid/src/fillet/convert.rs"
      provides: "Endpoint snapping in both convert_shell_in and convert_shell_out, plus sample_curve_to_nurbs boundary snapping"
      min_lines: 150
      contains: "snap_curve_endpoints"
    - path: "monstertruck-solid/src/fillet/tests.rs"
      provides: "Tests verifying endpoint snapping preserves ShellCondition::Closed"
      min_lines: 3000
      contains: "endpoint_snap"
  key_links:
    - from: "monstertruck-solid/src/fillet/convert.rs"
      to: "monstertruck-topology/src/shell.rs"
      via: "Snapped endpoints maintain shell closure condition"
      pattern: "absolute_front"
    - from: "monstertruck-solid/src/fillet/convert.rs"
      to: "monstertruck-solid/src/fillet/convert.rs"
      via: "snap_curve_endpoints used in convert_shell_in, convert_shell_out, and sample_curve_to_nurbs"
      pattern: "snap_curve_endpoints"
---

<objective>
Add endpoint snapping to ensure converted curve endpoints exactly match vertex positions throughout the fillet conversion pipeline. After the degree-3 interpolation upgrade (plan 22-1) and RevolutedCurve conversion (plan 22-2), curve endpoint positions may drift from vertex positions due to interpolation numerics. Snapping first/last control points in convert_shell_in, convert_shell_out, and sample_curve_to_nurbs prevents gap introduction that breaks shell closure. This covers FCONV-02 for both input and output conversion directions.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/fillet/convert.rs
@monstertruck-topology/src/edge.rs
@monstertruck-topology/src/shell.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Implement snap_curve_endpoints helper and integrate into convert_shell_in and convert_shell_out</name>
  <files>monstertruck-solid/src/fillet/convert.rs</files>
  <action>
Add a helper function and integrate it into both shell conversion directions.

**CRITICAL API NOTES:**
- `Edge::curve()` returns a CLONE (reads from `Arc<RwLock>`). There is NO `curve_mut()` method.
- `Edge::set_curve(&self, curve)` replaces the curve via interior mutability (`Arc<RwLock>`).
- `Shell::edge_iter()` returns an immutable iterator of `Edge` items, but since `set_curve` takes `&self`, this is sufficient for modifying curves.
- There is NO `Shell::edge_iter_mut()`. Use `shell.edge_iter()` which iterates via `face_iter().flat_map(Face::edge_iter)`.
- `NurbsCurve<V>` has `control_point_mut(idx: usize) -> &mut V` for indexed mutable access.

**Step 1: Add the snap helper function:**

```rust
/// Snaps the first and last control points of a clamped NURBS curve to
/// match the given target positions exactly. For clamped knot vectors
/// (produced by `uniform_knot`), the first/last control points directly
/// determine the curve's start/end positions.
fn snap_curve_endpoints(
    curve: &mut NurbsCurve<Vector4>,
    front: Point3,
    back: Point3,
) {
    let n = curve.control_points().len();
    if n == 0 { return; }
    let w0 = curve.control_points()[0][3];
    *curve.control_point_mut(0) = Vector4::new(front.x * w0, front.y * w0, front.z * w0, w0);
    if n > 1 {
        let wn = curve.control_points()[n - 1][3];
        *curve.control_point_mut(n - 1) = Vector4::new(back.x * wn, back.y * wn, back.z * wn, wn);
    }
}
```

Note: Uses `control_point_mut(idx)` for indexed mutable access (available on NurbsCurve<Vector4>). Does NOT use iterators to avoid borrow issues.

**Step 2: Integrate into convert_shell_in:**

In `convert_shell_in`, after the `shell.try_mapped(...)` call that creates `internal_shell`, iterate over each edge in the internal shell and snap the curve endpoints to match the edge's vertex positions. Use `edge_iter()` (not `edge_iter_mut()`) combined with `edge.curve()` + `edge.set_curve()` pattern:

```rust
let internal_shell: InternalShell = shell
    .try_mapped(
        |p| Some(*p),
        |c| Some(Curve::NurbsCurve(c.to_nurbs_curve())),
        |s| s.to_nurbs_surface(),
    )
    .ok_or(FilletError::UnsupportedGeometry {
        context: "failed to convert shell curves or surfaces to NURBS",
    })?;

// Snap curve endpoints to vertex positions after conversion.
// Use face -> boundaries -> edge traversal since Shell has no edge_iter_mut().
// Edge::set_curve takes &self (interior mutability via Arc<RwLock>), so
// immutable edge_iter() is sufficient.
for face in internal_shell.face_iter() {
    for wire in face.boundaries() {
        for edge in wire.edge_iter() {
            let front = edge.absolute_front().point();
            let back = edge.absolute_back().point();
            let mut curve = edge.curve();  // Returns a clone
            if let Curve::NurbsCurve(ref mut nc) = curve {
                snap_curve_endpoints(nc, front, back);
                edge.set_curve(curve.clone());  // Write back via Arc<RwLock>
            }
        }
    }
}
```

Alternatively, `internal_shell.edge_iter()` can be used directly since it returns `Edge` items that support `set_curve(&self, ...)`:

```rust
for edge in internal_shell.edge_iter() {
    let front = edge.absolute_front().point();
    let back = edge.absolute_back().point();
    let mut curve = edge.curve();  // Clone from Arc<RwLock>
    if let Curve::NurbsCurve(ref mut nc) = curve {
        snap_curve_endpoints(nc, front, back);
        edge.set_curve(curve);  // Replace via Arc<RwLock> interior mutability
    }
}
```

Use whichever compiles — `edge_iter()` is simpler if it returns owned `Edge` items (it does: `impl Iterator<Item = Edge<P, C>>`).

**Step 3: Integrate into convert_shell_out:**

In `convert_shell_out`, snap internal curves to vertex positions before converting to external types. Since the shell is borrowed as `&InternalShell`, use the same `edge_iter()` + `set_curve()` pattern (interior mutability through `&self`):

```rust
pub(super) fn convert_shell_out<C: FilletableCurve, S: FilletableSurface>(
    shell: &InternalShell,
) -> std::result::Result<monstertruck_topology::Shell<Point3, C, S>, FilletError> {
    // Snap internal curves to vertex positions before converting.
    // Edge::set_curve takes &self, so we can mutate through immutable shell reference.
    for edge in shell.edge_iter() {
        let front = edge.absolute_front().point();
        let back = edge.absolute_back().point();
        let mut curve = edge.curve();  // Clone
        if let Curve::NurbsCurve(ref mut nc) = curve {
            snap_curve_endpoints(nc, front, back);
            edge.set_curve(curve);  // Write back
        }
    }
    shell
        .try_mapped(
            |p| Some(*p),
            |c| {
                Some(match c {
                    Curve::NurbsCurve(nc) => C::from(nc.clone()),
                    Curve::ParameterCurve(pc) => C::from(pc.clone()),
                    Curve::IntersectionCurve(ic) => C::from(ic.clone()),
                })
            },
            |s| Some(S::from(s.clone())),
        )
        .ok_or(FilletError::UnsupportedGeometry {
            context: "failed to convert internal shell back to external types",
        })
}
```

Note: The `try_mapped` closure receives `&Curve` so it cannot snap inline. Snapping must happen as a pre-pass using `edge_iter()` + `set_curve()`.

**Step 4: Add boundary snapping in sample_curve_to_nurbs as belt-and-suspenders:**

After `try_interpolate` succeeds in `sample_curve_to_nurbs`, snap the first and last control points to the exact sampled boundary values (the evaluated points at t0 and t1):

```rust
let front = evaluate(t0);
let back = evaluate(t1);
match BsplineCurve::try_interpolate(knot, param_points) {
    Ok(bsp) => {
        let mut nc = NurbsCurve::from(bsp);
        snap_curve_endpoints(&mut nc, front, back);
        nc
    }
    Err(_) => { /* degree-1 fallback */ }
}
```

This ensures interpolation at boundaries regardless of numerics.
  </action>
  <verify>
Run `cargo build -p monstertruck-solid` to verify compilation. Run `cargo test -p monstertruck-solid -- fillet` to verify all tests pass. Inspect that `snap_curve_endpoints` is called in convert_shell_in, convert_shell_out, and sample_curve_to_nurbs.
  </verify>
  <done>Both convert_shell_in and convert_shell_out snap curve endpoints to vertex positions using edge.curve() + edge.set_curve() pattern, and sample_curve_to_nurbs snaps to exact boundary values.</done>
</task>

<task type="auto">
  <name>Task 2: Add endpoint snapping tests</name>
  <files>monstertruck-solid/src/fillet/tests.rs</files>
  <action>
Add tests that verify endpoint snapping preserves shell closure through the conversion round-trip.

**Test 1: `endpoint_snap_preserves_closure`**
1. Create a simple closed shell (e.g., a cube or tetrahedron) using the existing test helpers in tests.rs. Reuse the pattern from existing fillet tests like `fillet_semi_cube` (line 339) or `generic_fillet_identity` (line 735) which create closed shells.
2. Verify `shell.shell_condition()` is `ShellCondition::Closed`.
3. Run the shell through `convert_shell_in` followed by `convert_shell_out`.
4. Verify the resulting shell still has `ShellCondition::Closed`.
5. For each edge, verify that `edge.absolute_front().point()` matches the curve's evaluated start point (`edge.curve().subs(edge.range_tuple().0)`) within `1e-14` (machine epsilon scale).

**Test 2: `endpoint_snap_after_interpolation`**
1. Create a NurbsCurve via `sample_curve_to_nurbs` from a known curve (e.g., a quarter circle defined analytically).
2. Evaluate the curve at its start/end parameters.
3. Verify the curve's start/end points match the expected positions within `1e-14` (machine epsilon, not SNAP_TOLERANCE).
4. This validates that the degree-3 interpolation + snapping produces exact endpoints.

**Test 3: `endpoint_snap_intersection_curve_edge_roundtrip`**
1. Create an IntersectionCurve edge using the `build_face_with_intersection_curve_edge` helper function at line 3046 of tests.rs. This function already exists and builds an appropriate test fixture.
2. Build a shell containing this edge.
3. Convert through `convert_shell_in` then `convert_shell_out`.
4. Verify endpoint positions match vertex positions within `1e-14`.

Use `assert!((a - b).magnitude() < 1e-14, "...")` or similar machine-epsilon-level checks for the snapped endpoints.
  </action>
  <verify>
Run `cargo test -p monstertruck-solid -- endpoint_snap` to verify the new tests pass. Run the full fillet test suite to verify no regressions.
  </verify>
  <done>Endpoint snapping tests verify ShellCondition::Closed preservation and machine-epsilon endpoint accuracy for both conversion directions.</done>
</task>

</tasks>

<verification>
1. `cargo build --workspace` compiles without errors
2. `cargo test -p monstertruck-solid -- endpoint_snap` passes all new snapping tests
3. `cargo test -p monstertruck-solid -- fillet` passes all existing fillet tests
4. Grep for `snap_curve_endpoints` in convert.rs confirms usage in convert_shell_in, convert_shell_out, and sample_curve_to_nurbs
5. No ShellCondition::Closed regressions in any test
6. Grep for `curve_mut()` in convert.rs returns NO matches (uses set_curve pattern instead)
7. Grep for `edge_iter_mut()` in convert.rs returns NO matches (uses edge_iter() with set_curve)
</verification>

<success_criteria>
- Converted curve endpoints exactly match their source vertex positions within machine epsilon, preserving ShellCondition::Closed through the conversion round-trip (FCONV-02)
- Endpoint snapping is applied in both convert_shell_in and convert_shell_out directions (FCONV-02)
- sample_curve_to_nurbs snaps boundary control points after interpolation (FCONV-02)
- New tests explicitly verify the snapping behavior using existing test fixtures
- All code uses edge.curve() + edge.set_curve() pattern, NOT edge.curve_mut()
- All edge iteration uses shell.edge_iter() or face/wire/edge traversal, NOT edge_iter_mut()
</success_criteria>

<output>
After completion, create `.tendrion/phases/22-conversion-fidelity/22-3-SUMMARY.md`
</output>
