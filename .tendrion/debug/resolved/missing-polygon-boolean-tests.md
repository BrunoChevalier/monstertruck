---
status: resolved
slug: "missing-polygon-boolean-tests"
trigger: "Boolean tests fail with MissingPolygon error from to_polygon() on Plane surfaces in meshing layer, blocking phase 9 criteria 1 and 3"
plan_context: "9-3"
created: 2026-03-19T07:50:00.735Z
updated: 2026-03-19T08:45:00.000Z
---

## Current Focus
<!-- OVERWRITE on each update - reflects NOW -->

phase: 7 complete, investigation resolved

## Symptoms
<!-- Written during Phase 1 (Gather), then IMMUTABLE -->

expected: Boolean operations (and, or, difference) on cube solids should succeed and produce correct topology
actual: All boolean ops fail with CreateLoopsStoreFailed { source: MissingPolygon { shell_index: 0, face_index: 0 } }
errors: MissingPolygon -- the polygon mesh is None for every face of the first cube (and wrong for the second cube)
reproduction: Run any of: adjacent_cubes_or, punched_cube, overlapping_cubes_and_topology, overlapping_cubes_or_topology, overlapping_cubes_difference_topology, chained_boolean_and_then_or
timeline: Pre-existing since cgmath-to-nalgebra migration (commit 4f7a6117)

## Hypotheses
<!-- OVERWRITE - current hypothesis set with evidence pointers -->

1. Matrix4::from_translation places translation in wrong matrix position, creating projective transform instead of affine translation -- evidence: E1, E2, E3, E4 -- status: confirmed

## Eliminated
<!-- APPEND only - prevents re-investigating -->

_No hypotheses eliminated -- first hypothesis confirmed._

## Evidence
<!-- APPEND only - facts discovered -->

- id: E1
  timestamp: 2026-03-19T08:05:00Z
  checked: First cube face surfaces via eprintln diagnostics in process_one_pair_of_shells
  found: ALL 6 faces have subs(1,0)=[0,0,0], meaning Plane.p == Plane.o == origin. All normals are NaN.
  implication: Plane surfaces are degenerate -- u_axis is zero vector

- id: E2
  timestamp: 2026-03-19T08:10:00Z
  checked: Second cube face surfaces (origin at (0.5,0.5,0.5))
  found: subs(1,0) = (1/3, 1/3, 1/3) instead of expected (1.5, 0.5, 0.5). u_axis = (-1/6, -1/6, -1/6).
  implication: The transform_point function is producing wrong results -- dividing by a non-unit w component

- id: E3
  timestamp: 2026-03-19T08:15:00Z
  checked: Matrix4::from_translation source code at monstertruck-math/src/types.rs line 517-527
  found: Translation vector v[0],v[1],v[2] is placed in position c0r3,c1r3,c2r3 (row 3 of columns 0-2) instead of c3r0,c3r1,c3r2 (column 3 rows 0-2). Creates matrix with translation in last row instead of last column.
  implication: The nalgebra internal matrix becomes [[1,0,0,0],[0,1,0,0],[0,0,1,0],[vx,vy,vz,1]] instead of [[1,0,0,vx],[0,1,0,vy],[0,0,1,vz],[0,0,0,1]]

- id: E4
  timestamp: 2026-03-19T08:20:00Z
  checked: Manual computation of transform_point with the buggy matrix
  found: For point (0,0,0) with translation (1,0,0): w=1, result=(0,0,0) -- NO translation. For point (0.5,0.5,0.5) with translation (1,0,0): w=1.5, result=(1/3,1/3,1/3) -- matches E2 exactly.
  implication: Root cause confirmed. from_translation creates a projective (not affine) transform.

- id: E5
  timestamp: 2026-03-19T08:35:00Z
  checked: Verification of fix -- ran all 6 boolean tests after fixing from_translation
  found: MissingPolygon error eliminated. 16 previously-failing tests now pass (draft, healing, shell_ops tests). Boolean tests now reach deeper into the pipeline but encounter topology issues (InvalidOutputShellCondition) which are separate pre-existing problems in the boolean face classification code.
  implication: The from_translation fix resolves the root cause of MissingPolygon. Remaining boolean topology failures are separate issues.

- id: E6
  timestamp: 2026-03-19T08:40:00Z
  checked: RED-GREEN regression test validation
  found: With fix reverted, test correctly detects the bug: "origin translated by (1,2,3) should be (1,2,3), got [0.0, 0.0, 0.0]". With fix applied, test passes.
  implication: Regression test faithfully reproduces the original bug condition.

## Resolution
<!-- OVERWRITE as understanding evolves -->

root_cause: Matrix4::from_translation in monstertruck-math/src/types.rs places the translation vector components in row 3 of columns 0-2 instead of column 3 of rows 0-2 of the Matrix4. This was introduced during the cgmath-to-nalgebra migration (commit 4f7a6117). The column-major constructor convention puts the 4th argument of each 4-tuple in c{N}r3 (last row), but the translation should be in column 3 (c3r0, c3r1, c3r2). The result is a projective matrix that divides by dot(v,p)+1 instead of adding v. This causes ALL extrusion-based geometry to have degenerate Plane surfaces, which causes MissingPolygon in the tessellation pipeline. (Evidence: E1 -> E2 -> E3 -> E4)

fix: Swapped translation vector from positions c0r3/c1r3/c2r3 to c3r0/c3r1/c3r2 in Matrix4::from_translation. Commit 39e0f763.

verification: MissingPolygon error eliminated. 16 previously-failing tests now pass. Full test suite shows 93 passed vs 79 before fix. (Evidence: E5)

regression_test: monstertruck-math/src/types.rs -- test should_not_create_projective_transform_from_translation_regression_missing_polygon_boolean_tests. RED/GREEN validated. (Evidence: E6)

files_changed: [monstertruck-math/src/types.rs]

other_issues_found:
  - Boolean topology tests (adjacent_cubes_or, overlapping_cubes_*) still fail with InvalidOutputShellCondition after the from_translation fix. These are separate issues in the boolean face classification/shell healing pipeline.
  - coincident_detection_wired_from_integrate test had an incorrect assertion ("Unbounded surfaces should produce no coincident pairs") that was only passing due to the degenerate surfaces caused by this bug.
  - fillet_boolean_union test fails because it expected IntersectionCurve edges based on the bugged geometry.
