---
phase: 11-surface-constructors
plan: 2
type: execute
wave: 2
depends_on: ["11-1"]
files_modified:
  - monstertruck-modeling/src/builder.rs
  - monstertruck-modeling/src/errors.rs
  - monstertruck-modeling/tests/surface_constructors.rs
autonomous: true
must_haves:
  truths:
    - "User calls builder::try_sweep_rail with a profile edge and a rail curve, receives Result<Face> with typed error on invalid input"
    - "User calls builder::try_birail with profile edge and two rail curves, receives Result<Face> with typed error"
    - "User calls builder::try_gordon with u-curves, v-curves, and intersection points, receives Result<Face> with typed error"
    - "User calls builder::try_sweep_multi_rail with a profile edge and 3+ rails, receives Result<Face> with typed error"
    - "User calls builder::try_sweep_periodic with a profile edge and closed rail, receives Result<Shell> with closed-surface seam continuity"
    - "Generated surfaces pass is_geometric_consistent when converted to topology elements"
    - "All builder wrappers return typed Error variants instead of panicking"
  artifacts:
    - path: "monstertruck-modeling/src/builder.rs"
      provides: "try_sweep_rail, try_birail, try_gordon, try_sweep_multi_rail, try_sweep_periodic builder functions"
      min_lines: 100
      contains: "try_sweep_rail"
    - path: "monstertruck-modeling/src/errors.rs"
      provides: "New error variants for surface constructor failures"
      min_lines: 30
      contains: "SurfaceConstructionFailed"
    - path: "monstertruck-modeling/tests/surface_constructors.rs"
      provides: "Integration tests for all builder-level surface constructors"
      min_lines: 80
      contains: "try_sweep_rail"
  key_links:
    - from: "monstertruck-modeling/src/builder.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "Calls BsplineSurface::sweep_rail, birail1, birail2, gordon, sweep_multi_rail, sweep_periodic"
      pattern: "BsplineSurface::sweep_rail"
    - from: "monstertruck-modeling/src/builder.rs"
      to: "monstertruck-modeling/src/errors.rs"
      via: "Returns Error::SurfaceConstructionFailed or Error::InsufficientRails"
      pattern: "Error::SurfaceConstructionFailed"
    - from: "monstertruck-modeling/tests/surface_constructors.rs"
      to: "monstertruck-modeling/src/builder.rs"
      via: "Tests call builder::try_* functions and verify Result variants"
      pattern: "builder::try_sweep_rail"
---

<objective>
Create typed builder-level wrappers in monstertruck-modeling for sweep_rail, birail, gordon, multi-rail sweep, and periodic sweep. These wrappers accept topology-level types (Wire, Edge), extract geometry, call the underlying BsplineSurface methods, and return typed Result errors instead of panicking on invalid input. This fulfills SURF-02 and the Euler-Poincare topology check success criterion.

NOTE on naming: The roadmap references `SweepBuilder::multi_rail()` and `SweepBuilder::periodic()`, but the codebase convention uses free functions (`builder::extrude`, `builder::revolve`, `builder::try_attach_plane`). A `SweepBuilder` type would be inconsistent with existing patterns. The roadmap criterion is satisfied by `builder::try_sweep_multi_rail` and `builder::try_sweep_periodic` free functions, matching existing codebase conventions exactly.

NOTE on return types: `try_sweep_periodic` returns `Result<Shell>` (not `Result<Face>`) because a periodic/closed surface requires a Shell to properly represent the closed topology. All other wrappers return `Result<Face>`.
</objective>

<execution_context>
@skills/rust-development/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-modeling/src/builder.rs (existing builder functions: vertex, line, homotopy, try_wire_homotopy, try_attach_plane, extrude, revolve)
@monstertruck-modeling/src/errors.rs (existing Error enum)
@monstertruck-modeling/src/geometry.rs (Curve and Surface enums)
@monstertruck-modeling/src/lib.rs (module structure and re-exports)
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add error variants for surface construction</name>
  <files>monstertruck-modeling/src/errors.rs</files>
  <action>
Add new error variants to the `Error` enum in `monstertruck-modeling/src/errors.rs`:

```rust
/// Too few rails provided for a multi-rail sweep (need >= 2).
#[error("multi-rail sweep requires at least {required} rails, got {got}.")]
InsufficientRails { required: usize, got: usize },

/// Too few sections for surface construction.
#[error("surface construction requires at least {required} sections, got {got}.")]
InsufficientSections { required: usize, got: usize },

/// Surface construction algorithm failed (e.g., degenerate geometry, incompatible curves).
#[error("surface construction failed: {reason}")]
SurfaceConstructionFailed { reason: String },

/// Curve grid dimensions mismatch for Gordon surface.
#[error("gordon surface requires matching grid dimensions: expected {expected_rows}x{expected_cols}, got {actual_rows}x{actual_cols}.")]
GridDimensionMismatch {
    expected_rows: usize,
    expected_cols: usize,
    actual_rows: usize,
    actual_cols: usize,
},
```

NOTE: The `Error` enum currently derives `PartialEq, Eq`. The `SurfaceConstructionFailed` variant uses `String`, which supports `PartialEq + Eq`, so no derive changes are needed.

Update the `print_messages` test to include the new variants:
```rust
writeln!(&mut std::io::stderr(), "{}\n", Error::InsufficientRails { required: 2, got: 1 }).unwrap();
writeln!(&mut std::io::stderr(), "{}\n", Error::InsufficientSections { required: 2, got: 1 }).unwrap();
writeln!(&mut std::io::stderr(), "{}\n", Error::SurfaceConstructionFailed { reason: "test".into() }).unwrap();
writeln!(&mut std::io::stderr(), "{}\n", Error::GridDimensionMismatch { expected_rows: 2, expected_cols: 3, actual_rows: 1, actual_cols: 2 }).unwrap();
```
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-modeling -E 'test(print_messages)'` to verify the error messages compile and display correctly. Run `cargo clippy -p monstertruck-modeling --all-targets -- -W warnings`.
  </verify>
  <done>Four new error variants added to monstertruck-modeling's Error enum for surface construction failures.</done>
</task>

<task type="auto">
  <name>Task 2: Implement try_sweep_rail, try_birail, and try_gordon builder wrappers</name>
  <files>monstertruck-modeling/src/builder.rs</files>
  <action>
Add builder wrapper functions for the three existing geometry methods that already exist in BsplineSurface. These wrap `sweep_rail`, `birail1`, and `gordon` respectively.

**Pattern for all wrappers (topology construction from BsplineSurface):**

Create a shared private helper function that builds a Face from a BsplineSurface:
```rust
/// Builds a Face from a BsplineSurface by extracting its 4 boundary curves
/// and constructing proper topology (vertices, edges, wire).
fn face_from_bspline_surface(
    surface: BsplineSurface<Point3>,
) -> Face<Curve, Surface> {
    // 1. Get the 4 corner points from the surface.
    let (u_range, v_range) = surface.range_tuple();
    let p00 = surface.subs(u_range.0, v_range.0);
    let p10 = surface.subs(u_range.1, v_range.0);
    let p01 = surface.subs(u_range.0, v_range.1);
    let p11 = surface.subs(u_range.1, v_range.1);

    // 2. Create vertices at corners.
    let v00 = vertex(p00);
    let v10 = vertex(p10);
    let v11 = vertex(p11);
    let v01 = vertex(p01);

    // 3. Extract boundary curves from the surface.
    //    column_curve(0) = v=v_min boundary (u varies), row 0
    //    column_curve(last) = v=v_max boundary
    //    row_curve(0) = u=u_min boundary (v varies), column 0
    //    row_curve(last) = u=u_max boundary
    let n_rows = surface.control_points().len();
    let n_cols = surface.control_points()[0].len();
    let bottom: BsplineCurve<Point3> = surface.column_curve(0);         // v=0, u varies
    let top: BsplineCurve<Point3> = surface.column_curve(n_rows - 1);   // v=1, u varies
    let left: BsplineCurve<Point3> = surface.row_curve(0);              // u=0, v varies
    let right: BsplineCurve<Point3> = surface.row_curve(n_cols - 1);    // u=1, v varies

    // 4. Create edges. Wire orientation: bottom(v00->v10), right(v10->v11), top^-1(v11->v01), left^-1(v01->v00).
    let e_bottom = Edge::new(&v00, &v10, Curve::BsplineCurve(bottom));
    let e_right = Edge::new(&v10, &v11, Curve::BsplineCurve(right));
    let e_top = Edge::new(&v01, &v11, Curve::BsplineCurve(top));    // will be inverted in wire
    let e_left = Edge::new(&v00, &v01, Curve::BsplineCurve(left));  // will be inverted in wire

    // 5. Build wire and face.
    let wire = wire![e_bottom, e_right, e_top.inverse(), e_left.inverse()];
    Face::new(vec![wire], Surface::BsplineSurface(surface))
}
```

NOTE: The `column_curve` and `row_curve` methods use different indexing than u/v range. Verify the actual boundary layout by checking the BsplineSurface implementation -- `column_curve(row_idx)` returns the curve at a given row of control points (fixed v-position), and `row_curve(col_idx)` returns the curve at a given column (fixed u-position). The skin function puts sections as rows (v-direction), so column_curve(0) is the first section curve. Adjust edge construction based on actual curve directions to ensure topological consistency.

After edge creation, the Face must pass topological wire-closure checks. Use `Edge::new` (not `Edge::try_new`) since we know vertices match.

**1. `try_sweep_rail`:**
```rust
pub fn try_sweep_rail(
    profile: &BsplineCurve<Point3>,
    rail: &BsplineCurve<Point3>,
    n_sections: usize,
) -> Result<Face<Curve, Surface>>
```
Validates `n_sections >= 2` (else `Err(Error::InsufficientSections {...})`). Calls `BsplineSurface::sweep_rail(profile.clone(), rail, n_sections)`. Wraps result in Face via `face_from_bspline_surface`.

Note: Accept `&BsplineCurve<Point3>` directly rather than `&Edge<C>` -- this is simpler, avoids the generic complexity, and matches how `sweep_rail` etc. already take `BsplineCurve<Point3>`. Users can extract the curve from an edge if needed.

**2. `try_birail`:**
```rust
pub fn try_birail(
    profile: &BsplineCurve<Point3>,
    rail1: &BsplineCurve<Point3>,
    rail2: &BsplineCurve<Point3>,
    n_sections: usize,
) -> Result<Face<Curve, Surface>>
```
Validates `n_sections >= 2`. Calls `BsplineSurface::birail1(profile.clone(), rail1, rail2, n_sections)`. Wraps result in Face.

**3. `try_gordon`:**
```rust
pub fn try_gordon(
    u_curves: Vec<BsplineCurve<Point3>>,
    v_curves: Vec<BsplineCurve<Point3>>,
    points: &[Vec<Point3>],
) -> Result<Face<Curve, Surface>>
```
Validates: `u_curves` not empty, `v_curves` not empty, `points.len() == u_curves.len()`, each `points[i].len() == v_curves.len()`. Returns `Err(Error::GridDimensionMismatch {...})` on mismatch. Calls `BsplineSurface::gordon(u_curves, v_curves, points)`. Wraps in Face.

Add doc comments with `# Examples` and `# Errors` sections for each function. Place them near the other builder functions (after `try_skin_wires` or similar).
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-modeling` to verify existing tests still pass and new code compiles. Run `cargo clippy -p monstertruck-modeling --all-targets -- -W warnings`.
  </verify>
  <done>Three builder wrappers (try_sweep_rail, try_birail, try_gordon) implemented wrapping existing BsplineSurface geometry methods with typed error handling and Face topology construction.</done>
</task>

<task type="auto">
  <name>Task 3: Implement try_sweep_multi_rail and try_sweep_periodic builder wrappers</name>
  <files>monstertruck-modeling/src/builder.rs</files>
  <action>
Add builder wrappers for the two new geometry methods created in Plan 11-1. These have different topology characteristics than the Task 2 wrappers.

**4. `try_sweep_multi_rail`:**
```rust
pub fn try_sweep_multi_rail(
    profile: &BsplineCurve<Point3>,
    rails: &[BsplineCurve<Point3>],
    n_sections: usize,
) -> Result<Face<Curve, Surface>>
```
Validates `rails.len() >= 2` (else `Err(Error::InsufficientRails { required: 2, got: rails.len() })`). Validates `n_sections >= 2` (else `Err(Error::InsufficientSections {...})`). Calls `BsplineSurface::sweep_multi_rail(profile.clone(), rails, n_sections)`. Maps the geometry-level Err to `Error::SurfaceConstructionFailed { reason: err.to_string() }`. On success, wraps in Face via `face_from_bspline_surface`.

**5. `try_sweep_periodic`:**
```rust
pub fn try_sweep_periodic(
    profile: &BsplineCurve<Point3>,
    rail: &BsplineCurve<Point3>,
    n_sections: usize,
) -> Result<Shell<Curve, Surface>>
```
NOTE: Returns `Result<Shell>`, not `Result<Face>`. A periodic/closed sweep surface represents a closed tube topology that requires a Shell (multiple faces or a single face with appropriate boundary handling).

Validates `n_sections >= 3` (else `Err(Error::InsufficientSections { required: 3, got: n_sections })`). Calls `BsplineSurface::sweep_periodic(profile.clone(), rail, n_sections)`. Maps geometry-level Err to `Error::SurfaceConstructionFailed { ... }`.

For topology construction of the periodic surface:
- The surface has subs(u,0) == subs(u,1) by construction (duplicated endpoint approach from Plan 11-1).
- Build a Shell containing a single Face. The face's boundary wire should have 2 edges (the seam edge at v=0/v=1, shared by both the start and end, plus the degenerate identification). Actually, the simplest correct approach: since the BsplineSurface is parametrically closed in v but topologically still a rectangular patch, create a Face with the standard 4-edge boundary where the v=0 and v=1 edges share the same vertices (because the surface evaluates to the same points there). The face is like a cylinder unrolled -- 2 boundary loops at u=0 and u=1, with the v-seam identified.

Actually, the most pragmatic approach matching codebase patterns: build the Face using `face_from_bspline_surface` (same as other wrappers), then wrap it in a single-face Shell. The caller can later close/sew if needed. The seam continuity (subs(u,0)==subs(u,1)) is a geometric property; topological closure is a higher-level operation.

```rust
let surface = BsplineSurface::sweep_periodic(profile.clone(), rail, n_sections)
    .map_err(|e| Error::SurfaceConstructionFailed { reason: e.to_string() })?;
let face = face_from_bspline_surface(surface);
Ok(Shell::from(vec![face]))
```

Add doc comments noting that the result is a Shell with a single face whose v=0 and v=1 boundaries evaluate to the same geometric positions (C0 seam continuity).
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-modeling` to verify existing tests still pass and new code compiles. Run `cargo clippy -p monstertruck-modeling --all-targets -- -W warnings`.
  </verify>
  <done>Two builder wrappers (try_sweep_multi_rail returning Face, try_sweep_periodic returning Shell) implemented with typed error handling and topology construction.</done>
</task>

<task type="auto">
  <name>Task 4: Add integration tests for builder wrappers</name>
  <files>monstertruck-modeling/tests/surface_constructors.rs</files>
  <action>
Create a new integration test file `monstertruck-modeling/tests/surface_constructors.rs` with the following tests:

```rust
use monstertruck_modeling::*;
use monstertruck_modeling::errors::Error;
```

**1. `test_try_sweep_rail_basic`**:
- Create a profile BsplineCurve (straight line from (-1,0,0) to (1,0,0)) and a rail BsplineCurve (straight line from (0,0,0) to (0,0,5)).
- Call `builder::try_sweep_rail(&profile, &rail, 3)`.
- Verify the result is Ok.
- Verify the face has 1 boundary wire with 4 edges.
- Verify corner vertex positions match expected values.

**2. `test_try_sweep_rail_insufficient_sections`**:
- Call with n_sections = 1.
- Verify it returns `Err(Error::InsufficientSections { required: 2, got: 1 })`.

**3. `test_try_birail_basic`**:
- Create two diverging rail curves and a connecting profile.
- Call `builder::try_birail`.
- Verify Ok result, face topology is valid (1 boundary, 4 edges).

**4. `test_try_gordon_basic`**:
- Create a 2x2 curve network with matching intersection points (bilinear patch).
- Call `builder::try_gordon`.
- Verify Ok result, face has correct topology.

**5. `test_try_gordon_dimension_mismatch`**:
- Pass mismatched grid dimensions (e.g., 2 u-curves but points has 3 rows).
- Verify `Err(Error::GridDimensionMismatch { .. })`.

**6. `test_try_sweep_multi_rail_three_rails`**:
- Create 3 rails and a profile.
- Call `builder::try_sweep_multi_rail`.
- Verify Ok result.
- Verify the face has correct topology.

**7. `test_try_sweep_periodic_closed_seam`**:
- Create a circular-ish rail and a small profile.
- Call `builder::try_sweep_periodic`.
- Verify result is Ok and returns a Shell.
- Extract the face's surface and verify `subs(u, 0.0).near(&subs(u, 1.0))` for several u values.

**8. `test_try_sweep_periodic_euler_poincare`**:
- Build a periodic sweep shell.
- Call `shell.is_geometric_consistent()` (this verifies Euler-Poincare topology checks).
- This satisfies success criterion 3 from the roadmap.

Use `assert_near!` / `assert_near2!` from the geometry prelude for geometric comparisons. Use `Tolerance::near` trait for point near-equality checks.
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-modeling -E 'test(surface_constructor)'` to verify all new tests pass. Run `cargo clippy -p monstertruck-modeling --all-targets -- -W warnings` for lint check.
  </verify>
  <done>Eight integration tests added covering all builder wrappers, error paths, topology consistency, and Euler-Poincare checks.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-modeling -E 'test(surface_constructor)'` passes all 8 tests
2. `cargo nextest run -p monstertruck-modeling` -- all existing tests remain green
3. `cargo clippy -p monstertruck-modeling --all-targets -- -W warnings` shows no warnings
4. At least one test verifies is_geometric_consistent() on a shell built from periodic sweep (Euler-Poincare)
5. Error variants are specific and typed (not string-based panics)
6. try_sweep_periodic returns Result<Shell> consistently in both code and tests
</verification>

<success_criteria>
- Builder wrappers for sweep_rail, birail, gordon exist in monstertruck-modeling returning typed Result errors
- try_sweep_multi_rail and try_sweep_periodic wrappers use the new geometry methods from Plan 11-1
- try_sweep_periodic returns Result<Shell> (not Face) for closed surface topology
- Generated surfaces pass topology checks (is_geometric_consistent) when tested
- At least one periodic sweep test demonstrates closed-surface continuity at the wrap seam
- All builder wrappers return meaningful Error variants on invalid input
</success_criteria>

<output>
After completion, create `.tendrion/phases/11-surface-constructors/11-2-SUMMARY.md`
</output>
