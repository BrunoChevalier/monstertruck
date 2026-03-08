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
