---
phase: 2-numerical-robustness
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-core/src/tolerance.rs
  - monstertruck-core/src/newton.rs
  - monstertruck-core/tests/tolerance_propagation.rs
  - monstertruck-core/tests/newton_fallback.rs
autonomous: true
must_haves:
  truths:
    - "OperationTolerance struct correctly tracks accumulated error across sequential after_operation calls and reports effective_tolerance and within_budget"
    - "User calls Newton solver on a near-singular Jacobian and solver falls back to Levenberg-Marquardt or bisection instead of returning Err"
    - "User calls Newton solver on well-conditioned problems and results are identical to current behavior"
    - "Previously divergent test cases now converge via the fallback path"
    - "Tolerance struct carries operation count, accumulated error bound, and source operation tag"
  artifacts:
    - path: "monstertruck-core/src/tolerance.rs"
      provides: "OperationTolerance struct with per-operation tolerance propagation and accumulation tracking"
      min_lines: 80
      contains: "OperationTolerance"
    - path: "monstertruck-core/src/newton.rs"
      provides: "Newton solver with Levenberg-Marquardt and bisection fallbacks for near-singular Jacobians"
      min_lines: 120
      contains: "levenberg_marquardt"
    - path: "monstertruck-core/tests/tolerance_propagation.rs"
      provides: "Tests for tolerance propagation through chained operations"
      min_lines: 60
      contains: "propagat"
    - path: "monstertruck-core/tests/newton_fallback.rs"
      provides: "Tests for Newton solver fallback behavior on near-singular and degenerate cases"
      min_lines: 80
      contains: "fallback"
  key_links:
    - from: "monstertruck-core/src/tolerance.rs"
      to: "monstertruck-core/src/newton.rs"
      via: "Both modules live in monstertruck-core and share the tolerance module"
      pattern: "tolerance"
    - from: "monstertruck-core/src/newton.rs"
      to: "monstertruck-core/src/lib.rs"
      via: "Re-export of new solver variants and tolerance types"
      pattern: "pub mod newton"
---

<objective>
Implement per-operation tolerance propagation as a foundation for replacing the global TOLERANCE constant, and add Levenberg-Marquardt and bisection fallbacks to the Newton solver for near-singular Jacobians. These two foundational changes in monstertruck-core enable all downstream robustness improvements.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-core/src/tolerance.rs
@monstertruck-core/src/newton.rs
@monstertruck-core/src/lib.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write failing tests for tolerance propagation and Newton fallbacks</name>
  <files>monstertruck-core/tests/tolerance_propagation.rs, monstertruck-core/tests/newton_fallback.rs</files>
  <action>
Create two test files following TDD red-green-refactor:

**monstertruck-core/tests/tolerance_propagation.rs:**
- Test that `OperationTolerance::new(1e-6)` creates a tolerance with base value 1e-6 and zero accumulated error.
- Test that `op_tol.after_operation("boolean", local_error)` returns a new `OperationTolerance` with accumulated error increased by `local_error` and operation count incremented.
- Test chaining: boolean (err=1e-7) -> fillet (err=2e-7) -> tessellation (err=5e-7) produces accumulated error = 8e-7 and operation count = 3.
- Test `op_tol.effective_tolerance()` returns `base + accumulated_error`.
- Test `op_tol.within_budget()` returns true when accumulated_error < base, false otherwise.
- Test that the existing `Tolerance` trait and `TOLERANCE` constant remain available and unchanged for backward compatibility.
- Test that `near()` and `near2()` still work with the global constant.

**monstertruck-core/tests/newton_fallback.rs:**
- Test that `solve()` still converges for well-conditioned 1D problems (sqrt(2)).
- Test a near-singular 1D case: f(x) = x^3 near x=0 where f'(0) = 0. The standard Newton should fail but `solve_robust()` should converge.
- Test a 2D near-singular case using `Matrix2` with near-zero determinant. `solve_robust()` should fall back and converge.
- Test that `solve_robust()` returns `Ok` with the `NewtonLog` indicating fallback was used (add a `used_fallback()` method to NewtonLog).
- Test bisection fallback for 1D case where Levenberg-Marquardt also struggles: sign-changing bracket with very flat derivative.
- All tests should use `cargo nextest run` for verification.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core --test tolerance_propagation --test newton_fallback` and confirm all tests fail with compilation errors (types do not yet exist).</verify>
  <done>Failing test files created for both tolerance propagation and Newton solver fallbacks.</done>
</task>

<task type="auto">
  <name>Task 2: Implement OperationTolerance and update tolerance module</name>
  <files>monstertruck-core/src/tolerance.rs, monstertruck-core/src/lib.rs</files>
  <action>
Extend `monstertruck-core/src/tolerance.rs` with an `OperationTolerance` struct while keeping all existing code intact:

```rust
/// Per-operation tolerance tracking to prevent error accumulation across chained operations.
#[derive(Clone, Debug)]
pub struct OperationTolerance {
    /// Base tolerance for individual operations.
    base: f64,
    /// Accumulated error from prior operations in the pipeline.
    accumulated_error: f64,
    /// Number of operations that have contributed to accumulated error.
    operation_count: usize,
    /// Tag of the most recent operation.
    last_operation: Option<&'static str>,
}
```

Methods to implement:
- `new(base: f64) -> Self` -- creates with zero accumulated error.
- `after_operation(&self, operation: &'static str, local_error: f64) -> Self` -- returns new instance with updated accumulation.
- `effective_tolerance(&self) -> f64` -- returns `base + accumulated_error`.
- `within_budget(&self) -> bool` -- returns `accumulated_error < base`.
- `base(&self)`, `accumulated_error(&self)`, `operation_count(&self)`, `last_operation(&self)` -- getters.
- `from_global() -> Self` -- creates from the existing `TOLERANCE` constant.

Keep `TOLERANCE`, `TOLERANCE2`, the `Tolerance` trait, `Origin` trait, and all macros unchanged.
Ensure all comments end with a period.
Re-export `OperationTolerance` from `lib.rs` tolerance module (already re-exported via `pub mod tolerance`).
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core --test tolerance_propagation` and confirm tolerance propagation tests pass.</verify>
  <done>OperationTolerance struct implemented with propagation, accumulation, and budget checking.</done>
</task>

<task type="auto">
  <name>Task 3: Implement Newton solver fallbacks with Levenberg-Marquardt and bisection</name>
  <files>monstertruck-core/src/newton.rs</files>
  <action>
Add robust solver variants to `monstertruck-core/src/newton.rs` while keeping the existing `solve()` function unchanged:

1. **Add `SolveResult` struct** that wraps the solution value and metadata:
```rust
/// Result of a robust Newton solve including fallback metadata.
#[derive(Clone, Debug)]
pub struct SolveResult<V> {
    /// The converged solution.
    pub value: V,
    /// Whether a fallback method was used.
    pub used_fallback: bool,
}
```

2. **Add `levenberg_marquardt` fallback** for the general case:
   - When `Jacobian::invert()` returns `None`, add a damping factor `lambda` to the diagonal.
   - For `f64` Jacobian: `inv = 1.0 / (derivation + lambda)` where `lambda` starts at 1e-4 and grows by 10x per failure.
   - For matrix Jacobians: not needed initially (mark as TODO), just retry with damping on the diagonal.
   - Decrease lambda when step reduces residual, increase when it does not.

3. **Add `bisection_fallback_1d`** for 1D case only:
   - Only applicable when `V = f64` and `M = f64`.
   - Requires a bracket `[a, b]` where `f(a)` and `f(b)` have opposite signs.
   - Standard bisection with tolerance check.

4. **Add `solve_robust` function**:
```rust
pub fn solve_robust<V, M>(
    function: impl Fn(V) -> CalcOutput<V, M>,
    hint: V,
    trials: usize,
) -> Result<SolveResult<V>, NewtonLog<V>>
where
    V: Sub<Output = V> + Copy + Tolerance,
    M: Jacobian<V>,
```
   - First tries standard Newton (same as `solve`).
   - On failure due to degenerate Jacobian, tries Levenberg-Marquardt with damping.
   - Returns `SolveResult` with `used_fallback = true` if LM was needed.

5. **Add `solve_robust_1d` function** for scalar case:
   - Tries Newton, then LM, then bisection if a bracket can be found near the hint.
   - Searches for bracket by evaluating function at `hint - delta` and `hint + delta` for increasing `delta`.

6. **Update `NewtonLog`**:
   - Add `used_fallback: bool` field and `pub fn used_fallback(&self) -> bool` getter.

Keep existing `solve()` function signature and behavior exactly as-is.
Ensure all doc comments end with a period.
Use functional style where possible. Avoid `unsafe` code.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-core --test newton_fallback` and confirm all Newton fallback tests pass. Also run `cargo nextest run -p monstertruck-core` to confirm existing tests still pass.</verify>
  <done>Newton solver augmented with Levenberg-Marquardt and bisection fallbacks via solve_robust and solve_robust_1d functions.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-core` -- all existing and new tests pass.
2. `cargo test --benches -p monstertruck-core` -- benchmarks compile.
3. Global `TOLERANCE` constant is unchanged at 1.0e-6.
4. Existing `Tolerance` trait methods `near()` and `near2()` are unmodified.
5. `OperationTolerance` correctly tracks accumulated error across chained operations.
6. `solve_robust()` converges on previously-divergent near-singular cases.
7. `solve()` behavior is identical to before for all existing callers.
</verification>

<success_criteria>
- ROBUST-01 foundation: OperationTolerance struct provides per-operation tolerance propagation tracking
- ROBUST-02 complete: Newton solver has Levenberg-Marquardt and bisection fallbacks for near-singular Jacobians
- Backward compatibility: existing TOLERANCE constant, Tolerance trait, and solve() function unchanged
- All tests pass with cargo nextest run
</success_criteria>

<output>
After completion, create `.tendrion/phases/2-numerical-robustness/2-1-SUMMARY.md`
</output>
