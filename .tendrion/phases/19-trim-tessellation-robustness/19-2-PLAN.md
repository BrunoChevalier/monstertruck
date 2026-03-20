---
phase: 19-trim-tessellation-robustness
plan: 2
type: execute
wave: 2
depends_on: ["19-1"]
files_modified:
  - monstertruck-meshing/Cargo.toml
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/tests/tessellation/triangulation.rs
autonomous: true
must_haves:
  truths:
    - "When parameter search fails for a boundary point, PolyBoundaryPiece::try_new interpolates UV from neighbors instead of returning None"
    - "Faces that previously dropped due to parameter search failures now produce a PolygonMesh"
    - "The fallback UV interpolation uses the nearest preceding and following successful UVs as linear interpolation anchors"
    - "A log::warn! message is emitted each time fallback UV interpolation activates"
    - "All existing tessellation tests continue to pass"
  artifacts:
    - path: "monstertruck-meshing/src/tessellation/triangulation.rs"
      provides: "PolyBoundaryPiece::try_new with UV interpolation fallback on parameter search failure and logging"
      min_lines: 100
      contains: "fallback"
    - path: "monstertruck-meshing/tests/tessellation/triangulation.rs"
      provides: "Integration test verifying robust_triangulation recovers faces that regular triangulation drops"
      min_lines: 150
      contains: "fallback_recovers"
  key_links:
    - from: "monstertruck-meshing/src/tessellation/triangulation.rs"
      to: "monstertruck-meshing/src/tessellation/mod.rs"
      via: "PolyBoundaryPiece used by shell_create_polygon and cshell_tessellation"
      pattern: "PolyBoundaryPiece::try_new"
---

<objective>
Add fallback boundary projection in PolyBoundaryPiece::try_new so that when parameter search fails for individual boundary points, UV coordinates are interpolated from neighbors rather than the entire face being silently dropped. Include observability via log::warn! when fallback fires.
</objective>

<execution_context>
@skills/state-management/SKILL.md
</execution_context>

<context>
@monstertruck-meshing/src/tessellation/triangulation.rs
@monstertruck-meshing/src/tessellation/mod.rs
@monstertruck-meshing/Cargo.toml
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add log dependency and implement UV interpolation fallback in PolyBoundaryPiece::try_new</name>
  <files>monstertruck-meshing/Cargo.toml, monstertruck-meshing/src/tessellation/triangulation.rs</files>
  <action>
**Step 1: Add `log` dependency to monstertruck-meshing/Cargo.toml.**

Add `log = { workspace = true }` to the `[dependencies]` section (the `log` crate is already defined in the workspace root Cargo.toml as version "0.4").

**Step 2: Modify `PolyBoundaryPiece::try_new` (starting at line 302 of triangulation.rs).**

Replace the current implementation with a two-pass approach that recovers from partial parameter search failures. The current code (lines 302-368) uses `flat_map` + `collect::<Option<Vec<SurfacePoint>>>()` which means a single `None` from `sp()` drops the entire face.

Replace with:

```rust
fn try_new<S: PreMeshableSurface>(
    surface: &S,
    wire: impl Iterator<Item = PolylineCurve>,
    sp: impl SP<S>,
) -> Option<Self> {
    let (up, vp) = (surface.u_period(), surface.v_period());
    let (urange, vrange) = surface.try_range_tuple();
    let mut bdry3d: Vec<Point3> = wire
        .flat_map(|poly_edge| {
            let n = poly_edge.len() - 1;
            poly_edge.into_iter().take(n)
        })
        .collect();
    bdry3d.push(bdry3d[0]);

    // Pass 1: attempt parameter search for each boundary point.
    // Store Option<(f64, f64)> for each point; None means search failed.
    let mut uv_results: Vec<Option<(f64, f64)>> = Vec::with_capacity(bdry3d.len());
    let mut previous = None;
    let mut fail_count = 0usize;
    for pt in &bdry3d {
        let result = sp(surface, *pt, previous);
        if let Some((mut u, mut v)) = result {
            if let (Some(up), Some((u0, _))) = (up, previous) {
                u = get_mindiff(u, u0, up);
            }
            if let (Some(vp), Some((_, v0))) = (vp, previous) {
                v = get_mindiff(v, v0, vp);
            }
            previous = Some((u, v));
            uv_results.push(Some((u, v)));
        } else {
            fail_count += 1;
            uv_results.push(None);
        }
    }

    // If all lookups failed, we cannot recover -- return None.
    if uv_results.iter().all(|r| r.is_none()) {
        return None;
    }

    // Pass 2: fill in failed lookups via UV interpolation fallback.
    // Strategy: for each None entry, find the nearest preceding and
    // following successful UV (wrapping around the boundary loop) and
    // linearly interpolate based on index distance.
    if fail_count > 0 {
        log::warn!(
            "PolyBoundaryPiece: parameter search failed for {fail_count}/{} boundary points, \
             using UV interpolation fallback",
            uv_results.len(),
        );

        // Collect indices of None entries first to avoid borrow conflicts.
        let none_indices: Vec<usize> = uv_results
            .iter()
            .enumerate()
            .filter_map(|(i, r)| if r.is_none() { Some(i) } else { None })
            .collect();

        let n = uv_results.len();
        for i in none_indices {
            // Find nearest preceding success (wrapping around).
            let mut prev_anchor = None;
            for offset in 1..n {
                let j = (i + n - offset) % n;
                if let Some(uv) = uv_results[j] {
                    prev_anchor = Some((uv, offset));
                    break;
                }
            }
            // Find nearest following success (wrapping around).
            let mut next_anchor = None;
            for offset in 1..n {
                let j = (i + offset) % n;
                if let Some(uv) = uv_results[j] {
                    next_anchor = Some((uv, offset));
                    break;
                }
            }
            // Interpolate UV -- fallback to nearest if only one neighbor exists.
            let interpolated_uv = match (prev_anchor, next_anchor) {
                (Some(((pu, pv), pd)), Some(((nu, nv), nd))) => {
                    let t = pd as f64 / (pd + nd) as f64;
                    (pu + t * (nu - pu), pv + t * (nv - pv))
                }
                (Some(((pu, pv), _)), None) => (pu, pv),
                (None, Some(((nu, nv), _))) => (nu, nv),
                (None, None) => unreachable!("checked all-None case above"),
            };
            uv_results[i] = Some(interpolated_uv);
        }
    }

    // Build surface points, handling singularity crossings as before.
    previous = None;
    let mut vec: Vec<SurfacePoint> = Vec::with_capacity(uv_results.len());
    for (pt, uv_opt) in bdry3d.iter().zip(uv_results.iter()) {
        let (u, v) = uv_opt.expect("all UV values filled by pass 2");
        let entries: Vec<SurfacePoint> = (|| {
            if let Some((u0, v0)) = previous {
                if !u0.near(&u) && surface.uder(u0, v0).so_small() {
                    return vec![
                        (Point2::new(u, v0), *pt).into(),
                        (Point2::new(u, v), *pt).into(),
                    ];
                } else if !v0.near(&v) && surface.vder(u0, v0).so_small() {
                    return vec![
                        (Point2::new(u0, v), *pt).into(),
                        (Point2::new(u, v), *pt).into(),
                    ];
                }
            }
            vec![(Point2::new(u, v), *pt).into()]
        })();
        previous = Some((u, v));
        vec.extend(entries);
    }

    // Normalize periodic coordinates.
    let grav = vec.iter().fold(Point2::origin(), |g, p| g + p.uv.to_vec()) / vec.len() as f64;
    if let (Some(up), Some((u0, _))) = (up, urange) {
        let quot = f64::floor((grav.x - u0) / up);
        vec.iter_mut().for_each(|p| p.x -= quot * up);
    }
    if let (Some(vp), Some((v0, _))) = (vp, vrange) {
        let quot = f64::floor((grav.y - v0) / vp);
        vec.iter_mut().for_each(|p| p.y -= quot * vp);
    }
    let last = *vec.last().expect("boundary vec is non-empty");
    if !vec[0].near(&last) {
        let (u0, v0) = (last.uv[0], last.uv[1]);
        if surface.uder(u0, v0).so_small() || surface.vder(u0, v0).so_small() {
            vec.push(vec[0]);
        }
    }
    Some(Self(vec))
}
```

Key design decisions:
- The function still returns `None` when ALL points fail parameter search (no basis for interpolation).
- When only some points fail, UV is linearly interpolated between the nearest successful neighbors. This is geometrically reasonable because boundary polyline points are densely spaced.
- `log::warn!` is emitted when fallback activates, reporting the count of failed points vs total for observability.
- None indices are gathered into a Vec first (addressing S2 review finding), avoiding cascade where a freshly-interpolated value might be used as an anchor for a subsequent None. Each interpolation uses only original pass-1 successful values.
- The singularity-crossing logic (uder/vder checks) is preserved identically.
- The periodic normalization logic is preserved identically.
  </action>
  <verify>Run `cargo check -p monstertruck-meshing` to verify compilation. Run `cargo test -p monstertruck-meshing` to verify all existing tests pass.</verify>
  <done>PolyBoundaryPiece::try_new now interpolates UV from neighbors when parameter search fails, with log::warn! observability and documented cascade-safe interpolation.</done>
</task>

<task type="auto">
  <name>Task 2: Add unit tests for fallback UV interpolation</name>
  <files>monstertruck-meshing/src/tessellation/triangulation.rs</files>
  <action>
Add tests in the existing `#[cfg(test)]` module at the bottom of `triangulation.rs` (or create one if not present) to verify the fallback behavior.

The tests should verify:
1. `try_new` still returns `None` when ALL parameter searches fail.
2. `try_new` returns `Some` when only some parameter searches fail, producing interpolated UVs for the failed points.
3. The existing behavior is preserved when all parameter searches succeed.

Use `BSplineSurface` (available via `monstertruck_geometry::prelude::*` already imported in the test module) NOT `Plane` which is not available in monstertruck-meshing. Create a flat BSplineSurface for a simple test surface.

Look at the existing test code at the bottom of triangulation.rs (around line 1425+) to match patterns. The `by_search_parameter` function is available in the test module.

```rust
#[test]
fn try_new_fallback_partial_failure() {
    use monstertruck_geometry::prelude::*;

    // Build a simple flat BSplineSurface in xy-plane
    let knots = KnotVec::bezier_knot(1);
    let ctrl = vec![
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    ];
    let surface = BSplineSurface::new((knots.clone(), knots), ctrl);

    // Create boundary points forming a square
    let pts = vec![
        Point3::new(0.1, 0.1, 0.0),
        Point3::new(0.9, 0.1, 0.0),
        Point3::new(0.9, 0.9, 0.0),
        Point3::new(0.1, 0.9, 0.0),
    ];
    let poly = PolylineCurve(pts);

    // SP closure that fails for the third point (index 2)
    let call_count = std::cell::Cell::new(0usize);
    let sp = |surface: &BSplineSurface<Point3>, pt: Point3, hint: Option<(f64, f64)>| -> Option<(f64, f64)> {
        let idx = call_count.get();
        call_count.set(idx + 1);
        if idx == 2 {
            None
        } else {
            by_search_parameter(surface, pt, hint)
        }
    };

    let result = PolyBoundaryPiece::try_new(&surface, std::iter::once(poly), sp);
    assert!(result.is_some(), "should recover from partial failure via UV interpolation");
    let piece = result.unwrap();
    assert!(!piece.0.is_empty());
}

#[test]
fn try_new_all_failures_returns_none() {
    use monstertruck_geometry::prelude::*;

    let knots = KnotVec::bezier_knot(1);
    let ctrl = vec![
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)],
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
    ];
    let surface = BSplineSurface::new((knots.clone(), knots), ctrl);

    let pts = vec![
        Point3::new(0.1, 0.1, 0.0),
        Point3::new(0.9, 0.1, 0.0),
    ];
    let poly = PolylineCurve(pts);

    // SP that always fails
    let sp = |_: &BSplineSurface<Point3>, _: Point3, _: Option<(f64, f64)>| -> Option<(f64, f64)> { None };

    let result = PolyBoundaryPiece::try_new(&surface, std::iter::once(poly), sp);
    assert!(result.is_none(), "should return None when all lookups fail");
}
```

Adapt the test to use whatever exact surface types and KnotVec/KnotVector constructors are available. Check if the type is `KnotVec` or `KnotVector` and whether the constructor is `bezier_knot` or similar by examining existing imports and usage in the test module.
  </action>
  <verify>Run `cargo test -p monstertruck-meshing try_new_fallback` and `cargo test -p monstertruck-meshing try_new_all_failures` to verify both tests pass.</verify>
  <done>Unit tests added verifying fallback UV interpolation produces valid output for partial failures and returns None for total failures.</done>
</task>

<task type="auto">
  <name>Task 3: Add integration test verifying robust_triangulation recovers previously-dropped faces</name>
  <files>monstertruck-meshing/tests/tessellation/triangulation.rs</files>
  <action>
Add an integration test to `monstertruck-meshing/tests/tessellation/triangulation.rs` that validates the fallback improves face recovery. The test uses the existing `robust_closed` test pattern (already in the file at line 138) which constructs a cube with curved edges where `triangulation()` fails but `robust_triangulation()` succeeds.

The test must use a before/after comparison approach: tessellate a shape with both `triangulation()` and `robust_triangulation()`, then verify that `robust_triangulation` produces at least as many (or more) successfully meshed faces.

```rust
#[test]
fn fallback_recovers_faces_robust_vs_regular() {
    // Use the curved-edge cube from robust_closed -- triangulation() drops
    // all faces (returns None surfaces) but robust_triangulation() recovers them.
    let cube: Solid = {
        let v = builder::vertex(Point3::origin());
        let e = builder::extrude(&v, Vector3::unit_x());
        let f = builder::extrude(&e, Vector3::unit_y());
        builder::extrude(&f, Vector3::unit_z())
    };

    let o = Point3::new(0.5, 0.5, 0.5);
    cube.edge_iter().for_each(|edge| {
        let curve = edge.curve();
        if let Curve::Line(line) = curve {
            let m = line.subs(0.5);
            let p = m + 0.2 * (o - m);
            let bsp = BsplineCurve::new(KnotVector::bezier_knot(2), vec![line.0, p, line.1]);
            edge.set_curve(bsp.into());
        }
    });

    // Regular triangulation drops all faces on this model.
    let regular = cube.triangulation(0.01);
    let regular_meshed_count = regular
        .face_iter()
        .filter(|face| face.surface().is_some())
        .count();

    // Robust triangulation should recover faces via search_nearest_parameter
    // and now also via the UV interpolation fallback.
    let robust = cube.robust_triangulation(0.01);
    let robust_meshed_count = robust
        .face_iter()
        .filter(|face| face.surface().is_some())
        .count();

    // robust must recover strictly more faces than regular on this fixture.
    assert!(
        robust_meshed_count > regular_meshed_count,
        "robust_triangulation should recover more faces: robust={robust_meshed_count}, regular={regular_meshed_count}"
    );

    // Verify the robust mesh is actually closed (all faces recovered).
    let mut mesh = robust.to_polygon();
    mesh.put_together_same_attrs(TOLERANCE * 2.0)
        .remove_unused_attrs();
    assert_eq!(mesh.shell_condition(), ShellCondition::Closed);
}
```

This test:
- Uses a known fixture (curved-edge cube) where regular triangulation fails.
- Compares face counts between regular and robust triangulation (before/after approach per B2 finding).
- Verifies that robust_triangulation produces a closed mesh.
- Uses only types already imported in the test file (Solid, builder, BsplineCurve, KnotVector, Curve, etc. from `monstertruck_modeling::*`).
  </action>
  <verify>Run `cargo test -p monstertruck-meshing --test tessellation fallback_recovers` to verify the test passes.</verify>
  <done>Integration test added that verifies robust_triangulation recovers faces that regular triangulation drops, using before/after comparison on a curved-edge cube fixture.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-meshing` compiles successfully (including log dependency)
2. `cargo test -p monstertruck-meshing` passes all tests including new ones
3. `cargo test -p monstertruck-meshing --test tessellation` passes all existing and new tests
4. The fallback unit test confirms partial failure recovery works
5. The all-failures test confirms None is still returned when no UV data is available
6. The integration test confirms robust_triangulation recovers more faces than regular triangulation
7. `grep "log::warn" monstertruck-meshing/src/tessellation/triangulation.rs` shows the fallback logging
</verification>

<success_criteria>
- TRIM-01 satisfied: PolyBoundaryPiece::try_new falls back to UV interpolation from neighbors when parameter search fails
- Fallback activations are observable via log::warn! messages reporting failure counts
- Previously-dropped trimmed faces now tessellate successfully on the curved-edge cube fixture
- The fallback only activates when needed -- no behavior change for points where parameter search succeeds
- All existing tests continue to pass
</success_criteria>

<output>
After completion, create `.tendrion/phases/19-trim-tessellation-robustness/19-2-SUMMARY.md`
</output>
