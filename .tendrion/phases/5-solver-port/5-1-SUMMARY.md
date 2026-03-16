---
phase: 5-solver-port
plan: 1
tags: [polynomial, solver, port, math]
key-files:
  - monstertruck-math/src/polynomial.rs
  - monstertruck-math/Cargo.toml
  - monstertruck-math/src/lib.rs
decisions: []
metrics:
  tests_added: 7
  tests_passed: 7
  functions_ported: 5
  tdd_violations: 0
---

## What was built

- **monstertruck-math/src/polynomial.rs** -- New module with five polynomial solver functions ported from `matext4cgmath::solver`: `solve_quadratic`, `pre_solve_cubic`, `solve_cubic`, `pre_solve_quartic`, `solve_quartic`. All functions are generic over `BaseFloat` and return `Complex<F>` arrays. Newton refinement is applied in `pre_solve_cubic` and `pre_solve_quartic`.
- **monstertruck-math/Cargo.toml** -- Added `num-complex = "0.4"` dependency.
- **monstertruck-math/src/lib.rs** -- Added `pub mod polynomial` declaration and `pub use num_complex` re-export.

## Task commits

| Step | SHA | Message |
|------|-----|---------|
| RED | `d9160a8b` | test(polynomial): add failing tests for all five polynomial solvers |
| GREEN | `30722e4e` | feat(polynomial): implement all five polynomial solvers ported from matext4cgmath |
| REFACTOR | `ca717693` | refactor(polynomial): improve readability with Float alias and better doc comments |

## Decisions made

None. Faithful port of existing algorithm with no design changes.

## Deviations from plan

- Disambiguated `num_traits::Float` methods from nalgebra's `ComplexField` methods (which shadow `sqrt`, `signum`, `powf`, `abs`, `epsilon`, `powi`) by introducing a `use num_traits::Float as Fl` alias. This was a compilation necessity due to `BaseFloat` having both `Float` and `RealField` as supertraits.

## Self-check

- [x] `cargo nextest run -p monstertruck-math -E 'test(polynomial)'` -- 7/7 tests pass
- [x] `cargo clippy -p monstertruck-math --all-targets -- -W warnings` -- clean
- [x] All 5 public functions exported from `polynomial` module
- [x] All functions generic over `BaseFloat`
- [x] Newton refinement present in `pre_solve_cubic` and `pre_solve_quartic`
- [x] No references to `cgmath` or `matext4cgmath` in new code
