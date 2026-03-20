---
phase: 19-trim-tessellation-robustness
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-core/src/tolerance_constants.rs
  - monstertruck-core/tests/tolerance_constants.rs
  - monstertruck-meshing/src/tessellation/mod.rs
  - monstertruck-meshing/src/tessellation/triangulation.rs
autonomous: true
must_haves:
  truths:
    - "The 1.0e-3 closure threshold in PolyBoundary::new is replaced with a named constant derived from TESSELLATION_TOLERANCE"
    - "TessellationOptions::default() uses TESSELLATION_TOLERANCE instead of a hardcoded 0.01"
    - "All existing tessellation tests pass unchanged after the constant replacement"
    - "The new UV_CLOSURE_TOLERANCE constant is documented in tolerance_constants.rs"
  artifacts:
    - path: "monstertruck-core/src/tolerance_constants.rs"
      provides: "New UV_CLOSURE_TOLERANCE constant derived from TESSELLATION_TOLERANCE"
      min_lines: 75
      contains: "UV_CLOSURE_TOLERANCE"
    - path: "monstertruck-core/tests/tolerance_constants.rs"
      provides: "Unit test verifying UV_CLOSURE_TOLERANCE value and relationship"
      min_lines: 10
      contains: "uv_closure_tolerance_value"
    - path: "monstertruck-meshing/src/tessellation/triangulation.rs"
      provides: "Updated PolyBoundary::new using UV_CLOSURE_TOLERANCE instead of 1.0e-3"
      min_lines: 100
      contains: "UV_CLOSURE_TOLERANCE"
    - path: "monstertruck-meshing/src/tessellation/mod.rs"
      provides: "TessellationOptions::default() referencing TESSELLATION_TOLERANCE"
      min_lines: 400
      contains: "TESSELLATION_TOLERANCE"
  key_links:
    - from: "monstertruck-core/src/tolerance_constants.rs"
      to: "monstertruck-meshing/src/tessellation/triangulation.rs"
      via: "import of UV_CLOSURE_TOLERANCE constant"
      pattern: "use monstertruck_core::tolerance_constants::UV_CLOSURE_TOLERANCE"
    - from: "monstertruck-core/src/tolerance_constants.rs"
      to: "monstertruck-meshing/src/tessellation/mod.rs"
      via: "import of TESSELLATION_TOLERANCE constant"
      pattern: "use monstertruck_core::tolerance_constants::TESSELLATION_TOLERANCE"
---

<objective>
Replace hardcoded tessellation magic constants with expressions derived from centralized tolerance constants, so that all tessellation thresholds are traceable to a single source of truth in tolerance_constants.rs.
</objective>

<execution_context>
@skills/state-management/SKILL.md
</execution_context>

<context>
@monstertruck-core/src/tolerance_constants.rs
@monstertruck-meshing/src/tessellation/mod.rs
@monstertruck-meshing/src/tessellation/triangulation.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add UV_CLOSURE_TOLERANCE to centralized tolerance constants</name>
  <files>monstertruck-core/src/tolerance_constants.rs</files>
  <action>
Add a new constant `UV_CLOSURE_TOLERANCE` to `monstertruck-core/src/tolerance_constants.rs`. This constant replaces the hardcoded `1.0e-3` used in `PolyBoundary::new` (line 458 of triangulation.rs) for deciding whether a boundary piece's start and end UV points are close enough to be considered a closed loop.

The constant should be derived from `TESSELLATION_TOLERANCE`:

```rust
/// UV-space closure threshold for boundary piece loop detection.
///
/// When constructing a `PolyBoundary`, each `PolyBoundaryPiece` is classified
/// as closed or open based on whether its first and last UV-space points are
/// within this distance. A piece is closed when
/// `start.uv.distance(end.uv) < UV_CLOSURE_TOLERANCE`.
///
/// Derived as `TESSELLATION_TOLERANCE / 10.0` to be one order of magnitude
/// tighter than the geometric tessellation tolerance, since UV-space distances
/// are typically smaller than 3D distances.
///
/// Default: `0.001` (= `TESSELLATION_TOLERANCE / 10.0`)
pub const UV_CLOSURE_TOLERANCE: f64 = TESSELLATION_TOLERANCE / 10.0;
```

Place it after `PERIODIC_CLOSURE_RATIO` and before `G1_ANGLE_TOLERANCE` to keep related tessellation constants grouped together.
  </action>
  <verify>Run `cargo check -p monstertruck-core` to verify the constant compiles.</verify>
  <done>UV_CLOSURE_TOLERANCE constant added to tolerance_constants.rs with documentation explaining its derivation from TESSELLATION_TOLERANCE.</done>
</task>

<task type="auto">
  <name>Task 2: Replace magic constants in tessellation code with centralized constants</name>
  <files>monstertruck-meshing/src/tessellation/triangulation.rs, monstertruck-meshing/src/tessellation/mod.rs</files>
  <action>
Make two replacements in the tessellation code:

**In `triangulation.rs`:**

1. Add an import for the new constant at the top of the file:
```rust
use monstertruck_core::tolerance_constants::UV_CLOSURE_TOLERANCE;
```

2. In `PolyBoundary::new` (around line 458), replace the hardcoded `1.0e-3`:
```rust
// BEFORE:
match vec[0].uv.distance(vec[vec.len() - 1].uv) < 1.0e-3 {

// AFTER:
match vec[0].uv.distance(vec[vec.len() - 1].uv) < UV_CLOSURE_TOLERANCE {
```

**In `mod.rs`:**

1. Add an import for `TESSELLATION_TOLERANCE`:
```rust
use monstertruck_core::tolerance_constants::TESSELLATION_TOLERANCE;
```

2. In the `Default` impl for `TessellationOptions` (around line 61-68), replace the hardcoded `0.01`:
```rust
// BEFORE:
impl Default for TessellationOptions {
    fn default() -> Self {
        Self {
            tolerance: 0.01,
            ..
        }
    }
}

// AFTER:
impl Default for TessellationOptions {
    fn default() -> Self {
        Self {
            tolerance: TESSELLATION_TOLERANCE,
            ..
        }
    }
}
```

Do NOT change any other numeric values (e.g., `plane_tolerance: 0.01` in `QuadOptions::default()` is a different concept -- quad coplanarity, not tessellation tolerance).
  </action>
  <verify>Run `cargo check -p monstertruck-meshing` to verify compilation. Then run `cargo test -p monstertruck-meshing --test tessellation` to ensure all existing tests pass with the same numeric values.</verify>
  <done>Hardcoded 1.0e-3 closure threshold replaced with UV_CLOSURE_TOLERANCE and TessellationOptions::default() now uses TESSELLATION_TOLERANCE.</done>
</task>

<task type="auto">
  <name>Task 3: Add unit test verifying constant relationships</name>
  <files>monstertruck-core/tests/tolerance_constants.rs</files>
  <action>
Add test assertions to the existing `monstertruck-core/tests/tolerance_constants.rs` file to verify that the new constant has the expected value and relationship:

```rust
#[test]
fn uv_closure_tolerance_value() {
    use monstertruck_core::tolerance_constants::{UV_CLOSURE_TOLERANCE, TESSELLATION_TOLERANCE};
    // UV_CLOSURE_TOLERANCE should be TESSELLATION_TOLERANCE / 10
    assert!((UV_CLOSURE_TOLERANCE - TESSELLATION_TOLERANCE / 10.0).abs() < f64::EPSILON);
    // Must be positive
    assert!(UV_CLOSURE_TOLERANCE > 0.0);
    // Must be smaller than TESSELLATION_TOLERANCE
    assert!(UV_CLOSURE_TOLERANCE < TESSELLATION_TOLERANCE);
}
```

Read the existing file first to understand its test structure and add the new test in a consistent style.
  </action>
  <verify>Run `cargo test -p monstertruck-core uv_closure_tolerance_value` to verify the test passes.</verify>
  <done>Unit test added verifying UV_CLOSURE_TOLERANCE has the correct value and relationship to TESSELLATION_TOLERANCE.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-core` compiles successfully
2. `cargo check -p monstertruck-meshing` compiles successfully
3. `cargo test -p monstertruck-core` passes including new constant relationship test
4. `cargo test -p monstertruck-meshing --test tessellation` passes -- all existing tests produce identical results since numeric values are unchanged
5. `grep -r "1\.0e-3" monstertruck-meshing/src/tessellation/` returns no matches
6. `grep "tolerance: 0.01" monstertruck-meshing/src/tessellation/mod.rs` returns no matches in TessellationOptions::default
</verification>

<success_criteria>
- TRIM-02 satisfied: the 1.0e-3 closure threshold is replaced with UV_CLOSURE_TOLERANCE derived from TESSELLATION_TOLERANCE
- TessellationOptions::default() references TESSELLATION_TOLERANCE instead of hardcoded 0.01
- All existing tests pass with identical behavior (numeric values unchanged)
- New constant is documented with derivation rationale
</success_criteria>

<output>
After completion, create `.tendrion/phases/19-trim-tessellation-robustness/19-1-SUMMARY.md`
</output>
