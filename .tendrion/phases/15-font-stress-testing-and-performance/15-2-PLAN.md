---
phase: 15-font-stress-testing-and-performance
plan: 2
type: execute
wave: 2
depends_on: ["15-1"]
files_modified:
  - monstertruck-modeling/benches/font_profile_bench.rs
  - monstertruck-modeling/Cargo.toml
  - monstertruck-modeling/benches/BASELINE.md
autonomous: true
must_haves:
  truths:
    - "Developer runs `cargo bench -p monstertruck-modeling --features font` and gets throughput measurements for 1, 10, 100, and 1000 character text strings"
    - "Benchmark results include both glyph_profile (single glyph) and text_profile (multi-character) measurements"
    - "Baseline results are recorded in a BASELINE.md file with instructions for re-running"
    - "Benchmarks compile and run without --release flag for verification purposes"
    - "Stress corpus fixtures are also benchmarked for pathological geometry throughput"
  artifacts:
    - path: "monstertruck-modeling/benches/font_profile_bench.rs"
      provides: "Criterion benchmark suite for font profile pipeline throughput"
      min_lines: 100
      contains: "criterion_group"
    - path: "monstertruck-modeling/Cargo.toml"
      provides: "Updated with criterion dev-dependency and [[bench]] entry"
      min_lines: 35
      contains: "font_profile_bench"
    - path: "monstertruck-modeling/benches/BASELINE.md"
      provides: "Recorded baseline benchmark results with re-run instructions"
      min_lines: 30
      contains: "baseline"
  key_links:
    - from: "monstertruck-modeling/benches/font_profile_bench.rs"
      to: "monstertruck-modeling/src/text.rs"
      via: "benchmarks call glyph_profile and text_profile"
      pattern: "text_profile"
    - from: "monstertruck-modeling/benches/font_profile_bench.rs"
      to: "monstertruck-modeling/test-fixtures/stress-corpus/mod.rs"
      via: "benchmarks include stress corpus fixtures for pathological geometry throughput"
      pattern: "stress_corpus"
---

<objective>
Create a criterion benchmark suite measuring font profile pipeline throughput for text strings of 1, 10, 100, and 1000 characters, plus pathological geometry from the stress corpus, and record baseline results.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-modeling/src/text.rs
@monstertruck-modeling/Cargo.toml
@monstertruck-geometry/benches/nurbs_eval.rs
@monstertruck-meshing/benches/tessellation.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add criterion dependency and bench target to monstertruck-modeling</name>
  <files>monstertruck-modeling/Cargo.toml</files>
  <action>
Update `monstertruck-modeling/Cargo.toml` to add:

1. Add `criterion` to `[dev-dependencies]` using the workspace version: `criterion = { workspace = true }`

2. Add a `[[bench]]` section:
```toml
[[bench]]
name = "font_profile_bench"
harness = false
required-features = ["font"]
```

3. Also add `ttf-parser` to dev-dependencies (needed for bench to parse the font fixture without the font feature's dep path): `ttf-parser = { workspace = true }`

Follow the same pattern as `monstertruck-geometry/Cargo.toml` which already has criterion benchmarks.
  </action>
  <verify>`cargo check -p monstertruck-modeling --features font --benches` succeeds (after Task 2 creates the bench file).</verify>
  <done>Cargo.toml updated with criterion dev-dependency and bench target for font_profile_bench.</done>
</task>

<task type="auto">
  <name>Task 2: Create criterion benchmark suite for font profile throughput</name>
  <files>monstertruck-modeling/benches/font_profile_bench.rs</files>
  <action>
Create `monstertruck-modeling/benches/font_profile_bench.rs` with criterion benchmarks:

1. **Setup**: Load DejaVuSans.ttf via `include_bytes!` (same pattern as `font_pipeline.rs`). Parse the face. Create default `TextOptions`.

2. **Single glyph benchmarks** (`bench_glyph_profile` group):
   - `glyph_profile/simple_l` — Benchmark `glyph_profile` for 'l' (simple, 1 contour)
   - `glyph_profile/complex_B` — Benchmark `glyph_profile` for 'B' (3 contours with holes)
   - `glyph_profile/complex_at` — Benchmark `glyph_profile` for '@' (complex nested contours)

3. **Text string benchmarks** (`bench_text_profile` group) — the core FONT-04 requirement:
   - `text_profile/1_char` — Single character "H"
   - `text_profile/10_chars` — 10 characters "HelloWorld"
   - `text_profile/100_chars` — 100 characters (repeat a phrase to get 100 chars)
   - `text_profile/1000_chars` — 1000 characters (repeat a phrase to get 1000 chars)

   Use `BenchmarkGroup` with `criterion::BenchmarkId` for parameterized benchmarks. Generate the test strings programmatically by repeating a base string to reach target lengths.

4. **Stress corpus benchmarks** (`bench_stress_corpus` group):
   - Include the stress corpus fixtures from `test-fixtures/stress-corpus/mod.rs` using `#[path]` attribute
   - Benchmark `profile::attach_plane_normalized` on each stress fixture's wires
   - Only benchmark fixtures that succeed (skip those with known limitations)

5. **Full pipeline benchmarks** (`bench_full_pipeline` group):
   - `full_pipeline/glyph_to_solid` — Benchmark the full path: `glyph_profile` -> `solid_from_planar_profile` for glyph 'O'
   - `full_pipeline/text_to_wires_100` — Benchmark `text_profile` for 100-char string

Use `black_box` on all inputs and outputs. Follow the pattern from `monstertruck-meshing/benches/tessellation.rs`.

Use functional style per AGENTS.md. Use `criterion_group!` and `criterion_main!` macros.

Configure measurement time appropriately — use `.measurement_time(std::time::Duration::from_secs(10))` for the longer benchmarks (100+ chars) and `.sample_size(50)` for very slow ones (1000 chars).
  </action>
  <verify>`cargo test --benches -p monstertruck-modeling --features font` compiles and the bench test harness runs (per AGENTS.md: do NOT use `cargo bench` for verification, use `cargo test --benches`).</verify>
  <done>Criterion benchmark suite created with glyph_profile, text_profile (1/10/100/1000), stress corpus, and full pipeline groups.</done>
</task>

<task type="auto">
  <name>Task 3: Record baseline results and create re-run instructions</name>
  <files>monstertruck-modeling/benches/BASELINE.md</files>
  <action>
After verifying benchmarks compile, run the benchmarks once to establish a baseline:

```bash
cargo bench -p monstertruck-modeling --features font -- --output-format=bencher 2>&1
```

Create `monstertruck-modeling/benches/BASELINE.md` with:

1. **Date and Environment** — Record the date, Rust version (`rustc --version`), CPU info, and OS.

2. **Results Table** — A markdown table with columns:
   - Benchmark name
   - Time (ns/iter or us/iter as appropriate)
   - Throughput (chars/sec for text_profile benchmarks)

3. **Interpretation** — Brief notes on:
   - How throughput scales with text length (linear? sub-linear due to parallelism?)
   - Which stress corpus fixtures are significantly slower and why
   - Any unexpected results

4. **Re-run Instructions**:
   ```
   # Run all font benchmarks
   cargo bench -p monstertruck-modeling --features font

   # Run a specific benchmark group
   cargo bench -p monstertruck-modeling --features font -- text_profile

   # Compare against baseline
   cargo bench -p monstertruck-modeling --features font -- --baseline main

   # Save as new baseline
   cargo bench -p monstertruck-modeling --features font -- --save-baseline main
   ```

5. **CI Integration Notes** — Suggestions for integrating into CI (e.g., criterion's `--output-format=bencher` for machine parsing, regression detection thresholds).

Note: If `cargo bench` cannot run in the current environment (e.g., no hardware access), record placeholder results with a note to run on actual hardware. The key requirement is that the benchmark code compiles and the BASELINE.md has the structure for recording results.
  </action>
  <verify>BASELINE.md exists with the required sections. Benchmark results (or placeholder structure) are recorded.</verify>
  <done>Baseline results recorded in BASELINE.md with re-run instructions and interpretation notes.</done>
</task>

</tasks>

<verification>
1. `monstertruck-modeling/Cargo.toml` has criterion dev-dependency and `[[bench]]` entry with `required-features = ["font"]`
2. `cargo test --benches -p monstertruck-modeling --features font` compiles and passes
3. Benchmark suite measures throughput for 1, 10, 100, and 1000 character text strings
4. Stress corpus fixtures are also benchmarked
5. BASELINE.md records results with re-run instructions
6. Benchmark follows the established pattern from other crates (criterion_group, criterion_main, black_box)
</verification>

<success_criteria>
- A benchmark suite measures profile pipeline throughput for text strings of 1, 10, 100, and 1000 characters
- Benchmark results are recorded as a baseline with instructions for re-running
- Stress corpus fixtures from Plan 15-1 are included in benchmarks
- Benchmarks follow project conventions (criterion, black_box, functional style)
</success_criteria>

<output>
After completion, create `.tendrion/phases/15-font-stress-testing-and-performance/15-2-SUMMARY.md`
</output>
