# Migration Guide

## Upgrading to v0.5.3

### Overview

v0.5.3 focuses on API consistency, expanded surface construction, and improved test
reliability. All changes are additive or deprecation-based -- no existing public API
has been removed. Deprecated items still compile with warnings and will be removed in
v0.6.0.

Key improvements:

- RFC 430 (C-CASE) type renames across all crates.
- Fallible surface constructors (`try_*`) with typed option structs replace panicking
  positional-parameter APIs.
- New surface construction methods: ruled surfaces, Gordon surfaces from intersection
  grids, and geometry healing utilities.
- GPU test reliability on headless CI environments.
- I/O round-trip validation for STEP, OBJ, and STL formats.

### Breaking Changes

None. All changes in v0.5.3 are backward-compatible. Deprecated items remain
functional and will produce compiler warnings.

---

### Deprecated Function Replacements

#### Builder API Changes

| Deprecated | Replacement | Crate | Notes |
|---|---|---|---|
| `cone(wire, axis, angle, division)` | `revolve_wire(wire, origin, axis, angle, division)` | `monstertruck-modeling` | Now takes an explicit `origin` parameter; handles on-axis degenerate edges automatically. |

**Before:**

```rust
use monstertruck_modeling::builder::cone;

#[allow(deprecated)]
let shell = cone(&wire, axis, angle, division);
```

**After:**

```rust
use monstertruck_modeling::builder::revolve_wire;

let origin = wire.front_vertex().map_or(Point3::origin(), |v| v.point());
let shell = revolve_wire(&wire, origin, axis, angle, division);
```

#### Surface Constructor Changes

Old positional-parameter methods on `BsplineSurface` are deprecated in favor of
fallible `try_*` variants that accept typed option structs and return `Result`.

| Deprecated | Replacement | Options Struct |
|---|---|---|
| `BsplineSurface::skin(curves)` | `BsplineSurface::try_skin(curves, &options)` | `SkinOptions` |
| `BsplineSurface::sweep_rail(profile, rail, n)` | `BsplineSurface::try_sweep_rail(profile, rail, &options)` | `SweepRailOptions` |
| `BsplineSurface::birail1(profile, rail1, rail2, n)` | `BsplineSurface::try_birail1(profile, rail1, rail2, &options)` | `Birail1Options` |
| `BsplineSurface::birail2(p1, p2, rail1, rail2, n)` | `BsplineSurface::try_birail2(p1, p2, rail1, rail2, &options)` | `Birail2Options` |
| `BsplineSurface::gordon(u, v, pts)` | `BsplineSurface::try_gordon(u, v, pts, &options)` | `GordonOptions` |

**Before:**

```rust
// Panics on invalid input.
let surface = BsplineSurface::skin(curves);
let surface = BsplineSurface::sweep_rail(profile, &rail, 8);
let surface = BsplineSurface::gordon(u_curves, v_curves, &points);
```

**After:**

```rust
use monstertruck_geometry::nurbs::surface_options::*;

// Returns Result with descriptive errors.
let surface = BsplineSurface::try_skin(curves, &SkinOptions::default())?;
let surface = BsplineSurface::try_sweep_rail(
    profile, &rail, &SweepRailOptions { n_sections: 8, ..Default::default() },
)?;
let surface = BsplineSurface::try_gordon(
    u_curves, v_curves, &points, &GordonOptions::default(),
)?;
```

#### Curve Interpolation Renames

| Deprecated | Replacement | Crate |
|---|---|---|
| `BsplineCurve::interpole(knot_vec, pts)` | `BsplineCurve::interpolate(knot_vec, pts)` | `monstertruck-geometry` |
| `BsplineCurve::try_interpole(knot_vec, pts)` | `BsplineCurve::try_interpolate(knot_vec, pts)` | `monstertruck-geometry` |

**Before:**

```rust
let curve = BsplineCurve::interpole(knot_vec, points);
```

**After:**

```rust
let curve = BsplineCurve::interpolate(knot_vec, points);
```

#### Type Renames (RFC 430 C-CASE Convention)

All `SCREAMING_CASE` and inconsistent-case type aliases have been renamed to
`UpperCamelCase` per [RFC 430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md).
The old names remain as deprecated type aliases.

##### Core and Topology ID Types

| Deprecated | Replacement | Crate |
|---|---|---|
| `ID<T>` | `Id<T>` | `monstertruck-core` |
| `VertexID<P>` | `VertexId<P>` | `monstertruck-topology` |
| `EdgeID<C>` | `EdgeId<C>` | `monstertruck-topology` |
| `FaceID<S>` | `FaceId<S>` | `monstertruck-topology` |
| `RenderID` | `RenderId` | `monstertruck-gpu` |

**Before:**

```rust
use monstertruck_core::ID;
use monstertruck_topology::{VertexID, EdgeID, FaceID};
use monstertruck_gpu::RenderID;
```

**After:**

```rust
use monstertruck_core::Id;
use monstertruck_topology::{VertexId, EdgeId, FaceId};
use monstertruck_gpu::RenderId;
```

##### STEP Loader Types

| Deprecated | Replacement | Crate |
|---|---|---|
| `BSplineCurveForm` | `BsplineCurveForm` | `monstertruck-step` |
| `BSplineCurveWithKnots` | `BsplineCurveWithKnots` | `monstertruck-step` |
| `NonRationalBSplineCurve` | `NonRationalBsplineCurve` | `monstertruck-step` |
| `RationalBSplineCurve` | `RationalBsplineCurve` | `monstertruck-step` |
| `BSplineCurveAny` | `BsplineCurveAny` | `monstertruck-step` |
| `BSplineSurfaceAny` | `BsplineSurfaceAny` | `monstertruck-step` |
| `BSplineSurfaceForm` | `BsplineSurfaceForm` | `monstertruck-step` |
| `BSplineSurfaceWithKnots` | `BsplineSurfaceWithKnots` | `monstertruck-step` |
| `NonRationalBSplineSurface` | `NonRationalBsplineSurface` | `monstertruck-step` |
| `RationalBSplineSurface` | `RationalBsplineSurface` | `monstertruck-step` |
| `PCurve` | `Pcurve` | `monstertruck-step` |

**Before:**

```rust
use monstertruck_step::load::{BSplineCurveWithKnots, PCurve};
```

**After:**

```rust
use monstertruck_step::load::{BsplineCurveWithKnots, Pcurve};
```

##### Geometry Types

| Deprecated | Replacement | Crate |
|---|---|---|
| `BSplineCurve<P>` | `BsplineCurve<P>` | `monstertruck-geometry` |
| `BSplineSurface<P>` | `BsplineSurface<P>` | `monstertruck-geometry` |
| `KnotVec` | `KnotVector` | `monstertruck-geometry` |
| `PCurve<C, S>` | `ParameterCurve<C, S>` | `monstertruck-geometry` |

**Before:**

```rust
use monstertruck_geometry::prelude::{BSplineCurve, BSplineSurface, KnotVec};
```

**After:**

```rust
use monstertruck_geometry::prelude::{BsplineCurve, BsplineSurface, KnotVector};
```

#### Search Parameter Hints

| Deprecated | Replacement | Crate |
|---|---|---|
| `SPHint1D` | `SearchParameterHint1D` | `monstertruck-traits` |
| `SPHint2D` | `SearchParameterHint2D` | `monstertruck-traits` |

**Before:**

```rust
use monstertruck_traits::prelude::SPHint1D;

let hint = SPHint1D::Parameter(0.5);
```

**After:**

```rust
use monstertruck_traits::prelude::SearchParameterHint1D;

let hint = SearchParameterHint1D::Parameter(0.5);
```

---

### New API Patterns

#### Ruled Surface Construction

Construct a ruled surface between two boundary curves.

```rust
use monstertruck_geometry::nurbs::surface_options::RuledSurfaceOptions;

let surface = BsplineSurface::try_ruled(curve0, curve1, &RuledSurfaceOptions {})?;
```

#### Gordon Surface from Intersection Grid

Automatically compute intersection grid points from crossing curve networks instead
of supplying them manually.

```rust
use monstertruck_geometry::nurbs::surface_options::GordonOptions;

let surface = BsplineSurface::try_gordon_from_network(
    u_curves, v_curves, &GordonOptions::default(),
)?;
```

For cases where pre-computed grid points need validation against actual curve
intersections, use `try_gordon_verified`:

```rust
let surface = BsplineSurface::try_gordon_verified(
    u_curves, v_curves, &points, &GordonOptions::default(),
)?;
```

#### Geometry Healing

The `monstertruck-solid` crate provides topology healing utilities for solids loaded
from external STEP files:

- `SplitClosedEdgesAndFaces` -- split closed edges and faces that were imported as
  single entities from other CAD systems.
- `RobustSplitClosedEdgesAndFaces` -- a robust variant that handles edge cases in
  degenerate geometry.
- `extract_healed` -- extract a healed solid from a compressed representation.

#### GPU Test Skip on Headless CI

GPU and render tests now detect headless environments and skip gracefully using
`wgpu` adapter availability checks. No user action required -- tests that need a GPU
simply report as skipped when no adapter is found.

#### I/O Validation

STEP round-trip tests verify that export-then-reimport preserves geometry within
bounding-box tolerances. OBJ and STL format tests validate mesh output correctness.

---

### Version Upgrade Steps

1. **Update dependencies.** Set the monstertruck crate versions to `0.5.3` in your
   `Cargo.toml`.

2. **Build and review warnings.** Run `cargo build`. Deprecated items produce
   compiler warnings showing the replacement name.

3. **Rename ID types.** Find and replace the old SCREAMING_CASE / inconsistent-case
   type names:
   - `ID` -> `Id`
   - `VertexID` -> `VertexId`, `EdgeID` -> `EdgeId`, `FaceID` -> `FaceId`
   - `RenderID` -> `RenderId`
   - `BSplineCurve` -> `BsplineCurve`, `BSplineSurface` -> `BsplineSurface`
   - `KnotVec` -> `KnotVector`
   - `PCurve` -> `ParameterCurve` (geometry) or `Pcurve` (step)
   - `SPHint1D` -> `SearchParameterHint1D`, `SPHint2D` -> `SearchParameterHint2D`

4. **Replace deprecated STEP loader types.** Rename `BSplineCurveForm` ->
   `BsplineCurveForm`, and similarly for all types listed in the STEP Loader Types
   table above.

5. **Replace `builder::cone` with `builder::revolve_wire`.** The new function
   requires an explicit `origin` parameter. Use the wire's front vertex position or
   `Point3::origin()` as the origin.

6. **Replace `interpole` with `interpolate`.** Update calls to
   `BsplineCurve::interpole` and `BsplineCurve::try_interpole`.

7. **Migrate surface constructors.** Replace panicking surface constructors
   (`skin`, `sweep_rail`, `birail1`, `birail2`, `gordon`) with their `try_*`
   equivalents and handle the `Result` return type.

8. **Verify.** Run `cargo clippy --all-targets -- -W warnings` and confirm no
   remaining deprecation warnings.

---

### Compatibility Notes

- All deprecated items still compile in v0.5.3 with warnings.
- Deprecated items will be removed in v0.6.0.
- GPU/render tests now skip gracefully on headless machines -- no configuration
  changes needed.
- The `nom` parser is updated to v7+ and `quick-xml` to v0.30+ internally; these
  are transitive dependencies and require no user action.
