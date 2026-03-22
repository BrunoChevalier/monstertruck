---
phase: 22
name: conversion-fidelity
status: complete
plans_total: 3
plans_complete: 3
tdd_compliance: 100%
duration: 34m
---

## What Was Built

**Plan 22-1 (FCONV-01):** Upgraded `sample_curve_to_nurbs` and `sample_surface_to_nurbs` in `monstertruck-solid/src/fillet/convert.rs` from degree-1 piecewise-linear to degree-3 cubic interpolation via `BsplineCurve::try_interpolate` with `KnotVector::uniform_knot(3, n_points - 3)`. Added `greville_abscissae` helper and `try_degree3_surface` two-pass tensor product helper. Updated `sample_to_nurbs` in `monstertruck-modeling/src/fillet_impl.rs` to degree-3 with 24 sample points, covering `From<ParameterCurveLinear>`, `From<FilletIntersectionCurve>`, and `TryFrom<Curve>` paths. Degree-1 fallback retained.

**Plan 22-2 (FCONV-03):** Implemented `RevolutedCurve<NurbsCurve<Vector4>>::to_nurbs_surface()` via rational circle arc tensor product (degree-2, 9 control points for full 2π revolution) in `monstertruck-geometry/src/decorators/revolved_curve.rs`. Added `RevolutedCurve<BsplineCurve<Point3>>` convenience wrapper. Wired exact conversion into `TryFrom<Surface> for NurbsSurface<Vector4>` in `monstertruck-modeling/src/fillet_impl.rs`, replacing the prior `Err(())` fallback for `Surface::RevolutedCurve`.

**Plan 22-3 (FCONV-02):** Added `snap_curve_endpoints` and `snap_shell_endpoints` helpers in `monstertruck-solid/src/fillet/convert.rs`. Integrated endpoint snapping into `convert_shell_in`, `convert_shell_out`, and `sample_curve_to_nurbs`. Three tests verify: closure preservation via round-trip, boundary control point exactness within 1e-14, and IntersectionCurve edge snap within 1e-14.

## Requirement Coverage

| ID | Plan | Status |
|----|------|--------|
| FCONV-01 | 22-1 | Covered |
| FCONV-02 | 22-3 | Covered |
| FCONV-03 | 22-2 | Covered |

## Test Results

- 22-1: 3 tests added; 112/118 pass in monstertruck-solid (6 pre-existing failures), 31/31 in monstertruck-modeling
- 22-2: 4 tests added; 146/146 pass in monstertruck-geometry + monstertruck-modeling
- 22-3: 3 tests added; all 3 endpoint_snap tests pass; 6 pre-existing failures unchanged

## TDD Compliance

- Level: strict
- Cycles total: 3, compliant: 3 (100%)
- All plans followed RED-GREEN-REFACTOR commit sequence

## Deviations

- 52 auto-fix deviations (compile errors, clippy, formatting)
- 0 approval-needed deviations
- 6 pre-existing test failures in monstertruck-solid confirmed not caused by phase changes

## Key Decisions

- NURBS circle arc parameterization is nonlinear within quadrants (standard for rational Bezier circles); tests verify geometric exactness at knot breakpoints via distance-from-axis checks
- `sample_curve_to_nurbs` made `pub(super)` to enable direct testing from `tests.rs`
- Extracted `snap_shell_endpoints` helper to deduplicate snapping loops in `convert_shell_in`/`convert_shell_out`
- Reused `param_points` boundary values instead of redundant `evaluate(t0)/evaluate(t1)` calls in `sample_curve_to_nurbs`
