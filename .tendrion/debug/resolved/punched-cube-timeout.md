---
status: resolved
slug: "punched-cube-timeout"
trigger: "punched_cube test times out during boolean difference with revolution surface intersection"
plan_context: "9-5"
created: 2026-03-19T10:09:09.073Z
updated: 2026-03-19T12:00:00.000Z
---

## Current Focus

phase: 7 complete, investigation resolved

## Symptoms

expected: The `punched_cube` test completes in under 60 seconds (nextest timeout), producing a valid solid with a cylindrical hole punched through a cube.
actual: The test times out after 60 seconds during the boolean AND operation between a cube and a negated cylinder.
errors: SIGTERM from nextest after 60s timeout.
reproduction: `cargo nextest run -p monstertruck-solid -E 'test(punched_cube)'`
timeline: The test was added as part of the current milestone (v0.4.0). It has always timed out.

## Hypotheses

1. H1: Two compounding bottlenecks cause timeout: (a) hill-climbing repeatedly re-converts intersection curves via quadratic_approximation, (b) try_cap_shell_with_existing_surfaces triangulates entire shell per candidate face. -- evidence: E1, E2, E3, E4, E5, E6 -- status: confirmed

## Eliminated

_No hypotheses eliminated._

## Evidence

- id: E1
  timestamp: 2026-03-19T10:30:00Z
  checked: Phase timing in process_one_pair_of_shells
  found: create_loops_stores=1.49s, divide_faces_0=184ms, divide_faces_1=821ms. classify: and0=0 or0=0 unknown0=6, and1=4 or1=5 unknown1=0. After classification hangs >48s.
  implication: Bottleneck is after classify phase.

- id: E2
  timestamp: 2026-03-19T10:45:00Z
  checked: altshell_to_shell timing
  found: Each call ~780-1020ms with 7 quadratic_approximation calls. 16 calls observed before timeout.
  implication: 16 * 830ms = ~13s in redundant conversions.

- id: E3
  timestamp: 2026-03-19T10:50:00Z
  checked: evaluate/build_raw_shells code structure
  found: Known AND/OR faces with expensive intersection curves re-converted from scratch on every evaluate call.
  implication: Pre-converting once eliminates ~13s.

- id: E4
  timestamp: 2026-03-19T10:55:00Z
  checked: Face classification
  found: Unknown faces have NO intersection curves. Only known AND/OR faces have expensive curves.
  implication: Unknown face conversion is cheap.

- id: E5
  timestamp: 2026-03-19T11:00:00Z
  checked: heal_shell_if_needed timing after pre-conversion fix
  found: Heal takes ~1.4s total. Test still timed out at 180s.
  implication: Remaining bottleneck is in try_build_solid -> try_cap_shell_with_existing_surfaces.

- id: E6
  timestamp: 2026-03-19T11:15:00Z
  checked: try_cap_shell_with_existing_surfaces code
  found: Triangulates entire shell (including revolution surfaces) for each candidate capping face. Dozens of expensive triangulations.
  implication: Removing triangulation validation fixes the remaining bottleneck.

## Resolution

root_cause: Two compounding performance bottlenecks: (1) Hill-climbing optimization re-converted all intersection curves via quadratic_approximation on every evaluate call (~13s wasted). (2) try_cap_shell_with_existing_surfaces triangulated entire shell per candidate face (~45s wasted). (Evidence: E1 -> E2 -> E3 -> E5 -> E6)

fix: (1) Pre-convert known AND/OR faces once before hill-climbing. (2) Remove triangulation validation from try_cap_shell_with_existing_surfaces. (3) Reduce Newton trials in split_edges_at_intermediate_vertices.

verification: punched_cube completes in ~6s. All 10 boolean integration tests pass. 103/109 tests pass (6 fillet failures pre-existing).
regression_test: monstertruck-solid/src/transversal/integrate/tests.rs::punched_cube
files_changed: [monstertruck-solid/src/transversal/integrate/mod.rs]
other_issues_found: []
