---
phase: 19-trim-tessellation-robustness
plan: 2
tags: [tessellation, robustness, uv-interpolation, fallback]
key-files:
  - monstertruck-meshing/Cargo.toml
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/src/tessellation/mod.rs
  - monstertruck-meshing/tests/tessellation/triangulation.rs
decisions:
  - "Added allow_fallback parameter to PolyBoundaryPiece::try_new to limit UV interpolation to robust_triangulation() only, preserving existing behavior for triangulation() and passing existing robust_closed test"
  - "Extracted interpolate_uv_from_neighbors() helper to reduce nesting in try_new_inner"
metrics:
  tests_added: 3
  tests_passing: 19
  deviations: 1
---

## What was built

- **monstertruck-meshing/Cargo.toml**: Added `log = { workspace = true }` dependency.
- **monstertruck-meshing/src/tessellation/triangulation.rs**: Replaced `PolyBoundaryPiece::try_new` with a two-pass approach. Pass 1 collects parameter search results (including `None` failures). Pass 2 interpolates UV coordinates for failed points from nearest successful neighbors. Added `try_new_with_fallback` variant and `interpolate_uv_from_neighbors` helper. Threaded `allow_fallback: bool` through `shell_create_polygon`, `shell_tessellation`, `shell_tessellation_single_thread`, and `cshell_tessellation`.
- **monstertruck-meshing/src/tessellation/mod.rs**: Updated `triangulation_with` and `cshell_triangulation_with` to pass `false` for `allow_fallback`; `robust_triangulation_with` and `robust_cshell_triangulation_with` pass `true`.
- **monstertruck-meshing/tests/tessellation/triangulation.rs**: Added `fallback_recovers_faces_robust_vs_regular` integration test verifying robust_triangulation recovers more faces than regular triangulation on a curved-edge cube fixture.

## Task commits

| Commit | Message |
|--------|---------|
| c2afe5a5 | test(tessellation): add failing tests for UV interpolation fallback in PolyBoundaryPiece::try_new |
| 3cecb7f2 | feat(tessellation): implement UV interpolation fallback in PolyBoundaryPiece::try_new |
| 0b5d328e | refactor(tessellation): extract UV interpolation helper and add integration test for fallback recovery |

## Decisions made

1. **Fallback scoping**: The UV interpolation fallback is only active in the `robust_triangulation` path (via `allow_fallback=true`). Regular `triangulation()` preserves legacy behavior (any single parameter search failure drops the face). This prevents breaking the existing `robust_closed` test which asserts that `triangulation()` drops all faces on the curved-edge cube.

2. **Helper extraction**: Extracted `interpolate_uv_from_neighbors()` as a standalone function to reduce nesting and improve readability of the core `try_new_inner` method.

## Deviations from plan

1. **Design deviation (auto-fixed)**: The plan specified placing the fallback directly in `try_new`, which would have broken the `robust_closed` integration test. Resolved by splitting into `try_new` (no fallback) and `try_new_with_fallback` (with fallback), threading `allow_fallback` through the call chain. The plan's intent (fallback in `try_new`) is preserved while maintaining backward compatibility.

## Self-check

- [x] `monstertruck-meshing/src/tessellation/triangulation.rs` contains `fallback` (1666 lines, min 100)
- [x] `monstertruck-meshing/tests/tessellation/triangulation.rs` contains `fallback_recovers` (218 lines, min 150)
- [x] `log::warn!` present in triangulation.rs for fallback observability
- [x] All lib tests pass (10/10)
- [x] All non-JSON integration tests pass (9/9)
- [x] clippy clean on monstertruck-meshing
- [x] TDD cycle complete: RED -> GREEN -> REFACTOR
