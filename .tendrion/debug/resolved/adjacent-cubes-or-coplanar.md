---
status: resolved
slug: "adjacent-cubes-or-coplanar"
trigger: "adjacent_cubes_or test fails: boolean OR of adjacent cubes produces 2 shells instead of 1 due to coplanar face boundary not being welded"
plan_context: "9-5"
created: 2026-03-19T10:09:06.907Z
updated: 2026-03-19T11:15:00.000Z
---

## Current Focus
<!-- OVERWRITE on each update - reflects NOW -->

phase: 7 complete, investigation resolved
hypothesis: H1 confirmed, fix applied and verified
test: All tests pass, no regressions
expecting:
next_action: none

## Symptoms
<!-- Written during Phase 1 (Gather), then IMMUTABLE -->

expected: Boolean OR of two adjacent unit cubes produces 1 boundary shell, volume=2.0, 12 faces
actual: Produces 2 boundary shells (2 disconnected components of 6 faces each)
errors: assertion `left == right` failed: left=2, right=1 at tests.rs:125
reproduction: cargo nextest run -p monstertruck-solid -E 'test(adjacent_cubes_or)'
timeline: Existing test, failing after boolean pipeline improvements in v0.3.0

## Hypotheses
<!-- OVERWRITE - current hypothesis set with evidence pointers -->

1. [CONFIRMED] weld_compressed_shell merges coincident vertices but does NOT split edges passing through intermediate welded vertices -- evidence: E1, E2, E3, E4

## Eliminated
<!-- APPEND only - prevents re-investigating -->

- H2: Intersection curves for coplanar faces are degenerate. Refuted by E1.
- H3: Wrong face classification. Refuted by E1.

## Evidence
<!-- APPEND only - facts discovered -->

- id: E1
  timestamp: 2026-03-19T10:15:00Z
  checked: Debug output from adjacent_cubes_or test
  found: Face division works (7 faces from 6). OR shell has two disconnected groups that welding can't fully merge. Weld improves from (1,4,2) to (1,2,4) but not closed.
  implication: Welding merges some vertices but doesn't produce shared edges.

- id: E2
  timestamp: 2026-03-19T10:22:00Z
  checked: Compressed shell data for OR shell
  found: Pre-weld: 20 vertices, 32 edges. Post-weld: 18 vertices (2 merged), 32 edges (0 merged). Zero edge deduplication.
  implication: Vertex welding works but doesn't produce duplicate-vertex-pair edges.

- id: E3
  timestamp: 2026-03-19T10:25:00Z
  checked: Edge structure of cube0 L-shape and cube1 L-shape
  found: No shared edges. Shared vertices only at v9 and v10.
  implication: Two L-shapes share vertices but NOT edges.

- id: E4
  timestamp: 2026-03-19T10:28:00Z
  checked: Whether intermediate welded vertices lie on unsplit edges
  found: v9 lies on edges e9 and e31. v10 lies on edges e8 and e22. These edges are not split at those vertices.
  implication: ROOT CAUSE confirmed.

- id: E5
  timestamp: 2026-03-19T10:55:00Z
  checked: Fix applied and verified
  found: adjacent_cubes_or passes. Full test suite: 101 passed (vs 98 on master), 0 regressions.
  implication: Fix resolves the bug and also fixes crossing_edges and fillet_boolean_union.

## Resolution
<!-- OVERWRITE as understanding evolves -->

root_cause: weld_compressed_shell merges coincident vertices but does not split edges whose curves pass through those merged vertices. Adjacent faces retain unsplit edges while face division creates sub-edges, preventing sub-edge sharing between faces from different source shells. (Evidence: E1 -> E2 -> E3 -> E4)
fix: Added split_edges_at_intermediate_vertices() in monstertruck-solid/src/transversal/integrate/mod.rs. Detects edges passing through interior vertices using SearchNearestParameter, splits using Cut, deduplicates and compacts. Called from heal_shell_if_needed after weld.
verification: adjacent_cubes_or passes. Full suite: 101/109 pass (3 more than master baseline of 98/109). 0 regressions.
regression_test: monstertruck-solid/src/transversal/integrate/tests.rs::adjacent_cubes_or (pre-existing test, was the failing test that triggered this investigation)
files_changed: [monstertruck-solid/src/transversal/integrate/mod.rs]
other_issues_found: []
