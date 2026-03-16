---
phase: 5-solver-port
plan: 2
type: execute
wave: 2
depends_on: ["5-1"]
files_modified:
  - Cargo.toml
  - monstertruck-core/src/cgmath64.rs
  - monstertruck-geometry/src/specifieds/hyperbola.rs
  - monstertruck-geometry/src/specifieds/parabola.rs
autonomous: true
must_haves:
  truths:
    - "cargo build --workspace succeeds with zero errors"
    - "hyperbola.rs calls polynomial::solve_quartic instead of solver::solve_quartic"
    - "parabola.rs calls polynomial::pre_solve_cubic instead of solver::pre_solve_cubic"
    - "Existing parabola snp_test and sp_test pass without modification"
    - "No unresolved solver:: references remain in the workspace"
  artifacts:
    - path: "monstertruck-core/src/cgmath64.rs"
      provides: "Re-export of polynomial solver module so downstream crates can use it"
      min_lines: 10
      contains: "polynomial"
    - path: "monstertruck-geometry/src/specifieds/hyperbola.rs"
      provides: "Updated hyperbola solver call using polynomial module"
      min_lines: 50
      contains: "polynomial::solve_quartic"
    - path: "monstertruck-geometry/src/specifieds/parabola.rs"
      provides: "Updated parabola solver call using polynomial module"
      min_lines: 50
      contains: "polynomial::pre_solve_cubic"
  key_links:
    - from: "monstertruck-geometry/src/specifieds/hyperbola.rs"
      to: "monstertruck-math/src/polynomial.rs"
      via: "cgmath64 re-export chain: monstertruck-math -> monstertruck-core::cgmath64 -> monstertruck-geometry::base"
      pattern: "polynomial::solve_quartic"
    - from: "monstertruck-geometry/src/specifieds/parabola.rs"
      to: "monstertruck-math/src/polynomial.rs"
      via: "cgmath64 re-export chain: monstertruck-math -> monstertruck-core::cgmath64 -> monstertruck-geometry::base"
      pattern: "polynomial::pre_solve_cubic"
    - from: "monstertruck-core/src/cgmath64.rs"
      to: "monstertruck-math/src/polynomial.rs"
      via: "pub use re-export of polynomial module"
      pattern: "polynomial"
---

<objective>
Wire the new polynomial solver module into the existing call sites in monstertruck-geometry (hyperbola.rs and parabola.rs) so that `cargo build --workspace` succeeds with zero unresolved solver references, and existing tests pass.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@monstertruck-math/src/lib.rs
@monstertruck-core/src/cgmath64.rs
@monstertruck-geometry/src/specifieds/hyperbola.rs
@monstertruck-geometry/src/specifieds/parabola.rs
@monstertruck-geometry/src/specifieds/mod.rs
@monstertruck-geometry/src/lib.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Expose polynomial module through the re-export chain</name>
  <files>Cargo.toml, monstertruck-core/src/cgmath64.rs</files>
  <action>
The import chain for geometry types is: `monstertruck-math` -> `monstertruck-core::cgmath64` -> `monstertruck-geometry::base` -> `monstertruck-geometry::prelude`. The `solver::` references in hyperbola.rs and parabola.rs come through `use crate::{prelude::*, *}` in specifieds/mod.rs. We need the `polynomial` module accessible via this chain.

1. Add `num-complex = "0.4"` to `[workspace.dependencies]` in the root `Cargo.toml` (so it can be used as a workspace dependency).

2. Update `monstertruck-math/Cargo.toml` to use `num-complex = { workspace = true }` instead of the direct version specification added in Plan 5-1.

3. In `monstertruck-core/src/cgmath64.rs`, add a re-export of the polynomial module so it's available as `polynomial` in downstream crates:
   ```rust
   pub use monstertruck_math::polynomial;
   ```
   This makes `polynomial::solve_quartic` and `polynomial::pre_solve_cubic` available in any file that does `use crate::base::*` or `use crate::prelude::*` (since `base` re-exports `cgmath64::*`).

4. Verify that `monstertruck-core` already depends on `monstertruck-math` in its Cargo.toml (it does). No additional dependency changes needed for monstertruck-core.

5. For monstertruck-geometry, verify it depends on monstertruck-core (it does via `monstertruck-core = { workspace = true }` through monstertruck-traits). The `num_complex::Complex` type will be available through the `polynomial` module's return types — callers don't need to import num_complex directly since they destructure the Complex values.
  </action>
  <verify>Run `cargo check -p monstertruck-core` — compiles with the polynomial re-export.</verify>
  <done>Polynomial module is re-exported through cgmath64 and accessible in the prelude chain.</done>
</task>

<task type="auto">
  <name>Task 2: Update solver call sites in hyperbola.rs and parabola.rs</name>
  <files>monstertruck-geometry/src/specifieds/hyperbola.rs, monstertruck-geometry/src/specifieds/parabola.rs</files>
  <action>
1. In `monstertruck-geometry/src/specifieds/hyperbola.rs` line 82, change:
   ```rust
   solver::solve_quartic(a, b, c, d)
   ```
   to:
   ```rust
   polynomial::solve_quartic(a, b, c, d)
   ```
   No other changes needed — the function signature is identical (returns `[Complex<f64>; 4]`), and the `.into_iter().filter_map(|z| ...)` chain that follows works the same way since `z.im` and `z.re` are the same Complex fields.

2. In `monstertruck-geometry/src/specifieds/parabola.rs` line 84, change:
   ```rust
   solver::pre_solve_cubic(p, q)
   ```
   to:
   ```rust
   polynomial::pre_solve_cubic(p, q)
   ```
   Same rationale — identical signature, identical return type, identical downstream usage.

3. Verify no other `solver::` references exist in the workspace (there should be none based on the grep results).

Important: Do NOT modify any other code in these files. The surrounding logic (filter_map for imaginary-part filtering, `.so_small()` calls, distance comparisons) must remain exactly as-is.
  </action>
  <verify>Run `cargo check -p monstertruck-geometry` — compiles with zero errors. Run `grep -r "solver::" monstertruck-geometry/src/` — returns no matches.</verify>
  <done>Both call sites updated from solver:: to polynomial:: and compile cleanly.</done>
</task>

<task type="auto">
  <name>Task 3: Verify full workspace build and existing tests</name>
  <files>monstertruck-geometry/src/specifieds/hyperbola.rs, monstertruck-geometry/src/specifieds/parabola.rs</files>
  <action>
1. Run `cargo build --workspace` to confirm the entire workspace compiles. This is the primary success criterion from the phase requirements.

2. Run `cargo test -p monstertruck-geometry -- parabola` to confirm the existing `snp_test` and `sp_test` tests in parabola.rs pass without modification. These tests exercise `search_nearest_parameter` which calls `polynomial::pre_solve_cubic`.

3. Run `cargo test -p monstertruck-geometry -- hyperbola` to confirm any hyperbola tests pass.

4. Run `cargo test -p monstertruck-math -- polynomial` one final time to confirm the solver unit tests still pass in context of the full workspace.

If any test fails, investigate whether it's a regression from the port (signature mismatch, different return ordering) vs. a pre-existing issue. The solver functions must produce identical results to the matext4cgmath originals.
  </action>
  <verify>`cargo build --workspace` exits 0. `cargo test -p monstertruck-geometry -- parabola` passes. `cargo test -p monstertruck-math -- polynomial` passes.</verify>
  <done>Full workspace builds successfully, all existing geometry tests pass, solver port is complete.</done>
</task>

</tasks>

<verification>
1. `cargo build --workspace` succeeds with exit code 0
2. Zero `solver::` references remain in monstertruck-geometry source files
3. `cargo test -p monstertruck-geometry -- parabola` — snp_test and sp_test pass
4. `cargo test -p monstertruck-math -- polynomial` — all solver unit tests pass
5. The polynomial module is accessible via the standard prelude chain (monstertruck-math -> monstertruck-core::cgmath64 -> monstertruck-geometry::base)
</verification>

<success_criteria>
- `cargo build --workspace` succeeds with zero unresolved `solver::` references
- Existing tests that exercise hyperbola and parabola geometry pass without modification
- The wiring follows the established re-export pattern (cgmath64 -> base -> prelude)
- No unnecessary dependency additions to downstream crates
</success_criteria>

<output>
After completion, create `.tendrion/phases/5-solver-port/5-2-SUMMARY.md`
</output>
