---
phase: 16-tolerance-foundation-and-api-safety
plan: 3
tags: [refactoring, api-safety, deprecation, delegation]
key-files:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/tests/deprecated_delegation_test.rs
decisions: []
metrics:
  lines_removed: 225
  tests_added: 6
  tests_total_passed: 234
  deprecated_methods_refactored: 5
---

## What was built

Refactored all five deprecated surface constructor methods to delegate to their `try_*` counterparts instead of maintaining independent algorithm implementations.

### Files modified

- **monstertruck-geometry/src/nurbs/bspline_surface.rs**: Replaced full algorithm bodies of `skin()`, `gordon()`, `sweep_rail()`, `birail1()`, `birail2()` with thin delegation wrappers calling `try_skin()`, `try_gordon()`, `try_sweep_rail()`, `try_birail1()`, `try_birail2()` respectively. Removed `#[allow(deprecated)]` from all five methods (no longer needed since they call non-deprecated try_* methods). Added `#[allow(clippy::field_reassign_with_default)]` to three methods that use `Default::default()` + field mutation for `#[non_exhaustive]` option structs.

- **monstertruck-geometry/tests/deprecated_delegation_test.rs** (new): Six characterization tests verifying deprecated methods produce identical output to try_* methods at multiple sample points.

### Line reduction

- Before: 3414 lines
- After: 3189 lines
- Removed: 225 lines of duplicated algorithm logic

### Deprecated method body sizes (all under 10 lines)

| Method | Body lines |
|--------|-----------|
| `skin()` | 3 |
| `gordon()` | 3 |
| `sweep_rail()` | 5 |
| `birail1()` | 5 |
| `birail2()` | 5 |

## Task commits

| SHA | Message |
|-----|---------|
| `0c7f066b` | test(surface): add characterization tests for deprecated method delegation |
| `292521e5` | feat(surface): refactor all deprecated constructors to delegate to try_* methods |

## Deviations from plan

- RED tests passed immediately since this is a pure refactoring task (deprecated and try_* methods produced identical output already). Logged as deviation.
- Line reduction was 225 lines vs estimated 250-300 lines. Close to target.

## Self-check

- All 234 monstertruck-geometry tests pass
- All 98 monstertruck-modeling tests pass
- monstertruck-solid: 127/134 pass; 7 failures are pre-existing (verified by running against stashed changes)
- Clippy clean on monstertruck-geometry lib
- `#[allow(deprecated)]` only present on `sweep_multi_rail` and `sweep_periodic` (which legitimately call deprecated `skin()`) and test functions
- No duplicated algorithm logic between deprecated and try_* methods
