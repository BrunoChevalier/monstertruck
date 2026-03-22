---
phase: 22-conversion-fidelity
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/convert.rs
  - monstertruck-modeling/src/fillet_impl.rs
autonomous: true
must_haves:
  truths:
    - "sample_curve_to_nurbs produces a degree-3 BsplineCurve via try_interpolate instead of a degree-1 polyline"
    - "sample_surface_to_nurbs produces a degree-3 tensor product surface via two-pass row/column interpolation instead of a degree-1 grid"
    - "sample_to_nurbs in fillet_impl.rs produces degree-3 curves for ParameterCurveLinear and FilletIntersectionCurve conversions"
    - "All sampling paths use uniform_knot(3, n_points - 3) following the fair.rs pattern"
    - "Existing fillet tests continue to pass with the upgraded interpolation"
  artifacts:
    - path: "monstertruck-solid/src/fillet/convert.rs"
      provides: "Degree-3 cubic interpolation for sample_curve_to_nurbs and sample_surface_to_nurbs"
      min_lines: 80
      contains: "try_interpolate"
    - path: "monstertruck-modeling/src/fillet_impl.rs"
      provides: "Degree-3 cubic interpolation for sample_to_nurbs output path"
      min_lines: 50
      contains: "try_interpolate"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/bspline_curve.rs"
      to: "monstertruck-solid/src/fillet/convert.rs"
      via: "BsplineCurve::try_interpolate for degree-3 curve fitting"
      pattern: "BsplineCurve::try_interpolate"
    - from: "monstertruck-geometry/src/nurbs/bspline_curve.rs"
      to: "monstertruck-modeling/src/fillet_impl.rs"
      via: "BsplineCurve::try_interpolate for degree-3 output curves"
      pattern: "BsplineCurve::try_interpolate"
---

<objective>
Replace all degree-1 piecewise-linear sampling with degree-3 cubic interpolation across the fillet conversion pipeline: `sample_curve_to_nurbs` and `sample_surface_to_nurbs` in convert.rs, and `sample_to_nurbs` in fillet_impl.rs. This produces C1-continuous NURBS output that preserves geometric fidelity.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/fillet/convert.rs
@monstertruck-modeling/src/fillet_impl.rs
@monstertruck-geometry/src/nurbs/bspline_curve.rs
@monstertruck-geometry/src/nurbs/fair.rs
@monstertruck-geometry/src/t_spline/t_mesh.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Upgrade sample_curve_to_nurbs to degree-3 cubic interpolation</name>
  <files>monstertruck-solid/src/fillet/convert.rs</files>
  <action>
Replace the current `sample_curve_to_nurbs` function which creates a degree-1 polyline with `KnotVector::uniform_knot(1, sample_count)`.

**Critical: Follow the fair.rs knot vector sizing pattern.**

The relationship is: `KnotVector::uniform_knot(degree, division)` produces a knot vector of length `degree + 1 + division - 1 + degree + 1 = 2*(degree+1) + division - 1`. For `try_interpolate`, `n_points = knot_vec.len() - degree - 1`. So:
- `uniform_knot(3, division)` has length `3+1 + (division-1) + 3+1 = division + 7`
- Number of interpolation points = `(division + 7) - 3 - 1 = division + 3`

Therefore to fit `n_points` sample points: `division = n_points - 3`, i.e. `uniform_knot(3, n_points - 3)`.

This matches `fair.rs` line 89: `KnotVector::uniform_knot(degree, n_samples - degree)`.

New implementation:
1. Choose `n_points = sample_count + 1` (keeping the same total sample count as before, e.g., 24 spans → 25 points).
2. Create a degree-3 knot vector: `KnotVector::uniform_knot(3, n_points - 3)`.
3. Compute sample parameters uniformly in [0, 1]: `u_i = i / (n_points - 1)` for i in 0..n_points.
4. Map each u_i back to the curve's parameter range: `t_i = t0 + (t1 - t0) * u_i`.
5. Evaluate the curve at each `t_i` to get points.
6. Build parameter-point pairs: `Vec<(f64, Point3)>` using `u_i` as parameters (in [0, 1] since the knot vector is in [0, 1]).
7. Call `BsplineCurve::try_interpolate(knot_vec, parameter_points)`.
8. Wrap result in `NurbsCurve::from(...)`.
9. If `try_interpolate` fails, fall back to the current degree-1 approach as a safety net.

Reference code pattern from `fair.rs` line 82-90:
```rust
let n_points = sample_count + 1;
let knot = KnotVector::uniform_knot(3, n_points - 3);
let param_points: Vec<(f64, Point3)> = (0..n_points)
    .map(|i| {
        let u = i as f64 / (n_points - 1) as f64;
        let t = t0 + (t1 - t0) * u;
        (u, evaluate(t))
    })
    .collect();
match BsplineCurve::try_interpolate(knot, param_points) {
    Ok(bsp) => NurbsCurve::from(bsp),
    Err(_) => {
        // Degree-1 fallback (existing code)
        let points: Vec<Point3> = (0..=sample_count)
            .map(|i| t0 + (t1 - t0) * (i as f64) / (sample_count as f64))
            .map(&evaluate)
            .collect();
        let knot_vector = KnotVector::uniform_knot(1, sample_count);
        NurbsCurve::from(BsplineCurve::new(knot_vector, points))
    }
}
```

Ensure `n_points >= 4` (i.e., `sample_count >= 3`) for degree-3 to work. The CURVE_SAMPLE_COUNT constant is typically 24, so this is always satisfied.
  </action>
  <verify>
Run `cargo build -p monstertruck-solid` to confirm compilation. Run existing fillet tests: `cargo test -p monstertruck-solid -- fillet` to verify no regressions. Verify the function signature is unchanged (still takes `range`, `evaluate`, `sample_count`).
  </verify>
  <done>sample_curve_to_nurbs produces degree-3 cubic B-spline curves via try_interpolate instead of degree-1 polylines.</done>
</task>

<task type="auto">
  <name>Task 2: Upgrade sample_surface_to_nurbs to degree-3 tensor product interpolation</name>
  <files>monstertruck-solid/src/fillet/convert.rs</files>
  <action>
Replace the current `sample_surface_to_nurbs` which creates a degree-1 bilinear grid with `KnotVector::uniform_knot(1, sample_count)` in both u and v.

**Follow the t_mesh.rs two-pass interpolation pattern exactly (lines 2619-2674).**

New implementation:
1. Choose `n_points = sample_count + 1` in each direction.
2. Create degree-3 knot vectors: `let u_knot = KnotVector::uniform_knot(3, n_points - 3);` and same for v_knot.
3. Compute Greville abscissae for sampling parameters. Add a local helper (or inline):
```rust
fn greville_abscissae(knots: &KnotVector, degree: usize) -> Vec<f64> {
    let n = knots.len() - degree - 1;
    (0..n)
        .map(|i| (1..=degree).map(|j| knots[i + j]).sum::<f64>() / degree as f64)
        .collect()
}
```
The Greville abscissae are in [0, 1] (matching the knot vector range). They provide optimal interpolation parameters. The number of Greville points = `n_points` (matching the number of interpolation points).

4. Map Greville abscissae to the surface's (u, v) parameter ranges:
```rust
let u_grev = greville_abscissae(&u_knot, 3);
let v_grev = greville_abscissae(&v_knot, 3);
// Map to surface domain
let u_params: Vec<f64> = u_grev.iter().map(|&g| u0 + (u1 - u0) * g).collect();
let v_params: Vec<f64> = v_grev.iter().map(|&g| v0 + (v1 - v0) * g).collect();
```

5. Sample the surface at the tensor product grid: `surface.evaluate(u_params[i], v_params[j])`.

6. **First pass -- interpolate each row (v-direction):**
For each u-index `i`, collect points `[(v_grev[0], pt[i][0]), ..., (v_grev[n-1], pt[i][n-1])]` and call `BsplineCurve::try_interpolate(v_knot.clone(), row_params)`. Extract control points from the resulting curve. This matches t_mesh.rs lines 2639-2647.

7. **Second pass -- interpolate each column (u-direction):**
For each v control-point index `j`, collect the j-th control point from each row curve: `[(u_grev[0], intermediate[0][j]), ..., (u_grev[n-1], intermediate[n-1][j])]` and call `BsplineCurve::try_interpolate(u_knot.clone(), col_params)`. Extract control points. This matches t_mesh.rs lines 2657-2668.

8. **Transpose the result** from [V][U] to [U][V] ordering as required by BsplineSurface (t_mesh.rs lines 2671-2673):
```rust
let control_points: Vec<Vec<Point3>> = (0..n_points)
    .map(|i| (0..n_points).map(|j| col_cps[j][i]).collect())
    .collect();
```

9. Construct `BsplineSurface::new((u_knot, v_knot), control_points)` and wrap in `NurbsSurface::from(...)`.

10. If any interpolation step fails, fall back to the degree-1 approach (existing code).

The function still returns `Option<NurbsSurface<Vector4>>`. The `None` case from `try_range_tuple` is unchanged.
  </action>
  <verify>
Run `cargo build -p monstertruck-solid` and `cargo test -p monstertruck-solid -- fillet` to verify compilation and no regressions. Inspect that the function still returns `Option<NurbsSurface<Vector4>>`.
  </verify>
  <done>sample_surface_to_nurbs produces degree-3 tensor product B-spline surfaces via two-pass row/column interpolation instead of degree-1 bilinear grids.</done>
</task>

<task type="auto">
  <name>Task 3: Upgrade fillet_impl.rs sample_to_nurbs to degree-3</name>
  <files>monstertruck-modeling/src/fillet_impl.rs</files>
  <action>
Replace the `sample_to_nurbs` function in fillet_impl.rs which currently creates a degree-1 polyline with manual knot construction.

**Follow the same fair.rs pattern as Task 1.**

New implementation:
1. Choose `n_points = n + 1` where `n` is the span count parameter (currently 16, increase to 24).
2. Create knot vector: `KnotVector::uniform_knot(3, n_points - 3)`.
3. Sample the curve at `n_points` uniform parameters mapped from [0, 1] to [t0, t1].
4. Build parameter-point pairs with parameters in [0, 1].
5. Call `BsplineCurve::try_interpolate(knot_vec, parameter_points)`.
6. Wrap in `NurbsCurve::from(...)`.
7. Fall back to degree-1 (existing manual knot construction) if interpolation fails.

```rust
fn sample_to_nurbs(
    range: (f64, f64),
    subs: impl Fn(f64) -> Point3,
    n: usize,
) -> NurbsCurve<Vector4> {
    let (t0, t1) = range;
    let n_points = n + 1;
    let knot = KnotVector::uniform_knot(3, n_points - 3);
    let param_points: Vec<(f64, Point3)> = (0..n_points)
        .map(|i| {
            let u = i as f64 / (n_points - 1) as f64;
            let t = t0 + (t1 - t0) * u;
            (u, subs(t))
        })
        .collect();
    match BsplineCurve::try_interpolate(knot, param_points) {
        Ok(bsp) => NurbsCurve::from(bsp),
        Err(_) => {
            // Degree-1 fallback
            let pts: Vec<Point3> = (0..=n)
                .map(|i| subs(t0 + (t1 - t0) * (i as f64) / (n as f64)))
                .collect();
            let knots: Vec<f64> = (0..=n).map(|i| i as f64 / n as f64).collect();
            let knot_vec = KnotVector::from(
                std::iter::once(0.0)
                    .chain(knots.iter().copied())
                    .chain(std::iter::once(1.0))
                    .collect::<Vec<_>>(),
            );
            NurbsCurve::from(BsplineCurve::new(knot_vec, pts))
        }
    }
}
```

Update the sample count from 16 to 24 in both call sites:
- `From<ParameterCurveLinear> for Curve` (currently calls `sample_to_nurbs(range, |t| c.subs(t), 16)`)
- `From<FilletIntersectionCurve> for Curve` (currently calls `sample_to_nurbs(range, |t| c.subs(t), 16)`)

Also check the `TryFrom<Curve> for NurbsCurve<Vector4>` impl if it calls `sample_to_nurbs` and update that call site too.

Update the doc comment from "degree-1 NURBS polyline approximation" to "degree-3 NURBS cubic interpolation".
  </action>
  <verify>
Run `cargo build -p monstertruck-modeling` and `cargo test -p monstertruck-modeling` to verify compilation and no regressions. Verify that `From<ParameterCurveLinear>` and `From<FilletIntersectionCurve>` still produce `Curve::NurbsCurve(...)`.
  </verify>
  <done>All fillet output paths in fillet_impl.rs produce degree-3 cubic NURBS curves with the correct knot vector sizing.</done>
</task>

</tasks>

<verification>
1. `cargo build --workspace` compiles without errors
2. `cargo test -p monstertruck-solid -- fillet` passes all existing fillet tests
3. `cargo test -p monstertruck-modeling` passes all existing modeling tests
4. Grep for `uniform_knot(1,` in convert.rs and fillet_impl.rs returns no matches (all upgraded to degree 3)
5. Grep for `try_interpolate` in convert.rs and fillet_impl.rs confirms usage in all sampling functions
6. Grep for `n_points - 3` in convert.rs and fillet_impl.rs confirms correct knot vector sizing
</verification>

<success_criteria>
- sample_curve_to_nurbs and sample_surface_to_nurbs produce degree-3 NURBS with C1-continuous output instead of degree-1 piecewise-linear approximations (FCONV-01)
- Knot vector sizing follows the fair.rs pattern: uniform_knot(3, n_points - 3) where n_points is the number of sample points (FCONV-01)
- Surface interpolation uses two-pass row/column approach following the t_mesh.rs pattern (FCONV-01)
- All existing tests pass without regression
</success_criteria>

<output>
After completion, create `.tendrion/phases/22-conversion-fidelity/22-1-SUMMARY.md`
</output>
