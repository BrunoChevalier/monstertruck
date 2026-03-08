---
phase: 1-core-stabilization
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-modeling/src/geometry.rs
autonomous: true
must_haves:
  truths:
    - "Boolean operations producing IntersectionCurve variants no longer panic at runtime"
    - "lift_up() on an IntersectionCurve approximates via the leader curve instead of panicking"
    - "IncludeCurve for all surface types handles IntersectionCurve via knot-span sampling with iterative hints"
    - "ExtrudedCurve::to_same_geometry handles IntersectionCurve via homotopy of lifted curves"
    - "cargo test -p monstertruck-modeling passes with no regressions"
    - "Downstream monstertruck-solid tests exercising boolean paths pass"
  artifacts:
    - path: "monstertruck-modeling/src/geometry.rs"
      provides: "All 9 IntersectionCurve unimplemented!() arms replaced with working implementations"
      min_lines: 350
      contains: "IntersectionCurve"
  key_links:
    - from: "monstertruck-modeling/src/geometry.rs"
      to: "monstertruck-geometry/src/decorators/intersection_curve.rs"
      via: "IntersectionCurve::leader() used for approximation"
      pattern: "leader()"
---

<objective>
Replace all 9 `unimplemented!()` arms for `Curve::IntersectionCurve` in `monstertruck-modeling/src/geometry.rs` with working implementations so the boolean-to-modeling pipeline no longer panics when intersection curves feed back into modeling operations.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@monstertruck-modeling/src/geometry.rs
@monstertruck-geometry/src/decorators/intersection_curve.rs
@monstertruck-geometry/src/nurbs/bspline_surface.rs (IncludeCurve impl for reference)
@monstertruck-geometry/src/decorators/revolved_curve.rs (sub_include for reference)
@AGENTS.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Implement lift_up and IncludeCurve for IntersectionCurve</name>
  <files>monstertruck-modeling/src/geometry.rs</files>
  <action>
Replace the 8 `unimplemented!()` arms in `geometry.rs` for `Curve::IntersectionCurve` with implementations.

**`lift_up()` (line 106-108):**
The `IntersectionCurve` wraps a leader curve that is itself a `Curve`. Recursively call `lift_up()` on the leader:
```rust
Curve::IntersectionCurve(ic) => ic.leader().lift_up(),
```

**`IncludeCurve` for `BsplineSurface` (line 208), `NurbsSurface` (line 214), `Plane` (line 220):**

IMPORTANT: Do NOT use a control-point-only membership check. The existing `IncludeCurve` implementations in monstertruck-geometry use knot-span sampling with iterative parameter hints. For `IntersectionCurve`, use `lift_up()` to get the B-spline approximation, then delegate to the surface's existing `IncludeCurve<BsplineCurve<Point3>>` or `IncludeCurve<NurbsCurve<Vector4>>` impl which uses the proper knot-span sampling strategy internally.

For `BsplineSurface`:
```rust
Curve::IntersectionCurve(_) => {
    let lifted = curve.lift_up();
    let nurbs = NurbsCurve::new(lifted);
    surface.include(&nurbs)
}
```

For `NurbsSurface`:
```rust
Curve::IntersectionCurve(_) => {
    let lifted = curve.lift_up();
    let nurbs = NurbsCurve::new(lifted);
    surface.include(&nurbs)
}
```

For `Plane` (line 220):
The `Plane` arm on line 220 is inside `Surface::Plane(surface) => match curve { ... }`. The `IncludeCurve<Curve> for Plane` impl (line 273) already handles all curves via `lift_up()` + `search_parameter`. Use a similar approach:
```rust
Curve::IntersectionCurve(_) => {
    let lifted = curve.lift_up();
    let nurbs = NurbsCurve::new(lifted);
    surface.include(&nurbs)
}
```
Note: Check that `Plane` implements `IncludeCurve<NurbsCurve<Vector4>>`. If it does, this delegates correctly. If not, fall back to the point-sampling approach with knot-span sampling (sample per knot span with `search_parameter` using iterative hints), mirroring the `BsplineSurface<Point3>::include(&BsplineCurve<Point3>)` pattern:
```rust
Curve::IntersectionCurve(_) => {
    let lifted = curve.lift_up();
    let (knots, _) = lifted.knot_vec().to_single_multi();
    let degree = lifted.degree() * 6;
    let pt = lifted.subs(knots[0]).to_point();
    let mut hint = match surface.search_parameter(pt, None, 1) {
        Some(h) => h,
        None => return false,
    };
    for i in 1..knots.len() {
        for j in 1..=degree {
            let p = j as f64 / degree as f64;
            let t = knots[i - 1] * (1.0 - p) + knots[i] * p;
            let pt = ParametricCurve::subs(&lifted, t).to_point();
            hint = match surface.search_parameter(pt, Some(hint), 1) {
                Some(h) => h,
                None => return false,
            };
        }
    }
    true
}
```

**`IncludeCurve` for `RevolutedCurve` inner matches (lines 238, 251, 264):**
These arms are inside entity-curve-specific blocks where `surface` is a concrete `RevolutedCurve<BsplineCurve<Point3>>` (or `NurbsCurve` variant). These concrete types implement `IncludeCurve<BsplineCurve<Point3>>` via the `sub_include` function which uses proper knot-span sampling. Convert the intersection curve to a B-spline and delegate:

```rust
Curve::IntersectionCurve(_) => {
    let lifted = curve.lift_up();
    let bsp = BsplineCurve::new(
        lifted.knot_vec().clone(),
        lifted.control_points().iter().map(|v| v.to_point()).collect(),
    );
    surface.include(&bsp)
}
```
Here `surface` is the locally-constructed `RevolutedCurve<&BsplineCurve<Point3>>` (or `NurbsCurve` variant).

**`IncludeCurve` for `RevolutedCurve` outer match (line 267):**
This arm handles when the entity curve of the revolved surface is itself an `IntersectionCurve`. Use `surface` (the `Processor<RevolutedCurve<Curve>, Matrix4>`) which implements `SearchParameter`. Use a knot-span sampling approach directly on the processor:
```rust
Curve::IntersectionCurve(_) => {
    let lifted = curve.lift_up();
    let (knots, _) = lifted.knot_vec().to_single_multi();
    let degree = lifted.degree() * 6;
    let pt = lifted.subs(knots[0]).to_point();
    let mut hint = match surface.search_parameter(pt, None, 1) {
        Some(h) => h,
        None => return false,
    };
    for i in 1..knots.len() {
        for j in 1..=degree {
            let p = j as f64 / degree as f64;
            let t = knots[i - 1] * (1.0 - p) + knots[i] * p;
            let pt = ParametricCurve::subs(&lifted, t).to_point();
            hint = match surface.search_parameter(pt, Some(hint), 1) {
                Some(h) => h,
                None => return false,
            };
        }
    }
    true
}
```

For all implementations, ensure you verify that the types actually implement the required traits. If a delegation approach doesn't compile because the surface type doesn't implement `IncludeCurve<NurbsCurve<Vector4>>`, fall back to the explicit knot-span sampling pattern shown above.
  </action>
  <verify>Run `cargo test -p monstertruck-modeling --lib` and confirm all tests pass. Verify with `grep -c "unimplemented" monstertruck-modeling/src/geometry.rs` that the count is reduced to 0 (for IntersectionCurve) or only non-IntersectionCurve unimplemented arms remain.</verify>
  <done>All 8 IntersectionCurve unimplemented!() arms in lift_up() and IncludeCurve replaced with knot-span-sampling-based implementations.</done>
</task>

<task type="auto">
  <name>Task 2: Implement ExtrudedCurve::to_same_geometry for IntersectionCurve</name>
  <files>monstertruck-modeling/src/geometry.rs</files>
  <action>
Replace the `unimplemented!()` at line 357 in `ToSameGeometry<Surface> for ExtrudedCurve<Curve, Vector3>`.

IMPORTANT: In this match block, `curve0` is `self.entity_curve()` (a borrow of type `&Curve`), and `curve1` is `self.entity_curve().transformed(trsl)` (an owned `Curve`). The match is `match (curve0, curve1)`. For the `IntersectionCurve` arm, the pattern `(Curve::IntersectionCurve(_), Curve::IntersectionCurve(_))` binds `curve0` by reference and `curve1` by value/move.

Since `curve0` is borrowed from `self`, we can call `lift_up()` on it. For `curve1` which is the transformed copy, we also call `lift_up()`:

```rust
(Curve::IntersectionCurve(_), Curve::IntersectionCurve(_)) => {
    // Approximate both intersection curves via lift_up() to get
    // non-rationalized 4D B-spline curves, then build a homotopy surface.
    let c0 = curve0.lift_up();
    let c1 = curve1.lift_up();
    NurbsSurface::new(BsplineSurface::homotopy(c0, c1)).into()
}
```

This works because `lift_up()` takes `&self` and returns an owned `BsplineCurve<Vector4>`. Both `curve0` (a `&Curve` from `self.entity_curve()`) and `curve1` (an owned `Curve` from `transformed()`) can have `lift_up()` called on them. The pattern is consistent with the `NurbsCurve` arm (lines 350-356) which also produces a `NurbsSurface` via homotopy.

Note: `curve0` is `&Curve` (borrowed), so `curve0.lift_up()` works via auto-deref. `curve1` is `Curve` (owned), so `curve1.lift_up()` also works. No ownership issues.
  </action>
  <verify>Run `cargo test -p monstertruck-modeling --lib`. Confirm zero `unimplemented!()` calls remain for IntersectionCurve: `grep 'IntersectionCurve.*unimplemented\|unimplemented.*IntersectionCurve' monstertruck-modeling/src/geometry.rs` should return no matches. Run `cargo clippy --all-targets -- -W warnings` for the modeling crate to ensure no new warnings.</verify>
  <done>ExtrudedCurve::to_same_geometry handles IntersectionCurve via homotopy of lifted curves. Zero IntersectionCurve unimplemented!() arms remain in geometry.rs.</done>
</task>

<task type="auto">
  <name>Task 3: Verify downstream monstertruck-solid boolean paths</name>
  <files>monstertruck-modeling/src/geometry.rs</files>
  <action>
Run downstream tests in monstertruck-solid that exercise boolean operations (which produce IntersectionCurve edges that feed back into modeling operations).

1. Run `cargo test -p monstertruck-solid --lib` to verify all solid tests pass, including the boolean_shell_converts_for_fillet test that exercises IntersectionCurve -> NURBS conversion.

2. Run `cargo clippy --all-targets -- -W warnings` (workspace-wide per AGENTS.md policy) to ensure no warnings.

3. If any downstream test fails due to the new IntersectionCurve implementations, diagnose and fix the issue in geometry.rs. Common issues:
   - Type mismatches when delegating to `IncludeCurve` impls (may need to convert between BsplineCurve and NurbsCurve)
   - Missing trait implementations (e.g., `Plane` may not implement `IncludeCurve<NurbsCurve<Vector4>>`)
   - Numerical precision issues in knot-span sampling (try increasing trial count parameter)

4. Fix any clippy warnings introduced by the new code.
  </action>
  <verify>Run `cargo test -p monstertruck-solid --lib` and `cargo clippy --all-targets -- -W warnings`. Both must pass cleanly.</verify>
  <done>Downstream monstertruck-solid tests pass. No clippy warnings across workspace. IntersectionCurve implementations are verified end-to-end through boolean operation test paths.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-modeling --lib` passes with no failures
2. `grep -c 'unimplemented!' monstertruck-modeling/src/geometry.rs` returns 0 (no unimplemented arms remain)
3. `cargo test -p monstertruck-solid --lib` passes (downstream boolean paths work)
4. `cargo clippy --all-targets -- -W warnings` produces no warnings (per AGENTS.md policy)
5. The modeling pipeline can handle IntersectionCurve variants in lift_up(), IncludeCurve, and ExtrudedCurve without panicking
</verification>

<success_criteria>
- All 9 IntersectionCurve unimplemented!() arms in geometry.rs are replaced with working implementations
- IncludeCurve uses knot-span sampling with iterative hints, consistent with existing inclusion semantics
- ExtrudedCurve correctly uses lift_up() on both borrowed and owned Curve values
- No runtime panics when intersection curves feed back into modeling operations
- Downstream monstertruck-solid tests pass
- All existing tests continue to pass with no clippy warnings
</success_criteria>

<output>
After completion, create `.tendrion/phases/1-core-stabilization/1-1-SUMMARY.md`
</output>
