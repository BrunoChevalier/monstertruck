---
phase: 20-fixture-corpus-and-migration-documentation
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/lib.rs
autonomous: true
must_haves:
  truths:
    - "User reads doc comments on try_skin and sees a before/after migration example"
    - "User reads doc comments on try_sweep_rail and sees a before/after migration example"
    - "User reads doc comments on try_birail1 and sees a before/after migration example"
    - "User reads doc comments on try_birail2 and sees a before/after migration example"
    - "User reads doc comments on try_gordon and sees a before/after migration example"
    - "User reads crate-level docs on monstertruck-geometry and finds migration guidance section"
    - "Running cargo doc --no-deps -p monstertruck-geometry succeeds without warnings"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      provides: "Doc comments on all try_* functions with before/after migration examples"
      min_lines: 2600
      contains: "Migration"
    - path: "monstertruck-geometry/src/lib.rs"
      provides: "Crate-level migration guidance section"
      min_lines: 50
      contains: "Migration"
  key_links:
    - from: "monstertruck-geometry/src/lib.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "crate docs reference to try_* functions"
      pattern: "try_skin"
---

<objective>
Add migration guidance as doc comments on all try_* surface constructor functions and crate-level docs, with before/after examples showing the deprecated API vs. the new fallible API.
</objective>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs
@monstertruck-geometry/src/lib.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add migration doc comments to try_* surface constructor functions</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Enhance the doc comments on each of the following functions with a `# Migration` section containing before/after code examples. The migration section should appear after the existing `# Errors` section (or after the main description if no Errors section exists) and before any existing examples.

For each function, add a doc section like:

```rust
/// # Migration
///
/// **Before** (deprecated, panics on failure):
/// ```ignore
/// let surface = BsplineSurface::old_name(args...);
/// ```
///
/// **After** (fallible, with error handling):
/// ```ignore
/// use monstertruck_geometry::nurbs::surface_options::OptionsType;
/// let result = BsplineSurface::try_new_name(args..., &OptionsType::default());
/// match result {
///     Ok(surface) => { /* use surface */ }
///     Err(e) => eprintln!("Construction failed: {e}"),
/// }
/// ```
```

Functions to update (with their deprecated counterparts):

1. **`try_skin`** -- deprecated: `skin(curves)` -> new: `try_skin(curves, &SkinOptions::default())`
   Show how `SkinOptions` can be customized. Mention the error types that can be returned.

2. **`try_sweep_rail`** -- deprecated: `sweep_rail(profile, &rail, n_sections)` -> new: `try_sweep_rail(profile, &rail, &SweepRailOptions { n_sections, ..Default::default() })`

3. **`try_birail1`** -- deprecated: `birail1(profile, &rail1, &rail2, n_sections)` -> new: `try_birail1(profile, &rail1, &rail2, &Birail1Options { n_sections, ..Default::default() })`

4. **`try_birail2`** -- deprecated: `birail2(profile1, profile2, &rail1, &rail2, n_sections)` -> new: `try_birail2(profile1, profile2, &rail1, &rail2, &Birail2Options { n_sections, ..Default::default() })`

5. **`try_gordon`** -- deprecated: `gordon(u_curves, v_curves, &points)` -> new: `try_gordon(u_curves, v_curves, &points, &GordonOptions::default())`

6. **`try_gordon_from_network`** -- new function (no deprecated counterpart). Add a usage example and note that this is the recommended approach for Gordon surfaces when grid points are not pre-computed.

7. **`try_gordon_verified`** -- new function (no deprecated counterpart). Add a usage example showing how caller-supplied grid points are validated.

Also enhance the `#[deprecated]` annotation on each deprecated function to include a brief migration hint in the `note` field if not already present. The existing notes like `"use try_skin with SkinOptions"` are sufficient.

Use `/// ``` ignore` (or `/// ```no_run`) for examples that reference `truck_geometry` or types that are in the prelude, since doctests may not have the full context. Alternatively, use valid imports if the prelude re-exports what's needed.
  </action>
  <verify>Run `cargo doc --no-deps -p monstertruck-geometry 2>&1 | head -20` and verify no doc warnings. Optionally inspect the generated HTML to verify migration sections appear.</verify>
  <done>All seven try_* functions have Migration doc sections with before/after examples.</done>
</task>

<task type="auto">
  <name>Task 2: Add crate-level migration guidance to lib.rs</name>
  <files>monstertruck-geometry/src/lib.rs</files>
  <action>
Add a crate-level documentation section to `monstertruck-geometry/src/lib.rs` with migration guidance. Insert it after the existing crate-level doc comment (`//! Geometric primitives for CAD modeling...`) and before the `#![cfg_attr...]` line.

The section should cover:

```rust
//!
//! # Migration Guide (v0.5.0+)
//!
//! Surface constructor functions have been migrated from panicking APIs to
//! fallible `try_*` variants that return `Result`. The old functions are
//! deprecated and will be removed in a future version.
//!
//! ## Quick Reference
//!
//! | Deprecated | Replacement | Options type |
//! |-----------|-------------|-------------|
//! | `BsplineSurface::skin` | [`BsplineSurface::try_skin`] | [`SkinOptions`](nurbs::surface_options::SkinOptions) |
//! | `BsplineSurface::sweep_rail` | [`BsplineSurface::try_sweep_rail`] | [`SweepRailOptions`](nurbs::surface_options::SweepRailOptions) |
//! | `BsplineSurface::birail1` | [`BsplineSurface::try_birail1`] | [`Birail1Options`](nurbs::surface_options::Birail1Options) |
//! | `BsplineSurface::birail2` | [`BsplineSurface::try_birail2`] | [`Birail2Options`](nurbs::surface_options::Birail2Options) |
//! | `BsplineSurface::gordon` | [`BsplineSurface::try_gordon`] | [`GordonOptions`](nurbs::surface_options::GordonOptions) |
//!
//! ## New Functions (no deprecated counterpart)
//!
//! - [`BsplineSurface::try_gordon_from_network`]: Builds a Gordon surface by
//!   auto-computing intersection grid points from the curve network.
//! - [`BsplineSurface::try_gordon_verified`]: Builds a Gordon surface from
//!   caller-supplied grid points after validating each lies on both curves.
//!
//! ## Before / After Example
//!
//! ```ignore
//! // BEFORE (panics on failure):
//! let surface = BsplineSurface::skin(curves);
//!
//! // AFTER (returns Result):
//! use monstertruck_geometry::nurbs::surface_options::SkinOptions;
//! let surface = BsplineSurface::try_skin(curves, &SkinOptions::default())?;
//! ```
//!
//! All `try_*` functions return [`errors::Error`] with detailed diagnostics
//! via [`nurbs::surface_diagnostics::CurveNetworkDiagnostic`].
```

Ensure the crate doc renders correctly with `cargo doc`.
  </action>
  <verify>Run `cargo doc --no-deps -p monstertruck-geometry 2>&1 | head -20` and verify no doc warnings. Check that the migration guide table renders in the crate root docs.</verify>
  <done>Crate-level migration guidance added to lib.rs with quick reference table and before/after examples.</done>
</task>

</tasks>

<verification>
1. `cargo doc --no-deps -p monstertruck-geometry` succeeds without warnings
2. Each try_* function has a `# Migration` section in its doc comment
3. Crate-level docs contain a migration guide with a deprecated-to-replacement table
4. Before/after code examples are present in both crate-level and function-level docs
5. `cargo nextest run -p monstertruck-geometry` still passes (no regressions from doc changes)
</verification>

<success_criteria>
- Doc comments on all try_* functions include before/after migration examples (DOC-01)
- Crate-level docs include migration guidance with quick reference table (DOC-01)
- All existing tests still pass after documentation changes
</success_criteria>

<output>
After completion, create `.tendrion/phases/20-fixture-corpus-and-migration-documentation/20-2-SUMMARY.md`
</output>
