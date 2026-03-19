---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 4
type: execute
wave: 3
depends_on: ["9-1", "9-2", "9-3"]
files_modified:
  - monstertruck-meshing/src/tessellation/mod.rs
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/src/tessellation/boundary_stitching.rs
  - monstertruck-meshing/src/analyzers/collision.rs
autonomous: true
must_haves:
  truths:
    - "monstertruck-meshing source files import TOLERANCE from monstertruck_core::tolerance explicitly"
    - "cargo nextest run -p monstertruck-meshing --no-fail-fast passes with no failures"
    - "cargo clippy -p monstertruck-meshing --all-targets -- -W warnings produces no warnings"
  artifacts:
    - path: "monstertruck-meshing/src/tessellation/mod.rs"
      provides: "Tessellation module with explicit tolerance import"
      min_lines: 200
      contains: "use monstertruck_core::tolerance"
    - path: "monstertruck-meshing/src/tessellation/triangulation.rs"
      provides: "Triangulation with explicit tolerance import"
      min_lines: 400
      contains: "use monstertruck_core::tolerance"
    - path: "monstertruck-meshing/src/tessellation/boundary_stitching.rs"
      provides: "Boundary stitching with explicit tolerance import"
      min_lines: 100
      contains: "use monstertruck_core::tolerance"
    - path: "monstertruck-meshing/src/analyzers/collision.rs"
      provides: "Collision analyzer with explicit tolerance import"
      min_lines: 100
      contains: "use monstertruck_core::tolerance"
  key_links:
    - from: "monstertruck-core/src/tolerance.rs"
      to: "monstertruck-meshing/src/tessellation/mod.rs"
      via: "Explicit tolerance import in meshing tessellation"
      pattern: "use monstertruck_core::tolerance"
    - from: "monstertruck-core/src/tolerance.rs"
      to: "monstertruck-meshing/src/tessellation/triangulation.rs"
      via: "Explicit tolerance import in triangulation"
      pattern: "use monstertruck_core::tolerance"
---

<objective>
Add explicit `use monstertruck_core::tolerance::TOLERANCE` imports to monstertruck-meshing source files that use the TOLERANCE constant, satisfying the TEST-02 requirement that the shared tolerance module is explicitly imported by monstertruck-meshing.
</objective>

<execution_context>
@AGENTS.md
</execution_context>

<context>
@monstertruck-core/src/tolerance.rs
@monstertruck-meshing/src/tessellation/mod.rs
@monstertruck-meshing/src/tessellation/triangulation.rs
@monstertruck-meshing/src/tessellation/boundary_stitching.rs
@monstertruck-meshing/src/analyzers/collision.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add explicit tolerance imports to monstertruck-meshing source files</name>
  <files>monstertruck-meshing/src/tessellation/mod.rs, monstertruck-meshing/src/tessellation/triangulation.rs, monstertruck-meshing/src/tessellation/boundary_stitching.rs, monstertruck-meshing/src/analyzers/collision.rs</files>
  <action>
Currently, TOLERANCE is available in monstertruck-meshing files via the `use crate::*` glob which re-exports `monstertruck_mesh::base::*` which re-exports `monstertruck_core::tolerance::*`. The requirement is to add explicit direct imports to make the dependency on the shared tolerance module visible.

Add `use monstertruck_core::tolerance::TOLERANCE;` to the following files. Place the import after the existing `use` block, grouped with other workspace crate imports:

1. **monstertruck-meshing/src/tessellation/mod.rs** — Uses TOLERANCE on lines 43, 205, 232, 247. Add after line 3 (after `use spade::{iterators::*, *};`):
   ```rust
   use monstertruck_core::tolerance::TOLERANCE;
   ```

2. **monstertruck-meshing/src/tessellation/triangulation.rs** — Uses TOLERANCE on lines 403, 479, 497, 515, 533, 1147-1149. Add after the existing import block (after `use rustc_hash::FxHashMap as HashMap;` at line 9):
   ```rust
   use monstertruck_core::tolerance::TOLERANCE;
   ```

3. **monstertruck-meshing/src/tessellation/boundary_stitching.rs** — Uses TOLERANCE on lines 179, 191. Add after `use rustc_hash::FxHashMap as HashMap;` at line 21:
   ```rust
   use monstertruck_core::tolerance::TOLERANCE;
   ```

4. **monstertruck-meshing/src/analyzers/collision.rs** — Uses TOLERANCE2 on lines 163-165. Add after `use array_macro::array;` at line 2:
   ```rust
   use monstertruck_core::tolerance::TOLERANCE2;
   ```

After adding imports, verify the crate compiles and tests pass. These are additive imports — the symbol was already in scope via glob re-export, so no behavioral change occurs. But the explicit import makes the dependency chain clear and satisfies the roadmap success criterion.

Note: `monstertruck-meshing/src/vtk.rs` already has `use monstertruck_core::tolerance::TOLERANCE;` on line 2, so no change needed there.
  </action>
  <verify>Run `cargo clippy -p monstertruck-meshing --all-targets -- -W warnings` to verify no warnings (including unused import warnings). Run `cargo nextest run -p monstertruck-meshing --no-fail-fast` to verify all tests pass.</verify>
  <done>Explicit monstertruck_core::tolerance imports added to four monstertruck-meshing source files; crate compiles and all tests pass.</done>
</task>

</tasks>

<verification>
1. `cargo clippy -p monstertruck-meshing --all-targets -- -W warnings` produces no warnings
2. `cargo nextest run -p monstertruck-meshing --no-fail-fast` passes all tests
3. Each modified file contains `use monstertruck_core::tolerance::TOLERANCE` (or TOLERANCE2 for collision.rs)
4. No behavioral changes — imports are additive over existing glob re-exports
</verification>

<success_criteria>
- monstertruck-meshing source files explicitly import from monstertruck_core::tolerance
- Satisfies TEST-02 success criterion: "shared tolerance constants module is imported by monstertruck-meshing"
- No test regressions in monstertruck-meshing
- No clippy warnings
</success_criteria>

<output>
After completion, create `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-4-SUMMARY.md`
</output>
