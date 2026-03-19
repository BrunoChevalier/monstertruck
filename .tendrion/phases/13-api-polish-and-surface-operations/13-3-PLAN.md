---
phase: 13-api-polish-and-surface-operations
plan: 3
type: execute
wave: 2
depends_on: ["13-1"]
files_modified:
  - monstertruck-modeling/src/builder.rs
  - monstertruck-modeling/src/errors.rs
  - monstertruck-modeling/src/lib.rs
autonomous: true
must_haves:
  truths:
    - "User calls builder::try_sweep_rail_with_options passing a SweepRailOptions and gets a Face with proper topology"
    - "User calls builder::try_birail_with_options passing Birail1Options and gets a Face"
    - "User calls builder::try_gordon_with_options passing GordonOptions and gets a Face"
    - "User providing invalid curve networks to modeling-layer builders receives actionable error messages with diagnostic details"
    - "Existing try_sweep_rail, try_birail, try_gordon functions continue to work (backward compatible)"
    - "Modeling error variants wrap geometry-level CurveNetworkDiagnostic for full error chain visibility"
  artifacts:
    - path: "monstertruck-modeling/src/builder.rs"
      provides: "Option-struct-based builder functions: try_sweep_rail_with_options, try_birail_with_options, try_birail2_with_options, try_gordon_with_options"
      min_lines: 700
      contains: "try_sweep_rail_with_options"
    - path: "monstertruck-modeling/src/errors.rs"
      provides: "Extended modeling Error with CurveNetworkDiagnostic wrapping"
      min_lines: 130
      contains: "CurveNetworkDiagnostic"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/surface_options.rs"
      to: "monstertruck-modeling/src/builder.rs"
      via: "Builder functions accept option structs from geometry crate"
      pattern: "SweepRailOptions"
    - from: "monstertruck-geometry/src/errors.rs"
      to: "monstertruck-modeling/src/errors.rs"
      via: "Modeling errors wrap geometry CurveNetworkIncompatible errors"
      pattern: "CurveNetworkIncompatible"
    - from: "monstertruck-modeling/src/builder.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "Builder calls BsplineSurface::try_sweep_rail etc. with option structs"
      pattern: "try_sweep_rail"
---

<objective>
Wire the geometry-level option structs and diagnostic errors into the monstertruck-modeling builder layer, providing option-based builder functions with full diagnostic error propagation, while keeping existing builder APIs backward-compatible.
</objective>

<execution_context>
@skills/state-management/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-modeling/src/builder.rs
@monstertruck-modeling/src/errors.rs
@monstertruck-geometry/src/nurbs/surface_options.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Extend modeling errors with geometry diagnostic wrapping</name>
  <files>monstertruck-modeling/src/errors.rs</files>
  <action>
Add a new variant to `monstertruck_modeling::errors::Error` that wraps geometry-level curve network diagnostics:

```rust
/// Curve network is incompatible — wraps detailed geometry-level diagnostics.
#[error("curve network diagnostic: {0}")]
CurveNetworkDiagnostic(#[from] monstertruck_geometry::errors::Error),
```

Or, if `#[from]` on `monstertruck_geometry::errors::Error` conflicts (because it's a broad type), add a more specific variant:

```rust
/// Geometry-level error during surface construction.
#[error(transparent)]
FromGeometry(monstertruck_geometry::errors::Error),
```

With a manual `From` impl:
```rust
impl From<monstertruck_geometry::errors::Error> for Error {
    fn from(e: monstertruck_geometry::errors::Error) -> Self {
        Error::FromGeometry(e)
    }
}
```

This allows `?` propagation from geometry `try_*` calls in the builder. Check if PartialEq on the modeling Error is an issue — if monstertruck_geometry::errors::Error doesn't derive PartialEq for all new variants (e.g., String fields), adjust accordingly (the modeling Error already has `PartialEq, Eq` — may need to change to just `PartialEq` or remove `Eq` if geometry errors contain Strings).

Update the `print_messages` test to include the new variant.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling -E 'test(print_messages)'` to verify the error test passes.</verify>
  <done>Modeling error enum extended with geometry error wrapping, enabling `?` propagation from geometry try_* methods.</done>
</task>

<task type="auto">
  <name>Task 2: Add option-struct-based builder functions</name>
  <files>monstertruck-modeling/src/builder.rs</files>
  <action>
Add new builder functions alongside the existing ones:

```rust
/// Sweeps a profile along a single rail with options, returning a Face.
///
/// Uses the geometry-level `try_sweep_rail` with full diagnostic errors.
pub fn try_sweep_rail_with_options(
    profile: &BsplineCurve<Point3>,
    rail: &BsplineCurve<Point3>,
    options: &SweepRailOptions,
) -> Result<Face<Curve, Surface>> {
    let surface = BsplineSurface::try_sweep_rail(profile.clone(), rail, options)?;
    Ok(face_from_bspline_surface(surface))
}
```

Similarly:
- `try_birail_with_options(profile, rail1, rail2, options: &Birail1Options) -> Result<Face<Curve, Surface>>`
- `try_birail2_with_options(profile1, profile2, rail1, rail2, options: &Birail2Options) -> Result<Face<Curve, Surface>>`
- `try_gordon_with_options(u_curves, v_curves, points, options: &GordonOptions) -> Result<Face<Curve, Surface>>`

Each function:
1. Calls the corresponding geometry-level `try_*` method with the option struct.
2. Uses `?` to propagate geometry errors (which auto-convert via `From`).
3. Wraps the resulting BsplineSurface into a Face via `face_from_bspline_surface`.

Rewrite the existing `try_sweep_rail`, `try_birail`, `try_gordon` to delegate to the new `*_with_options` variants (constructing a default options struct internally), so the old APIs remain but use the improved error handling under the hood. Mark them with `#[deprecated]` annotations.

Add re-exports of option struct types via `monstertruck_modeling::prelude` or the crate root so users don't need to import from geometry directly. Check `monstertruck-modeling/src/lib.rs` for the appropriate re-export location.

Add doc-tests for each new function showing both success and error cases.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --lib` to verify all tests pass. Run `cargo clippy -p monstertruck-modeling -- -W warnings`.</verify>
  <done>Option-struct-based builder functions added with error propagation, old APIs preserved as deprecated delegates.</done>
</task>

<task type="auto">
  <name>Task 3: Integration test for full diagnostic error chain</name>
  <files>monstertruck-modeling/src/builder.rs</files>
  <action>
Add integration-style tests (in `#[cfg(test)]` module at the bottom of builder.rs or in a separate test file) that verify:

1. **Diagnostic propagation**: Call `try_gordon_with_options` with mismatched grid dimensions and verify the error is `Error::FromGeometry(monstertruck_geometry::errors::Error::CurveNetworkIncompatible(CurveNetworkDiagnostic::GridDimensionMismatch { .. }))`. Assert that the error message contains the specific dimension numbers.

2. **Insufficient sections**: Call `try_sweep_rail_with_options` with `n_sections: 1` and verify the error chain.

3. **Endpoint mismatch (birail)**: Create a profile whose start doesn't match rail1's start, call `try_birail_with_options`, and verify CurveNetworkDiagnostic::EndpointMismatch is in the error.

4. **Success path**: Verify that the happy path with valid inputs produces correct topology (4-edge wire boundary).

These tests ensure the full error chain from geometry through modeling is working correctly and that users get actionable diagnostics.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --lib` to verify all tests pass.</verify>
  <done>Integration tests verify full diagnostic error chain from geometry to modeling layer.</done>
</task>

</tasks>

<verification>
1. Option-struct-based builder functions compile and produce correct Face topology
2. Error propagation from geometry to modeling layer works via `?` operator
3. Diagnostic error messages contain specific details (dimension numbers, distances, curve indices)
4. Existing try_sweep_rail, try_birail, try_gordon continue to work without breaking changes
5. `cargo nextest run -p monstertruck-modeling --lib` passes all tests
6. `cargo clippy -p monstertruck-modeling -- -W warnings` shows no new warnings
</verification>

<success_criteria>
- API-01 (modeling layer): Builder functions accept typed option structs for surface constructors
- API-02 (modeling layer): Invalid curve networks produce actionable diagnostics with full error chain from geometry through modeling
- Backward compatibility maintained for all existing builder functions
</success_criteria>

<output>
After completion, create `.tendrion/phases/13-api-polish-and-surface-operations/13-3-SUMMARY.md`
</output>
