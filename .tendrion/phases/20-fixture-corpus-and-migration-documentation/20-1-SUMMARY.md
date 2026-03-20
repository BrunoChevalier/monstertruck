---
phase: 20-fixture-corpus-and-migration-documentation
plan: 1
tags: [test-fixtures, nurbs, pathological, integration-tests]
key-files:
  - monstertruck-geometry/src/nurbs/test_fixtures.rs
  - monstertruck-geometry/tests/pathological_surface_test.rs
  - monstertruck-geometry/tests/test_fixtures_smoke.rs
decisions: []
metrics:
  new_fixtures: 7
  new_unit_tests: 7
  new_integration_tests: 6
  new_smoke_tests: 7
  total_test_fixtures_lines: 576
  total_pathological_test_lines: 151
---

## What was built

Expanded the NURBS test fixture corpus with 7 new fixtures and added comprehensive tests.

### Files modified

- **monstertruck-geometry/src/nurbs/test_fixtures.rs** (576 lines): Added 7 new fixture functions:
  - FIXTURE-01 (pathological rail/section): `fixture_inflection_rail`, `fixture_converging_rails`, `fixture_degenerate_section`, `fixture_cusped_rail`
  - FIXTURE-02 (near-degenerate NURBS): `fixture_near_zero_jacobian_surface`, `fixture_near_zero_weight_nurbs`, `fixture_collapsed_control_polygon_surface`
  - Added 7 unit tests verifying structural validity of each fixture

### Files created

- **monstertruck-geometry/tests/pathological_surface_test.rs** (151 lines): 6 integration tests exercising fixtures through `try_sweep_rail` and `try_birail1` surface constructors
- **monstertruck-geometry/tests/test_fixtures_smoke.rs** (updated): Added 7 smoke tests for new fixtures

## Test results

- 17/17 unit tests pass (`test_fixtures`)
- 6/6 integration tests pass (`pathological_surface_test`)
- 17/17 smoke tests pass (`test_fixtures_smoke`)
- 106/106 lib tests pass (no regressions)
- No panics in any pathological case

## Deviations

- Combined RED+GREEN for fixture constructors: pure data constructors cannot be tested in Rust without the function signature existing. Logged as auto-fix/dependency deviation.
