# Review Context: Plan 1-1 Spec Compliance

## Review Parameters
- **Plan ID:** 1-1
- **Stage:** spec-compliance
- **Round:** 1 of 2
- **Commit Range:** `f39103cdb3065282d0397a86da5a45bca25f9000..b580094e94289e071fe2ca5954a58a58e289da3e`
- **Review Output:** `.tendrion/phases/1-core-stabilization/1-1-REVIEW-spec.md`

## Scope Reviewed
- `.tendrion/phases/1-core-stabilization/1-1-PLAN.md`
- `.tendrion/phases/1-core-stabilization/1-1-SUMMARY.md`
- `monstertruck-modeling/src/geometry.rs`
- `monstertruck-geometry/src/decorators/intersection_curve.rs`
- `monstertruck-geometry/src/decorators/revolved_curve.rs`
- `monstertruck-geometry/src/nurbs/bspline_surface.rs`
- `monstertruck-geometry/src/specifieds/plane.rs`

## Commit Facts
- Commits in range: `b580094e`
- Changed files:
  - `.tendrion/STATE.md`
  - `.tendrion/phases/1-core-stabilization/1-1-SUMMARY.md`
  - `monstertruck-modeling/src/geometry.rs`
  - `monstertruck-modeling/tests/intersection_curve_impls.rs`

## Must-Have Verification Matrix
1. **All 9 `IntersectionCurve` `unimplemented!()` arms replaced in `geometry.rs`:** **PASS**.
   - Evidence: no `unimplemented!()` remains in `monstertruck-modeling/src/geometry.rs`; replacement arms present at `lift_up`, `IncludeCurve`, `ExtrudedCurve` sites.

2. **`lift_up()` approximates via leader curve:** **PASS**.
   - Evidence: `Curve::IntersectionCurve(ic) => ic.leader().lift_up()` (`monstertruck-modeling/src/geometry.rs:112`).

3. **`IncludeCurve` uses knot-span sampling with iterative hints for all surface types:** **PARTIAL / FAIL**.
   - Evidence (sampling): Revoluted outer `IntersectionCurve` arm uses knot-span sampling with iterative `search_parameter` hints (`monstertruck-modeling/src/geometry.rs:295-315`).
   - Evidence (non-sampling path): Plane branch delegates to `Plane::include(NurbsCurve)` (`monstertruck-modeling/src/geometry.rs:232-235`), and that impl is control-point/endpoint coplanarity (`monstertruck-geometry/src/specifieds/plane.rs:196-212`).

4. **`ExtrudedCurve::to_same_geometry` handles `IntersectionCurve` via homotopy:** **PASS**.
   - Evidence: `BsplineSurface::homotopy(c0, c1)` in `IntersectionCurve` arm (`monstertruck-modeling/src/geometry.rs:410-414`).

5. **`cargo test -p monstertruck-modeling` passes:** **UNVERIFIED IN REVIEW ENV**.
   - Evidence from summary claims pass (`.tendrion/phases/1-core-stabilization/1-1-SUMMARY.md:40`).

6. **Downstream `monstertruck-solid` tests pass:** **FAIL (not completed)**.
   - Evidence: summary deviation states downstream test verification could not be completed (`.tendrion/phases/1-core-stabilization/1-1-SUMMARY.md:46`).

7. **Workspace clippy clean:** **FAIL (not executed as specified)**.
   - Evidence: summary only reports crate-scoped clippy (`.tendrion/phases/1-core-stabilization/1-1-SUMMARY.md:41`), while plan requires workspace command (`.tendrion/phases/1-core-stabilization/1-1-PLAN.md:201`, `:210`, `:220`).

## Key Link Check
- Required pattern `leader()` in `geometry.rs` for approximation link to `intersection_curve` is present (`monstertruck-modeling/src/geometry.rs:112`, `:411`, `:412`).

## Local Command Notes
- `cargo` commands could not be executed in this environment:
  - `/snap/bin/cargo` fails with `snap-confine ... required permitted capability cap_dac_override not found`.
- Review conclusions on test/clippy compliance therefore rely on committed plan + summary + code inspection, not live reruns.

## Findings Carried Into Review
- `F1` High: Downstream `monstertruck-solid` verification incomplete.
- `F2` High: Workspace-wide clippy requirement unmet.
- `F3` Medium: Plane `IntersectionCurve` include path is not knot-span/iterative-hint based.
