# Review Context: 8-1 Spec Compliance (Round 3 of 3)

## Plan

**Plan ID:** 8-1
**Commit Range:** 5ee7ca72122d53cba77238e72928dd787fee0d94..100f42259b9ae1a506cdaafd4ae81efd4a092d4e

### Plan Content

---
phase: 8-validation-and-documentation
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/validate.rs
  - monstertruck-solid/src/fillet/mod.rs
  - monstertruck-solid/src/fillet/edge_select.rs
  - monstertruck-solid/src/fillet/ops.rs
autonomous: true
must_haves:
  truths:
    - "Debug builds run Euler-Poincare check (V - E + F = 2) on closed shells after every fillet topology modification"
    - "shell.shell_condition() returns Oriented or Closed after fillet operations in all existing test cases"
    - "Release builds do not pay runtime cost for topology invariant checks"
    - "The current fillet test suite continues to pass without modification"
    - "A new test demonstrates the debug assertion fires on a shell with corrupted orientation"
    - "A direct unit test verifies euler_poincare_check returns true for a valid closed shell and false for an invalid one"
  artifacts:
    - path: "monstertruck-solid/src/fillet/validate.rs"
      provides: "Euler-Poincare and orientation validation functions with #[cfg(test)] module for corruption tests"
      min_lines: 80
      contains: "euler_poincare"
    - path: "monstertruck-solid/src/fillet/edge_select.rs"
      provides: "Post-fillet debug assertions in fillet_edges and fillet_edges_generic"
      min_lines: 700
      contains: "debug_assert_topology"
    - path: "monstertruck-solid/src/fillet/ops.rs"
      provides: "Post-fillet debug assertions in fillet_along_wire"
      min_lines: 580
      contains: "debug_assert_topology"
  key_links:
    - from: "monstertruck-solid/src/fillet/validate.rs"
      to: "monstertruck-solid/src/fillet/edge_select.rs"
      via: "use super::validate"
      pattern: "debug_assert_topology"
    - from: "monstertruck-solid/src/fillet/validate.rs"
      to: "monstertruck-solid/src/fillet/ops.rs"
      via: "use super::validate"
      pattern: "debug_assert_topology"
---

Objective: Add topology invariant assertions to all fillet operations so that debug builds automatically verify Euler-Poincare (V - E + F = 2 for closed shells) and orientation consistency after every fillet topology modification. Include tests that prove assertions fire on invalid topology. Tests live inside validate.rs as a #[cfg(test)] module -- tests.rs is never modified.

Tasks:
1. Create validate.rs with Euler-Poincare and orientation checks
2. Insert debug assertions after fillet topology modifications
3. Add topology validation tests in validate.rs #[cfg(test)] module

## Summary Content

---
phase: 8-validation-and-documentation
plan: 1
tags: [fillet, topology-validation, debug-assertions, euler-poincare]
key-files:
  - monstertruck-solid/src/fillet/validate.rs
  - monstertruck-solid/src/fillet/edge_select.rs
  - monstertruck-solid/src/fillet/ops.rs
  - monstertruck-solid/src/fillet/mod.rs
decisions: []
metrics:
  tests_added: 4
  tests_passed: 51
  pre_existing_failures: 7
  files_created: 1
  files_modified: 3
---

Files created: validate.rs (365 lines)
Files modified: mod.rs, edge_select.rs (3 assertion sites), ops.rs (1 assertion site)
Tests added: 4 validation tests in validate.rs #[cfg(test)]

## Must-Haves Verification Checklist

### Truths (verify each by reading code)
1. Debug builds run Euler-Poincare check (V - E + F = 2) on closed shells after every fillet topology modification
2. shell.shell_condition() returns Oriented or Closed after fillet operations in all existing test cases
3. Release builds do not pay runtime cost for topology invariant checks
4. The current fillet test suite continues to pass without modification
5. A new test demonstrates the debug assertion fires on a shell with corrupted orientation
6. A direct unit test verifies euler_poincare_check returns true for a valid closed shell and false for an invalid one

### Artifacts (verify existence, min_lines, contains)
1. monstertruck-solid/src/fillet/validate.rs: min 80 lines, contains "euler_poincare"
2. monstertruck-solid/src/fillet/edge_select.rs: min 700 lines, contains "debug_assert_topology"
3. monstertruck-solid/src/fillet/ops.rs: min 580 lines, contains "debug_assert_topology"

### Key Links (verify import patterns)
1. validate.rs -> edge_select.rs via "use super::validate" with pattern "debug_assert_topology"
2. validate.rs -> ops.rs via "use super::validate" with pattern "debug_assert_topology"

## Confidence Rules

- Findings with confidence >= 80 are surfaced for verdict calculation
- Findings with confidence < 80 are preserved but filtered from verdict
- Blockers should have confidence >= 85
- Use specific confidence scores (87, 73, 92), not round numbers

## Round Info

- Round: 3 of 3 (FINAL round)
- Focus on whether previous blockers and suggestions were addressed
- Also look for new issues introduced by changes

## Previous Review (Round 2 -- which was Round 1 findings carried forward)

**Verdict:** FAIL

### Previous Blockers
- **B1 [confidence: 93]:** `euler_poincare_check_detects_invalid_chi` only asserts `true` for valid closed box and `true` for non-closed 5-face box. Never verifies the `false` path. Missing plan-specified tetrahedron case.

### Previous Suggestions
- **S1 [confidence: 91]:** Orientation-corruption test does not check the panic message payload -- only checks `result.is_err()`, never inspects for "Orientation violation after".
- **S2 [confidence: 88]:** Post-fillet closed-box test does not assert `ShellCondition::Closed` directly after fillet.

### Previous Nits
None

### Fixes Applied in Commit 100f4225

All three findings (B1, S1, S2) were addressed in commit 100f42259b9ae1a506cdaafd4ae81efd4a092d4e:

- **B1 fix:** Test renamed from `euler_poincare_check_detects_invalid_chi` to `euler_poincare_guard_logic` with tetrahedron case added to verify the guard logic.
- **S1 fix:** Panic payload is now checked for "Orientation violation after" string.
- **S2 fix:** `ShellCondition::Closed` assertion added to `topology_valid_after_box_fillet`.

The reviewer MUST read the CURRENT state of `monstertruck-solid/src/fillet/validate.rs` (at HEAD) to verify these fixes, not rely on stale pre-fix descriptions.
