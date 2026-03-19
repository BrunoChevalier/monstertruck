---
phase: 16-tolerance-foundation-and-api-safety
plan: 2
tags: [api-safety, non_exhaustive, future-proofing]
key-files:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/tests/try_surface_constructors_test.rs
  - monstertruck-modeling/src/builder.rs
decisions:
  - "Updated integration test construction syntax (struct literal -> Default + field mutation) per plan; test logic unchanged"
metrics:
  tests_passed: 46
  tests_failed: 0
  deviations: 1
---

## What was built

Added `#[non_exhaustive]` attribute to all five surface constructor option structs, preventing external crates from using struct literal construction. This ensures future field additions will not be breaking changes.

## Files modified

- **monstertruck-geometry/src/nurbs/surface_options.rs**: Added `#[non_exhaustive]` to `SweepRailOptions`, `Birail1Options`, `Birail2Options`, `GordonOptions`, `SkinOptions`. Added doc examples showing `Default + field mutation` construction pattern.
- **monstertruck-geometry/tests/try_surface_constructors_test.rs**: Updated 7 struct literal construction sites to use `Default::default()` + field mutation (required because integration tests are external crate context).
- **monstertruck-modeling/src/builder.rs**: Updated 1 struct literal construction site (`SweepRailOptions { n_sections: 1, ..SweepRailOptions::default() }` -> `let mut opts = SweepRailOptions::default(); opts.n_sections = 1;`).

## Verification

- 33 monstertruck-geometry surface tests: all passed
- 13 monstertruck-modeling builder tests: all passed
- 5 new doc examples in surface_options.rs: all compiled and passed
- All existing modeling doc tests for option-based builders: passed
- Pre-existing failures (deprecated API doc examples, vertex tuple conversion, sweep_multi_rail): unchanged, not related to this plan

## Deviations

1. TDD exemption logged: `#[non_exhaustive]` is a compile-time attribute with no runtime behavior change. RED test would pass identically before and after. Verification is compilation success.
