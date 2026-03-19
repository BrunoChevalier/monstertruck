---
phase: 15-font-stress-testing-and-performance
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-modeling/test-fixtures/stress-corpus/README.md
  - monstertruck-modeling/test-fixtures/stress-corpus/self_intersecting.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/near_zero_area.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/deeply_nested.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/degenerate.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/mod.rs
  - monstertruck-modeling/tests/font_stress_corpus.rs
autonomous: true
must_haves:
  truths:
    - "Developer runs `cargo nextest run -p monstertruck-modeling --features font font_stress` and all stress corpus regression tests pass or document known limitations"
    - "Each corpus entry has a documented failure mode description explaining what pathological geometry it tests"
    - "At least 10 distinct pathological fixture entries exist in the corpus"
    - "Self-intersecting contour fixtures exercise the pipeline with overlapping Bezier segments"
    - "Near-zero-area loop fixtures test contours with negligibly small enclosed regions"
    - "Deeply nested hole fixtures test contours with 3+ levels of containment"
    - "Degenerate geometry fixtures test coincident control points, single-point degeneracies, and reverse-wound holes"
  artifacts:
    - path: "monstertruck-modeling/test-fixtures/stress-corpus/README.md"
      provides: "Documentation of each corpus entry with failure mode descriptions"
      min_lines: 50
      contains: "self-intersecting"
    - path: "monstertruck-modeling/tests/font_stress_corpus.rs"
      provides: "Regression tests for all stress corpus entries"
      min_lines: 200
      contains: "stress"
    - path: "monstertruck-modeling/test-fixtures/stress-corpus/mod.rs"
      provides: "Module that exposes all pathological fixture constructors"
      min_lines: 20
      contains: "pub fn"
  key_links:
    - from: "monstertruck-modeling/test-fixtures/stress-corpus/mod.rs"
      to: "monstertruck-modeling/tests/font_stress_corpus.rs"
      via: "test imports fixture constructors to exercise pathological geometry"
      pattern: "stress_corpus"
---

<objective>
Create a curated corpus of at least 10 pathological font geometry fixtures with documented failure modes, and regression tests that either pass or document known limitations with tracking comments.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-modeling/src/text.rs
@monstertruck-modeling/tests/font_pipeline.rs
@monstertruck-modeling/src/profile.rs
@monstertruck-modeling/src/builder.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create pathological geometry fixture constructors (10+ entries)</name>
  <files>monstertruck-modeling/test-fixtures/stress-corpus/mod.rs, monstertruck-modeling/test-fixtures/stress-corpus/self_intersecting.rs, monstertruck-modeling/test-fixtures/stress-corpus/near_zero_area.rs, monstertruck-modeling/test-fixtures/stress-corpus/deeply_nested.rs, monstertruck-modeling/test-fixtures/stress-corpus/degenerate.rs</files>
  <action>
Create a `stress-corpus/` directory under `monstertruck-modeling/test-fixtures/` with synthetic pathological font geometry fixtures. These fixtures use `builder::vertex`, `builder::line`, and `builder::bezier` to construct `Wire` objects that simulate problematic font outlines.

Create `mod.rs` that re-exports all fixture sub-modules and provides a convenience function `all_fixtures() -> Vec<(&'static str, Vec<Wire>)>` returning all named fixtures.

Create at least 10 fixture constructors across four sub-modules:

1. **self_intersecting.rs** — 3 fixtures:
   - `self_intersecting_cubic()` — A figure-8 contour where cubic Bezier segments cross each other, constructed via `builder::bezier` with control points that force a crossing. Returns `Vec<Wire>` with one self-intersecting wire.
   - `bow_tie_contour()` — Two triangular loops sharing a single vertex, creating a self-intersecting outline via `builder::line` edges that cross at a shared point.
   - `overlapping_contours()` — Two separate closed wires whose bounding boxes overlap significantly (e.g., two rectangles offset by half their width). Tests containment classification when contours overlap.

2. **near_zero_area.rs** — 3 fixtures:
   - `near_zero_area_sliver()` — A long, extremely thin rectangular wire (width ~1e-8) with near-zero enclosed area, constructed via `builder::line`.
   - `collapsed_quad_bezier()` — A contour built from quadratic Beziers (via `builder::bezier` with one interior control point) where the control points nearly coincide with the endpoints (offset by ~1e-10), producing degenerate micro-curves.
   - `micro_feature_loop()` — A tiny closed contour (all coordinates within a 1e-6 x 1e-6 bounding box) simulating a sub-pixel glyph feature. Tests whether the pipeline preserves or filters micro-features.

3. **deeply_nested.rs** — 2 fixtures:
   - `deeply_nested_holes()` — Returns a `Vec<Wire>` with 5 concentric rectangular wires simulating deeply nested containment (outer > hole > island > inner-hole > innermost-island). Uses `builder::line` with decreasing rectangle sizes.
   - `high_loop_count()` — Returns a `Vec<Wire>` with 20+ small non-overlapping rectangular wires arranged in a grid pattern inside a large outer rectangle. Tests scalability of containment classification with many contours.

4. **degenerate.rs** — 3 fixtures:
   - `coincident_control_points()` — A closed contour of cubic Bezier edges where all control points coincide with one endpoint, effectively making each edge a near-degenerate line. Tests whether the pipeline handles Bezier edges that collapse to points.
   - `reverse_wound_hole()` — An outer CCW rectangle with an inner rectangle that is explicitly wound CW (via `wire.inverse()`), testing that normalization correctly handles pre-wound holes without double-inverting.
   - `single_point_degeneracy()` — A wire where two consecutive vertices are at the same point (within tolerance ~1e-12), creating a zero-length edge. Tests graceful handling of degenerate edge geometry.

Each fixture function returns `Vec<Wire>` (using `monstertruck_modeling::Wire`). Include doc comments on each function explaining:
- What pathological geometry it represents
- What failure mode it targets (e.g., "Tests whether profile normalization handles self-intersection without panicking")
- Real-world font analogs if applicable (e.g., "Similar to certain CJK glyphs with complex stroke crossings")

Use `builder::vertex`, `builder::line`, `builder::bezier` to construct edges and wires. Follow functional Rust style per AGENTS.md — iterator chains, `collect()`, direct initialization.

Note: The `builder::bezier` function signature is `bezier(vertex0, vertex1, inter_points: Vec<Point3>)` where `inter_points` are the interior control points (1 point = quadratic, 2 points = cubic).
  </action>
  <verify>Files exist. Each fixture function compiles and returns `Vec&lt;Wire&gt;`. At least 10 distinct fixture constructors exist across the modules. `cargo check -p monstertruck-modeling --features font` succeeds. Count constructors by grepping for `pub fn` in the stress-corpus directory.</verify>
  <done>Eleven pathological font geometry fixture constructors created across four modules with doc comments describing failure modes.</done>
</task>

<task type="auto">
  <name>Task 2: Write regression tests for all corpus entries plus real-font stress tests</name>
  <files>monstertruck-modeling/tests/font_stress_corpus.rs</files>
  <action>
Create `monstertruck-modeling/tests/font_stress_corpus.rs` with regression tests for each stress corpus fixture. The test file should:

1. Import fixtures from the `test-fixtures/stress-corpus/` module (using `#[path]` attribute: `#[path = "../test-fixtures/stress-corpus/mod.rs"] mod stress_corpus;`).

2. For each of the 11 fixture constructors, write a dedicated test that:
   - Constructs the pathological wires using the fixture function
   - Asserts all returned wires are closed (`w.is_closed()`) — except for fixtures that intentionally produce open wires, which should be documented
   - Asserts wire count is as expected for the fixture
   - Attempts `profile::attach_plane_normalized(wires)` and either:
     - Asserts success if the pipeline handles it correctly, OR
     - Catches the error and documents it with a `// KNOWN LIMITATION: ...` comment plus a `// TODO(font-stress): Track issue for ...` comment
   - For fixtures that produce valid faces, attempts `profile::solid_from_planar_profile` extrusion and runs `profile::validate_solid` on the result
   - Records wire count and edge count per wire in test assertions or debug output

3. Include a `stress_all_fixtures_no_panic` test that iterates `stress_corpus::all_fixtures()` and verifies that constructing each fixture does not panic, even if downstream pipeline operations fail gracefully.

4. Include tests for real font glyphs that exercise stress patterns using the existing `DejaVuSans.ttf`:
   - `stress_glyph_at_sign` — The `@` glyph, which has complex nested contours
   - `stress_glyph_ampersand` — The `&` glyph, which has self-intersecting-like curves
   - `stress_glyph_percent` — The `%` glyph, which has multiple small circular contours

5. Add a `stress_dejavusans_full_ascii` test that iterates all printable ASCII characters (0x21..=0x7E), calls `glyph_profile` on each, and asserts no panics occur. This serves as a fuzz-like regression net.

Use `#![cfg(feature = "font")]` at the top of the file.

Test naming convention: prefix all tests with `font_stress_` for easy filtering.
  </action>
  <verify>`cargo nextest run -p monstertruck-modeling --features font font_stress` runs all tests. Known limitation tests have appropriate comments. Count tests by grepping for `#[test]` — expect at least 15 tests (11 fixtures + all_fixtures + 3 real glyphs + ASCII sweep).</verify>
  <done>Regression tests written for all 11 corpus entries, plus iterator test, DejaVuSans stress glyph tests, and full-ASCII sweep.</done>
</task>

<task type="auto">
  <name>Task 3: Document corpus with failure mode catalog</name>
  <files>monstertruck-modeling/test-fixtures/stress-corpus/README.md</files>
  <action>
Create `monstertruck-modeling/test-fixtures/stress-corpus/README.md` documenting the corpus:

1. **Overview** — Purpose of the stress corpus and how to run the tests.

2. **Corpus Entries Table** — A table with columns:
   - Entry name (fixture function name)
   - Source file
   - Failure mode category (self-intersection, degenerate area, deep nesting, degenerate geometry, etc.)
   - Description of pathological geometry
   - Current status (pass / known-limitation with description)

   Include all 11 synthetic fixtures plus the 3 real-glyph stress tests (14 entries total).

3. **Failure Mode Categories** — Detailed description of each category:
   - Self-intersecting contours: where edges cross, creating ambiguous interior
   - Overlapping contours: separate wires with overlapping bounding boxes
   - Near-zero-area loops: contours whose enclosed area approaches machine epsilon
   - Micro-features: sub-pixel geometry that may be below processing thresholds
   - Degenerate Bezier curves: control points coincident with endpoints
   - Deeply nested holes: 3+ levels of containment hierarchy
   - High contour counts: many wires in a single profile
   - Reverse-wound holes: pre-inverted hole wires
   - Single-point degeneracies: zero-length edges
   - Coincident control points: Bezier edges that collapse to points
   - Complex real-world glyphs: `@`, `&`, `%` from DejaVuSans

4. **Adding New Entries** — Instructions for adding new fixtures to the corpus:
   - Create the fixture function in the appropriate sub-module
   - Add it to the `all_fixtures()` list in `mod.rs`
   - Write a dedicated test in `font_stress_corpus.rs`
   - Update this README table

5. **Running Tests** — Command to run: `cargo nextest run -p monstertruck-modeling --features font font_stress`
  </action>
  <verify>README.md exists with at least 10 corpus entries documented. All categories are described. The table has 14+ entries (11 synthetic + 3 real-glyph).</verify>
  <done>Stress corpus README created with failure mode catalog documenting all 14 entries and their status.</done>
</task>

</tasks>

<verification>
1. At least 10 distinct pathological fixture constructors exist in `stress-corpus/` (target: 11)
2. All fixtures compile: `cargo check -p monstertruck-modeling --features font`
3. Regression tests run: `cargo nextest run -p monstertruck-modeling --features font font_stress`
4. Each corpus entry has a documented failure mode in the README
5. Known limitations have `TODO(font-stress)` tracking comments
6. Full-ASCII sweep test covers all printable characters without panics
7. The `all_fixtures()` iterator function returns all 11 synthetic fixtures
8. README table has 14+ documented entries (11 synthetic + 3 real-glyph)
</verification>

<success_criteria>
- A curated corpus of at least 10 pathological font fixtures exists with documented failure modes
- Each corpus entry has a regression test that either passes or documents a known limitation with a tracking comment
- Failure categories include self-intersecting contours, near-zero-area loops, deeply nested holes, overlapping contours, degenerate Bezier curves, micro-features, coincident control points, reverse-wound holes, single-point degeneracies, and high loop counts
- The corpus is extensible with clear instructions for adding new entries
</success_criteria>

<output>
After completion, create `.tendrion/phases/15-font-stress-testing-and-performance/15-1-SUMMARY.md`
</output>
