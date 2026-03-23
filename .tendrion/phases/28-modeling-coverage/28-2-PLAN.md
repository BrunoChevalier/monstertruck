---
phase: 28-modeling-coverage
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-modeling/tests/text_module_test.rs
  - monstertruck-modeling/tests/geometry_test.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo nextest run -p monstertruck-modeling --features font -E 'test(text_module)' and all tests pass"
    - "User runs cargo nextest run -p monstertruck-modeling -E 'test(geometry_)' and all tests pass"
    - "TextOptions default values and custom configuration (scale, z, closure_tolerance) are verified in tests distinct from font_pipeline.rs"
    - "Edge cases not covered elsewhere are tested: empty string returns empty vec, space glyph returns error"
    - "Geometry enum Curve and Surface variant construction and conversion are tested"
  artifacts:
    - path: "monstertruck-modeling/tests/text_module_test.rs"
      provides: "Tests for text module public functions: TextOptions, glyph_profile edge cases, text_profile edge cases"
      min_lines: 120
      contains: "text::TextOptions"
    - path: "monstertruck-modeling/tests/geometry_test.rs"
      provides: "Tests for Curve and Surface enum variants, conversions, and geometry module public API"
      min_lines: 80
      contains: "Curve::Line"
  key_links:
    - from: "monstertruck-modeling/tests/text_module_test.rs"
      to: "monstertruck-modeling/src/text.rs"
      via: "direct text module API calls"
      pattern: "text::glyph_profile"
    - from: "monstertruck-modeling/tests/geometry_test.rs"
      to: "monstertruck-modeling/src/geometry.rs"
      via: "direct geometry enum construction and trait usage"
      pattern: "Curve::Line"
---

<objective>
Add tests for the text/glyph module public functions and the geometry module enum variants to cover remaining untested code paths and push monstertruck-modeling coverage above 45%.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-modeling/src/text.rs
@monstertruck-modeling/src/geometry.rs
@monstertruck-modeling/Cargo.toml
</context>

<tasks>

<task type="auto">
  <name>Task 1: Text module public function tests</name>
  <files>monstertruck-modeling/tests/text_module_test.rs</files>
  <action>
Create a new integration test file `monstertruck-modeling/tests/text_module_test.rs` with `#![cfg(feature = "font")]` at the top. This tests the text module's public API focusing on areas NOT already covered by font_pipeline.rs (which tests the full pipeline from text to solid). These tests focus on: TextOptions configuration, edge cases (empty strings, space-only, no-outline glyphs), and direct glyph_profile behavior rather than the full pipeline.

Use the same font fixture pattern:
```rust
#![cfg(feature = "font")]

use monstertruck_modeling::*;

const FONT_BYTES: &[u8] = include_bytes!("../test-fixtures/DejaVuSans.ttf");

fn face() -> ttf_parser::Face<'static> {
    ttf_parser::Face::parse(FONT_BYTES, 0).expect("valid TTF")
}
```

Tests to implement (ALL distinct from existing font_pipeline.rs which covers: glyph contour counts, y_flip, glyph/text profile to solid pipeline, spacing, space skipping, merge_profiles):

**TextOptions struct tests (NOT tested in font_pipeline.rs):**
1. **text_module_options_default**: Verify `TextOptions::default()` field values: y_flip=true, z=0.0, scale=None, closure_tolerance=1e-7.

2. **text_module_options_custom_scale**: Create TextOptions with scale=Some(0.01). Call glyph_profile for 'H'. Verify all vertex coordinates are within the scaled range (< 1.0 absolute) vs default scale which would produce larger values.

3. **text_module_options_custom_z**: Create TextOptions with z=5.0. Call glyph_profile. Verify all vertex Z coordinates are near 5.0 (font_pipeline.rs only tests default z=0.0).

4. **text_module_options_closure_tolerance**: Create TextOptions with closure_tolerance=1e-3 (looser). Verify glyph_profile still succeeds for a complex glyph.

**Edge cases NOT covered by font_pipeline.rs:**
5. **text_module_text_empty_string**: Call `text_profile` with "". Should return Ok with empty vec.

6. **text_module_glyph_no_outline**: Get glyph ID for space character via face.glyph_index(' '). Call `glyph_profile` — should return error (space has no outline).

7. **text_module_options_debug_display**: Verify TextOptions implements Debug (call format! on it, verify non-empty output).

Run all text tests with `--features font` flag.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling --features font -E 'test(text_module)' --no-fail-fast` and verify all tests pass.</verify>
  <done>Text module tests for TextOptions, glyph_profile, and text_profile public functions created and passing.</done>
</task>

<task type="auto">
  <name>Task 2: Geometry module tests</name>
  <files>monstertruck-modeling/tests/geometry_test.rs</files>
  <action>
Create a new integration test file `monstertruck-modeling/tests/geometry_test.rs` testing the geometry module's Curve and Surface enum variants and their trait implementations.

First, read the geometry.rs file to understand the enum variants and their trait implementations. The Curve and Surface enums wrap various geometry types from monstertruck-geometry.

Tests to implement:

1. **geometry_curve_line_construction**: Construct a `Curve::Line(Line(p0, p1))`. Verify `subs(0.0)` gives p0 and `subs(1.0)` gives p1.

2. **geometry_curve_bspline_construction**: Construct a `Curve::BsplineCurve` from a degree-1 BsplineCurve. Verify `subs` at t=0 and t=1 gives the endpoints.

3. **geometry_curve_range**: Construct various Curve variants. Verify `range_tuple()` returns valid (t0, t1) with t0 < t1.

4. **geometry_curve_der_finite**: For a Line curve, verify `der(0.5)` returns a non-zero vector in the expected direction.

5. **geometry_surface_plane_construction**: Construct a `Surface::Plane(plane)`. Verify `subs(0.0, 0.0)` gives the plane origin.

6. **geometry_surface_bspline_construction**: Construct a `Surface::BsplineSurface` from a simple bilinear surface. Verify `subs` at corners.

7. **geometry_surface_normal**: For a `Surface::Plane`, verify `normal(0.0, 0.0)` is perpendicular to the plane's u and v directions.

8. **geometry_curve_clone_and_eq**: Verify Curve variants implement Clone correctly -- clone a curve and verify it produces the same subs values.

9. **geometry_surface_search_parameter**: For a Plane surface, call `search_parameter` with a known point on the plane. Verify it finds the (u, v) parameters.

10. **geometry_curve_inverse**: For supported Curve variants, call `inverse()` and verify that `subs(t0)` of the inverted curve equals `subs(t1)` of the original.

Use imports:
```rust
use monstertruck_modeling::*;
```

For floating-point comparisons, use `Tolerance::near` or `assert_near!`.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-modeling -E 'test(geometry_)' --no-fail-fast` and verify all tests pass.</verify>
  <done>Geometry module tests for Curve and Surface enum variants created and passing.</done>
</task>

<task type="auto">
  <name>Task 3: Verify combined coverage and clippy cleanliness</name>
  <files>monstertruck-modeling/tests/text_module_test.rs, monstertruck-modeling/tests/geometry_test.rs</files>
  <action>
Run clippy on all new and existing test files to ensure no warnings:
```bash
cargo clippy -p monstertruck-modeling --tests --features font -- -W warnings
```

Fix any clippy warnings in the new test files.

Then run the full monstertruck-modeling test suite including the font feature:
```bash
cargo nextest run -p monstertruck-modeling --features font --no-fail-fast
```

Confirm that all existing tests still pass alongside the new ones.

Finally, verify the overall coverage improvement by running the full test suite for the package and checking that the combined new tests (from both plans) push coverage from 27% toward 45%+:
```bash
cargo nextest run -p monstertruck-modeling --features font,solid-ops,fillet --no-fail-fast
```
  </action>
  <verify>Clippy reports no warnings, full test suite passes including all features, coverage trajectory confirmed.</verify>
  <done>All new and existing tests pass with all features enabled, clippy clean, combined coverage from both plans targets 45%+.</done>
</task>

</tasks>

<verification>
1. All tests in text_module_test.rs pass with `cargo nextest run -p monstertruck-modeling --features font -E 'test(text_module)'`
2. All tests in geometry_test.rs pass with `cargo nextest run -p monstertruck-modeling -E 'test(geometry_)'`
3. No clippy warnings on `cargo clippy -p monstertruck-modeling --tests --features font -- -W warnings`
4. Full test suite passes: `cargo nextest run -p monstertruck-modeling --features font --no-fail-fast`
5. Text/glyph module public functions each have at least one test
6. Geometry Curve/Surface enum variants have construction and trait tests
</verification>

<success_criteria>
- Text/glyph module public functions (TextOptions, glyph_profile, text_profile) each have at least one test
- Geometry module Curve and Surface enum variants have construction and basic trait tests
- Combined with Plan 28-1 tests, monstertruck-modeling test coverage reaches 45%+ target
- All tests are deterministic and pass consistently
- No clippy warnings introduced
</success_criteria>

<output>
After completion, create `.tendrion/phases/28-modeling-coverage/28-2-SUMMARY.md`
</output>
