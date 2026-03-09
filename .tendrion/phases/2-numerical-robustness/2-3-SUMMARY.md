---
phase: 2-numerical-robustness
plan: 3
tags: [tessellation, boundary-stitching, seam-elimination]
key-files:
  - monstertruck-meshing/src/tessellation/boundary_stitching.rs
  - monstertruck-meshing/src/tessellation/mod.rs
  - monstertruck-meshing/tests/tessellation/boundary_stitching.rs
  - monstertruck-meshing/tests/tessellation/main.rs
decisions:
  - "Integrated stitching at the public API level (triangulation_with, robust_triangulation_with, cshell_triangulation_with, robust_cshell_triangulation_with) rather than inside shell_tessellation, ensuring all tessellation paths benefit."
  - "Added stitch_compressed_boundaries for CompressedShell support alongside stitch_boundaries for Shell."
  - "Stitching is always-on (no opt-in flag) since it fixes a correctness issue with no meaningful performance cost."
metrics:
  tests_added: 7
  tests_passed: 49
  source_lines: 194
  test_lines: 182
  deviations: 0
---

## What was built

### New files
- `monstertruck-meshing/src/tessellation/boundary_stitching.rs` (194 lines): Boundary-aware vertex stitching module. Contains `stitch_boundaries` for `Shell` and `stitch_compressed_boundaries` for `CompressedShell`. Uses canonical polyline positions from shared topological edges to unify polygon mesh positions at face boundaries.
- `monstertruck-meshing/tests/tessellation/boundary_stitching.rs` (182 lines): 5 integration tests (`no_seams_on_cube`, `no_seams_on_cylinder`, `boundary_vertices_shared_between_adjacent_faces`, `stitched_tessellation_preserves_vertex_count`, `stitch_respects_tolerance`) plus 2 unit tests for the internal `replace_near_positions` helper.

### Modified files
- `monstertruck-meshing/src/tessellation/mod.rs`: Added module declaration, public re-exports, and integrated stitching calls into all four `*_triangulation_with` functions.
- `monstertruck-meshing/tests/tessellation/main.rs`: Added `mod boundary_stitching;` for test discovery.

## Algorithm

1. Build `HashMap<EdgeId, Vec<Point3>>` mapping each topological edge to its canonical polyline positions.
2. For each face, collect oriented boundary points from the canonical map.
3. Replace polygon mesh positions within tolerance of any canonical point with the exact canonical value.

This ensures adjacent faces use bitwise-identical positions along shared edges, making `put_together_same_attrs` merging more reliable.

## Verification

- All 49 monstertruck-meshing tests pass (47 existing + 2 new unit tests).
- 5 new integration tests pass covering cube, cylinder, shared-edge vertex identity, vertex count preservation, and multi-tolerance scenarios.
- No clippy warnings in monstertruck-meshing.
- Pre-existing fillet test failures in monstertruck-solid confirmed unrelated.

## Deviations

None.
