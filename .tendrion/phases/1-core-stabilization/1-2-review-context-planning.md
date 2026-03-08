# Review Context: Plan 1-2 (Planning Review)

**Round:** 1 of 2
**Plan ID:** 1-2
**Phase:** 1 - Core Stabilization

---

## Plan Under Review

---
phase: 1-core-stabilization
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-derive/Cargo.toml
  - monstertruck-derive/src/lib.rs
  - Cargo.toml
autonomous: true
must_haves:
  truths:
    - "monstertruck-derive compiles without depending on the deprecated proc-macro-error crate"
    - "All 21 derive macros using #[proc_macro_error] continue to work correctly"
    - "cargo test -p monstertruck-derive passes"
    - "cargo test --workspace passes (no downstream breakage)"
  artifacts:
    - path: "monstertruck-derive/Cargo.toml"
      provides: "Updated dependency from proc-macro-error to proc-macro-error2 or removed entirely"
      min_lines: 10
      contains: "proc-macro"
    - path: "monstertruck-derive/src/lib.rs"
      provides: "All proc_macro_error attributes updated to work with replacement"
      min_lines: 100
      contains: "proc_macro"
  key_links:
    - from: "monstertruck-derive/Cargo.toml"
      to: "Cargo.toml"
      via: "workspace dependency declaration"
      pattern: "proc-macro"
---

<objective>
Replace the deprecated `proc-macro-error` dependency in `monstertruck-derive` with `proc-macro-error2` (a maintained fork with the same API) so the crate no longer depends on abandoned upstream code.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@monstertruck-derive/Cargo.toml
@monstertruck-derive/src/lib.rs
@Cargo.toml
@AGENTS.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Replace proc-macro-error with proc-macro-error2</name>
  <files>Cargo.toml, monstertruck-derive/Cargo.toml, monstertruck-derive/src/lib.rs</files>
  <action>
`proc-macro-error2` is a maintained fork of `proc-macro-error` with an identical API surface. The migration is a dependency swap with a crate rename.

**Step 1: Update workspace Cargo.toml (root):**
On line 49, replace:
```toml
proc-macro-error = "1"
```
with:
```toml
proc-macro-error2 = "2"
```

**Step 2: Update monstertruck-derive/Cargo.toml:**
On line 15, replace:
```toml
proc-macro-error = { workspace = true }
```
with:
```toml
proc-macro-error2 = { workspace = true }
```

**Step 3: Update monstertruck-derive/src/lib.rs:**
Replace the import on line 16:
```rust
use proc_macro_error::proc_macro_error;
```
with:
```rust
use proc_macro_error2::proc_macro_error;
```

The `#[proc_macro_error]` attribute macro name stays the same -- only the import path changes. All 21 usages of `#[proc_macro_error]` throughout lib.rs will continue to work because the attribute name is identical in the proc-macro-error2 crate.

Note: The crate does NOT use `abort!`, `abort_call_site!`, `emit_error!`, or `emit_warning!` macros from proc-macro-error (confirmed by grep). Only the `#[proc_macro_error]` attribute is used, making this a clean swap.
  </action>
  <verify>Run `cargo test -p monstertruck-derive` to verify the derive crate compiles and its tests pass. Then run `cargo test -p monstertruck-modeling --lib` to verify downstream crates that use the derive macros still work. Run `cargo clippy -p monstertruck-derive -- -W warnings` for lint cleanliness.</verify>
  <done>Deprecated proc-macro-error replaced with proc-macro-error2. All derive macros compile and function correctly.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-derive` passes
2. `cargo test -p monstertruck-modeling --lib` passes (uses derived traits)
3. `cargo clippy -p monstertruck-derive -- -W warnings` produces no warnings
4. `grep "proc-macro-error\"" Cargo.toml` returns no matches (old dep removed)
5. `grep "proc-macro-error2" Cargo.toml` returns a match (new dep present)
</verification>

<success_criteria>
- The deprecated proc-macro-error crate is no longer a dependency
- proc-macro-error2 is the replacement with identical functionality
- All 21 derive macros continue to work with no behavioral changes
- No downstream compilation or test failures
</success_criteria>

<output>
After completion, create `.tendrion/phases/1-core-stabilization/1-2-SUMMARY.md`
</output>

---

## Sibling Plans

| Plan ID | Wave | Objective |
|---------|------|-----------|
| 1-1 | 1 | Replace all 9 unimplemented!() arms for Curve::IntersectionCurve in geometry.rs |
| 1-3 | 1 | Audit and reduce unwrap() density in non-test code across monstertruck-solid and monstertruck-meshing |
| 1-4 | 2 | Add criterion-based benchmarking infrastructure to three core crates |

Full sibling plans can be read from .tendrion/phases/1-core-stabilization/{sibling_plan_id}-PLAN.md if cross-plan analysis is needed.

---

## Roadmap (Phase 1: Core Stabilization)

**Goal**: Boolean-to-modeling pipeline no longer panics, unwrap density is materially reduced, deprecated dependencies are replaced, and performance regressions are detectable
**Depends on**: None
**Requirements**: CORE-01, CORE-02, CORE-03, CORE-04
**Success Criteria** (what must be TRUE):
  1. All 9 IntersectionCurve unimplemented!() arms in monstertruck-modeling/src/geometry.rs are replaced with working implementations and exercised by tests
  2. unwrap() calls in monstertruck-solid and monstertruck-meshing are reduced by at least 50%, with remaining calls annotated with SAFETY comments
  3. monstertruck-derive compiles without proc-macro-error; the deprecated crate is removed from the dependency tree
  4. Running `cargo bench` executes criterion or divan benchmarks covering NURBS evaluation, tessellation, and boolean operations

---

## Repository Validation Snapshot (For This Review)

- `Cargo.toml:49` currently declares `proc-macro-error = "1"`.
- `monstertruck-derive/Cargo.toml:15` currently declares `proc-macro-error = { workspace = true }`.
- `monstertruck-derive/src/lib.rs:16` currently imports `use proc_macro_error::proc_macro_error;`.
- `monstertruck-derive/src/lib.rs` has 20 `#[proc_macro_error]` attributes and 20 `#[proc_macro_derive(...)]` macros.
- Repository-wide non-lockfile references to `proc-macro-error` are limited to root `Cargo.toml`, `monstertruck-derive/Cargo.toml`, and `monstertruck-derive/src/lib.rs`.
- `Cargo.lock` currently still contains transitive `proc-macro-error` from external crates (for example `ruststep-derive` and `structopt-derive`), so direct-dependency migration in `monstertruck-derive` will not necessarily remove the crate from the full dependency graph.
