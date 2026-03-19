---
phase: 13
title: API Polish and Surface Operations
status: complete
plans_executed: 3
plans_total: 3
tests_added: 48
deviations_auto_fix: 37
deviations_approval_needed: 0
tdd_compliance: 0%
---

## What Was Built

### Plan 13-1: Option Structs and Diagnostic Errors (Geometry Layer)
- Typed option structs (`SweepRailOptions`, `Birail1Options`, `Birail2Options`, `GordonOptions`, `SkinOptions`) with `Default` impls.
- `CurveNetworkDiagnostic` enum with 7 diagnostic variants for detailed error reporting.
- Fallible `try_*` methods on `BsplineSurface` for all surface constructors.
- Old positional-parameter APIs deprecated with `#[deprecated]` annotations.
- 36 tests across 4 test files.

### Plan 13-2: Surface Split and Sub-Patch Extraction
- `split_at_u`, `split_at_v`, `sub_patch` on `BsplineSurface` and `NurbsSurface`.
- 5 integration tests verifying evaluation preservation across splits and sub-patches.

### Plan 13-3: Modeling Layer Option-Based Builders
- `try_sweep_rail_with_options`, `try_birail_with_options`, `try_birail2_with_options`, `try_gordon_with_options` builder functions.
- `Error::FromGeometry` variant with `From<geometry::Error>` for `?` propagation.
- Re-exports of option structs in modeling crate root.
- 7 tests covering success paths and diagnostic error chain propagation.

## Requirement Coverage

| Requirement | Plans | Status |
|-------------|-------|--------|
| API-01 | 13-1, 13-3 | Covered |
| API-02 | 13-1, 13-3 | Covered |
| SURF-03 | 13-2 | Covered |

## Test Results

- 48 tests added across geometry and modeling crates
- All tests reported passing in plan summaries

## Deviations

- 37 auto-fix deviations (global count, not phase-specific)
- 0 approval-needed deviations

## Decisions Made

- Old `try_sweep_rail`/`try_birail`/`try_gordon` kept as independent implementations (not delegates) to preserve existing test assertions on error variant types.
- Removed `Eq` derive from modeling `Error` enum due to `f64` fields in geometry errors.
- `surface_options.rs` accepted at 67 lines (below plan's 100-line minimum) as functionally complete.

## TDD Compliance

- Level: strict
- Compliance: 0% (3/3 plans missing REFACTOR commits per strict mode rules)
- All plans followed red-green cycles; refactor step was implicit rather than separate commits.
