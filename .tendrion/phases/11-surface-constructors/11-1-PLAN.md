---
phase: 11-surface-constructors
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/tests/bspsurface.rs
autonomous: true
must_haves:
  truths:
    - "User calls BsplineSurface::sweep_multi_rail with 3+ rails and a profile, receives a valid BsplineSurface"
    - "User calls BsplineSurface::sweep_periodic with a profile and rail, receives a closed BsplineSurface where subs(u,0) == subs(u,1)"
    - "Multi-rail sweep interpolates the profile at each rail position"
    - "Periodic sweep produces C0-continuous wrap seam verified by point evaluation"
    - "Both methods return Err instead of panicking on invalid input (e.g., fewer than 2 rails, empty profile, singular covariance matrix)"
    - "Affine fitting returns Err for degenerate configurations (collinear or coincident reference points producing a singular covariance matrix)"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      provides: "sweep_multi_rail and sweep_periodic methods on BsplineSurface<Point3>"
      min_lines: 80
      contains: "sweep_multi_rail"
    - path: "monstertruck-geometry/tests/bspsurface.rs"
      provides: "Integration tests for multi-rail and periodic sweep"
      min_lines: 40
      contains: "sweep_multi_rail"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/src/nurbs/compat.rs"
      via: "make_curves_compatible used to normalize sections before skinning"
      pattern: "make_curves_compatible"
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "skin method used internally by both new sweep variants"
      pattern: "BsplineSurface::skin"
---

<objective>
Add multi-rail sweep and periodic sweep surface constructors to BsplineSurface in monstertruck-geometry. These implement the SURF-01 requirement as geometry-level methods `sweep_multi_rail` and `sweep_periodic` on `BsplineSurface<Point3>`.

NOTE on naming: The roadmap references `SweepBuilder::multi_rail()` and `SweepBuilder::periodic()`, but the codebase convention uses free functions and associated methods (e.g., `builder::extrude`, `builder::revolve`, `BsplineSurface::sweep_rail`). A `SweepBuilder` type would be inconsistent with existing patterns. The roadmap criterion is satisfied by `BsplineSurface::sweep_multi_rail` (geometry) and `builder::try_sweep_multi_rail` / `builder::try_sweep_periodic` (topology wrappers in Plan 11-2).

NOTE on Euler-Poincare: This plan operates at the geometry level only (BsplineSurface methods). Euler-Poincare topology checks are deferred to Plan 11-2 where the topology conversion (Face/Shell construction) and `is_geometric_consistent()` validation happens.
</objective>

<execution_context>
@skills/rust-development/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs (lines 1477-1802: existing skin, sweep_rail, birail1, birail2 methods)
@monstertruck-geometry/src/nurbs/compat.rs (make_curves_compatible, make_surfaces_compatible)
</context>

<tasks>

<task type="auto">
  <name>Task 1: Implement BsplineSurface::sweep_multi_rail</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Add a `sweep_multi_rail` method to `impl BsplineSurface<Point3>` (the same impl block that contains `sweep_rail`, `birail1`, `birail2`).

Signature:
```rust
pub fn sweep_multi_rail(
    profile: BsplineCurve<Point3>,
    rails: &[BsplineCurve<Point3>],
    n_sections: usize,
) -> Result<BsplineSurface<Point3>, &'static str>
```

Algorithm (multi-rail approach):
1. Validate: `rails.len() >= 2`, `n_sections >= 2`, profile not empty. Return `Err` on failure.
2. Compute reference points on the profile for each rail: find the profile parameter closest to each rail's start point using `search_nearest_parameter`, then evaluate the profile at those parameters. These are the "anchor" points that will be mapped to rail positions.
3. For each section index `i` (0..n_sections):
   a. Compute parameter `t_i` uniformly from rail start to rail end.
   b. Collect the target positions from each rail at `t_i`.
   c. Compute an affine transform that maps the profile's anchor points to these target positions:
      - For 2 rails: use the same scale+rotate+translate approach as birail1 (map profile chord to target chord).
      - For 3+ rails: use a least-squares affine fit.
   d. Apply this transform to the profile's control points to produce section curve `i`.
4. Call `BsplineSurface::skin(sections)` to produce the final surface.

For the least-squares affine fitting with 3+ rails:
1. Compute centroid_ref = mean of reference anchor points; centroid_target = mean of target points.
2. Center both point sets: ref_centered[k] = ref[k] - centroid_ref, target_centered[k] = target[k] - centroid_target.
3. Build covariance matrix H = sum(ref_centered[k] * target_centered[k]^T) (3x3).
4. Compute the transformation M via: M = (H^T * H)^(-1) * H^T applied to the target-centered points. Actually, the simpler correct approach: M = sum(target * ref^T) * (sum(ref * ref^T))^(-1). This is the standard least-squares solution.
5. If the `ref * ref^T` matrix is singular (determinant below TOLERANCE), return `Err("degenerate rail configuration: reference points are collinear or coincident")`.
6. Apply: transformed_pt = centroid_target + M * (pt - centroid_ref).

Create a private helper function `affine_fit_3x3` that takes reference points and target points, returns `Result<(Matrix3, Vector3), &'static str>` where the Matrix3 is the linear part and Vector3 is the translation.

Add doc comments with a `# Examples` section showing usage with 3 rails. Add a `# Errors` section documenting when it returns Err (fewer than 2 rails, fewer than 2 sections, degenerate/singular configuration).
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-geometry -E 'test(sweep_multi_rail)'` to verify the new tests pass. Also run `cargo clippy -p monstertruck-geometry --all-targets -- -W warnings` to check for warnings.
  </verify>
  <done>BsplineSurface::sweep_multi_rail implemented with least-squares affine fitting for 3+ rails, returning Err for invalid input and degenerate configurations.</done>
</task>

<task type="auto">
  <name>Task 2: Implement BsplineSurface::sweep_periodic</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Add a `sweep_periodic` method to `impl BsplineSurface<Point3>`.

Signature:
```rust
pub fn sweep_periodic(
    profile: BsplineCurve<Point3>,
    rail: &BsplineCurve<Point3>,
    n_sections: usize,
) -> Result<BsplineSurface<Point3>, &'static str>
```

Algorithm (duplicated-endpoint approach -- this is the committed design):
1. Validate: `n_sections >= 3` (need at least 3 sections for a closed loop), profile and rail not empty. Return `Err` on failure.
2. Sample `n_sections` positions along the rail at parameters uniformly spaced from `t_start` to `t_end`. The rail should represent a closed path (first and last points near each other), though this is not strictly enforced.
3. For each section `i` (0..n_sections), compute the tangent-aligned frame at the rail point using the same `rotation_between` logic as `sweep_rail`, then translate the profile to the rail position.
4. Produce `n_sections + 1` section curves where `section[n_sections]` is a clone of `section[0]` (duplicated endpoint). This ensures the skinned surface has `subs(u, 0) == subs(u, 1)` by construction, achieving C0 continuity at the seam.
5. Call `BsplineSurface::skin(sections)` to produce the final surface.

This is the "duplicated-endpoint" approach. It guarantees C0 seam continuity by construction without requiring periodic knot vector manipulation. The resulting surface evaluates identically at v=0 and v=1 because those sections share control points.

Add doc comments with:
- A `# Examples` section showing a circular rail and verifying `subs(u, 0.0).near(&subs(u, 1.0))`.
- A `# Errors` section documenting when it returns Err (too few sections, empty input).
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-geometry -E 'test(sweep_periodic)'` to verify the new tests pass. Also run `cargo clippy -p monstertruck-geometry --all-targets -- -W warnings`.
  </verify>
  <done>BsplineSurface::sweep_periodic implemented with duplicated-endpoint approach for C0 seam continuity, returning Err for invalid input.</done>
</task>

<task type="auto">
  <name>Task 3: Add integration tests for multi-rail and periodic sweep</name>
  <files>monstertruck-geometry/tests/bspsurface.rs</files>
  <action>
Add tests within the existing bspsurface.rs integration test file covering:

1. `test_sweep_multi_rail_three_rails`: Create 3 diverging rail curves (e.g., straight lines in z-direction with different x-offsets) and a connecting profile. Call `sweep_multi_rail`. Verify:
   - The result is Ok.
   - The surface evaluates to points near the rail positions at the expected v-parameters.
   - The u-degree matches the profile's degree.

2. `test_sweep_multi_rail_error_on_single_rail`: Pass a single rail, verify `Err` is returned.

3. `test_sweep_multi_rail_matches_birail1_for_two_rails`: With 2 rails, verify `sweep_multi_rail` produces results very close to `birail1` (within tolerance). Compare several surface evaluation points.

4. `test_sweep_multi_rail_degenerate_collinear_rails`: Create 3 rails whose start points are collinear (all on the same line). This produces a singular covariance matrix for the affine fit. Verify that `sweep_multi_rail` returns an `Err` containing a message about degenerate configuration.

5. `test_sweep_periodic_closed_seam`: Create a circular-ish rail (e.g., use 4+ BsplineCurve control points approximating a circle, with start near end). Sweep a small profile. Verify:
   - Result is Ok.
   - `surface.subs(u, 0.0).near(&surface.subs(u, 1.0))` for several u values (C0 continuity at seam).

6. `test_sweep_periodic_error_on_too_few_sections`: Pass `n_sections = 2`, verify `Err`.

Use `assert_near2!` macro from the geometry prelude for point comparisons.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-geometry -E 'test(sweep_multi_rail) | test(sweep_periodic)'` -- all tests pass. Run `cargo clippy -p monstertruck-geometry --all-targets -- -W warnings` -- no warnings.
  </verify>
  <done>Six integration tests added covering multi-rail sweep (3 rails, error cases, 2-rail equivalence, degenerate configuration) and periodic sweep (closed seam, error case).</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-geometry -E 'test(sweep_multi_rail) | test(sweep_periodic)'` passes all 6 tests
2. `cargo clippy -p monstertruck-geometry --all-targets -- -W warnings` shows no warnings
3. Existing sweep_rail, birail1, birail2, gordon tests still pass: `cargo nextest run -p monstertruck-geometry`
4. The periodic sweep test demonstrates subs(u, 0) == subs(u, 1) within tolerance
5. The degenerate collinear test demonstrates Err return for singular covariance matrix
</verification>

<success_criteria>
- sweep_multi_rail produces valid BsplineSurface for 3+ rail curves
- sweep_multi_rail returns Err for degenerate configurations (singular covariance matrix from collinear/coincident reference points)
- sweep_periodic produces a closed surface with C0 continuity at the wrap seam using the duplicated-endpoint approach
- Both methods return typed errors instead of panicking on invalid input
- All existing geometry tests remain green
</success_criteria>

<output>
After completion, create `.tendrion/phases/11-surface-constructors/11-1-SUMMARY.md`
</output>
