# Review Context: Plan 1-1 (Planning Review, Round 2 of 2)

## Plan Under Review

- Plan ID: `1-1`
- Plan path: `.tendrion/phases/1-core-stabilization/1-1-PLAN.md`
- Stage: `planning`
- Round: `2 of 2`

---

## Previous Round Blockers Verification

1. `B1` -- **Unresolved**.
   - Previous blocker: Task 2 had a non-compilable ownership pattern for the `ExtrudedCurve` `IntersectionCurve` arm.
   - Current evidence: the revised snippet still uses `(Curve::IntersectionCurve(_), Curve::IntersectionCurve(_))` and then references `curve1` in the arm body (`1-1-PLAN.md:176-181`).
   - Code context: in actual implementation structure, `curve1` is moved into `match (curve0, curve1)` (`monstertruck-modeling/src/geometry.rs:343`), and the current `IntersectionCurve` arm is at `monstertruck-modeling/src/geometry.rs:357`.

2. `B2` -- **Resolved**.
   - Previous blocker: `IncludeCurve` guidance used control-point-only checks.
   - Current evidence: plan now explicitly requires knot-span sampling and iterative hints (`1-1-PLAN.md:62-63`, `1-1-PLAN.md:91-115`, `1-1-PLAN.md:133-157`), and delegates to existing `IncludeCurve` implementations where available.

---

## Additional Technical Observation

1. Task 1’s revoluted-surface inner-arm snippet converts lifted homogeneous control points directly to `Point3` before inclusion (`1-1-PLAN.md:123-126`).
   - This can drop rational weight information for rational leaders.
   - Existing `RevolutedCurve` implementations already support `IncludeCurve<NurbsCurve<Vector4>>` (`monstertruck-geometry/src/decorators/revolved_curve.rs:460-489`), so preserving the lifted `NurbsCurve` is feasible.

---

## Sibling Plans

| Plan | Wave | Objective |
|------|------|-----------|
| 1-2 | 1 | Replace the deprecated proc-macro-error dependency in monstertruck-derive with proc-macro-error2 |
| 1-3 | 1 | Achieve 50% reduction in production unwrap() density across monstertruck-solid and monstertruck-meshing |
| 1-4 | 2 | Add criterion-based benchmarking infrastructure with CI integration |

