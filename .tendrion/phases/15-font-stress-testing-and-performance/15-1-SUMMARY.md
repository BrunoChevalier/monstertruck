---
phase: 15-font-stress-testing-and-performance
plan: 1
tags: [font, stress-testing, fixtures, regression]
key-files:
  - monstertruck-modeling/test-fixtures/stress-corpus/mod.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/self_intersecting.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/near_zero_area.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/deeply_nested.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/degenerate.rs
  - monstertruck-modeling/tests/font_stress_corpus.rs
  - monstertruck-modeling/test-fixtures/stress-corpus/README.md
decisions: []
metrics:
  fixture-constructors: 11
  test-count: 16
  tests-passing: 16
  readme-entries: 14
---

## What Was Built

Created a stress corpus of 11 pathological font geometry fixtures across four categories, with 16 regression tests and a failure mode catalog.

### Files Created

- **`test-fixtures/stress-corpus/mod.rs`** -- Module root exposing `all_fixtures()` and shared `make_rectangle` helper.
- **`test-fixtures/stress-corpus/self_intersecting.rs`** -- 3 fixtures: figure-8 cubic, bow-tie contour, overlapping contours.
- **`test-fixtures/stress-corpus/near_zero_area.rs`** -- 3 fixtures: thin sliver, collapsed quad Bezier, micro-feature loop.
- **`test-fixtures/stress-corpus/deeply_nested.rs`** -- 2 fixtures: 5-level nested holes, 21-wire grid.
- **`test-fixtures/stress-corpus/degenerate.rs`** -- 3 fixtures: coincident control points, reverse-wound hole, zero-length edge.
- **`tests/font_stress_corpus.rs`** -- 16 tests: 11 per-fixture + all_fixtures iterator + 3 real glyph stress tests (@, &, %) + full ASCII sweep.
- **`test-fixtures/stress-corpus/README.md`** -- Failure mode catalog with 14 entries (11 synthetic + 3 real-glyph).

## Task Commits

| SHA | Message |
|-----|---------|
| `8eabb30a` | test(font-stress): add failing tests for pathological font geometry stress corpus |
| `36763623` | feat(font-stress): implement 11 pathological font geometry fixture constructors |
| `fd7c0f7e` | refactor(font-stress): consolidate make_rectangle helper into shared module |
| `9873ad86` | docs(font-stress): add stress corpus README with failure mode catalog |

## Deviations

- Pre-existing compilation error in `intersection_curve_impls.rs` (ambiguous `truncate` method) prevented running `cargo nextest run -p monstertruck-modeling --features font font_stress` without `--test font_stress_corpus` filter. Logged as auto-fix deviation. Not caused by this plan.

## Self-Check

- All 16 tests pass via `cargo nextest run -p monstertruck-modeling --features font --test font_stress_corpus`.
- 11 fixture constructors confirmed via `pub fn` grep across stress-corpus directory.
- README table has 14 documented entries.
- All fixtures compile and return `Vec<Wire>`.
- TDD cycle completed: RED -> GREEN -> REFACTOR.
