---
phase: 16-tolerance-foundation-and-api-safety
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-geometry/src/nurbs/surface_options.rs
  - monstertruck-geometry/tests/surface_types_test.rs
  - monstertruck-geometry/tests/try_surface_constructors_test.rs
  - monstertruck-geometry/tests/try_gordon_skin_test.rs
  - monstertruck-modeling/src/builder.rs
autonomous: true
must_haves:
  truths:
    - "User cannot construct GordonOptions with struct literal from outside monstertruck-geometry crate"
    - "User cannot construct SkinOptions with struct literal from outside monstertruck-geometry crate"
    - "User cannot construct SweepRailOptions with struct literal from outside monstertruck-geometry crate"
    - "User cannot construct Birail1Options with struct literal from outside monstertruck-geometry crate"
    - "User cannot construct Birail2Options with struct literal from outside monstertruck-geometry crate"
    - "User constructs all option structs via Default::default() and field setters, and all existing tests pass"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/surface_options.rs"
      provides: "All five option structs with #[non_exhaustive] attribute"
      min_lines: 60
      contains: "#[non_exhaustive]"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/surface_options.rs"
      to: "monstertruck-modeling/src/builder.rs"
      via: "Option struct imports must still compile with non_exhaustive"
      pattern: "SweepRailOptions"
    - from: "monstertruck-geometry/src/nurbs/surface_options.rs"
      to: "monstertruck-geometry/tests/try_surface_constructors_test.rs"
      via: "Test struct literal construction updated to use Default + field override"
      pattern: "Default::default"
---

<objective>
Add #[non_exhaustive] to all five surface constructor option structs (GordonOptions, SkinOptions, SweepRailOptions, Birail1Options, Birail2Options) and update all downstream struct literal construction sites to use Default::default() with field overrides via the `..` syntax.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/surface_options.rs
@monstertruck-geometry/tests/try_surface_constructors_test.rs
@monstertruck-modeling/src/builder.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Add #[non_exhaustive] to option structs</name>
  <files>monstertruck-geometry/src/nurbs/surface_options.rs</files>
  <action>
Add `#[non_exhaustive]` attribute to all five option structs. The attribute goes directly above each `pub struct` declaration, after any doc comments and other derive attributes.

**SweepRailOptions** (line 12-13):
```rust
/// Options for single-rail sweep surface construction.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SweepRailOptions {
```

**Birail1Options** (line 29-31):
```rust
/// Options for single-profile birail surface construction.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Birail1Options {
```

**Birail2Options** (line 42-44):
```rust
/// Options for dual-profile birail surface construction.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Birail2Options {
```

**GordonOptions** (line 55-60):
```rust
/// Options for Gordon surface construction.
///
/// Currently a marker struct. Future versions may add tolerance fields
/// for controlling intersection point matching.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct GordonOptions {}
```

**SkinOptions** (line 62-67):
```rust
/// Options for skin surface construction.
///
/// Currently a marker struct. Future versions may add fields to control
/// v-direction degree or parameterization strategy.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct SkinOptions {}
```

Important: `#[non_exhaustive]` prevents struct literal construction from *outside* the defining crate. Within monstertruck-geometry itself, struct literals still work. So the try_* methods in bspline_surface.rs (same crate) need no changes. But tests in monstertruck-geometry are also in the same crate, so they also need no changes for compilation. However, monstertruck-modeling is a *different* crate, so any struct literal construction there will break.

Check the FrameRule enum as well -- it does NOT need `#[non_exhaustive]` per the requirements (only the five option structs are listed).
  </action>
  <verify>Run `cargo check -p monstertruck-geometry` to confirm the geometry crate still compiles (same-crate struct literals are fine).</verify>
  <done>#[non_exhaustive] attribute added to all five surface constructor option structs.</done>
</task>

<task type="auto">
  <name>Task 2: Fix downstream crate struct literal construction</name>
  <files>monstertruck-modeling/src/builder.rs</files>
  <action>
Since monstertruck-modeling is a *separate* crate from monstertruck-geometry, any direct struct literal construction of the option structs will fail to compile after adding `#[non_exhaustive]`.

Search monstertruck-modeling/src/builder.rs for direct struct literal construction patterns. Based on the codebase analysis, these are in the test module (around lines 1374-1488). Update each occurrence:

1. Lines like `let opts = SweepRailOptions::default();` -- these are FINE, `Default::default()` works with `#[non_exhaustive]`.

2. Lines like:
```rust
let opts = SweepRailOptions {
    n_sections: 3,
    ..SweepRailOptions::default()
};
```
These will FAIL because struct literal syntax is forbidden for non_exhaustive structs from external crates. Replace with:
```rust
let mut opts = SweepRailOptions::default();
opts.n_sections = 3;
```

3. Also check any usage like `Birail1Options { n_sections: 3 }` and replace similarly.

Scan the entire builder.rs for any struct literal construction of these five types. The doc examples (lines ~473-483) may also contain struct literal patterns that need updating.

Also check monstertruck-modeling/src/lib.rs for re-exports -- those are fine (re-exporting the types, not constructing them).

Search monstertruck-wasm and other downstream crates for struct literal construction of these types and fix any occurrences.
  </action>
  <verify>Run `cargo check -p monstertruck-modeling` to confirm compilation with the non_exhaustive change. Run `cargo test -p monstertruck-modeling` to verify all builder tests pass.</verify>
  <done>All downstream struct literal constructions updated to use Default + field mutation pattern.</done>
</task>

<task type="auto">
  <name>Task 3: Verify cross-crate protection and update doc examples</name>
  <files>monstertruck-geometry/src/nurbs/surface_options.rs, monstertruck-modeling/src/builder.rs</files>
  <action>
1. Verify that doc examples in the option structs themselves (surface_options.rs) show the correct construction pattern. Since GordonOptions and SkinOptions are marker structs (no fields), their `Default::default()` is the only way to construct them. Add a brief doc example to each struct showing construction:

For SweepRailOptions, update or add a doc example:
```rust
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::SweepRailOptions;
/// let mut opts = SweepRailOptions::default();
/// opts.n_sections = 5;
/// ```
```

For GordonOptions and SkinOptions (no fields to set):
```rust
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::GordonOptions;
/// let opts = GordonOptions::default();
/// ```
```

Similar for Birail1Options and Birail2Options.

2. Update any doc examples in monstertruck-modeling/src/builder.rs (the try_sweep_rail_with_options etc. functions) that use struct literal construction to use the `Default + field mutation` pattern instead.
  </action>
  <verify>Run `cargo test --doc -p monstertruck-geometry` and `cargo test --doc -p monstertruck-modeling` to verify all doc examples compile and pass.</verify>
  <done>Doc examples updated to demonstrate non_exhaustive-safe construction patterns.</done>
</task>

</tasks>

<verification>
1. `cargo check -p monstertruck-geometry` passes
2. `cargo check -p monstertruck-modeling` passes
3. `cargo check` (full workspace) passes
4. `cargo test -p monstertruck-geometry` passes (all surface constructor tests)
5. `cargo test -p monstertruck-modeling` passes (builder tests)
6. All five structs have `#[non_exhaustive]` attribute in surface_options.rs
7. No struct literal construction of these types exists outside monstertruck-geometry
8. Doc examples compile and demonstrate the correct construction pattern
</verification>

<success_criteria>
- TOLAPI-02 is fully satisfied: all five option structs have #[non_exhaustive]
- All downstream code compiles and tests pass
- Future field additions to these structs will not cause breaking changes for external consumers
</success_criteria>

<output>
After completion, create `16-tolerance-foundation-and-api-safety/16-2-SUMMARY.md`
</output>
