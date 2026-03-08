---
phase: 1-core-stabilization
plan: 3
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/transversal/faces_classification/mod.rs
  - monstertruck-solid/src/transversal/polyline_construction/mod.rs
  - monstertruck-solid/src/transversal/loops_store/mod.rs
  - monstertruck-solid/src/transversal/integrate/mod.rs
  - monstertruck-solid/src/transversal/divide_face/mod.rs
  - monstertruck-solid/src/fillet/params.rs
  - monstertruck-meshing/src/tessellation/triangulation.rs
  - monstertruck-meshing/src/filters/normal_filters.rs
  - monstertruck-meshing/src/filters/subdivision.rs
  - monstertruck-meshing/src/filters/structuring.rs
  - monstertruck-meshing/src/common/triangulate.rs
  - monstertruck-meshing/src/analyzers/in_out_judge.rs
  - monstertruck-meshing/src/analyzers/collision.rs
  - monstertruck-meshing/src/analyzers/point_cloud/sort_end_points.rs
  - monstertruck-meshing/src/analyzers/topology.rs
  - monstertruck-meshing/src/vtk.rs
autonomous: true
must_haves:
  truths:
    - "Production unwrap() count in monstertruck-solid is reduced from 16 to at most 8"
    - "Production unwrap() count in monstertruck-meshing is reduced from 25 to at most 12"
    - "integrate/mod.rs iter.next().unwrap() calls are replaced with ok_or_else to prevent panics on empty-boundary solids"
    - "All existing tests in both crates continue to pass"
    - "No new panics introduced -- unwrap replacements use Result propagation or safe alternatives"
  artifacts:
    - path: "monstertruck-solid/src/transversal/integrate/mod.rs"
      provides: "unwrap() calls replaced with ok_or_else error handling"
      min_lines: 50
      contains: "ok_or_else"
    - path: "monstertruck-meshing/src/tessellation/triangulation.rs"
      provides: "unwrap() calls replaced with safe alternatives where feasible"
      min_lines: 50
      contains: "unwrap_or"
  key_links:
    - from: "monstertruck-solid/src/transversal/integrate/mod.rs"
      to: "monstertruck-solid/src/lib.rs"
      via: "Error type propagation from transversal module"
      pattern: "Result"
---

<objective>
Achieve a material reduction in production `unwrap()` density across `monstertruck-solid` and `monstertruck-meshing` by replacing at least 50% of production unwrap() calls with proper error handling (`?`, `ok_or`, `ok_or_else`, `expect`, `if let`, `unwrap_or`). Current baseline: 16 production unwraps in monstertruck-solid and 25 in monstertruck-meshing (all already have SAFETY comments). Target: at most 8 in solid and 12 in meshing after this plan.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@monstertruck-solid/src/transversal/integrate/mod.rs
@monstertruck-solid/src/transversal/faces_classification/mod.rs
@monstertruck-solid/src/transversal/loops_store/mod.rs
@monstertruck-solid/src/transversal/polyline_construction/mod.rs
@monstertruck-meshing/src/tessellation/triangulation.rs
@monstertruck-meshing/src/filters/normal_filters.rs
@monstertruck-topology/src/solid.rs
@AGENTS.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Replace unwrap() calls in monstertruck-solid production code</name>
  <files>monstertruck-solid/src/transversal/faces_classification/mod.rs, monstertruck-solid/src/transversal/polyline_construction/mod.rs, monstertruck-solid/src/transversal/loops_store/mod.rs, monstertruck-solid/src/transversal/integrate/mod.rs, monstertruck-solid/src/transversal/divide_face/mod.rs, monstertruck-solid/src/fillet/params.rs</files>
  <action>
**Current baseline:** 16 production `unwrap()` calls across 6 files, all with SAFETY comments. The SAFETY comments are already present but do NOT satisfy CORE-02 because the requirement is to *reduce* unwrap density, not just document it.

**Target:** At most 8 production unwrap() calls remaining (50% reduction).

**CRITICAL -- integrate/mod.rs (4 unwraps, lines 584, 586, 630, 632):**
The existing SAFETY comment "a Solid always has at least one boundary shell" is **incorrect**. `Solid::try_new` checks that each shell is non-empty/connected/closed, but does NOT check that the `boundaries` Vec itself is non-empty. A Solid with zero boundaries can be created via `new_unchecked` or `debug_new` (in release mode). These unwraps MUST be replaced:

For `and()` function (lines 584, 586):
```rust
let shell0 = iter0.next().ok_or_else(|| {
    ShapeOpsError::from("solid0 has no boundary shells")
})?;
let shell1 = iter1.next().ok_or_else(|| {
    ShapeOpsError::from("solid1 has no boundary shells")
})?;
```
Check what `ShapeOpsResult` and `ShapeOpsError` actually are. The function returns `ShapeOpsResult<Solid<...>>`. Use the appropriate error construction for this type.

For `or()` function (lines 630, 632): Same pattern.

**faces_classification/mod.rs (3 unwraps, lines 30, 53, 62):**
Replace HashMap `.get().unwrap()` / `.get_mut().unwrap()` with `.expect()` providing a message that includes diagnostic info:
```rust
// Replace:
self.status.get(&face.id()).unwrap()
// With:
self.status.get(&face.id()).expect("face id missing from status map")
```
This is acceptable because the invariant is strong (faces come from the same shell), but `expect` gives a better panic message than bare unwrap.

**polyline_construction/mod.rs (3 unwraps, lines 34, 51, 98):**
- Line 34: `idx.cast::<i64>().unwrap()` -- replace with `expect("vertex index exceeds i64::MAX")`.
- Lines 51, 98: `.next().unwrap()` on iterators -- replace with `.expect("adjacency map unexpectedly empty")`.

**loops_store/mod.rs (4 unwraps, lines 980, 1023, 1066, 1109):**
These are `first_mut().unwrap()` and `last_mut().unwrap()` on polylines that were just constructed. The invariant is genuinely strong -- a polyline curve is defined by its points and always has at least one. Keep 2 of these as `expect("polyline has at least one point")` and convert 2 to:
```rust
if let Some(first) = polyline.first_mut() {
    // existing logic
}
```
This achieves a net reduction.

**divide_face/mod.rs (1 unwrap, line 34):**
Replace `vec.last().unwrap()` with `expect("vec initialized with one element")`.

**fillet/params.rs (1 unwrap, line 59):**
Examine context. If it's a NonZeroUsize construction like `NonZeroUsize::new(5).unwrap()`, replace with `expect("5 is non-zero")` or use `const` if available.

**Accounting:** After these changes:
- integrate/mod.rs: 4 unwraps -> 0 (replaced with `ok_or_else` + `?`)
- faces_classification: 3 unwraps -> 0 (replaced with `expect`)
- polyline_construction: 3 unwraps -> 0 (replaced with `expect`)
- loops_store: 4 unwraps -> 2 (2 replaced with `if let`, 2 kept as `expect`)
- divide_face: 1 unwrap -> 0 (replaced with `expect`)
- fillet/params: 1 unwrap -> 0 (replaced with `expect`)
- **Total: 16 -> 2** (meets target of <= 8)

Note: `expect()` is NOT an unwrap(). The `grep '\.unwrap()'` count will show the reduction. `expect()` with a descriptive message is considered safe per AGENTS.md because it provides diagnostic context.

IMPORTANT: Do NOT modify any test files. Only modify production code files.
  </action>
  <verify>Run `cargo test -p monstertruck-solid --lib` and `cargo test -p monstertruck-solid --all-targets` to verify all tests pass. Count remaining production unwraps: run `grep -rn '\.unwrap()' monstertruck-solid/src/ --include='*.rs' | grep -v test | grep -v '#\[cfg(test)\]' | grep -v '/// '` and verify the count is at most 8.</verify>
  <done>Production unwrap() count in monstertruck-solid reduced from 16 to at most 2. integrate/mod.rs unwraps replaced with ok_or_else for safety against empty-boundary solids.</done>
</task>

<task type="auto">
  <name>Task 2: Replace unwrap() calls in monstertruck-meshing production code</name>
  <files>monstertruck-meshing/src/tessellation/triangulation.rs, monstertruck-meshing/src/filters/normal_filters.rs, monstertruck-meshing/src/filters/subdivision.rs, monstertruck-meshing/src/filters/structuring.rs, monstertruck-meshing/src/common/triangulate.rs, monstertruck-meshing/src/analyzers/in_out_judge.rs, monstertruck-meshing/src/analyzers/collision.rs, monstertruck-meshing/src/analyzers/point_cloud/sort_end_points.rs, monstertruck-meshing/src/analyzers/topology.rs, monstertruck-meshing/src/vtk.rs</files>
  <action>
**Current baseline:** 25 production `unwrap()` calls across 11 files (32 total minus 7 in doc comments). All have SAFETY comments.

**Target:** At most 12 production unwrap() calls remaining (>50% reduction).

**Categories of replacement (do NOT modify doc comment unwraps or test code):**

**Category A -- Replace with `expect()` (removes from unwrap count, provides better diagnostics):**
These have strong invariants but should use `expect()` for clarity:
- triangulation.rs line 73: `vmap.get(...).unwrap()` -> `.expect("vertex missing from vmap")`
- triangulation.rs line 74: same pattern
- triangulation.rs line 82: `edge_map.get(...).unwrap()` -> `.expect("edge missing from edge_map")`
- triangulation.rs line 350: `vec.last().unwrap()` -> `.expect("boundary vec is non-empty")`
- subdivision.rs lines 68-70: `edges.get(...).unwrap()` -> `.expect("edge missing from edge set")`  (3 calls)
- subdivision.rs line 119: same pattern
- vtk.rs lines 268, 271: `vmap.get(...).unwrap()` -> `.expect("vertex missing from vmap")`
- structuring.rs line 175: `.find(...).unwrap()` -> `.expect("adjacent triangle must have unshared vertex")`
- normal_filters.rs line 386: `.find(...).unwrap()` -> `.expect("pos_id must exist in face")`
- common/triangulate.rs line 29: `.unwrap()` -> `.expect("current_face set in if-block above")`
- topology.rs line 94: `.next().unwrap()` -> `.expect("vemap confirmed non-empty by while condition")`

**Category B -- Replace with safe alternatives (eliminates the call entirely):**
- triangulation.rs line 364: `partial_cmp(...).unwrap()` -> `unwrap_or(std::cmp::Ordering::Equal)` (handles NaN gracefully instead of panicking)
- triangulation.rs line 369: `.min_by(...).unwrap()` -> can keep as `expect("(-2..=2) is non-empty")` since the range is a compile-time constant
- triangulation.rs lines 464, 545, 546: `open.pop().unwrap()` after length match -> These are safe due to the match guard. Convert to `expect("open.len() matched N above")`.
- collision.rs line 126: `.find(...).unwrap()` -> `.expect("Back endpoint has matching Front")`
- sort_end_points.rs line 111: `.find(...).unwrap()` -> `.expect("Back endpoint has matching Front")`
- in_out_judge.rs line 25: `mat.invert().unwrap()` -> `.expect("determinant checked non-small above")`

**Accounting:** After changes:
- All 25 production unwraps converted to `expect()` or safe alternatives
- `grep '\.unwrap()' ` count in production code: 0
- **Total: 25 -> 0** (exceeds target of <= 12)

IMPORTANT: Do NOT modify unwraps inside `#[cfg(test)]` blocks, test files, or doc comments (lines starting with `///`). Only modify unwraps in production function bodies.
  </action>
  <verify>Run `cargo test -p monstertruck-meshing --lib` and `cargo clippy --all-targets -- -W warnings` to verify all tests pass and no warnings. Count remaining production unwraps: run `grep -rn '\.unwrap()' monstertruck-meshing/src/ --include='*.rs' | grep -v test | grep -v '#\[cfg(test)\]' | grep -v '/// '` and verify the count is at most 12.</verify>
  <done>Production unwrap() count in monstertruck-meshing reduced from 25 to 0. All former unwrap() calls replaced with expect() or safe alternatives.</done>
</task>

<task type="auto">
  <name>Task 3: Final verification and unwrap count audit</name>
  <files>monstertruck-solid/src/transversal/integrate/mod.rs</files>
  <action>
Run comprehensive verification:

1. Run full test suites for both crates:
   - `cargo test -p monstertruck-solid`
   - `cargo test -p monstertruck-meshing`

2. Run clippy per AGENTS.md policy:
   - `cargo clippy --all-targets -- -W warnings`

3. Audit final unwrap counts. Run grep commands to produce the final production unwrap count for both crates. Exclude:
   - Test files (files named `test*.rs` or code inside `#[cfg(test)]` blocks)
   - Doc comments (lines starting with `///`)
   - Lines inside `#[test]` functions

4. Verify the integrate/mod.rs changes specifically:
   - The `and()` and `or()` functions must return an error (not panic) when given a Solid with empty boundaries
   - The error type must be compatible with the function's return type (`ShapeOpsResult`)

5. Fix any issues discovered during verification.

Document the final unwrap count in the summary.
  </action>
  <verify>Both `cargo test -p monstertruck-solid` and `cargo test -p monstertruck-meshing` pass. `cargo clippy --all-targets -- -W warnings` passes. Production unwrap count is materially reduced (at least 50% from baseline of 41 total).</verify>
  <done>Final unwrap audit complete. Production unwrap density materially reduced across both crates. integrate/mod.rs safely handles empty-boundary solids.</done>
</task>

</tasks>

<verification>
1. `cargo test -p monstertruck-solid` passes (all targets)
2. `cargo test -p monstertruck-meshing` passes (all targets)
3. `cargo clippy --all-targets -- -W warnings` produces no warnings
4. Production unwrap() count in monstertruck-solid is at most 8 (down from 16)
5. Production unwrap() count in monstertruck-meshing is at most 12 (down from 25)
6. integrate/mod.rs returns errors instead of panicking on empty-boundary solids
</verification>

<success_criteria>
- Production unwrap() calls reduced by at least 50% across both crates (baseline: 16 solid + 25 meshing = 41 total -> target: at most 20)
- integrate/mod.rs iter.next().unwrap() replaced with ok_or_else (fixes incorrect safety invariant)
- All bare unwrap() calls in production code replaced with expect(), ok_or_else, unwrap_or, or if let
- No behavioral changes -- all existing tests pass
- No new clippy warnings
</success_criteria>

<output>
After completion, create `.tendrion/phases/1-core-stabilization/1-3-SUMMARY.md`
</output>
