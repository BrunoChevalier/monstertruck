---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 5
tags: [verification, boolean, topology, weld-fix]
key-files:
  - monstertruck-solid/src/transversal/integrate/mod.rs
  - monstertruck-solid/src/transversal/edge_cases.rs
decisions:
  - "Added vertex/edge welding in heal_shell_if_needed to fix disconnected boolean shells (auto-fix for BOOL-01)"
  - "Fixed detect_coincident_faces to skip self-face comparison"
metrics:
  tests_fixed: 5
  tests_remaining_failures: 9
  tests_timeout: 2
  commit: 7a5ab203
---

## What was built

This plan was originally verification-only but diagnosed and fixed production code issues blocking boolean test passage.

### Files modified

- **monstertruck-solid/src/transversal/integrate/mod.rs**: Added `weld_compressed_shell()` function that merges geometrically coincident vertices and deduplicates edges connecting the same vertex pair. Integrated into `heal_shell_if_needed()` as a pre-healing stage (stage 0.5). This fixes the root cause of disconnected boolean shells: face division creates separate edge instances for each input shell along intersection curves, and without welding the combined AND/OR shells have topological gaps.

- **monstertruck-solid/src/transversal/edge_cases.rs**: Fixed `detect_coincident_faces()` to skip self-face comparisons (same face ID). Previously, comparing a shell with itself reported every face as coincident with itself because Plane surfaces have bounded parameter ranges [0,1]x[0,1].

## Test results

### Task 1: Boolean unit tests and edge cases

| Test suite | Result |
|---|---|
| `cargo nextest run -p monstertruck-solid --lib` | 130 passed, 9 failed, 2 timed out |
| `cargo nextest run -p monstertruck-solid --test boolean_edge_cases` | 7 passed, 0 failed |
| `cargo nextest run -p monstertruck-math --lib` | 8 passed, 0 failed |

**Tests fixed by this plan (5):**
- `overlapping_cubes_and_topology` -- PASS
- `overlapping_cubes_or_topology` -- PASS
- `overlapping_cubes_difference_topology` -- PASS
- `chained_boolean_and_then_or` -- PASS
- `coincident_detection_wired_from_integrate` -- PASS

**Pre-existing failures (9 + 2 timeouts), NOT caused by MissingPolygon fix:**
- `adjacent_cubes_or` -- coplanar face topology issue; intersection edges not fully placed on both shells
- `crossing_edges` -- pre-existing loops_store test failure
- 7 fillet tests (`test_unit_circle`, `generic_fillet_*`, `fillet_boolean_union`) -- pre-existing fillet issues
- `punched_cube` -- timeout (revolution surface intersection)
- `boolean_shell_converts_for_fillet` -- timeout (depends on boolean + fillet pipeline)

### Task 2: Workspace-wide quality checks

| Check | Result |
|---|---|
| `cargo clippy --all-targets -- -W warnings` | Pre-existing errors in monstertruck-traits, monstertruck-gpu, monstertruck-render; monstertruck-solid lib is clean |
| `cargo fmt --all -- --check` | Clean (no formatting issues) |
| Workspace-wide tests | Multiple crates have pre-existing test compilation errors (monstertruck-geometry, monstertruck-traits, monstertruck-step, monstertruck-modeling, monstertruck-meshing integration tests, monstertruck-render); core crates pass |

## Commits

| SHA | Message |
|---|---|
| `7a5ab203` | fix(boolean): weld coincident vertices/edges in boolean shell assembly and skip self-face in coincident detection |

## Root cause analysis

The MissingPolygon fix (`Matrix4::from_translation` column placement, commit c6dc8027) resolved the `MissingPolygon` error that prevented boolean operations from reaching the face classification stage. However, the boolean pipeline had a deeper topological issue: after face classification, AND/OR faces from two different input shells had separate edge instances along intersection curves. Since edge identity in monstertruck-topology is based on `Arc` pointer equality, these separate edges prevented the combined shell from being topologically closed.

The `weld_compressed_shell` function resolves this by operating on the serialized `CompressedShell` format: it merges vertices within tolerance, then deduplicates edges that connect the same (welded) vertex pair. This is applied as stage 0.5 of `heal_shell_if_needed`, before the existing compress+heal+extract pipeline.

## Known issues requiring follow-up

1. **Coplanar face boolean operations** (`adjacent_cubes_or`): When two solids share a coplanar face boundary (touching but not overlapping), the loops_store cannot resolve all 4 vertex placements for intersection curves. The `all_indices_placed` guard prevents edge addition, causing the combined shell to remain disconnected even after welding.

2. **Revolution surface boolean timeout** (`punched_cube`): Cylinder-cube intersection triggers complex intersection curve calculations that exceed the 180s test timeout.

3. **Pre-existing workspace compilation issues**: Multiple crates (geometry, traits, step, modeling, meshing, render, gpu) have test compilation errors unrelated to phase 9 changes.
