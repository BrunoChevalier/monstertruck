---
status: resolved
slug: "crossing-edges-loops-store"
trigger: "crossing_edges test fails in loops_store with all_indices_placed guard failure during boolean operation"
plan_context: "9-5"
created: 2026-03-19T10:09:08.179Z
updated: 2026-03-19T10:55:00.000Z
---

## Current Focus

phase: 7 complete, investigation resolved
hypothesis: H1 confirmed and fixed
test: RED-GREEN verified with existing crossing_edges test
expecting: n/a
next_action: none

## Symptoms

expected: create_loops_stores returns a LoopsStore where each face's Loops has 2 BoundaryWires (the original boundary split by the intersection curve)
actual: geom_loops_store0[0].len() == 1 (only 1 BoundaryWire instead of expected 2) -- the intersection curve was never added to split the boundary
errors: assertion `left == right` failed, left: 1, right: 2 at tests.rs:632
reproduction: cargo nextest run -p monstertruck-solid --lib -E 'test(loops_store::tests::crossing_edges)'
timeline: Test exists in codebase, failing since current state

## Hypotheses

1. H1: has_edge_between_points falsely detects boundary edge segments as duplicates of intersection curves -- evidence: E1, E3, E4 -- status: CONFIRMED
2. H2: Intersection curve endpoints are too close (near check) -- evidence: E1, E2 -- status: eliminated
3. H3: No intersection curves found between faces -- evidence: E1 -- status: eliminated

## Eliminated

- H2: pv0=[0.7045, 0, 0] and pv1=[-0.7045, 0, 0] are not near, gv0 and gv1 are not near. Evidence: E2
- H3: Intersection curves ARE found for face_pair (0,0) and (1,1) with is_closed=false. Evidence: E1

## Evidence

- id: E1
  timestamp: 2026-03-19T10:15:00Z
  checked: Debug output from create_loops_stores_with_tolerance
  found: Two intersection curves found -- face_pair(0,0) with front=[0.7045,0,0] back=[-0.7045,0,0] and face_pair(1,1). Both have is_closed=false. All four idx values (idx00, idx01, idx10, idx11) are true for both curves.
  implication: Intersection curves exist and endpoints are placed on boundaries, but edges are not being added

- id: E2
  timestamp: 2026-03-19T10:20:00Z
  checked: pv_near and gv_near checks
  found: pv0=[0.7045, 0, -1e-16] pv1=[-0.7045, 0, -1e-16] -- not near. gv0=[0.7071, 0, 1.4e-8] gv1=[-0.7071, 0, 1.4e-8] -- not near.
  implication: The near check is not the cause of skipping

- id: E3
  timestamp: 2026-03-19T10:25:00Z
  checked: duplicated0 and duplicated1 values
  found: For BOTH face_pairs, duplicated0=true AND duplicated1=true. The code hits "if duplicated0 && duplicated1 { return Ok(()); }" and skips adding the edges.
  implication: has_edge_between_points is returning true for both stores, causing the intersection curve to be skipped as a false duplicate

- id: E4
  timestamp: 2026-03-19T10:30:00Z
  checked: Root cause analysis of why has_edge_between_points returns true
  found: After add_polygon_vertex_with_tolerance splits boundary edges at intersection curve endpoints, a boundary edge segment now exists between those exact endpoints. has_edge_between_points only checks endpoint proximity (not curve geometry), so it considers this boundary segment a "duplicate" of the intersection curve. But the boundary edge follows the face boundary while the intersection curve crosses the face interior -- they are geometrically different paths.
  implication: The duplicate detection function needs to also verify the curve interior, not just endpoints.

- id: E5
  timestamp: 2026-03-19T10:40:00Z
  checked: Fix verification -- added midpoint check to has_edge_between_points
  found: All 3 loops_store tests pass (crossing_edges, independent_intersection, rotated_intersection). adjacent_cubes_or also passes.
  implication: The fix correctly distinguishes boundary edge segments from intersection curve duplicates.

## Resolution

root_cause: has_edge_between_points() only compared edge endpoint positions, not curve interior geometry. When an intersection curve's endpoints lie on the same boundary edge, vertex placement splits that boundary edge creating a segment between those endpoints. This boundary segment triggers a false-positive duplicate detection, preventing the intersection curve from being added to the loops. (Evidence: E1 -> E3 -> E4)

fix: Added a midpoint parameter to has_edge_between_points(). After matching endpoints, the function now also evaluates the existing edge's curve at its parametric midpoint and compares it with the intersection polyline's midpoint. This distinguishes boundary edge segments (which follow the face boundary) from actual duplicate intersection curves (which would have matching interior geometry).

verification: All 3 loops_store tests pass. adjacent_cubes_or passes. RED-GREEN cycle confirmed.
regression_test: monstertruck-solid/src/transversal/loops_store/tests.rs::crossing_edges (existing test, verified RED-GREEN)
files_changed: [monstertruck-solid/src/transversal/loops_store/mod.rs]
other_issues_found:
  - Pre-existing compilation errors in integrate/tests.rs (punched_cube_diag function)
  - Pre-existing fillet test failures (6 tests)
  - Pre-existing punched_cube timeout
