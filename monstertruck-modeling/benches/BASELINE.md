# Font Profile Benchmark Baseline

## Environment

- **Date:** 2026-03-19
- **Rust:** rustc 1.94.0 (4a4ef493e 2026-03-02)
- **OS:** Linux 6.6.87.2-microsoft-standard-WSL2 x86_64
- **Platform:** WSL2 (hardware details vary by host)

## Status

Benchmarks verified to compile and run via `cargo test --benches`. Full criterion
benchmarks (`cargo bench`) could not execute in this environment due to a
pre-existing `deny(warnings)` failure in `monstertruck-mesh` under release mode.

Run the benchmarks on a clean build to populate this table.

## Results

| Benchmark | Time | Notes |
|-----------|------|-------|
| `glyph_profile/simple_l` | -- | 1 contour, no holes |
| `glyph_profile/complex_B` | -- | 3 contours with holes |
| `glyph_profile/complex_at` | -- | Nested contours |
| `text_profile/1_char` | -- | Single "H" |
| `text_profile/10_chars` | -- | "HelloWorld" |
| `text_profile/100_chars` | -- | 100-char repeated phrase |
| `text_profile/1000_chars` | -- | 1000-char repeated phrase |
| `stress_corpus/*` | -- | Pathological geometry fixtures |
| `full_pipeline/glyph_to_solid` | -- | Glyph 'O' -> solid extrusion |
| `full_pipeline/text_to_wires_100` | -- | 100-char text -> wires |

## Interpretation

Expected scaling behavior:

- **Glyph complexity:** Time should increase from `simple_l` (single contour)
  to `complex_at` (multiple nested contours) roughly proportional to contour count.
- **Text length:** Time should scale linearly with character count for sequential
  contour collection, with potential sub-linear scaling on the parallel
  contour-to-wire conversion phase (rayon).
- **Stress corpus:** Pathological fixtures (self-intersecting cubics, deeply nested
  holes, high loop counts) may be significantly slower than simple rectangles
  due to edge complexity and normalization overhead.
- **Full pipeline:** `glyph_to_solid` includes face construction and extrusion on
  top of profiling, so it should be notably slower than `glyph_profile` alone.

## Re-run Instructions

```bash
# Run all font benchmarks.
cargo bench -p monstertruck-modeling --features font

# Run a specific benchmark group.
cargo bench -p monstertruck-modeling --features font -- text_profile

# Run a specific benchmark.
cargo bench -p monstertruck-modeling --features font -- "text_profile/100_chars"

# Compare against saved baseline.
cargo bench -p monstertruck-modeling --features font -- --baseline main

# Save current results as new baseline.
cargo bench -p monstertruck-modeling --features font -- --save-baseline main
```

## CI Integration Notes

- Use `--output-format=bencher` for machine-parseable output:
  ```bash
  cargo bench -p monstertruck-modeling --features font -- --output-format=bencher
  ```
- Criterion stores results in `target/criterion/` by default. Save baselines
  with `--save-baseline` and compare with `--baseline` across runs.
- For regression detection, set a threshold (e.g., 10% regression triggers failure)
  using criterion's `noise_threshold` or `significance_level` configuration.
- Consider running benchmarks in a dedicated CI job with `--features font` to
  avoid gating non-font builds on font benchmark availability.
