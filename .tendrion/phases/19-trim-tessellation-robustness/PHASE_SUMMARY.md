---
phase: 19
title: trim-tessellation-robustness
status: complete
tdd_compliance: 50% (1/2 cycles fully compliant; plan 19-1 missing REFACTOR commit)
---

## What Was Built

**Plan 19-1 (TRIM-02 - Centralized tolerance constants):**
- Added `UV_CLOSURE_TOLERANCE = TESSELLATION_TOLERANCE / 10.0` to `monstertruck-core/src/tolerance_constants.rs`
- Replaced hardcoded `1.0e-3` in `PolyBoundary::new` with `UV_CLOSURE_TOLERANCE`
- Replaced hardcoded `0.01` in `TessellationOptions::default()` with `TESSELLATION_TOLERANCE`
- Added test verifying constant's value and relationship to `TESSELLATION_TOLERANCE`

**Plan 19-2 (TRIM-01 - UV interpolation fallback):**
- Implemented two-pass approach in `PolyBoundaryPiece::try_new_with_fallback`: pass 1 collects parameter search results (including `None` failures), pass 2 interpolates UV for failed points from nearest successful neighbors
- Added `allow_fallback` flag threaded through `shell_create_polygon`, `shell_tessellation`, `shell_tessellation_single_thread`, `cshell_tessellation`
- `robust_triangulation` paths pass `allow_fallback=true`; legacy `triangulation` paths pass `false` for backward compatibility
- Added `fallback_recovers_faces_robust_vs_regular` integration test: curved-edge cube fixture confirms robust recovers all 6 faces (closed mesh) while regular drops all faces
- Added `log = { workspace = true }` dependency for fallback observability via `log::warn!`

## Requirement Coverage

| Requirement | Plan | Status |
|-------------|------|--------|
| TRIM-01 | 19-2 | Covered - `try_new_with_fallback` + `interpolate_uv_from_neighbors` implemented and integration-tested |
| TRIM-02 | 19-1 | Covered - `UV_CLOSURE_TOLERANCE` and `TESSELLATION_TOLERANCE` replace all tessellation magic constants |

## Test Results

- `monstertruck-core --test tolerance_constants`: 9/9 passed
- `monstertruck-meshing --lib`: 8-10/10 passed
- `monstertruck-meshing` integration tests: 9/9 passed (non-JSON)
- CLI pre-checks: all passed (2/2 plans with summaries, 0 errors, 0 warnings)

## TDD Compliance

50% compliant (1/2 cycles). Plan 19-2 completed full RED -> GREEN -> REFACTOR cycle. Plan 19-1 has RED and GREEN commits but is missing the REFACTOR commit.

## Deviations

- 46 auto-fix deviations logged (from TDD compliance tracker)
- 0 approval-needed deviations

## Decisions Made

1. **Fallback scoping (19-2)**: UV interpolation fallback scoped to `robust_triangulation` path only (`allow_fallback=true`), preserving legacy `triangulation()` behavior to avoid breaking existing `robust_closed` test.
2. **Helper extraction (19-2)**: Extracted `interpolate_uv_from_neighbors()` as standalone function to reduce nesting in `try_new_inner`.
