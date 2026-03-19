---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 4
tags: [tolerance, imports, meshing, gap-fix]
key-files:
  - monstertruck-meshing/src/tessellation/mod.rs
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/src/tessellation/boundary_stitching.rs
  - monstertruck-meshing/src/analyzers/collision.rs
decisions:
  - "boundary_stitching.rs: placed explicit TOLERANCE import inside #[cfg(test)] module because TOLERANCE is only used in tests within that file; placing it at module scope triggered unused-import warning"
metrics:
  tests_passed: 25
  tests_failed: 5
  tests_skipped: 1
  pre_existing_failures: 5
  clippy_warnings: 0
---

## What was built

Added explicit `use monstertruck_core::tolerance::{TOLERANCE,TOLERANCE2}` imports to four monstertruck-meshing source files. These symbols were previously available via glob re-export chains (`crate::*` -> `monstertruck_mesh::*` -> `base::*` -> `tolerance::*`). The explicit imports make the dependency on the shared tolerance module visible and satisfy the TEST-02 success criterion.

### Files modified

- **monstertruck-meshing/src/tessellation/mod.rs** -- Added `use monstertruck_core::tolerance::TOLERANCE;` at line 2.
- **monstertruck-meshing/src/tessellation/triangulation.rs** -- Added `use monstertruck_core::tolerance::TOLERANCE;` at line 9.
- **monstertruck-meshing/src/tessellation/boundary_stitching.rs** -- Added `use monstertruck_core::tolerance::TOLERANCE;` inside `#[cfg(test)] mod tests` (line 169), since TOLERANCE is only used in test code within this file.
- **monstertruck-meshing/src/analyzers/collision.rs** -- Added `use monstertruck_core::tolerance::TOLERANCE2;` at line 3.

## Task commits

| SHA | Message |
|-----|---------|
| `1c1b462a` | feat(meshing): add explicit monstertruck_core::tolerance imports to monstertruck-meshing |

## Deviations from plan

1. **boundary_stitching.rs import placement**: The plan specified adding the import at module scope after `use rustc_hash::FxHashMap as HashMap;`. However, TOLERANCE is only used inside the `#[cfg(test)]` block in this file (the non-test functions take `tolerance: f64` as a parameter). Placing the import at module scope triggered an `unused_imports` warning. Moved the import into the test module to keep clippy clean.

2. **Pre-existing test compilation errors**: `monstertruck-meshing/tests/vtk.rs` (12 `truncate` method ambiguity errors) and `tests/analyzers/collision.rs` (1 `cross` reference error) fail to compile on master before and after this change. These are unrelated to tolerance imports.

3. **Pre-existing test failures**: 5 tessellation integration tests fail with JSON deserialization errors ("invalid type: map, expected a matrix array"). These exist on master before this change.

## Self-check

- All four target files contain `use monstertruck_core::tolerance` -- verified via grep.
- `cargo clippy -p monstertruck-meshing --lib` -- zero warnings for monstertruck-meshing.
- `cargo nextest run -p monstertruck-meshing --lib` -- 8/8 unit tests pass (1 skipped).
- `cargo nextest run -p monstertruck-meshing --test tessellation --test filters` -- 17 pass, 5 fail (all pre-existing).
- No behavioral changes introduced. Imports are purely additive over existing glob re-exports.
