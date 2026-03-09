---
phase: 2-numerical-robustness
plan: 2
tags: [fuzzing, cargo-fuzz, nurbs, knot-vector, step-parser]
key-files:
  - monstertruck-geometry/fuzz/Cargo.toml
  - monstertruck-geometry/fuzz/fuzz_targets/nurbs_eval.rs
  - monstertruck-geometry/fuzz/fuzz_targets/knot_vector.rs
  - monstertruck-step/fuzz/Cargo.toml
  - monstertruck-step/fuzz/fuzz_targets/step_parse.rs
  - monstertruck-geometry/fuzz/corpus/nurbs_eval/.gitkeep
  - monstertruck-geometry/fuzz/corpus/knot_vector/.gitkeep
  - monstertruck-step/fuzz/corpus/step_parse/minimal.step
decisions: []
metrics:
  tests_before: 84
  tests_after: 84
  fuzz_targets: 3
---

## What Was Built

### Geometry Fuzz Targets (monstertruck-geometry/fuzz/)

- **Cargo.toml**: Standalone workspace with `libfuzzer-sys` and `arbitrary` dependencies, declaring two binary targets (`nurbs_eval`, `knot_vector`).
- **fuzz_targets/nurbs_eval.rs**: Structured fuzzing of `BsplineCurve` evaluation. Uses `Arbitrary` to generate degree (1--5), control points (as `Vector4` with weight 1.0), and parameter `t`. Exercises `subs()`, `der()`, and `der2()` via `ParametricCurve` trait. Filters NaN/Inf inputs and uses `try_new` to avoid panics on invalid configurations.
- **fuzz_targets/knot_vector.rs**: Structured fuzzing of `KnotVector` operations. Exercises `try_bspline_basis_functions`, `add_knot`, `multiplicity`, `range_length`, `try_normalize`, `invert`, `to_single_multi`, and `from_single_multi`. Filters NaN/Inf knot values.

### STEP Parser Fuzz Target (monstertruck-step/fuzz/)

- **Cargo.toml**: Standalone workspace with `libfuzzer-sys` and `ruststep = "0.4"` (matching workspace version), declaring one binary target (`step_parse`).
- **fuzz_targets/step_parse.rs**: Byte-level fuzzing of `ruststep::parser::parse`. Converts arbitrary bytes to UTF-8, feeds to parser, and exercises data section iteration on successful parses.

### Seed Corpora

- `monstertruck-geometry/fuzz/corpus/nurbs_eval/.gitkeep`
- `monstertruck-geometry/fuzz/corpus/knot_vector/.gitkeep`
- `monstertruck-step/fuzz/corpus/step_parse/minimal.step` -- minimal valid STEP file as seed.

## Deviations

1. **TDD exemption**: Fuzz targets are test infrastructure (binary harnesses), not library code with testable behavior. TDD RED-GREEN cycle not applicable.
2. **Nightly unavailable**: `cargo-fuzz` requires nightly Rust. Only stable toolchain installed. Fuzz targets verified via `cargo build` in standalone fuzz workspaces. Short fuzz runs (5-second validation) skipped.

## Verification

- All three fuzz targets compile successfully with `cargo build` in their respective fuzz directories.
- All 84 existing `monstertruck-geometry` tests pass.
- `cargo fmt --all` applied.
- Seed corpus directories exist for all three targets.

## Self-Check

- [x] `monstertruck-geometry/fuzz/Cargo.toml` exists with two targets
- [x] `monstertruck-step/fuzz/Cargo.toml` exists with one target
- [x] All fuzz targets compile without errors
- [x] Seed corpus directories present
- [x] Existing tests unaffected (84/84 pass)
