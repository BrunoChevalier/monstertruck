---
phase: 14-profile-solid-pipeline
plan: 2
type: tdd
wave: 2
depends_on: ["14-1"]
files_modified:
  - monstertruck-modeling/src/profile.rs
  - monstertruck-modeling/src/text.rs
  - monstertruck-modeling/tests/font_pipeline.rs
autonomous: true
must_haves:
  truths:
    - "User combines font glyph wires with a custom rectangular sketch loop into a single face, and the face has correct boundary count (glyph contours + custom loop)"
    - "User creates a mixed profile face where a glyph hole is inside a custom outer loop, and winding normalization handles it correctly"
    - "User creates a face with a custom outer loop containing multiple glyph-sourced holes, and all holes are detected and oriented properly"
    - "User calls merge_profiles to combine two Vec<Wire> sources and gets a single flat Vec<Wire> suitable for attach_plane_normalized"
  artifacts:
    - path: "monstertruck-modeling/src/profile.rs"
      provides: "merge_profiles function for combining wire sets from different sources"
      min_lines: 300
      contains: "merge_profiles"
    - path: "monstertruck-modeling/tests/font_pipeline.rs"
      provides: "Integration tests for mixed glyph + custom profile combinations"
      min_lines: 290
      contains: "mixed_glyph_custom"
  key_links:
    - from: "monstertruck-modeling/src/profile.rs"
      to: "monstertruck-modeling/src/text.rs"
      via: "merge_profiles combines wires from text::glyph_profile with custom wires"
      pattern: "merge_profiles"
    - from: "monstertruck-modeling/tests/font_pipeline.rs"
      to: "monstertruck-modeling/src/profile.rs"
      via: "tests call profile::merge_profiles and profile::attach_plane_normalized"
      pattern: "profile::merge_profiles"
---

<objective>
Enable combining font glyph outlines with arbitrary CAD sketch loops into a single face or solid, with correct winding and hole detection. Users will be able to merge wire sets from different sources (font glyphs, custom sketches) and construct valid planar faces from the combination.
</objective>

<execution_context>
</execution_context>

<context>
@monstertruck-modeling/src/profile.rs
@monstertruck-modeling/src/text.rs
@monstertruck-modeling/tests/font_pipeline.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write tests for mixed glyph + custom profile combinations</name>
  <files>monstertruck-modeling/tests/font_pipeline.rs</files>
  <action>
Add integration tests to `monstertruck-modeling/tests/font_pipeline.rs` for mixed glyph + custom profile scenarios. All tests should be `#[cfg(feature = "font")]`.

**Tests to add:**

1. `mixed_glyph_custom_outer_with_glyph_holes` -- Create a large custom rectangular outer loop. Extract glyph wires for 'O' (which has outer + hole contours). Use `profile::merge_profiles` to combine the custom outer rectangle with the glyph's hole contours (not its outer contour). Call `profile::attach_plane_normalized` on the merged set. Assert the face has boundaries matching the total wire count and that the face is valid.

2. `mixed_glyph_custom_face_construction` -- Create a custom outer rectangle. Extract wires for glyph 'l' (no holes, single contour). Merge both sets -- the glyph wire should be detected as a hole inside the larger rectangle. Call `attach_plane_normalized`. Assert 2 boundaries.

3. `mixed_multiple_glyphs_as_holes` -- Create a large custom outer rectangle. Extract wires for glyphs 'I' and 'l' (each single contour). Position them at different offsets using `text::text_profile` for "Il". Merge the custom rectangle with all glyph wires. The glyph outlines should be classified as holes. Assert the face has 3 boundaries (1 outer + 2 holes).

4. `mixed_glyph_custom_solid_extrusion` -- Combine a custom outer rectangle with glyph 'O' holes, create a face, then extrude to solid via `profile::solid_from_planar_profile`. Assert `is_geometric_consistent()`.

5. `merge_profiles_basic` -- Test `profile::merge_profiles` with two Vec<Wire> inputs. Assert the result is a flat Vec<Wire> with the combined count.

6. `merge_profiles_empty_second` -- Merge a non-empty set with an empty set. Assert the result equals the first set's count.

Use helper functions:
```rust
fn large_rect_wire() -> Wire {
    // Create a rectangle large enough to contain scaled glyph outlines
    rect_wire(-2.0, -2.0, 2.0, 2.0)
}

fn rect_wire(x0: f64, y0: f64, x1: f64, y1: f64) -> Wire {
    // Same pattern as profile_test.rs
}
```

Note: Glyph wires from `text::glyph_profile` are typically very small (normalized by units_per_em). The custom outer rectangle must be scaled to contain them, or use a `TextOptions` with explicit scale to make glyphs larger. Use `TextOptions { scale: Some(1.0), .. }` for 1:1 font-unit coordinates, then size the outer rectangle accordingly (e.g., 2000x2000 for typical font metrics).
  </action>
  <verify>Run `cargo test -p monstertruck-modeling --features font --test font_pipeline` -- new tests should fail to compile since `merge_profiles` doesn't exist yet.</verify>
  <done>Integration tests for mixed glyph + custom profile combinations written.</done>
</task>

<task type="auto">
  <name>Task 2: Implement merge_profiles and supporting logic</name>
  <files>monstertruck-modeling/src/profile.rs</files>
  <action>
Add a `merge_profiles` function to `monstertruck-modeling/src/profile.rs`:

```rust
/// Merges multiple sets of wires into a single flat wire collection.
///
/// This is the primary entry point for combining wires from different sources
/// (e.g., font glyph outlines, custom sketch loops) into a single profile
/// that can be passed to [`attach_plane_normalized`] or
/// [`solid_from_planar_profile`].
///
/// The merged wires do not need pre-normalized winding -- the downstream
/// `attach_plane_normalized` call handles outer/hole classification and
/// winding normalization automatically.
///
/// # Examples
///
/// ```ignore
/// let glyph_wires = text::glyph_profile(&face, glyph_id, &opts)?;
/// let custom_outer = vec![my_rectangle_wire];
/// let merged = profile::merge_profiles(vec![custom_outer, glyph_wires]);
/// let face = profile::attach_plane_normalized(merged)?;
/// ```
pub fn merge_profiles<C>(wire_sets: Vec<Vec<Wire<C>>>) -> Vec<Wire<C>> {
    wire_sets.into_iter().flatten().collect()
}
```

This function is intentionally simple -- it's a semantic entry point that:
1. Documents the intended workflow of combining wire sources
2. Flattens multiple Vec<Wire> into one
3. Relies on `attach_plane_normalized` for the actual classification

Additionally, add a convenience function for the common case:

```rust
/// Constructs a [`Face`] from a mixture of wire sources with automatic
/// outer/hole classification.
///
/// Equivalent to calling [`merge_profiles`] followed by
/// [`attach_plane_normalized`], but provides a single-call API for the
/// common use case.
pub fn face_from_mixed_profiles<C, S>(wire_sets: Vec<Vec<Wire<C>>>) -> Result<Face<C, S>>
where
    C: ParametricCurve3D + BoundedCurve + Clone + Invertible,
    Plane: IncludeCurve<C> + ToSameGeometry<S>,
{
    let merged = merge_profiles(wire_sets);
    attach_plane_normalized(merged)
}
```

Also add unit tests in the `#[cfg(test)] mod tests` block within profile.rs:
- `merge_profiles_flat`: merge 3 single-wire vecs, assert result has 3 wires
- `merge_profiles_empty`: merge with empty vec, assert no panic
- `merge_profiles_mixed_sizes`: merge a 2-wire vec with a 1-wire vec, assert 3 total
  </action>
  <verify>Run `cargo test -p monstertruck-modeling profile` for unit tests and `cargo test -p monstertruck-modeling --features font --test font_pipeline` for integration tests -- all should pass.</verify>
  <done>merge_profiles and face_from_mixed_profiles implemented, unit tests passing, and font_pipeline integration tests passing.</done>
</task>

</tasks>

<verification>
1. All existing font_pipeline tests still pass: `cargo test -p monstertruck-modeling --features font --test font_pipeline`
2. All existing profile tests still pass: `cargo test -p monstertruck-modeling --test profile_test`
3. New mixed glyph+custom tests pass: `cargo test -p monstertruck-modeling --features font --test font_pipeline mixed`
4. Unit tests for merge_profiles pass: `cargo test -p monstertruck-modeling profile::tests::merge`
5. No compiler warnings from the modified files
</verification>

<success_criteria>
- A face can be constructed from a mixture of font glyph outlines and user-defined sketch loops
- Correct winding normalization handles mixed sources (glyph wires + custom wires)
- Hole detection works when glyph contours are inside a custom outer loop
- Multiple glyph sources can be combined with custom geometry in a single profile
- merge_profiles correctly flattens multiple wire sets
- All constructed faces pass geometric consistency checks
</success_criteria>

<output>
After completion, create `.tendrion/phases/14-profile-solid-pipeline/14-2-SUMMARY.md`
</output>
