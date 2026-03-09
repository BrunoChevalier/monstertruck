---
phase: 2-numerical-robustness
plan: 1
tags: [tolerance, newton, robustness, tdd]
key-files:
  - monstertruck-core/src/tolerance.rs
  - monstertruck-core/src/newton.rs
  - monstertruck-core/tests/tolerance_propagation.rs
  - monstertruck-core/tests/newton_fallback.rs
decisions: []
metrics:
  tests_added: 15
  tests_total: 43
  all_passing: true
  clippy_clean: true
---

## What Was Built

### `monstertruck-core/src/tolerance.rs`
- Added `OperationTolerance` struct with per-operation tolerance tracking: `new()`, `from_global()`, `after_operation()`, `effective_tolerance()`, `within_budget()`, and getters for `base`, `accumulated_error`, `operation_count`, `last_operation`.
- All existing code (`TOLERANCE`, `TOLERANCE2`, `Tolerance` trait, `Origin` trait, macros) unchanged.

### `monstertruck-core/src/newton.rs`
- Added `SolveResult<V>` struct wrapping solution value and fallback metadata.
- Added `solve_robust<V, M>()` -- generic robust solver with Levenberg-Marquardt fallback.
- Added `solve_robust_1d()` -- 1D-specific robust solver with Newton -> LM -> bisection cascade.
- Added `Jacobian::invert_damped()` method for LM damping on Jacobian diagonal.
- Added `NewtonLog::used_fallback()` getter and internal `set_used_fallback()`.
- Existing `solve()` function signature and behavior unchanged.
- Improved doc comments to end with periods.

### `monstertruck-core/tests/tolerance_propagation.rs`
- 9 tests: construction, after_operation, chaining, effective_tolerance, within_budget (true/false), from_global, backward compat for constant and trait methods.

### `monstertruck-core/tests/newton_fallback.rs`
- 6 tests: well-conditioned convergence, near-singular 1D with fallback, 2D near-singular matrix, fallback reporting, bisection fallback for flat derivatives, well-conditioned no-fallback verification.

## TDD Compliance

- RED: Tests written first, confirmed failing with compilation errors (types not yet defined).
- GREEN: Minimal implementation to pass all tests.
- REFACTOR: Cleaned up doc comments, simplified match nesting in LM solver.

## Verification

- `cargo nextest run -p monstertruck-core`: 43/43 tests pass.
- `cargo test --benches -p monstertruck-core`: benchmarks compile.
- `cargo clippy -p monstertruck-core --all-targets -- -W warnings`: clean.
- `cargo fmt --all`: applied.
- `TOLERANCE` constant unchanged at 1.0e-6.
- Existing `solve()` behavior identical for all callers.

## Deviations

None.
