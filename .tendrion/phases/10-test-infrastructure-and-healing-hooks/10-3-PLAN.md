---
phase: 10-test-infrastructure-and-healing-hooks
plan: 3
type: execute
wave: 2
depends_on: ["10-1", "10-2"]
files_modified:
  - monstertruck-solid/tests/healing_fixtures.rs
autonomous: true
must_haves:
  truths:
    - "User runs cargo test and at least 3 degenerate-geometry fixtures trigger healing code paths"
    - "User runs cargo test and all fixture-based healing tests produce valid topology (no panics, no timeouts)"
    - "Each healing test verifies the output shell has ShellCondition::Regular or better"
    - "Tests exercise sweep_rail, birail, and gordon fixture shells through heal_surface_shell"
    - "Tests include a glyph-profile fixture that is swept and healed successfully"
  artifacts:
    - path: "monstertruck-solid/tests/healing_fixtures.rs"
      provides: "Integration tests exercising fixture corpus through healing pipeline"
      min_lines: 120
      contains: "heal_surface_shell"
  key_links:
    - from: "monstertruck-solid/tests/fixture_helpers.rs"
      to: "monstertruck-solid/tests/healing_fixtures.rs"
      via: "Integration test imports fixture helpers"
      pattern: "mod fixture_helpers"
    - from: "monstertruck-solid/src/healing/surface_healing.rs"
      to: "monstertruck-solid/tests/healing_fixtures.rs"
      via: "Tests call heal_surface_shell from public API"
      pattern: "monstertruck_solid::heal_surface_shell"
---

<objective>
Create integration tests that exercise the fixture corpus through the healing pipeline, proving that at least 3 degenerate-geometry fixtures trigger healing code paths and produce valid topology. This validates the end-to-end chain: fixture creation -> surface construction -> topology healing -> valid shell.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/tests/feature_integration.rs
@monstertruck-solid/tests/boolean_edge_cases.rs
@monstertruck-solid/src/healing/mod.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create integration tests for fixture-driven healing</name>
  <files>monstertruck-solid/tests/healing_fixtures.rs</files>
  <action>
Create `monstertruck-solid/tests/healing_fixtures.rs` as a new integration test file. This file imports the fixture helpers from plan 1 and calls `heal_surface_shell` from plan 2.

Include the `fixture_helpers` module:
```rust
mod fixture_helpers;
```

The test file should contain these test functions:

**1. Sweep rail with kinked rail (triggers gap welding):**
```rust
#[test]
fn heal_sweep_rail_kinked() {
    // Load the kinked-rail fixture shell
    // Call heal_surface_shell with tol = 0.05
    // Assert Ok result
    // Assert shell condition is at least Regular
    // Assert the shell has the expected number of faces (1 or more)
}
```

**2. Birail with diverging rails (triggers gap welding + degenerate removal):**
```rust
#[test]
fn heal_birail_diverging() {
    // Load the diverging-rails fixture shell
    // Call heal_surface_shell
    // Assert Ok result
    // Assert valid topology
}
```

**3. Gordon with degenerate curves (triggers all healing stages):**
```rust
#[test]
fn heal_gordon_degenerate() {
    // Load the gordon-degenerate fixture shell
    // Call heal_surface_shell
    // Assert Ok result
    // Assert valid topology
}
```

**4. Collapsed edge shell (triggers degenerate edge removal):**
```rust
#[test]
fn heal_collapsed_edge() {
    // Load the collapsed-edge fixture shell
    // Call heal_surface_shell
    // Assert Ok result
    // Verify degenerate edges were removed (shell has fewer edges than input)
}
```

**5. Glyph profile sweep healing:**
```rust
#[test]
fn heal_glyph_sweep() {
    // Use the glyph sharp-corners fixture to create a profile
    // Sweep it along a simple rail using BsplineSurface::sweep_rail
    // Build a CompressedShell from the result
    // Call heal_surface_shell
    // Assert Ok
}
```

**6. Panic safety test (no fixture causes panic):**
```rust
#[test]
fn all_fixtures_no_panic() {
    // Iterate over all FIXTURE_NAMES from fixture_helpers
    // For each, load the fixture and call heal_surface_shell inside catch_unwind
    // Assert none panic
    // Log which ones succeeded vs returned Err (Err is OK, panic is not)
}
```

**7. Timeout safety (ensure healing doesn't hang):**
Use the pattern from existing tests: wrap healing calls in a thread with a timeout. Each fixture should complete healing within 10 seconds.

Each test should:
- Use `std::panic::{AssertUnwindSafe, catch_unwind}` for panic safety where appropriate
- Check `ShellCondition` on successful results using `shell.shell_condition()`
- Print diagnostic info on failure (fixture name, error details) for CI debugging

Follow the existing test patterns from `boolean_edge_cases.rs` and `feature_integration.rs`.
  </action>
  <verify>Run `cargo test -p monstertruck-solid --test healing_fixtures` and confirm all tests pass with no panics or timeouts. Verify that at least 3 tests actually exercise healing code paths (check that weld_gap_edges or remove_degenerate_edges returns nonzero counts — this can be verified by the tests themselves printing or asserting on healing metrics).</verify>
  <done>Integration test file created with 7 tests covering all fixture types through the healing pipeline, all passing without panics or timeouts.</done>
</task>

<task type="auto">
  <name>Task 2: Verify full test suite and cross-crate consistency</name>
  <files>monstertruck-solid/tests/healing_fixtures.rs, monstertruck-solid/tests/fixture_helpers.rs</files>
  <action>
Run the complete test suite to verify no regressions:

1. Run `cargo test -p monstertruck-geometry` — verify fixture module doesn't break existing tests
2. Run `cargo test -p monstertruck-solid` — verify all tests pass including new healing_fixtures
3. Run `cargo test -p monstertruck-modeling --features font` — verify font/text module still works (relevant since we have glyph fixtures)
4. Run `cargo clippy --workspace -- -D warnings` to catch any new warnings

If any test fails:
- Adjust fixture parameters (tolerances, control point positions) to make the geometry tractable
- If a fixture genuinely cannot be healed (too degenerate), ensure the test asserts `Err` with the appropriate `SurfaceHealingError` variant rather than panicking
- Update `fixture_helpers.rs` if fixture names or signatures changed during plan 1 execution

Ensure the `all_fixtures_no_panic` test actually catches at least 3 fixtures that go through non-trivial healing (not just pass-through). This may require adjusting fixture geometry to ensure they actually have topology issues that need repair.
  </action>
  <verify>Run `cargo test --workspace` and confirm zero failures. Run `cargo clippy --workspace -- -D warnings` with no new warnings.</verify>
  <done>Full workspace test suite passes with zero failures, all healing fixture tests exercise actual healing code paths, no clippy warnings.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-solid --test healing_fixtures` passes all 7 tests
2. At least 3 tests trigger actual healing operations (gap welding or degenerate edge removal with nonzero counts)
3. No test panics — all failures are returned as `Result::Err`
4. No test takes more than 10 seconds
5. `cargo test --workspace` passes with no regressions
6. Success criteria 3 from ROADMAP is met: "At least 3 degenerate-geometry fixtures trigger healing code paths and produce valid topology"
7. Success criteria 4 from ROADMAP is met: "Running cargo test with the new fixtures produces no panics or timeouts"
</verification>

<success_criteria>
- Fixtures from plan 1 are successfully loaded and processed through healing from plan 2
- At least 3 degenerate geometries trigger non-trivial healing and produce valid topology
- All fixture-based tests pass without panics or timeouts
- Full workspace cargo test passes with zero regressions
</success_criteria>

<output>
After completion, create `.tendrion/phases/10-test-infrastructure-and-healing-hooks/10-3-SUMMARY.md`
</output>
