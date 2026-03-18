---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 2
tags: [boolean, face-classification, healing, resilience, diagnostics]
key-files:
  - monstertruck-solid/src/transversal/faces_classification/mod.rs
  - monstertruck-solid/src/transversal/integrate/mod.rs
  - monstertruck-solid/src/transversal/mod.rs
  - monstertruck-solid/src/transversal/edge_cases.rs
  - monstertruck-solid/src/transversal/faces_classification/tests.rs
  - monstertruck-solid/src/transversal/integrate/tests.rs
decisions:
  - "Used FxHashSet for majority-edge scoring in integrate_by_component for O(1) lookups"
  - "Conservative default `false` (outside/OR) for failed unknown-face classification"
  - "Coincident detection wired as logging-only behind MT_BOOL_DEBUG_COINCIDENT env var"
  - "3-stage healing fallback: healed > unhealed > original (never returns None)"
metrics:
  tests_added: 6
  tests_passing: 10
  pre_existing_failures: 3
  deviations: 1
---

## What was built

### Task 1: Hardened face classification and enabled edge_cases module
- **`faces_classification/mod.rs`**: Replaced `integrate_by_component` with robust majority-edge scoring using `FxHashSet`. Empty boundary components are skipped instead of panicking. The old code accessed `boundary[0][0].id()` which panicked when a component had no boundary edges.
- **`mod.rs`**: Changed `#[allow(dead_code)] mod edge_cases;` to `pub(crate) mod edge_cases;`.
- **`edge_cases.rs`**: Added `#[allow(dead_code)]` on specific unused items (`TangentFacePair`, `detect_tangent_faces`, `is_pole_degenerate`, `handle_degenerate_intersection`) rather than on the whole module.

### Task 2: Resilient unknown-face classification and diagnostic coincident logging
- **`integrate/mod.rs`**: Added diagnostic coincident-face logging behind `MT_BOOL_DEBUG_COINCIDENT` env var. This is logging-only and does not influence classification.
- **`integrate/mod.rs`**: Replaced `try_for_each` + `ok_or(UnknownClassificationFailed)` with a resilient loop that falls back to `false` (outside/OR) for individual face classification failures. Both shell0 and shell1 loops are updated. The `UnknownClassificationFailed` error variant is preserved but no longer returned from this code path.

### Task 3: Shell healing with 3-stage fallback
- **`integrate/mod.rs`**: Replaced `heal_shell_if_needed` with a 3-stage fallback: (1) compress + heal + extract, (2) compress without heal + extract, (3) pick best among healed, unhealed, and original. Never returns `None` for non-empty shells. Debug logging behind `MT_BOOL_DEBUG_HEAL`.
- **`integrate/mod.rs`**: Added early return to `try_cap_shell_with_existing_surfaces` for already-closed shells.
- **`integrate/mod.rs`**: Added diagnostic logging for dropped invalid boundaries in `try_build_solid` behind `MT_BOOL_DEBUG_COMPONENTS`.

## Tests added
- `integrate_by_component_empty_boundary_no_panic`: Verifies face classification does not panic when a component has empty boundaries (RED-GREEN confirmed).
- `integrate_by_component_majority_edge_scoring`: Verifies majority-edge scoring classifies tied faces correctly.
- `coincident_detection_wired_from_integrate`: Verifies the edge_cases module is accessible from integrate.
- `heal_shell_never_returns_none`: Verifies heal returns Some for closed shells.
- `heal_shell_open_shell_returns_some`: Verifies heal returns Some for open shells with quality no worse than input.
- `cap_shell_early_return_for_closed`: Verifies capping does not add faces to already-closed shells.

## Deviations
- Tasks 2-3 changes are defensive error-path hardening. Could not construct synthetic geometry triggering `classify_unknown_face` returning `None` or `Shell::extract` failing after `robust_split`. Tests verify new code compiles, module wiring works, and existing behavior is preserved.

## Pre-existing test failures (not caused by this plan)
- `transversal::integrate::tests::adjacent_cubes_or`
- `transversal::integrate::tests::punched_cube`
- `transversal::loops_store::tests::crossing_edges`
