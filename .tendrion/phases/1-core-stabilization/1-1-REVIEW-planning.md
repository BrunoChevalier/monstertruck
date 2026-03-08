# Planning Review: Plan 1-1 (Round 2 of 2)

**Plan:** `1-1`  
**Phase:** `1-core-stabilization`  
**Verdict:** `CHANGES_REQUESTED`

## Previous Blockers Status

- `B1` (Task 2 ownership/compilability): **Unresolved**.
- `B2` (IncludeCurve control-point-only strategy): **Resolved**.

## Findings

1. **High** -- Task 2 still contains a non-compilable ownership pattern for the `ExtrudedCurve` `IntersectionCurve` arm.
   - Evidence: revised snippet keeps pattern `(Curve::IntersectionCurve(_), Curve::IntersectionCurve(_))` but uses `curve1` in the body (`.tendrion/phases/1-core-stabilization/1-1-PLAN.md:176-181`).
   - Code context: `curve1` is moved into `match (curve0, curve1)` (`monstertruck-modeling/src/geometry.rs:343`), and current unresolved arm is at (`monstertruck-modeling/src/geometry.rs:357`).
   - Impact: following this task as written leads to borrow-of-moved-value compilation failure.
   - Required change: bind the second tuple element in the match arm (or precompute lifted curves before the match) so the transformed `Curve` is still usable.

2. **Medium** -- Task 1’s revoluted inner-arm guidance can lose rational information unnecessarily.
   - Evidence: plan converts lifted `Vector4` control points to `Point3` and rebuilds `BsplineCurve<Point3>` (`.tendrion/phases/1-core-stabilization/1-1-PLAN.md:123-126`).
   - Evidence: revoluted surfaces already implement `IncludeCurve<NurbsCurve<Vector4>>` across entity-curve variants (`monstertruck-geometry/src/decorators/revolved_curve.rs:460-489`).
   - Impact: rational leaders can be distorted by control-point projection, weakening inclusion correctness.
   - Required change: prefer delegating with `NurbsCurve::new(lifted)` instead of projecting to `BsplineCurve<Point3>`.

## Review Notes

- Static-analysis review for planning stage; no runtime execution required.
