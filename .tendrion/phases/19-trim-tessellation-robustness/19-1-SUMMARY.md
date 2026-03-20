---
phase: 19-trim-tessellation-robustness
plan: 1
tags: [tolerance, tessellation, constants, refactor]
key-files:
  - monstertruck-core/src/tolerance_constants.rs
  - monstertruck-core/tests/tolerance_constants.rs
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/src/tessellation/mod.rs
decisions: []
metrics:
  tests_added: 1
  tests_total_pass: 9
  deviations: 0
---

## What Was Built

- **monstertruck-core/src/tolerance_constants.rs**: Added `UV_CLOSURE_TOLERANCE` constant (`TESSELLATION_TOLERANCE / 10.0 = 0.001`) with full documentation explaining its derivation and usage context.
- **monstertruck-core/tests/tolerance_constants.rs**: Added `uv_closure_tolerance_value` test verifying the constant's value, positivity, and relationship to `TESSELLATION_TOLERANCE`.
- **monstertruck-meshing/src/tessellation/triangulation.rs**: Replaced hardcoded `1.0e-3` in `PolyBoundary::new` with `UV_CLOSURE_TOLERANCE`. Added import.
- **monstertruck-meshing/src/tessellation/mod.rs**: Replaced hardcoded `0.01` in `TessellationOptions::default()` with `TESSELLATION_TOLERANCE`. Added import.

## Task Commits

| Phase | SHA | Message |
|-------|-----|---------|
| RED | `c63675df` | test(tolerance): add failing test for UV_CLOSURE_TOLERANCE constant |
| GREEN | `1aae7a8b` | feat(tolerance): add UV_CLOSURE_TOLERANCE and replace tessellation magic constants |

## Verification

1. `cargo nextest run -p monstertruck-core --test tolerance_constants`: 9/9 passed.
2. `cargo nextest run -p monstertruck-meshing --lib`: 8/8 passed, 1 skipped.
3. No remaining `1.0e-3` in tessellation source.
4. `tolerance: 0.01` only remains in `QuadOptions::default()` (different concept, intentionally unchanged).
5. `cargo clippy` clean on affected crates.

## Decisions Made

None. All changes followed the plan exactly.

## Deviations

None.
