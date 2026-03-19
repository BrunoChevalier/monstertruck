---
target: 15-2
type: implementation
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: code-quality
date: 2026-03-19
verdict: PASS
---

## Verdict

**PASS** -- No blockers found. The benchmark code is well-structured, follows project conventions, compiles and passes all test harness checks, and clippy reports no warnings for this crate. Two suggestions and two nits are noted below.

## Findings

### Blockers

None

### Suggestions

#### S1: Stress corpus benchmarks measure clone + computation [confidence: 88]
- **Confidence:** 88
- **File:** monstertruck-modeling/benches/font_profile_bench.rs:122-126
- **Issue:** The `wires.clone()` on line 124 runs inside the benchmark iteration, meaning the benchmark measures clone cost plus `attach_plane_normalized` cost together. Criterion provides `iter_batched` specifically for cases where the measured function consumes its input. Using `iter_batched(|| wires.clone(), |w| black_box(profile::attach_plane_normalized::<Curve, Surface>(w)), BatchSize::SmallInput)` would separate setup from measurement and yield more accurate throughput numbers.
- **Impact:** Benchmark results will include clone overhead, making the normalization step appear slower than it actually is. For small fixtures the clone cost may be negligible, but for larger stress fixtures it could skew results.
- **Suggested fix:** Replace `b.iter(|| { let w = wires.clone(); ... })` with `b.iter_batched(|| wires.clone(), |w| black_box(profile::attach_plane_normalized::<Curve, Surface>(w)), criterion::BatchSize::SmallInput)`.

#### S2: Unnecessary explicit Bencher type annotations [confidence: 86]
- **Confidence:** 86
- **File:** monstertruck-modeling/benches/font_profile_bench.rs:148,161
- **Issue:** Lines 148 and 161 use `|b: &mut criterion::Bencher<'_, WallTime>|` while all other closures in the same file and all other benchmarks in the project use the simpler `|b|` form. The explicit annotation provides no value (type inference handles it) and causes the lines to exceed the formatter's width, resulting in `cargo fmt` diffs. The `WallTime` import on line 12 is also only needed for these annotations.
- **Impact:** Inconsistency with the rest of the benchmark file and project benchmarks. Creates unnecessary formatting drift.
- **Suggested fix:** Replace `|b: &mut criterion::Bencher<'_, WallTime>|` with `|b|` on both lines and remove `measurement::WallTime` from the import.

### Nits

#### N1: cargo fmt diffs in new file [confidence: 91]
- **Confidence:** 91
- **File:** monstertruck-modeling/benches/font_profile_bench.rs:148,161,168-183
- **Issue:** The file has three areas that differ from `cargo fmt` output: the explicit Bencher type annotations cause line-length reformatting, and the multi-line `criterion_group!` macro invocations would be collapsed to single lines by the formatter. Running `cargo fmt` would fix these.

#### N2: Test clone on line 119 is redundant given the filter-then-bench pattern [confidence: 72]
- **Confidence:** 72
- **File:** monstertruck-modeling/benches/font_profile_bench.rs:119-120
- **Issue:** The test clone on line 119 checks whether `attach_plane_normalized` succeeds before setting up the benchmark. Since this runs once at setup time, the cost is immaterial, but it could be simplified by using a `filter` + `for_each` iterator chain per the project's functional style preference.

## Summary

The benchmark suite is well-organized into four logical groups, follows established project patterns (`#[path]` for fixtures, `include_bytes!` for font data, `criterion_group!`/`criterion_main!`), and all benchmark test harness runs pass. The code is readable with good comments and doc-comments. The main quality improvement opportunity is using `iter_batched` for the stress corpus benchmarks to avoid measuring clone overhead, and running `cargo fmt` to resolve minor formatting drift.
