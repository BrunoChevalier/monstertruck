---
phase: 7-integration-mode
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/fillet/params.rs
  - monstertruck-solid/src/fillet/mod.rs
  - monstertruck-solid/src/lib.rs
  - monstertruck-solid/src/fillet/ops.rs
  - monstertruck-solid/src/fillet/edge_select.rs
  - monstertruck-solid/src/fillet/tests.rs
autonomous: true
must_haves:
  truths:
    - "User constructs FilletOptions with mode field set to FilletMode::KeepSeparateFace and behavior is identical to current default"
    - "User constructs FilletOptions with mode field set to FilletMode::IntegrateVisual and the code compiles and runs"
    - "User sets extend_mode and corner_mode on FilletOptions and both values are stored and accessible"
    - "User calling fillet_edges without specifying mode gets KeepSeparateFace by default"
    - "User calling fillet_edges with None params gets default FilletOptions including KeepSeparateFace mode"
    - "Existing tests continue to pass unchanged"
  artifacts:
    - path: "monstertruck-solid/src/fillet/params.rs"
      provides: "FilletMode enum, ExtendMode enum, CornerMode enum, updated FilletOptions struct"
      min_lines: 100
      contains: "FilletMode"
    - path: "monstertruck-solid/src/fillet/mod.rs"
      provides: "Public re-exports for new types"
      min_lines: 20
      contains: "FilletMode"
    - path: "monstertruck-solid/src/lib.rs"
      provides: "Top-level re-exports for FilletMode, ExtendMode, CornerMode"
      min_lines: 30
      contains: "FilletMode"
  key_links:
    - from: "monstertruck-solid/src/fillet/params.rs"
      to: "monstertruck-solid/src/fillet/ops.rs"
      via: "FilletOptions.mode field read in fillet functions"
      pattern: "options.mode"
    - from: "monstertruck-solid/src/fillet/params.rs"
      to: "monstertruck-solid/src/lib.rs"
      via: "Re-export chain from params -> mod -> lib"
      pattern: "FilletMode"
---

<objective>
Extend FilletOptions with a FilletMode enum (KeepSeparateFace / IntegrateVisual), plus ExtendMode and CornerMode fields, so that the fillet pipeline accepts and threads these new options through all fillet operations without changing existing behavior.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-solid/src/fillet/params.rs
@monstertruck-solid/src/fillet/ops.rs
@monstertruck-solid/src/fillet/mod.rs
@monstertruck-solid/src/lib.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Define FilletMode, ExtendMode, and CornerMode enums and extend FilletOptions</name>
  <files>monstertruck-solid/src/fillet/params.rs</files>
  <action>
Add three new enums and update FilletOptions in `params.rs`:

1. **FilletMode enum**:
```rust
/// Controls how fillet faces relate to host faces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FilletMode {
    /// Fillet face is a separate topological face (current behavior).
    #[default]
    KeepSeparateFace,
    /// Fillet face is a separate face annotated with G1/G2 continuity
    /// constraints at shared edges, enabling seamless tessellation.
    IntegrateVisual,
}
```

2. **ExtendMode enum**:
```rust
/// Controls how fillet surfaces extend beyond edge endpoints.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ExtendMode {
    /// Extend fillet surfaces beyond endpoints when possible (current behavior).
    #[default]
    Auto,
    /// Never extend fillet surfaces beyond endpoints.
    NoExtend,
}
```

3. **CornerMode enum**:
```rust
/// Controls how fillet corners (where multiple fillets meet) are handled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CornerMode {
    /// Default corner handling.
    #[default]
    Auto,
    /// Trim corners to a sharp intersection.
    Trim,
    /// Blend corners with a smooth transition.
    Blend,
}
```

4. **Update FilletOptions** to include the three new fields:
```rust
pub struct FilletOptions {
    pub radius: RadiusSpec,
    pub divisions: NonZeroUsize,
    pub profile: FilletProfile,
    /// Fillet-to-host-face integration mode.
    pub mode: FilletMode,
    /// How fillet surfaces extend beyond edge endpoints.
    pub extend_mode: ExtendMode,
    /// How fillet corners are handled.
    pub corner_mode: CornerMode,
}
```

5. **Update Default impl** to use `FilletMode::KeepSeparateFace`, `ExtendMode::Auto`, `CornerMode::Auto`.

6. **Add builder methods**:
```rust
pub fn with_mode(mut self, mode: FilletMode) -> Self {
    self.mode = mode;
    self
}
pub fn with_extend_mode(mut self, extend_mode: ExtendMode) -> Self {
    self.extend_mode = extend_mode;
    self
}
pub fn with_corner_mode(mut self, corner_mode: CornerMode) -> Self {
    self.corner_mode = corner_mode;
    self
}
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid -E 'test(fillet::tests)' --no-run` to verify compilation without running tests yet (struct literal updates needed first).</verify>
  <done>FilletMode, ExtendMode, and CornerMode enums defined; FilletOptions extended with mode, extend_mode, corner_mode fields; builder methods added; Default impl includes new fields.</done>
</task>

<task type="auto">
  <name>Task 2: Update public re-exports and thread options through fillet operations</name>
  <files>monstertruck-solid/src/fillet/mod.rs, monstertruck-solid/src/lib.rs, monstertruck-solid/src/fillet/ops.rs, monstertruck-solid/src/fillet/edge_select.rs</files>
  <action>
1. **Update `fillet/mod.rs`** to re-export the new types:
   - Add `FilletMode`, `ExtendMode`, `CornerMode` to the `pub use params::...` line.

2. **Update `lib.rs`** to re-export the new types:
   - Add `FilletMode`, `ExtendMode`, `CornerMode` to the `pub use fillet::...` line.

3. **Thread `options.mode` through `ops.rs`**:
   In `fillet()`, `fillet_with_side()`, and `fillet_along_wire()`, the options are already passed through. No behavioral change is needed for `KeepSeparateFace` mode. For `IntegrateVisual`, the actual behavior will be implemented in Plan 2. For now, both modes produce the same output.

   In `fillet_along_wire()`, read `options.mode` and store it in a local variable for future use:
   ```rust
   let _mode = options.mode;
   ```
   This ensures the field is accessed and will generate a warning if removed, signaling Plan 2's integration point.

4. **Update `edge_select.rs`**:
   In `fillet_edges`, there are 3 places where `FilletOptions { ... }` is constructed manually for sub-chains (lines 560, 611, 663). Each needs the three new fields propagated from the parent options:
   ```rust
   chain_opts = FilletOptions {
       radius: RadiusSpec::Constant(radii[edge_id_to_idx[&original_eid]]),
       divisions: options.divisions,
       profile: options.profile.clone(),
       mode: options.mode,
       extend_mode: options.extend_mode,
       corner_mode: options.corner_mode,
   };
   ```
   Search for all `FilletOptions {` struct literals in `edge_select.rs` and add the three new fields to each.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid -E 'test(fillet)' --no-run` to verify compilation succeeds.</verify>
  <done>New types re-exported from mod.rs and lib.rs; options threaded through all fillet operation paths; all struct literal constructions updated.</done>
</task>

<task type="auto">
  <name>Task 3: Add unit tests for new types, default path, and backward compatibility</name>
  <files>monstertruck-solid/src/fillet/tests.rs</files>
  <action>
Add the following tests to `tests.rs`. Import the new types by updating the existing `use super::{...}` block to also include `FilletMode, ExtendMode, CornerMode` (either directly or through `super::params`).

1. **Test default mode is KeepSeparateFace**:
```rust
#[test]
fn default_fillet_mode_is_keep_separate() {
    let opts = FilletOptions::default();
    assert_eq!(opts.mode, super::params::FilletMode::KeepSeparateFace);
    assert_eq!(opts.extend_mode, super::params::ExtendMode::Auto);
    assert_eq!(opts.corner_mode, super::params::CornerMode::Auto);
}
```

2. **Test builder methods**:
```rust
#[test]
fn fillet_options_builder_methods() {
    use super::params::{FilletMode, ExtendMode, CornerMode};
    let opts = FilletOptions::constant(0.5)
        .with_mode(FilletMode::IntegrateVisual)
        .with_extend_mode(ExtendMode::NoExtend)
        .with_corner_mode(CornerMode::Blend);
    assert_eq!(opts.mode, FilletMode::IntegrateVisual);
    assert_eq!(opts.extend_mode, ExtendMode::NoExtend);
    assert_eq!(opts.corner_mode, CornerMode::Blend);
}
```

3. **Test fillet_edges with None params uses default path** (blocker S1):
```rust
#[test]
fn fillet_edges_none_params_uses_default() {
    let (mut shell, edge, _) = build_box_shell();
    let initial_face_count = shell.len();

    // Call fillet_edges with None to exercise the default FilletOptions path.
    // The default radius is 0.1, which is small enough for the box edges.
    fillet_edges(&mut shell, &[edge[5].id()], None).unwrap();

    // A fillet face should have been added.
    assert!(
        shell.len() > initial_face_count,
        "expected fillet face to be added with default options"
    );

    // Verify the shell can still be triangulated.
    let _poly = shell.robust_triangulation(0.001).to_polygon();
}
```

4. **Run the full existing test suite** to confirm backward compatibility. All existing fillet tests must pass without modification since the new fields default to the current behavior.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid -E 'test(fillet)'` to verify all tests pass including the new ones.</verify>
  <done>Unit tests added for FilletMode default, builder methods, and fillet_edges None-params default path; all existing tests pass.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-solid -E 'test(fillet)'` passes all tests (existing + new)
2. `FilletMode`, `ExtendMode`, `CornerMode` are publicly accessible from `monstertruck_solid::FilletMode` etc.
3. `FilletOptions::default().mode` is `FilletMode::KeepSeparateFace`
4. All struct literal constructions of `FilletOptions` in `edge_select.rs` include the three new fields
5. `fillet_edges(&mut shell, &[eid], None)` exercises the default path and succeeds
</verification>

<success_criteria>
- FilletOptions accepts mode, extend_mode, and corner_mode fields (INTEG-01, INTEG-02)
- KeepSeparateFace is the default mode and produces identical output to pre-change behavior
- IntegrateVisual mode is accepted and compiles (behavioral implementation in Plan 2)
- fillet_edges with None params works correctly using default FilletOptions
- All existing fillet tests pass unchanged
</success_criteria>

<output>
After completion, create `.tendrion/phases/7-integration-mode/7-1-SUMMARY.md`
</output>
