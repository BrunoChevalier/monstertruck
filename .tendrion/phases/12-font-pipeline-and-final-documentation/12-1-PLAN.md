---
phase: 12-font-pipeline-and-final-documentation
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-modeling/Cargo.toml
  - monstertruck-modeling/tests/font_pipeline.rs
  - monstertruck-modeling/test-fixtures/DejaVuSans.ttf
autonomous: true
must_haves:
  truths:
    - "Running cargo nextest run with --features font executes font_pipeline tests that load a real TTF font"
    - "Glyph profile for 'O' produces at least 2 wires (outer contour + inner hole)"
    - "Glyph profile for 'B' produces at least 3 wires (outer + 2 holes)"
    - "Generated wires are all closed and pass Wire::is_closed()"
    - "Wire sets can be passed through attach_plane_normalized to produce a valid Face with correct boundary count"
    - "Wire sets can be extruded via solid_from_planar_profile to produce a geometrically consistent Solid"
    - "text_profile for multi-character strings produces wires with correct horizontal advance spacing"
  artifacts:
    - path: "monstertruck-modeling/tests/font_pipeline.rs"
      provides: "End-to-end integration tests for font outline to wire/face/solid pipeline"
      min_lines: 120
      contains: "glyph_profile"
    - path: "monstertruck-modeling/test-fixtures/DejaVuSans.ttf"
      provides: "Real font fixture for integration tests"
      min_lines: 1
      contains: ""
  key_links:
    - from: "monstertruck-modeling/tests/font_pipeline.rs"
      to: "monstertruck-modeling/src/text.rs"
      via: "imports text::glyph_profile and text::text_profile"
      pattern: "text::glyph_profile"
    - from: "monstertruck-modeling/tests/font_pipeline.rs"
      to: "monstertruck-modeling/src/profile.rs"
      via: "imports profile::attach_plane_normalized and solid_from_planar_profile"
      pattern: "profile::attach_plane_normalized"
---

<objective>
Create end-to-end integration tests that load a real TrueType font, extract glyph profiles for characters with holes (O, B, 8), verify inner loop preservation through Wire topology, and confirm the full pipeline through face creation and solid extrusion.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-modeling/src/text.rs
@monstertruck-modeling/src/profile.rs
@monstertruck-modeling/tests/profile_test.rs
@AYAM_PORT_PLAN.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add font fixture and configure test dependencies</name>
  <files>monstertruck-modeling/test-fixtures/DejaVuSans.ttf, monstertruck-modeling/Cargo.toml</files>
  <action>
1. Create the `monstertruck-modeling/test-fixtures/` directory.
2. Copy `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf` into `monstertruck-modeling/test-fixtures/DejaVuSans.ttf`. DejaVu Sans is licensed under a permissive free license and is ideal for testing because:
   - 'O' has 2 contours (outer + hole)
   - 'B' has 3 contours (outer + 2 holes)
   - '8' has 3 contours (outer + 2 holes)
   - Space and other characters have no outlines (tests skip behavior)

3. Update `monstertruck-modeling/Cargo.toml` to add `ttf-parser` to `[dev-dependencies]` so integration tests can use it without requiring the `font` feature to be active for dev builds. Actually, since the `text` module is behind `#[cfg(feature = "font")]`, the integration tests MUST be run with `--features font`. Add this note in the test file.

   In `[dev-dependencies]`, no changes needed since `ttf-parser` is already a workspace dependency used via the `font` feature. The test file will use `#![cfg(feature = "font")]` at the top.
  </action>
  <verify>Confirm `monstertruck-modeling/test-fixtures/DejaVuSans.ttf` exists and is a valid TTF file (non-zero size). Confirm Cargo.toml compiles with `cargo check -p monstertruck-modeling --features font`.</verify>
  <done>Font fixture copied and Cargo.toml validated for font feature compilation.</done>
</task>

<task type="auto">
  <name>Task 2: Write end-to-end glyph profile tests with hole verification</name>
  <files>monstertruck-modeling/tests/font_pipeline.rs</files>
  <action>
Create `monstertruck-modeling/tests/font_pipeline.rs` with the following structure:

```rust
//! End-to-end integration tests for the font outline -> wire -> face -> solid pipeline.
//!
//! These tests require the `font` feature flag:
//!   cargo nextest run -p monstertruck-modeling --features font
#![cfg(feature = "font")]

use monstertruck_modeling::*;

/// Path to the bundled DejaVu Sans font fixture.
const FONT_BYTES: &[u8] = include_bytes!("../test-fixtures/DejaVuSans.ttf");

fn face() -> ttf_parser::Face<'static> {
    ttf_parser::Face::parse(FONT_BYTES, 0).expect("valid TTF")
}

fn default_opts() -> text::TextOptions {
    text::TextOptions::default()
}
```

Then write these test functions:

**Test: `glyph_o_has_hole`**
- Load DejaVuSans, look up glyph for 'O'
- Call `text::glyph_profile(&face, glyph_id, &opts)`
- Assert `wires.len() >= 2` (outer + at least 1 inner hole)
- Assert all wires are closed (`w.is_closed()`)
- Assert all wires have at least 3 edges

**Test: `glyph_b_has_two_holes`**
- Same for 'B'
- Assert `wires.len() >= 3` (outer + 2 holes)
- Assert all wires are closed

**Test: `glyph_8_has_two_holes`**
- Same for '8'
- Assert `wires.len() >= 3`

**Test: `glyph_i_has_no_holes`**
- 'I' or 'l' (lowercase L) should have exactly 1 contour
- Assert `wires.len() == 1`
- Assert wire is closed

**Test: `glyph_profile_face_with_holes`**
- Get wires for 'O'
- Pass through `profile::attach_plane_normalized(wires)`
- Assert face has boundaries count matching wire count (outer + holes)

**Test: `glyph_profile_solid_extrusion`**
- Get wires for 'O'
- Call `profile::solid_from_planar_profile(wires, Vector3::new(0.0, 0.0, 1.0))`
- Assert solid is created successfully (`.is_ok()`)
- Assert `solid.is_geometric_consistent()`
- Assert shell has expected face count: 2 caps + N outer sides + M inner sides

**Test: `glyph_b_solid_extrusion`**
- Same pipeline for 'B' (more complex: 2 holes)
- Assert solid created and is geometrically consistent

**Test: `text_profile_hello`**
- Call `text::text_profile(&face, "HO", &opts)`
- Assert wires are returned (H has no holes but has outlines, O has hole)
- Assert total wire count > 2 (H contributes outlines, O contributes outer + hole)
- Assert all wires are closed

**Test: `text_profile_spacing`**
- Call `text::text_profile(&face, "II", &opts)`
- Get wires for each character
- Verify that the second character's wires have X coordinates offset from the first (horizontal advance applied)
- Sample a vertex from first wire and second wire, assert second has greater X coordinate

**Test: `text_profile_space_skipped`**
- Call `text::text_profile(&face, "I I", &opts)`
- Space has no outline so wire count should equal count for "II"
- But spacing should be wider (advance applied for space)

**Test: `glyph_profile_y_flip`**
- Extract wires for 'O' with default opts (y_flip: true)
- Extract wires for 'O' with y_flip: false
- Sample vertex Y coordinates, assert they have opposite signs

NOTE: For looking up glyph IDs, use `face.glyph_index(char)` which returns `Option<GlyphId>`.

Each test should be self-contained and load the font fresh (or reuse the const bytes).
  </action>
  <verify>
Run `cargo nextest run -p monstertruck-modeling --features font -- font_pipeline` and verify all tests pass.
Run `cargo clippy -p monstertruck-modeling --features font --tests -- -W warnings` and verify no warnings.
  </verify>
  <done>End-to-end font pipeline integration tests written and passing, covering single glyph hole preservation, face creation, solid extrusion, multi-character text profiles, and spacing verification.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-modeling --features font -- font_pipeline` passes all tests
2. `cargo clippy -p monstertruck-modeling --features font --tests -- -W warnings` produces no warnings
3. Tests for 'O' verify >= 2 wires (hole preserved)
4. Tests for 'B' verify >= 3 wires (both holes preserved)
5. Face created from glyph wires has correct boundary count
6. Solid from glyph profile is geometrically consistent
7. text_profile produces wires with correct horizontal advance spacing
</verification>

<success_criteria>
- Real font fixture is loaded and parsed in integration tests
- Glyphs with holes (O, B, 8) produce multiple wires with inner loops preserved
- Generated wires produce valid Face via attach_plane_normalized
- Generated wires produce valid geometrically-consistent Solid via solid_from_planar_profile
- Multi-character text_profile tests verify horizontal advance and wire topology
- All tests pass with cargo nextest run and no clippy warnings
</success_criteria>

<output>
After completion, create `.tendrion/phases/12-font-pipeline-and-final-documentation/12-1-SUMMARY.md`
</output>
