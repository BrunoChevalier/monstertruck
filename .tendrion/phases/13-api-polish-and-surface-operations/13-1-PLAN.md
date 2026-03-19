---
phase: 13-api-polish-and-surface-operations
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/src/nurbs/surface_diagnostics.rs
  - monstertruck-geometry/src/nurbs/mod.rs
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/errors.rs
autonomous: true
must_haves:
  truths:
    - "User creates a SweepRailOptions struct with frame_rule and n_sections fields and passes it to try_sweep_rail"
    - "User creates a BirailOptions struct with n_sections and passes it to try_birail1/try_birail2"
    - "User creates a GordonOptions struct and passes it to try_gordon"
    - "User creates a SkinOptions struct and passes it to try_skin"
    - "User calling try_sweep_rail/try_birail/try_gordon/try_skin with invalid inputs gets a Result<_, Error> with diagnostic details instead of a panic"
    - "Existing positional-parameter APIs are preserved as deprecated wrappers for backward compatibility"
    - "User receives CurveNetworkDiagnostic errors describing exactly which curves are incompatible and why"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/surface_options.rs"
      provides: "Typed option structs: SweepRailOptions, Birail1Options, Birail2Options, GordonOptions, SkinOptions with Default impls"
      min_lines: 100
      contains: "SkinOptions"
    - path: "monstertruck-geometry/src/nurbs/surface_diagnostics.rs"
      provides: "CurveNetworkDiagnostic enum and helper validation functions for curve network compatibility checking"
      min_lines: 60
      contains: "CurveNetworkDiagnostic"
    - path: "monstertruck-geometry/src/errors.rs"
      provides: "Extended Error enum with SurfaceConstructionFailed and CurveNetworkIncompatible variants"
      min_lines: 270
      contains: "CurveNetworkIncompatible"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/surface_options.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      via: "Option structs consumed by try_sweep_rail, try_birail1, try_birail2, try_gordon, try_skin methods"
      pattern: "SkinOptions"
    - from: "monstertruck-geometry/src/nurbs/surface_diagnostics.rs"
      to: "monstertruck-geometry/src/errors.rs"
      via: "CurveNetworkDiagnostic used in Error::CurveNetworkIncompatible variant"
      pattern: "CurveNetworkDiagnostic"
---

<objective>
Introduce typed option structs for all surface constructors (sweep_rail, birail1, birail2, gordon, skin) at the geometry level, and add rich diagnostic error reporting for invalid curve networks, replacing panics with Result-returning fallible alternatives.
</objective>

<execution_context>
@skills/state-management/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs
@monstertruck-geometry/src/errors.rs
@monstertruck-geometry/src/nurbs/mod.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Define option structs and diagnostic types</name>
  <files>monstertruck-geometry/src/nurbs/surface_options.rs, monstertruck-geometry/src/nurbs/surface_diagnostics.rs, monstertruck-geometry/src/nurbs/mod.rs</files>
  <action>
Create `monstertruck-geometry/src/nurbs/surface_options.rs` with:

```rust
/// Frame rule for sweep operations — controls how the profile is oriented along the rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrameRule {
    /// Tangent-aligned framing (rotate profile to match rail tangent at each section).
    #[default]
    TangentAligned,
    /// Fixed-up framing (project profile using a fixed up-vector).
    FixedUp,
}

/// Options for single-rail sweep surface construction.
#[derive(Debug, Clone)]
pub struct SweepRailOptions {
    pub n_sections: usize,
    pub frame_rule: FrameRule,
}

impl Default for SweepRailOptions {
    fn default() -> Self {
        Self { n_sections: 10, frame_rule: FrameRule::TangentAligned }
    }
}

/// Options for single-profile birail surface construction.
#[derive(Debug, Clone)]
pub struct Birail1Options {
    pub n_sections: usize,
}

impl Default for Birail1Options {
    fn default() -> Self {
        Self { n_sections: 10 }
    }
}

/// Options for dual-profile birail surface construction.
#[derive(Debug, Clone)]
pub struct Birail2Options {
    pub n_sections: usize,
}

impl Default for Birail2Options {
    fn default() -> Self {
        Self { n_sections: 10 }
    }
}

/// Options for Gordon surface construction.
///
/// Currently a marker struct. Future versions may add tolerance fields
/// for controlling intersection point matching.
#[derive(Debug, Clone, Default)]
pub struct GordonOptions {}

/// Options for skin surface construction.
///
/// Currently a marker struct. Future versions may add fields to control
/// v-direction degree or parameterization strategy.
#[derive(Debug, Clone, Default)]
pub struct SkinOptions {}
```

Create `monstertruck-geometry/src/nurbs/surface_diagnostics.rs` with:

```rust
use std::fmt;

/// Diagnostic information about curve network incompatibilities.
#[derive(Debug, Clone, PartialEq)]
pub enum CurveNetworkDiagnostic {
    /// Too few curves provided.
    InsufficientCurves { required: usize, got: usize },
    /// Too few sections requested.
    InsufficientSections { required: usize, got: usize },
    /// Curve endpoints do not meet at expected intersection points.
    EndpointMismatch { curve_index: usize, expected: [f64; 3], actual: [f64; 3], distance: f64 },
    /// Curves have incompatible parameter domains.
    DomainMismatch { curve_a: usize, curve_b: usize, range_a: (f64, f64), range_b: (f64, f64) },
    /// Grid dimension mismatch for Gordon surface.
    GridDimensionMismatch { expected_rows: usize, expected_cols: usize, actual_rows: usize, actual_cols: usize },
    /// Compatibility normalization failed.
    CompatNormalizationFailed { reason: String },
    /// Degenerate geometry (e.g., zero-length chord).
    DegenerateGeometry { description: String },
}

impl fmt::Display for CurveNetworkDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientCurves { required, got } =>
                write!(f, "insufficient curves: need at least {required}, got {got}"),
            Self::InsufficientSections { required, got } =>
                write!(f, "insufficient sections: need at least {required}, got {got}"),
            Self::EndpointMismatch { curve_index, expected, actual, distance } =>
                write!(f, "endpoint mismatch on curve {curve_index}: expected {expected:?}, got {actual:?} (distance {distance:.6})"),
            Self::DomainMismatch { curve_a, curve_b, range_a, range_b } =>
                write!(f, "domain mismatch: curve {curve_a} has range {range_a:?}, curve {curve_b} has range {range_b:?}"),
            Self::GridDimensionMismatch { expected_rows, expected_cols, actual_rows, actual_cols } =>
                write!(f, "grid dimension mismatch: expected {expected_rows}x{expected_cols}, got {actual_rows}x{actual_cols}"),
            Self::CompatNormalizationFailed { reason } =>
                write!(f, "compatibility normalization failed: {reason}"),
            Self::DegenerateGeometry { description } =>
                write!(f, "degenerate geometry: {description}"),
        }
    }
}

impl std::error::Error for CurveNetworkDiagnostic {}
```

Add `pub mod surface_options;` and `pub mod surface_diagnostics;` to `monstertruck-geometry/src/nurbs/mod.rs`. Ensure the types are re-exported through the prelude if one exists, or at a minimum are publicly accessible from `monstertruck_geometry::nurbs::surface_options::*` and `monstertruck_geometry::nurbs::surface_diagnostics::*`.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry --lib` to confirm compilation. Verify the new modules are accessible.</verify>
  <done>Option structs (including SkinOptions) and diagnostic types are defined, compiled, and exported from the nurbs module.</done>
</task>

<task type="auto">
  <name>Task 2: Extend geometry error enum with curve network diagnostics</name>
  <files>monstertruck-geometry/src/errors.rs</files>
  <action>
Add new variants to the `Error` enum in `monstertruck-geometry/src/errors.rs`:

```rust
/// Surface construction failed due to invalid curve network.
#[error("curve network is incompatible: {0}")]
CurveNetworkIncompatible(CurveNetworkDiagnostic),

/// Surface construction requires more sections.
#[error("surface construction requires at least {required} sections, got {got}")]
InsufficientSections { required: usize, got: usize },

/// Surface construction failed due to degenerate or invalid inputs.
#[error("surface construction failed: {reason}")]
SurfaceConstructionFailed { reason: String },
```

IMPORTANT: Do NOT use `#[from]` on the CurveNetworkDiagnostic variant. The Error enum derives `PartialEq`, and CurveNetworkDiagnostic contains String fields which implement PartialEq but adding `#[from]` would require `std::error::Error` on CurveNetworkDiagnostic (which we do implement), but it may conflict with other From impls. Use a manual `From<CurveNetworkDiagnostic>` impl instead:

```rust
impl From<CurveNetworkDiagnostic> for Error {
    fn from(d: CurveNetworkDiagnostic) -> Self {
        Error::CurveNetworkIncompatible(d)
    }
}
```

Add the necessary import for `CurveNetworkDiagnostic` at the top of the file (`use crate::nurbs::surface_diagnostics::CurveNetworkDiagnostic;` or equivalent).

Update the `print_messages` test to include the new variants:
```rust
writeln!(stderr, "{}\n", Error::CurveNetworkIncompatible(
    CurveNetworkDiagnostic::InsufficientCurves { required: 2, got: 0 }
)).unwrap();
writeln!(stderr, "{}\n", Error::InsufficientSections { required: 2, got: 1 }).unwrap();
writeln!(stderr, "{}\n", Error::SurfaceConstructionFailed { reason: "test".into() }).unwrap();
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry -E 'test(print_messages)'` to verify the test passes.</verify>
  <done>Error enum extended with diagnostic variants and test updated.</done>
</task>

<task type="auto">
  <name>Task 3: Add fallible option-based constructors for sweep_rail, birail1, birail2</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Add new `try_*` methods to `impl BsplineSurface<Point3>` (the block starting around line 1518 which contains the existing sweep_rail, birail1, birail2) that accept option structs and return `Result`:

1. `try_sweep_rail(profile: BsplineCurve<Point3>, rail: &BsplineCurve<Point3>, options: &SweepRailOptions) -> errors::Result<BsplineSurface<Point3>>`:
   - Validates n_sections >= 2 (returns Error::InsufficientSections).
   - Validates rail tangent is not degenerate (returns Error::SurfaceConstructionFailed or CurveNetworkDiagnostic::DegenerateGeometry).
   - Delegates to the existing sweep_rail logic but uses `?` instead of panic.

2. `try_birail1(profile: BsplineCurve<Point3>, rail1: &BsplineCurve<Point3>, rail2: &BsplineCurve<Point3>, options: &Birail1Options) -> errors::Result<BsplineSurface<Point3>>`:
   - Validates n_sections >= 2.
   - Validates profile chord is non-degenerate.
   - Returns CurveNetworkDiagnostic on endpoint mismatch if profile start != rail1 start.

3. `try_birail2(profile1: BsplineCurve<Point3>, profile2: BsplineCurve<Point3>, rail1: &BsplineCurve<Point3>, rail2: &BsplineCurve<Point3>, options: &Birail2Options) -> errors::Result<BsplineSurface<Point3>>`:
   - Same validations plus checks both profiles.
   - Validates compatibility normalization succeeds (convert `.expect()` to `?` with `CurveNetworkDiagnostic::CompatNormalizationFailed`).

Mark the old positional-parameter methods (`sweep_rail`, `birail1`, `birail2`) with `#[deprecated(since = "0.5.0", note = "use try_sweep_rail with SweepRailOptions")]` (and analogous notes) but keep them functional.

Add `#[cfg(test)]` tests for both success and error paths for each `try_*` method. Test at least:
- Successful construction with default options
- InsufficientSections error when n_sections < 2
- One diagnostic error case per method

Import the option structs at the top of the file: `use super::surface_options::{SweepRailOptions, Birail1Options, Birail2Options};`
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry --lib` to verify all tests pass. Run `cargo clippy -p monstertruck-geometry -- -W warnings` to check for warnings (allow deprecation warnings from internal usage).</verify>
  <done>Fallible option-struct-based surface constructors for sweep_rail, birail1, birail2 added with tests.</done>
</task>

<task type="auto">
  <name>Task 4: Add fallible option-based constructors for gordon and skin</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Add `try_gordon` and `try_skin` methods:

1. For `impl<P: ControlPoint<f64> + Tolerance> BsplineSurface<P>` (the block around line 2145 containing `gordon`):
   `try_gordon(u_curves: Vec<BsplineCurve<P>>, v_curves: Vec<BsplineCurve<P>>, points: Vec<Vec<P>>, options: &GordonOptions) -> errors::Result<BsplineSurface<P>>`:
   - Validates curve counts (non-empty).
   - Returns `CurveNetworkDiagnostic::GridDimensionMismatch` if points grid doesn't match curve counts.
   - Converts `.expect()` calls for compatibility normalization to `?` with `CurveNetworkDiagnostic::CompatNormalizationFailed`.
   - Mark the old `gordon` method with `#[deprecated(since = "0.5.0", note = "use try_gordon with GordonOptions")]`.

2. For `impl<P: ControlPoint<f64> + Tolerance> BsplineSurface<P>` (the block around line 704 containing `skin`):
   `try_skin(curves: Vec<BsplineCurve<P>>, options: &SkinOptions) -> errors::Result<BsplineSurface<P>>`:
   - Validates curves is non-empty: return `Error::CurveNetworkIncompatible(CurveNetworkDiagnostic::InsufficientCurves { required: 1, got: 0 })` instead of the existing `assert!(!curves.is_empty())`.
   - Converts the `.expect("skin: compatibility normalization failed on non-empty curve set")` to `?` with `map_err` wrapping in `CurveNetworkDiagnostic::CompatNormalizationFailed`.
   - Otherwise contains the same logic as the existing `skin` method.
   - Mark the old `skin` method with `#[deprecated(since = "0.5.0", note = "use try_skin with SkinOptions")]`.

Import the option structs: `use super::surface_options::{GordonOptions, SkinOptions};`

Add `#[cfg(test)]` tests for both try_gordon and try_skin:
- `try_skin` success with 1 curve (degenerate), 2 curves (homotopy), 3+ curves (general)
- `try_skin` error with empty curves vec
- `try_gordon` success with valid network
- `try_gordon` error with grid dimension mismatch

Note on trait bounds: `try_gordon` and `try_skin` require `P: ControlPoint<f64> + Tolerance` (matching the existing `gordon` and `skin` methods).
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry --lib` to verify all tests pass including the new ones. Run `cargo clippy -p monstertruck-geometry -- -W warnings`.</verify>
  <done>Fallible option-struct-based constructors for gordon and skin added with comprehensive tests.</done>
</task>

</tasks>

<verification>
1. All new types (SweepRailOptions, Birail1Options, Birail2Options, GordonOptions, SkinOptions, CurveNetworkDiagnostic) compile and are accessible from their respective modules
2. Each try_* method (try_sweep_rail, try_birail1, try_birail2, try_gordon, try_skin) returns appropriate CurveNetworkDiagnostic errors for invalid inputs instead of panicking
3. Deprecated wrappers for the old positional-parameter APIs still compile and work
4. `cargo nextest run -p monstertruck-geometry --lib` passes all tests
5. `cargo clippy -p monstertruck-geometry -- -W warnings` shows no new warnings (excluding deprecation warnings from internal usage)
</verification>

<success_criteria>
- API-01 (geometry layer): All surface constructors (sweep_rail, birail1, birail2, gordon, skin) accept typed option structs via new try_* methods
- API-02 (geometry layer): Invalid curve networks produce CurveNetworkDiagnostic errors with detailed information
- Old APIs preserved with deprecation annotations for backward compatibility
</success_criteria>

<output>
After completion, create `.tendrion/phases/13-api-polish-and-surface-operations/13-1-SUMMARY.md`
</output>
