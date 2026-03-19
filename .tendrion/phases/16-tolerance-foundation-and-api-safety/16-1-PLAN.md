---
phase: 16-tolerance-foundation-and-api-safety
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-core/src/tolerance_constants.rs
  - monstertruck-core/src/lib.rs
  - monstertruck-core/src/tolerance.rs
  - monstertruck-solid/src/fillet/integrate.rs
  - monstertruck-solid/src/transversal/loops_store/mod.rs
  - monstertruck-solid/src/transversal/integrate/mod.rs
autonomous: true
must_haves:
  truths:
    - "User imports SNAP_TOLERANCE from monstertruck_core::tolerance_constants and gets 10.0 * TOLERANCE (1.0e-5)"
    - "User imports VERTEX_MERGE_TOLERANCE from monstertruck_core::tolerance_constants and gets 100.0 * TOLERANCE (1.0e-4)"
    - "User imports TESSELLATION_TOLERANCE from monstertruck_core::tolerance_constants and gets 0.01"
    - "User imports PERIODIC_CLOSURE_RATIO from monstertruck_core::tolerance_constants and gets 0.01"
    - "User imports G1_ANGLE_TOLERANCE from monstertruck_core::tolerance_constants and gets 0.0175"
    - "User imports G2_CURVATURE_TOLERANCE from monstertruck_core::tolerance_constants and gets 0.10"
    - "All existing tests pass with identical behavior after refactoring local constants to use centralized ones"
  artifacts:
    - path: "monstertruck-core/src/tolerance_constants.rs"
      provides: "Centralized tolerance constants module with documented defaults"
      min_lines: 30
      contains: "SNAP_TOLERANCE"
  key_links:
    - from: "monstertruck-core/src/tolerance_constants.rs"
      to: "monstertruck-core/src/lib.rs"
      via: "pub mod declaration and re-export"
      pattern: "pub mod tolerance_constants"
    - from: "monstertruck-core/src/tolerance_constants.rs"
      to: "monstertruck-solid/src/fillet/integrate.rs"
      via: "import replacing local G1/G2 constants"
      pattern: "G1_ANGLE_TOLERANCE"
    - from: "monstertruck-core/src/tolerance_constants.rs"
      to: "monstertruck-solid/src/transversal/loops_store/mod.rs"
      via: "import replacing hardcoded tolerance expressions"
      pattern: "SNAP_TOLERANCE"
    - from: "monstertruck-core/src/tolerance_constants.rs"
      to: "monstertruck-solid/src/transversal/integrate/mod.rs"
      via: "import replacing hardcoded 100.0 * TOLERANCE for weld_tol"
      pattern: "VERTEX_MERGE_TOLERANCE"
---

<objective>
Create a centralized tolerance_constants module in monstertruck-core that exports all shared tolerance constants (SNAP_TOLERANCE, VERTEX_MERGE_TOLERANCE, TESSELLATION_TOLERANCE, PERIODIC_CLOSURE_RATIO, G1_ANGLE_TOLERANCE, G2_CURVATURE_TOLERANCE) with defaults calibrated to preserve existing behavior, then refactor all call sites in monstertruck-solid to use them.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-core/src/tolerance.rs
@monstertruck-core/src/lib.rs
@monstertruck-solid/src/fillet/integrate.rs
@monstertruck-solid/src/transversal/loops_store/mod.rs
@monstertruck-solid/src/transversal/integrate/mod.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create tolerance_constants module</name>
  <files>monstertruck-core/src/tolerance_constants.rs, monstertruck-core/src/lib.rs</files>
  <action>
Create a new file `monstertruck-core/src/tolerance_constants.rs` with the following centralized constants. Each constant must have a doc comment explaining its purpose and the numeric value.

```rust
//! Centralized tolerance constants for the monstertruck workspace.
//!
//! These constants consolidate magic numbers that were previously scattered
//! across multiple crates. Changing a value here affects all consumers.
//!
//! # Derivation
//!
//! - [`SNAP_TOLERANCE`]: Used in boolean/intersection operations to snap nearby
//!   points onto boundary curves. Historically `10.0 * TOLERANCE` = `1.0e-5`.
//! - [`VERTEX_MERGE_TOLERANCE`]: Grid-based vertex merging in transversal loop
//!   construction. Historically `100.0 * TOLERANCE` = `1.0e-4`.
//! - [`TESSELLATION_TOLERANCE`]: Default geometric tolerance for tessellation
//!   of curves and surfaces. Historically `0.01`.
//! - [`PERIODIC_CLOSURE_RATIO`]: Ratio threshold for detecting periodic curve
//!   closure. Historically `0.01`.
//! - [`G1_ANGLE_TOLERANCE`]: Angle in radians for G1 tangent continuity
//!   classification (~1 degree). Historically `0.0175`.
//! - [`G2_CURVATURE_TOLERANCE`]: Relative tolerance for mean curvature matching
//!   in G2 continuity classification (10%). Historically `0.10`.

use super::tolerance::TOLERANCE;

/// Snap tolerance for boolean/intersection vertex snapping.
///
/// Points closer than this distance are considered coincident during
/// intersection curve construction and loop stitching.
///
/// Default: `10.0 * TOLERANCE` = `1.0e-5`
pub const SNAP_TOLERANCE: f64 = 10.0 * TOLERANCE;

/// Grid-based vertex merge tolerance for transversal loop construction.
///
/// Used to quantize 3D points into an integer grid for O(1) duplicate detection
/// during boolean operation loop assembly.
///
/// Default: `100.0 * TOLERANCE` = `1.0e-4`
pub const VERTEX_MERGE_TOLERANCE: f64 = 100.0 * TOLERANCE;

/// Default geometric tolerance for curve and surface tessellation.
///
/// Controls the maximum deviation between the tessellated polygon/mesh
/// and the underlying exact geometry.
///
/// Default: `0.01`
pub const TESSELLATION_TOLERANCE: f64 = 0.01;

/// Ratio threshold for detecting periodic curve closure.
///
/// A curve is considered closed when the gap between its endpoints
/// is less than `PERIODIC_CLOSURE_RATIO * total_arc_length`.
///
/// Default: `0.01` (1% of arc length)
pub const PERIODIC_CLOSURE_RATIO: f64 = 0.01;

/// Angle tolerance in radians for G1 tangent continuity classification.
///
/// Two adjacent surfaces are G1-continuous along a shared edge if the
/// angle between their normals is below this threshold at every sample point.
///
/// Default: `0.0175` (~1 degree)
pub const G1_ANGLE_TOLERANCE: f64 = 0.0175;

/// Relative tolerance for mean curvature matching in G2 continuity classification.
///
/// Two adjacent surfaces are G2-continuous along a shared edge if,
/// in addition to G1 continuity, the relative difference in mean curvature
/// `|kappa_1 - kappa_2| / max(|kappa_1|, |kappa_2|)` is below this threshold.
///
/// Default: `0.10` (10%)
pub const G2_CURVATURE_TOLERANCE: f64 = 0.10;
```

Then add to `monstertruck-core/src/lib.rs`:
1. Add `pub mod tolerance_constants;` after the existing `pub mod tolerance;` line.
2. Do NOT add any re-exports at the crate root -- keep it as a module that consumers explicitly import from.
  </action>
  <verify>Run `cargo check -p monstertruck-core` to confirm the new module compiles without errors.</verify>
  <done>tolerance_constants module created and compiles as part of monstertruck-core.</done>
</task>

<task type="auto">
  <name>Task 2: Refactor all monstertruck-solid call sites to use centralized constants</name>
  <files>monstertruck-solid/src/fillet/integrate.rs, monstertruck-solid/src/transversal/loops_store/mod.rs, monstertruck-solid/src/transversal/integrate/mod.rs</files>
  <action>
Refactor all hardcoded tolerance expressions in monstertruck-solid to import from the new centralized module. There are four call sites across three files:

**monstertruck-solid/src/fillet/integrate.rs** (around lines 39-42):
- Remove the local `const G1_ANGLE_TOLERANCE: f64 = 0.0175;`
- Remove the local `const G2_CURVATURE_TOLERANCE: f64 = 0.10;`
- Add import: `use monstertruck_core::tolerance_constants::{G1_ANGLE_TOLERANCE, G2_CURVATURE_TOLERANCE};`
- The usage sites remain unchanged since they reference the same names.

**monstertruck-solid/src/transversal/loops_store/mod.rs** -- THREE sites to update:

1. Line ~824 -- snap_tol clamping. Replace the expression with the centralized constant directly:
   ```rust
   let snap_tol = f64::max(snap_tol, SNAP_TOLERANCE);
   ```
   This is functionally identical (`SNAP_TOLERANCE` = `10.0 * TOLERANCE`) but now uses the named constant.

2. Line ~827 -- vertex_merge_tol. Replace:
   ```rust
   let vertex_merge_tol = VERTEX_MERGE_TOLERANCE;
   ```

3. Line ~1385 -- the `create_loops_stores` convenience function passes `10.0 * TOLERANCE` as a literal argument. Replace:
   ```rust
   create_loops_stores_with_tolerance(
       geom_shell0,
       poly_shell0,
       geom_shell1,
       poly_shell1,
       SNAP_TOLERANCE,
   )
   ```

- Add import at top of file: `use monstertruck_core::tolerance_constants::{SNAP_TOLERANCE, VERTEX_MERGE_TOLERANCE};`

**monstertruck-solid/src/transversal/integrate/mod.rs** -- ONE site:
- Line ~586: Replace `let weld_tol = f64::max(tol, 100.0 * TOLERANCE);` with:
  ```rust
  let weld_tol = f64::max(tol, VERTEX_MERGE_TOLERANCE);
  ```
- Add import at top of file: `use monstertruck_core::tolerance_constants::VERTEX_MERGE_TOLERANCE;`

Note: The `TESSELLATION_TOLERANCE` and `PERIODIC_CLOSURE_RATIO` constants are new names for values that currently exist as numeric literals in other crates (e.g., `0.01` in TessellationOptions). Do NOT refactor those call sites in this plan -- they live in different crates and the refactoring will be done in future phases. This plan only centralizes the constants and updates the call sites in monstertruck-solid.
  </action>
  <verify>Run `cargo check -p monstertruck-solid` to confirm compilation. Run `cargo test -p monstertruck-solid` to ensure no regressions across all transversal and fillet tests.</verify>
  <done>All four tolerance expression sites in monstertruck-solid replaced with imports from centralized module. No hardcoded tolerance multiplier expressions remain in these files. Existing behavior preserved.</done>
</task>

<task type="auto">
  <name>Task 3: Add cross-reference documentation</name>
  <files>monstertruck-core/src/tolerance.rs</files>
  <action>
Update the module-level doc comment in `monstertruck-core/src/tolerance.rs` to add a cross-reference to the new `tolerance_constants` module. After the existing "When to use a local constant instead" section (around line 32), add:

```rust
//!
//! ## Domain-specific constants
//!
//! See [`tolerance_constants`](crate::tolerance_constants) for named constants
//! derived from [`TOLERANCE`] that serve specific geometric operations
//! (snapping, vertex merging, G1/G2 classification, tessellation).
```

This ensures developers discovering `TOLERANCE` can find the domain-specific constants.
  </action>
  <verify>Run `cargo doc -p monstertruck-core --no-deps` and verify the cross-reference renders correctly (no broken links).</verify>
  <done>Cross-reference documentation added linking tolerance.rs to the new tolerance_constants module.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-core` passes with the new tolerance_constants module
2. `cargo check -p monstertruck-solid` passes with refactored imports
3. `cargo test -p monstertruck-core` passes (existing tolerance tests unaffected)
4. `cargo test -p monstertruck-solid` passes (fillet and transversal tests preserve behavior)
5. All six constants (SNAP_TOLERANCE, VERTEX_MERGE_TOLERANCE, TESSELLATION_TOLERANCE, PERIODIC_CLOSURE_RATIO, G1_ANGLE_TOLERANCE, G2_CURVATURE_TOLERANCE) are exported from monstertruck_core::tolerance_constants
6. No hardcoded `100.0 * TOLERANCE` remains in loops_store/mod.rs or integrate/mod.rs
7. No hardcoded `10.0 * TOLERANCE` remains in loops_store/mod.rs
8. No local G1_ANGLE_TOLERANCE / G2_CURVATURE_TOLERANCE remain in fillet/integrate.rs
</verification>

<success_criteria>
- TOLAPI-01 is fully satisfied: all six constants centralized with documented defaults
- All four monstertruck-solid call sites refactored (including the two previously missed sites)
- Existing test behavior is preserved (same numeric values, no regressions)
- Future phases can import these constants instead of hardcoding values
</success_criteria>

<output>
After completion, create `16-tolerance-foundation-and-api-safety/16-1-SUMMARY.md`
</output>
