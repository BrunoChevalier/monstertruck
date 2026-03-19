---
phase: 10-test-infrastructure-and-healing-hooks
plan: 2
tags: [healing, topology, surface-constructors]
key-files:
  - monstertruck-solid/src/healing/surface_healing.rs
  - monstertruck-solid/src/healing/mod.rs
  - monstertruck-solid/src/lib.rs
decisions: []
metrics:
  tests_added: 5
  tests_passed: 5
  lines_added: ~470
  deviations: 0
---

## What was built

Topological healing hooks for surfaces produced by `sweep_rail`, `birail`, and `gordon` constructors.

### Files created

- **`monstertruck-solid/src/healing/surface_healing.rs`** (613 lines): Core healing module with:
  - `UnionFind` struct for vertex merging (union-find with path compression and rank).
  - `weld_gap_edges()`: Detects near-coincident vertices and merges them via union-find, remapping all edge vertex indices.
  - `remove_degenerate_edges()`: Removes zero-length edges (same start/end vertex, curve length below tolerance) and compacts the edge array with index remapping.
  - `find_non_manifold_edges()`: Checks that every edge appears exactly twice (forward + reversed) across all face boundaries.
  - `heal_surface_shell()`: Public entry point composing weld -> split-closed -> remove-degenerate -> manifold-check -> extract.
  - `SurfaceHealingError` enum with `NonManifoldEdges`, `TopologyExtractFailed`, `TooManyGaps` variants.
  - 5 unit tests exercising each function independently and the full pipeline.

### Files modified

- **`monstertruck-solid/src/healing/mod.rs`**: Added `mod surface_healing` and `pub use surface_healing::{SurfaceHealingError, heal_surface_shell}`.
- **`monstertruck-solid/src/lib.rs`**: Added `SurfaceHealingError` and `heal_surface_shell` to the public re-exports. Updated crate doc to mention the new healing hooks.

## Test results

5 tests added, all passing:
- `test_weld_gap_edges` -- verifies vertex merging with near-coincident vertices
- `test_remove_degenerate_edges` -- verifies zero-length edge removal
- `test_find_non_manifold_edges_manifold` -- verifies boundary detection on open shell
- `test_find_non_manifold_edges_closed` -- verifies clean result on closed tetrahedron
- `test_heal_surface_shell_pipeline` -- full pipeline with Plane surface and deliberate gaps

No regressions in existing healing tests (103/109 pass; 6 pre-existing fillet failures unrelated to this plan).

## Deviations

None.
