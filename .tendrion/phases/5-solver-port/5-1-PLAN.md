---
phase: 5-solver-port
plan: 1
type: tdd
wave: 1
depends_on: []
files_modified:
  - monstertruck-math/Cargo.toml
  - monstertruck-math/src/polynomial.rs
  - monstertruck-math/src/lib.rs
autonomous: true
must_haves:
  truths:
    - "solve_quadratic(3.0, -4.0) returns roots at -4.0 and 1.0"
    - "pre_solve_cubic(-7.0, -6.0) returns roots at -2.0, -1.0, and 3.0"
    - "solve_cubic(-3.0, 0.0, 4.0) returns roots at -1.0, 2.0, and 2.0"
    - "pre_solve_quartic(-5.0, 0.0, 4.0) returns roots at -2.0, -1.0, 1.0, and 2.0"
    - "solve_quartic(1.0, -7.0, -1.0, 6.0) returns roots at -3.0, -1.0, 1.0, and 2.0"
    - "All solver functions use Newton refinement for polishing"
    - "All solver functions are generic over BaseFloat"
  artifacts:
    - path: "monstertruck-math/src/polynomial.rs"
      provides: "Polynomial solver functions ported from matext4cgmath with Algorithm 954 rescaling"
      min_lines: 150
      contains: "solve_quartic"
    - path: "monstertruck-math/Cargo.toml"
      provides: "num-complex dependency for Complex return types"
      min_lines: 10
      contains: "num-complex"
  key_links:
    - from: "monstertruck-math/src/polynomial.rs"
      to: "monstertruck-math/src/traits.rs"
      via: "BaseFloat trait bound on all solver functions"
      pattern: "BaseFloat"
    - from: "monstertruck-math/src/lib.rs"
      to: "monstertruck-math/src/polynomial.rs"
      via: "pub mod polynomial declaration and re-export"
      pattern: "pub mod polynomial"
---

<objective>
Create a `polynomial` module in monstertruck-math containing all five solver functions (solve_quadratic, pre_solve_cubic, solve_cubic, pre_solve_quartic, solve_quartic) ported from the matext4cgmath crate, with comprehensive tests verifying numerical correctness.
</objective>

<execution_context>
@skills/tdd-enforcement/SKILL.md
</execution_context>

<context>
@target/doc/src/matext4cgmath/solver.rs.html
@monstertruck-math/src/lib.rs
@monstertruck-math/src/traits.rs
@monstertruck-math/Cargo.toml
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add num-complex dependency and declare polynomial module</name>
  <files>monstertruck-math/Cargo.toml, monstertruck-math/src/lib.rs</files>
  <action>
1. Add `num-complex = "0.4"` to `[dependencies]` in `monstertruck-math/Cargo.toml`.

2. In `monstertruck-math/src/lib.rs`:
   - Add `/// Polynomial equation solvers (quadratic, cubic, quartic).` doc comment
   - Add `pub mod polynomial;` declaration (after the existing `pub mod types;` line)
   - Add `pub use num_complex;` re-export (after the existing `pub use num_traits;` line) so downstream crates can use `Complex` without adding their own dep

3. Create an empty `monstertruck-math/src/polynomial.rs` file with a module doc comment:
   ```rust
   //! Solvers for polynomial equations up to degree 4.
   //!
   //! Ported from `matext4cgmath::solver`. All functions return roots as
   //! `Complex<F>` arrays and apply Newton refinement for numerical precision.
   ```
  </action>
  <verify>Run `cargo check -p monstertruck-math` — should compile with the new empty module and dependency.</verify>
  <done>num-complex dependency added, polynomial module declared and compiles.</done>
</task>

<task type="auto">
  <name>Task 2: Port all five solver functions with Newton refinement</name>
  <files>monstertruck-math/src/polynomial.rs</files>
  <action>
Port the following five functions from the original matext4cgmath solver (visible in `target/doc/src/matext4cgmath/solver.rs.html`) into `monstertruck-math/src/polynomial.rs`. Each function must:
- Use `crate::traits::BaseFloat` as the generic bound (not `matext4cgmath::BaseFloat`)
- Use `num_complex::Complex<F>` for return types
- Include Newton refinement (polishing) loops exactly as in the original
- Include doc comments with the equation being solved

The five functions and their signatures:

```rust
use crate::traits::BaseFloat;
use num_complex::Complex;

/// Solve x² + ax + b = 0.
pub fn solve_quadratic<F: BaseFloat>(a: F, b: F) -> [Complex<F>; 2]

/// Solve x³ + px + q = 0 (depressed cubic).
pub fn pre_solve_cubic<F: BaseFloat>(p: F, q: F) -> [Complex<F>; 3]

/// Solve x³ + ax² + bx + c = 0.
pub fn solve_cubic<F: BaseFloat>(a: F, b: F, c: F) -> [Complex<F>; 3]

/// Solve x⁴ + px² + qx + r = 0 (depressed quartic).
pub fn pre_solve_quartic<F: BaseFloat>(p: F, q: F, r: F) -> [Complex<F>; 4]

/// Solve x⁴ + ax³ + bx² + cx + d = 0.
pub fn solve_quartic<F: BaseFloat>(a: F, b: F, c: F, d: F) -> [Complex<F>; 4]
```

Implementation notes for the port:
- The original `crate::*` import brought in `BaseFloat` and `Complex` — replace with explicit imports from `crate::traits::BaseFloat` and `num_complex::Complex`.
- `Complex::powf`, `Complex::norm`, `Complex::norm_sqr`, `Complex::sqrt` are all available in `num_complex 0.4`.
- `F::powi(-F::one(), i)` in `pre_solve_quartic` uses the `Float::powi` method which is part of `num_traits::Float`, already a supertrait of `BaseFloat`.
- The `solve_cubic` function calls `pre_solve_cubic` internally.
- The `solve_quartic` function calls `pre_solve_quartic` internally, which calls `solve_cubic` internally.
- Preserve the exact numerical algorithm — do NOT simplify or "improve" the math.

The Newton refinement pattern (used in both `pre_solve_cubic` and `pre_solve_quartic`):
```rust
let eps_2 = F::sqrt(F::epsilon());
res.iter_mut().for_each(|x| {
    let mut f = /* evaluate polynomial at *x */;
    let mut f_prime = /* evaluate derivative at *x */;
    while f.norm() > eps_2 * f_prime.norm() {
        if f_prime.norm() < eps_2 { return; }
        *x -= f / f_prime;
        // re-evaluate f and f_prime
    }
});
```
  </action>
  <verify>Run `cargo check -p monstertruck-math` — all five functions compile without errors.</verify>
  <done>All five polynomial solver functions ported with Newton refinement and compile successfully.</done>
</task>

<task type="auto">
  <name>Task 3: Add unit tests for all solver functions</name>
  <files>monstertruck-math/src/polynomial.rs</files>
  <action>
Add a `#[cfg(test)] mod tests` block at the bottom of `polynomial.rs` with the following tests. Use `const EPS: f64 = 1.0e-10;` for quadratic/cubic and `const EPS_QUARTIC: f64 = 1.0e-7;` for quartic (matching original tolerances).

Tests to include:

1. `test_solve_quadratic_real_roots` — solve x² + 3x - 4 = 0 (roots: -4, 1). Verify both roots have negligible imaginary parts and correct real parts.

2. `test_solve_quadratic_complex_roots` — solve x² + 2x + 5 = 0 (roots: -1±2i). Verify both complex roots.

3. `test_pre_solve_cubic` — solve x³ - 7x - 6 = 0 (roots: -2, -1, 3). Sort by real part, verify.

4. `test_solve_cubic` — solve x³ - 3x² + 4 = 0 (roots: -1, 2, 2). Sort by real part, verify.

5. `test_pre_solve_quartic` — solve x⁴ - 5x² + 4 = 0 (roots: -2, -1, 1, 2). Sort by real part, verify.

6. `test_solve_quartic` — solve x⁴ + x³ - 7x² - x + 6 = 0 (roots: -3, -1, 1, 2). Sort by real part, verify.

7. `test_solve_quartic_substitution_check` — for each root r of solve_quartic(1, -7, -1, 6), verify r⁴ + r³ - 7r² - r + 6 ≈ 0.

Each test should follow this pattern:
```rust
#[test]
fn test_name() {
    let mut res = function_call(args);
    res.sort_by(|x, y| x.re.partial_cmp(&y.re).unwrap());
    let ans = [Complex::from(expected_root_1), ...];
    for (x, y) in res.iter().zip(ans.iter()) {
        assert!(Complex::norm(x - y) < EPS, "got {x}, expected {y}");
    }
}
```
  </action>
  <verify>Run `cargo test -p monstertruck-math -- polynomial` — all tests pass.</verify>
  <done>Seven unit tests added and all pass, confirming numerical correctness of ported solvers.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-math` compiles cleanly
2. `cargo test -p monstertruck-math -- polynomial` — all 7 tests pass
3. `polynomial.rs` exports all 5 public functions: solve_quadratic, pre_solve_cubic, solve_cubic, pre_solve_quartic, solve_quartic
4. All functions are generic over `BaseFloat` (not hardcoded to f64)
5. Newton refinement loops are present in pre_solve_cubic and pre_solve_quartic
6. No references to cgmath or matext4cgmath remain in the new code
</verification>

<success_criteria>
- monstertruck-math compiles with the new polynomial module
- All solver functions produce numerically correct results verified by tests
- Functions use the existing BaseFloat trait from monstertruck-math (no new trait dependencies)
- Code is a faithful port of the matext4cgmath solver with Newton polishing
</success_criteria>

<output>
After completion, create `.tendrion/phases/5-solver-port/5-1-SUMMARY.md`
</output>
