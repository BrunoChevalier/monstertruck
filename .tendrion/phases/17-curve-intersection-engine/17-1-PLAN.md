---
phase: 17-curve-intersection-engine
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/curve_intersect.rs
  - monstertruck-geometry/src/nurbs/mod.rs
  - monstertruck-geometry/tests/curve_intersect.rs
autonomous: true
must_haves:
  truths:
    - "User calls curve_intersect::find_intersections with two BsplineCurve<Point3> values and receives a Vec of CurveIntersection results containing parameter pairs"
    - "User calls find_intersections on two crossing cubic curves and gets intersection parameters accurate within SNAP_TOLERANCE"
    - "User calls find_intersections on two non-intersecting curves and receives an empty Vec"
    - "User calls find_intersections on curves that touch tangentially and receives the tangent intersection point"
    - "The module is accessible as monstertruck_geometry::nurbs::curve_intersect from downstream crates"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/curve_intersect.rs"
      provides: "Curve-curve intersection algorithm using subdivision and Newton refinement"
      min_lines: 150
      contains: "find_intersections"
    - path: "monstertruck-geometry/tests/curve_intersect.rs"
      provides: "Integration tests for curve intersection including crossing, non-intersecting, and tangent cases"
      min_lines: 80
      contains: "find_intersections"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/curve_intersect.rs"
      to: "monstertruck-geometry/src/nurbs/mod.rs"
      via: "module declaration"
      pattern: "pub mod curve_intersect"
    - from: "monstertruck-geometry/src/nurbs/curve_intersect.rs"
      to: "monstertruck-core/src/tolerance_constants.rs"
      via: "tolerance import"
      pattern: "SNAP_TOLERANCE"
---

<objective>
Create the curve-curve intersection module in monstertruck-geometry with the core algorithm: bounding-box subdivision to isolate candidate regions, followed by Newton-Raphson refinement to converge on precise intersection parameters. The module handles standard cases (transversal crossings, no intersection, tangent touch) and exports a public API usable by downstream Gordon grid and trim code.
</objective>

<execution_context>
@skills/state-management/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/mod.rs
@monstertruck-geometry/src/nurbs/bspline_curve.rs
@monstertruck-core/src/tolerance_constants.rs
@monstertruck-core/src/bounding_box.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create curve_intersect module with types and subdivision algorithm</name>
  <files>monstertruck-geometry/src/nurbs/curve_intersect.rs, monstertruck-geometry/src/nurbs/mod.rs</files>
  <action>
Create `monstertruck-geometry/src/nurbs/curve_intersect.rs` with the following structure:

1. **Module doc comment** explaining the subdivision/refinement approach.

2. **Imports**: Use existing crate types:
   ```rust
   use super::*;
   use monstertruck_core::bounding_box::BoundingBox;
   use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
   ```

3. **Result type** `CurveIntersection`:
   ```rust
   /// A single intersection result between two curves.
   #[derive(Clone, Debug, PartialEq)]
   pub struct CurveIntersection {
       /// Parameter on the first curve.
       pub t0: f64,
       /// Parameter on the second curve.
       pub t1: f64,
       /// The intersection point in space.
       pub point: Point3,
   }
   ```

4. **Public entry point** `find_intersections`:
   ```rust
   /// Finds all intersection points between two B-spline curves.
   ///
   /// Uses a subdivision approach: recursively splits curves via bounding-box
   /// overlap tests, then refines candidate pairs with Newton-Raphson iteration.
   /// Returns intersection parameters accurate within [`SNAP_TOLERANCE`].
   ///
   /// # Arguments
   /// * `curve0` - First B-spline curve.
   /// * `curve1` - Second B-spline curve.
   ///
   /// # Returns
   /// A vector of [`CurveIntersection`] values, one per intersection point,
   /// sorted by `t0`.
   pub fn find_intersections<P>(
       curve0: &BsplineCurve<P>,
       curve1: &BsplineCurve<P>,
   ) -> Vec<CurveIntersection>
   where
       P: ControlPoint<f64>
           + EuclideanSpace<Scalar = f64, Diff = <P as ControlPoint<f64>>::Diff>
           + MetricSpace<Metric = f64>
           + Tolerance
           + Bounded<Scalar = f64>,
       <P as ControlPoint<f64>>::Diff: InnerSpace<Scalar = f64> + Tolerance,
   ```
   This function:
   - Calls `subdivide_and_collect` with initial parameter ranges from `range_tuple()`.
   - Deduplicates results (merge points within SNAP_TOLERANCE of each other).
   - Sorts by `t0`.

5. **Subdivision function** `subdivide_and_collect` (private):
   - Takes two curves, their parameter sub-ranges `(t0_start, t0_end)` and `(t1_start, t1_end)`, recursion depth limit (max ~50).
   - Computes bounding boxes via `roughly_bounding_box()` on the sub-curves (use `cut` to extract sub-arcs).
   - If bounding boxes do not overlap (intersection via `^` is empty), return empty.
   - If both sub-ranges are smaller than SNAP_TOLERANCE, call `newton_refine` with midpoints as initial guess.
   - Otherwise, split the curve with the larger parameter range at its midpoint, recurse on each half paired with the other curve.

   **IMPORTANT**: To avoid excessive cloning and cutting, use a helper that evaluates the control-point bounding box over a parameter sub-range. The approach:
   - Cut the curve at the sub-range boundaries to get the sub-arc, then use `roughly_bounding_box()`.
   - Cache or pass sub-curves to avoid redundant cutting.

6. **Newton refinement** `newton_refine` (private):
   - Given initial parameter guesses `(t0, t1)`, iteratively solve:
     ```
     C0(t0) - C1(t1) = 0
     ```
     Using the Jacobian `[C0'(t0), -C1'(t1)]` (2D or 3D system, solved via least-squares for 3D curves -> 2 unknowns).
   - Converge when `|C0(t0) - C1(t1)| < SNAP_TOLERANCE` or max iterations (20) reached.
   - Clamp parameters to their valid ranges.
   - Return `Some(CurveIntersection)` if converged, `None` otherwise.

   For the Newton step with 3D curves and 2 unknowns, use the pseudo-inverse:
   ```
   J = [C0'(t0) | -C1'(t1)]  (3x2 matrix)
   delta = (J^T J)^{-1} J^T * (C1(t1) - C0(t0))
   t0 += delta[0]
   t1 += delta[1]
   ```

7. **Deduplication helper** `deduplicate_intersections` (private):
   - Merge intersections where both `|t0_a - t0_b| < SNAP_TOLERANCE` and `|t1_a - t1_b| < SNAP_TOLERANCE`.

Add `pub mod curve_intersect;` to `monstertruck-geometry/src/nurbs/mod.rs` after the existing `pub mod surface_diagnostics;` line.

The generic bounds should match BsplineCurve's existing patterns (see `ParameterDivision1D` impl and `roughly_bounding_box` method). For the public API, also provide a convenience wrapper for `NurbsCurve<V>` that delegates to the underlying `BsplineCurve` after extracting it via `non_rationalized()`, with appropriate coordinate conversion.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-geometry --lib` to verify the module compiles without errors. Run `cargo clippy -p monstertruck-geometry -- -W warnings` to verify no warnings.
  </verify>
  <done>The curve_intersect module exists with CurveIntersection type, find_intersections entry point, subdivision logic, and Newton refinement. It compiles and passes clippy.</done>
</task>

<task type="auto">
  <name>Task 2: Write integration tests for standard intersection cases</name>
  <files>monstertruck-geometry/tests/curve_intersect.rs</files>
  <action>
Create `monstertruck-geometry/tests/curve_intersect.rs` with integration tests:

```rust
use monstertruck_geometry::prelude::*;
use monstertruck_geometry::nurbs::curve_intersect::{find_intersections, CurveIntersection};
use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
```

1. **Test: two_crossing_lines** - Two linear B-spline curves (degree 1) that cross at a known point. Verify the returned intersection has parameters matching the expected values within SNAP_TOLERANCE.

2. **Test: two_crossing_cubics** - Two cubic B-spline curves that cross at a known parameter. Create curve0 as a cubic going from (0,0,0) to (2,2,0) and curve1 going from (2,0,0) to (0,2,0). They should intersect near (1,1,0). Verify intersection point accuracy.

3. **Test: non_intersecting_curves** - Two curves that are spatially separated (different z-planes or widely separated). Verify empty result.

4. **Test: tangent_intersection** - Two curves that touch tangentially at exactly one point. For example, a parabola-like curve and its tangent line at the vertex. Verify a single intersection is returned.

5. **Test: multiple_intersections** - Two curves (e.g., a wavy cubic and a line) that cross at 2-3 points. Verify the correct count and that results are sorted by t0.

6. **Test: identical_endpoint** - Two curves sharing a common endpoint. Verify the endpoint intersection is found.

For constructing test curves, use `BsplineCurve::new(KnotVector::bezier_knot(degree), control_points)` for Bezier segments, or use `KnotVector::from(vec![...])` for multi-span curves.

Each test should assert:
- Correct number of intersections.
- Parameter values within SNAP_TOLERANCE of expected.
- Point coordinates within SNAP_TOLERANCE of expected.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-geometry -E 'test(curve_intersect)' --no-fail-fast` and verify all tests pass.
  </verify>
  <done>Integration tests for crossing lines, crossing cubics, non-intersecting, tangent, multiple intersections, and shared endpoint cases all pass.</done>
</task>

<task type="auto">
  <name>Task 3: Handle degenerate cases and add self-intersection support</name>
  <files>monstertruck-geometry/src/nurbs/curve_intersect.rs, monstertruck-geometry/tests/curve_intersect.rs</files>
  <action>
Extend the curve_intersect module with degenerate case handling:

1. **Parallel/overlapping curves**: When two curve segments are nearly parallel and overlapping, the subdivision may produce many false candidates. Add early termination logic:
   - If after subdivision, Newton refinement fails to converge for a candidate pair, discard it (do not panic).
   - If both curves evaluate to nearly the same point at their midpoints but tangent vectors are nearly parallel (dot product of normalized tangents > 0.999), treat as overlapping segment and skip (return no intersection for that sub-region, since overlap is not a point intersection).

2. **Self-intersection detection**: Add a public function:
   ```rust
   /// Finds self-intersection points of a single B-spline curve.
   ///
   /// Subdivides the curve into non-overlapping sub-arcs and tests each
   /// pair for intersections, excluding adjacent/identical segments.
   pub fn find_self_intersections<P>(
       curve: &BsplineCurve<P>,
   ) -> Vec<CurveIntersection>
   ```
   Implementation:
   - Split the curve's parameter range into N sub-arcs (N = number of non-trivial knot spans, or a minimum of 4).
   - Test all non-adjacent pairs (skip pairs where sub-arcs share an endpoint).
   - Use the existing `find_intersections` on each pair of sub-arcs.
   - Deduplicate results.

3. **Guard against panics**: Ensure all division operations check for zero denominators. The Newton Jacobian `J^T J` inversion must handle singular/near-singular cases (determinant < TOLERANCE^2) by returning `None` instead of panicking.

4. **Add tests** for degenerate cases in the test file:
   - `test_parallel_curves`: Two curves with the same shape but offset. Should return empty.
   - `test_self_intersection_figure_eight`: A curve that crosses itself (e.g., a lemniscate-like shape via control points). Verify one self-intersection is found.
   - `test_self_intersection_simple_curve`: A simple curve with no self-intersection. Verify empty result.
   - `test_near_tangent_no_panic`: Two curves that are nearly tangent but slightly separated. Should return empty without panicking.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-geometry -E 'test(curve_intersect)' --no-fail-fast` and verify all tests pass including the new degenerate case tests. Run `cargo clippy -p monstertruck-geometry -- -W warnings` to verify no warnings.
  </verify>
  <done>Degenerate case handling (parallel curves, singular Jacobian, self-intersections) is implemented. All tests pass without panics.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-geometry --lib` passes with no failures.
2. `cargo nextest run -p monstertruck-geometry -E 'test(curve_intersect)'` passes all intersection tests.
3. `cargo clippy -p monstertruck-geometry -- -W warnings` produces no warnings.
4. `monstertruck_geometry::nurbs::curve_intersect::find_intersections` is public and importable.
5. `monstertruck_geometry::nurbs::curve_intersect::find_self_intersections` is public and importable.
6. Intersection accuracy is within SNAP_TOLERANCE (1.0e-5) for all test cases.
7. No panics on degenerate inputs (parallel, tangent, singular Jacobian).
</verification>

<success_criteria>
- curve_intersect.rs exists in monstertruck-geometry/src/nurbs/ and exports find_intersections and find_self_intersections.
- CurveIntersection result type contains t0, t1, and point fields.
- Subdivision/refinement algorithm correctly finds transversal, tangent, and self-intersections.
- All degenerate cases (parallel, overlapping, singular Jacobian) are handled without panics.
- Intersection results are accurate within SNAP_TOLERANCE.
- Integration tests cover crossing, non-intersecting, tangent, multiple, self-intersection, and parallel cases.
</success_criteria>

<output>
After completion, create `.tendrion/phases/17-curve-intersection-engine/17-1-SUMMARY.md`
</output>
