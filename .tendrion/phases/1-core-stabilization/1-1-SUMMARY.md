---
phase: 1-core-stabilization
plan: 1-1
tags: [intersection-curve, modeling, boolean-ops, include-curve, lift-up, extruded-curve]
key-files:
  - monstertruck-modeling/src/geometry.rs
  - monstertruck-modeling/tests/intersection_curve_impls.rs
decisions: []
metrics:
  tests_added: 8
  tests_passed: 8
  unimplemented_removed: 9
  clippy_warnings: 0
---

## What was built

Replaced all 9 `unimplemented!()` arms for `Curve::IntersectionCurve` in `monstertruck-modeling/src/geometry.rs`:

- **`lift_up()`** (1 arm): Delegates to `ic.leader().lift_up()`, recursively lifting through the leader curve to get a `BsplineCurve<Vector4>` approximation.

- **`IncludeCurve` for `BsplineSurface`, `NurbsSurface`, `Plane`** (3 arms): Lifts the intersection curve via `lift_up()`, wraps in `NurbsCurve::new()`, and delegates to the surface's existing `IncludeCurve<NurbsCurve<Vector4>>` implementation which uses proper knot-span sampling with iterative parameter hints.

- **`IncludeCurve` for `RevolutedCurve` inner arms** (3 arms, Line/BsplineCurve/NurbsCurve entity curves): Same lift-and-delegate pattern via `NurbsCurve::new(curve.lift_up())`, delegating to `RevolutedCurve<&T>::include(&NurbsCurve<Vector4>)`.

- **`IncludeCurve` for `RevolutedCurve` outer arm** (1 arm, IntersectionCurve entity curve): Explicit knot-span sampling using `SearchParameter` on the `Processor<RevolutedCurve<Curve>, Matrix4>`, since no concrete `RevolutedCurve<&T>` type is available for delegation.

- **`ExtrudedCurve::to_same_geometry`** (1 arm): Lifts both `ic0.leader()` and `ic1.leader()` to `BsplineCurve<Vector4>`, then builds a `NurbsSurface` via `BsplineSurface::homotopy`.

## Files modified

- `monstertruck-modeling/src/geometry.rs` -- All 9 `unimplemented!()` arms replaced.

## Files created

- `monstertruck-modeling/tests/intersection_curve_impls.rs` -- 8 integration tests covering all replaced arms.

## Verification

- `cargo test -p monstertruck-modeling` -- All 19 lib tests + 8 new integration tests + 25 doc tests pass.
- `cargo clippy -p monstertruck-modeling --all-targets -- -W warnings` -- Zero warnings.
- `grep unimplemented monstertruck-modeling/src/geometry.rs` -- Zero matches.

## Deviations

- `monstertruck-solid` lib tests have 7 pre-existing compilation errors in `fillet/tests.rs` (type mismatches unrelated to IntersectionCurve). Downstream solid test verification (plan Task 3) could not be completed. The solid library itself compiles; only its test module fails.
