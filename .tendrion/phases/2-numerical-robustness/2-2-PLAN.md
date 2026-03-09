---
phase: 2-numerical-robustness
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/fuzz/Cargo.toml
  - monstertruck-geometry/fuzz/fuzz_targets/nurbs_eval.rs
  - monstertruck-geometry/fuzz/fuzz_targets/knot_vector.rs
  - monstertruck-step/fuzz/Cargo.toml
  - monstertruck-step/fuzz/fuzz_targets/step_parse.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo fuzz run nurbs_eval for 60 seconds against NURBS evaluation and it completes without crashes"
    - "User runs cargo fuzz run knot_vector for 60 seconds against knot vector manipulation and it completes without crashes"
    - "User runs cargo fuzz run step_parse for 60 seconds against STEP parsing and it completes without crashes"
    - "Fuzz targets exercise critical code paths: BsplineCurve::subs, KnotVector::bspline_basis_functions, ruststep::parser::parse"
  artifacts:
    - path: "monstertruck-geometry/fuzz/Cargo.toml"
      provides: "Cargo configuration for geometry fuzzing targets"
      min_lines: 15
      contains: "cargo-fuzz"
    - path: "monstertruck-geometry/fuzz/fuzz_targets/nurbs_eval.rs"
      provides: "Fuzz target exercising BsplineCurve and NurbsCurve evaluation with arbitrary inputs"
      min_lines: 40
      contains: "fuzz_target"
    - path: "monstertruck-geometry/fuzz/fuzz_targets/knot_vector.rs"
      provides: "Fuzz target exercising KnotVector construction, manipulation, and basis function evaluation"
      min_lines: 40
      contains: "fuzz_target"
    - path: "monstertruck-step/fuzz/Cargo.toml"
      provides: "Cargo configuration for STEP parsing fuzzing target"
      min_lines: 15
      contains: "cargo-fuzz"
    - path: "monstertruck-step/fuzz/fuzz_targets/step_parse.rs"
      provides: "Fuzz target exercising STEP file parsing with arbitrary byte sequences"
      min_lines: 25
      contains: "fuzz_target"
  key_links:
    - from: "monstertruck-geometry/fuzz/fuzz_targets/nurbs_eval.rs"
      to: "monstertruck-geometry/src/nurbs/bspline_curve.rs"
      via: "Fuzz target calls BsplineCurve::subs and NurbsCurve evaluation"
      pattern: "BsplineCurve"
    - from: "monstertruck-geometry/fuzz/fuzz_targets/knot_vector.rs"
      to: "monstertruck-geometry/src/nurbs/knot_vector.rs"
      via: "Fuzz target calls KnotVector methods"
      pattern: "KnotVector"
    - from: "monstertruck-step/fuzz/fuzz_targets/step_parse.rs"
      to: "monstertruck-step/src/lib.rs"
      via: "Fuzz target feeds arbitrary bytes to STEP parser"
      pattern: "ruststep"
---

<objective>
Create cargo-fuzz targets for NURBS evaluation, knot vector manipulation, and STEP parsing to catch panics, out-of-bounds accesses, and assertion failures in the numerical core under adversarial inputs.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/mod.rs
@monstertruck-geometry/src/nurbs/knot_vector.rs
@monstertruck-geometry/src/nurbs/bspline_curve.rs
@monstertruck-step/src/lib.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create geometry fuzzing targets for NURBS and knot vectors</name>
  <files>monstertruck-geometry/fuzz/Cargo.toml, monstertruck-geometry/fuzz/fuzz_targets/nurbs_eval.rs, monstertruck-geometry/fuzz/fuzz_targets/knot_vector.rs</files>
  <action>
Create the fuzz directory structure for monstertruck-geometry:

**monstertruck-geometry/fuzz/Cargo.toml:**
```toml
[package]
name = "monstertruck-geometry-fuzz"
version = "0.0.0"
publish = false
edition = "2021"

[package.metadata]
cargo-fuzz = true

[dependencies]
libfuzzer-sys = "0.4"
arbitrary = { version = "1", features = ["derive"] }

[dependencies.monstertruck-geometry]
path = ".."

[[bin]]
name = "nurbs_eval"
path = "fuzz_targets/nurbs_eval.rs"
doc = false

[[bin]]
name = "knot_vector"
path = "fuzz_targets/knot_vector.rs"
doc = false

[workspace]
members = ["."]
```

**monstertruck-geometry/fuzz/fuzz_targets/nurbs_eval.rs:**
- Use `arbitrary::Arbitrary` to derive structured input: degree (u8, clamped to 1..=5), number of control points (u8, clamped to degree+1..=20), control point coordinates (Vec of f64 triples), parameter t (f64).
- Build a `KnotVector::uniform_knot(degree, n - degree)` and a `BsplineCurve::new(knot_vec, control_points)`.
- Call `curve.subs(t)` with the parameter, catching any panic gracefully (the fuzz harness will detect panics).
- Also test `curve.der(t)` and `curve.der2(t)` if the types support it.
- Filter out NaN/Inf control point values with early return to focus fuzzing on valid-ish inputs.
- Use `monstertruck_geometry::prelude::*`.

**monstertruck-geometry/fuzz/fuzz_targets/knot_vector.rs:**
- Use `arbitrary::Arbitrary` to derive: a Vec<f64> of knot values (length 2..=30), degree (u8), parameter t (f64).
- Construct `KnotVector::from(knots)`.
- Call `try_bspline_basis_functions(degree, 0, t)` -- should never panic.
- Call `add_knot(t)`, `multiplicity(0)`, `range_length()`.
- Test `try_normalize()`, `invert()`, `to_single_multi()`, `from_single_multi()`.
- Filter NaN/Inf values from input knots.
  </action>
  <verify>Run `cd monstertruck-geometry && cargo fuzz build` to verify the targets compile. If cargo-fuzz is not installed, verify with `cd monstertruck-geometry/fuzz && cargo build` instead.</verify>
  <done>Geometry fuzzing targets created for NURBS evaluation and knot vector manipulation.</done>
</task>

<task type="auto">
  <name>Task 2: Create STEP parsing fuzzing target</name>
  <files>monstertruck-step/fuzz/Cargo.toml, monstertruck-step/fuzz/fuzz_targets/step_parse.rs</files>
  <action>
Create the fuzz directory structure for monstertruck-step:

**monstertruck-step/fuzz/Cargo.toml:**
```toml
[package]
name = "monstertruck-step-fuzz"
version = "0.0.0"
publish = false
edition = "2021"

[package.metadata]
cargo-fuzz = true

[dependencies]
libfuzzer-sys = "0.4"

[dependencies.ruststep]
version = "*"

[workspace]
members = ["."]
```

Note: Check the `ruststep` version used in monstertruck-step/Cargo.toml and match it. The fuzz target only needs the parser, not the full monstertruck-step crate, to avoid heavy compilation.

**monstertruck-step/fuzz/fuzz_targets/step_parse.rs:**
- Accept arbitrary `&[u8]` input.
- Convert to `&str` via `std::str::from_utf8()`. If not valid UTF-8, return early.
- Call `ruststep::parser::parse(&input_str)`. This should never panic regardless of input.
- If parsing succeeds, optionally try to extract entities from the first data section to exercise more code paths.
- The target exercises the STEP parser's robustness against malformed input.
  </action>
  <verify>Run `cd monstertruck-step && cargo fuzz build` to verify the target compiles. If cargo-fuzz is not installed, verify with `cd monstertruck-step/fuzz && cargo build` instead.</verify>
  <done>STEP parsing fuzzing target created.</done>
</task>

<task type="auto">
  <name>Task 3: Validate fuzz targets with short runs and add seed corpora</name>
  <files>monstertruck-geometry/fuzz/corpus/nurbs_eval/.gitkeep, monstertruck-geometry/fuzz/corpus/knot_vector/.gitkeep, monstertruck-step/fuzz/corpus/step_parse/minimal.step</files>
  <action>
1. Create seed corpus directories:
   - `monstertruck-geometry/fuzz/corpus/nurbs_eval/` with `.gitkeep`.
   - `monstertruck-geometry/fuzz/corpus/knot_vector/` with `.gitkeep`.
   - `monstertruck-step/fuzz/corpus/step_parse/` with a minimal valid STEP file as seed:
     ```
     ISO-10303-21;
     HEADER;
     FILE_DESCRIPTION((''), '2;1');
     FILE_NAME('test.step', '2024-01-01', (''), (''), '', '', '');
     FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));
     ENDSEC;
     DATA;
     #1=CARTESIAN_POINT('',(0.0,0.0,0.0));
     ENDSEC;
     END-ISO-10303-21;
     ```

2. Run each fuzz target for a brief sanity check (5 seconds) if cargo-fuzz is available:
   - `cd monstertruck-geometry && cargo fuzz run nurbs_eval -- -max_total_time=5`
   - `cd monstertruck-geometry && cargo fuzz run knot_vector -- -max_total_time=5`
   - `cd monstertruck-step && cargo fuzz run step_parse -- -max_total_time=5`

   If cargo-fuzz is not installed, install it with `cargo install cargo-fuzz` first. If installation fails (e.g., nightly not available), document the blocker but still verify the targets compile.

3. If any fuzz target finds a crash during the 5-second run, log it but do not fix it in this plan -- that is the purpose of fuzzing.
  </action>
  <verify>Verify corpus directories exist. If cargo-fuzz ran, confirm no crashes in 5-second runs. If cargo-fuzz is unavailable, confirm the fuzz targets compile with `cargo build`.</verify>
  <done>Seed corpora created and fuzz targets validated with short runs or compilation check.</done>
</task>

</tasks>

<verification>
1. `monstertruck-geometry/fuzz/Cargo.toml` exists and declares two fuzz targets.
2. `monstertruck-step/fuzz/Cargo.toml` exists and declares one fuzz target.
3. All three fuzz targets compile without errors.
4. Seed corpus directories exist for all three targets.
5. A 60-second fuzz run (manual verification) completes without crashes for all targets.
</verification>

<success_criteria>
- ROBUST-05 complete: cargo-fuzz targets exist for NURBS evaluation, knot vector manipulation, and STEP parsing
- All fuzz targets compile and run without immediate crashes
- Seed corpora provide starting points for effective fuzzing
</success_criteria>

<output>
After completion, create `.tendrion/phases/2-numerical-robustness/2-2-SUMMARY.md`
</output>
