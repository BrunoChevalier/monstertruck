---
phase: 22-conversion-fidelity
plan: 2
tags: [geometry, nurbs, conversion, fillet]
key-files:
  - monstertruck-geometry/src/decorators/revolved_curve.rs
  - monstertruck-modeling/src/fillet_impl.rs
decisions:
  - "NURBS circle arc parameterization is nonlinear within quadrants (standard for rational Bezier circles); tests verify geometric exactness at knot breakpoints and via distance-from-axis checks rather than point-by-point parameter matching"
metrics:
  tests_added: 4
  tests_passing: 146
  lines_added: ~120
---

## What was built

### monstertruck-geometry/src/decorators/revolved_curve.rs
- Added `RevolutedCurve<NurbsCurve<Vector4>>::to_nurbs_surface()` -- exact conversion via rational circle arc tensor product (degree-2, 9 control points for full 2*PI revolution).
- Added `RevolutedCurve<BsplineCurve<Point3>>::to_nurbs_surface()` -- convenience wrapper that lifts the BsplineCurve to NurbsCurve first.
- Extracted `CIRCLE_COS`, `CIRCLE_SIN`, `CIRCLE_W` constants and `full_revolution_knot_vector()` helper.
- Handles w=0 control points (points at infinity) correctly by working in homogeneous coordinates.
- 3 unit tests: line around Y-axis (cylinder), rational half-circle around X-axis (sphere), BsplineCurve convenience path.

### monstertruck-modeling/src/fillet_impl.rs
- Updated `TryFrom<Surface> for NurbsSurface<Vector4>` to handle `Surface::RevolutedCurve` via exact conversion instead of returning `Err(())`.
- Unwraps the `Processor`, converts the inner `Curve` to `NurbsCurve<Vector4>`, calls `to_nurbs_surface()`, then applies transform and orientation.
- 1 test verifying the TryFrom succeeds and produces geometrically correct output.

## Task commits

| SHA | Message |
|-----|---------|
| 96d22960 | test(geometry): add failing tests for RevolutedCurve::to_nurbs_surface exact conversion |
| 6075cc2d | feat(geometry): implement exact RevolutedCurve::to_nurbs_surface via rational circle arc tensor product |
| 9fcfb297 | refactor(geometry): extract circle arc constants and simplify to_nurbs_surface structure |
| 5e9491a4 | test(modeling): add failing test for TryFrom<Surface> with RevolutedCurve |
| f7c36cd9 | feat(modeling): wire RevolutedCurve exact conversion into TryFrom<Surface> for NurbsSurface |

## Deviations

- Pre-existing test failures in `fillet::geometry::test_unit_circle` (proptest numerical precision) and `fillet::tests::generic_fillet_identity` (assertion) -- not caused by this plan.
- Pre-existing clippy error in `monstertruck-geometry/src/nurbs/test_fixtures.rs` (type_complexity) blocks workspace-wide clippy.

## Self-check

- `cargo nextest run -p monstertruck-geometry -p monstertruck-modeling --lib --features fillet` -- 146/146 pass.
- `RevolutedCurve(_) |` no longer appears in the `Err(())` arm of `TryFrom<Surface>`.
- `to_nurbs_surface()` method produces geometrically exact surfaces verified by distance-from-axis checks.
