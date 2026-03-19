---
phase: 14-profile-solid-pipeline
plan: 2
tags: [profile, font, merge, mixed-profiles, tdd]
key-files:
  - monstertruck-modeling/src/profile.rs
  - monstertruck-modeling/tests/font_pipeline.rs
decisions: []
metrics:
  tests_added: 9
  tests_passed: 36
  deviations: 0
---

## What was built

**monstertruck-modeling/src/profile.rs** -- Added two public functions:
- `merge_profiles<C>(wire_sets: Vec<Vec<Wire<C>>>) -> Vec<Wire<C>>`: Flattens multiple wire sets from different sources (font glyphs, custom sketches) into a single `Vec<Wire>` for downstream face construction.
- `face_from_mixed_profiles<C, S>(wire_sets: Vec<Vec<Wire<C>>>) -> Result<Face<C, S>>`: Convenience wrapper combining `merge_profiles` + `attach_plane_normalized`.
- 3 unit tests: `merge_profiles_flat`, `merge_profiles_empty`, `merge_profiles_mixed_sizes`.

**monstertruck-modeling/tests/font_pipeline.rs** -- Added 6 integration tests:
- `mixed_glyph_custom_outer_with_glyph_holes`: Custom rectangle + glyph 'O' hole contours.
- `mixed_glyph_custom_face_construction`: Custom rectangle + glyph 'l' as hole.
- `mixed_multiple_glyphs_as_holes`: Custom rectangle + "Il" text wires (2 holes).
- `mixed_glyph_custom_solid_extrusion`: Mixed profile extruded to solid, geometric consistency verified.
- `merge_profiles_basic`: Two non-empty sets merged.
- `merge_profiles_empty_second`: Non-empty + empty set merged.

## Task commits

| Step | SHA | Message |
|------|-----|---------|
| RED | `9d53c113` | test(profile): add failing tests for mixed glyph + custom profile combinations |
| GREEN | `c23c2c8c` | feat(profile): implement merge_profiles and face_from_mixed_profiles |

## Verification

- All 17 font_pipeline tests pass (11 existing + 6 new).
- All 19 profile_test integration tests pass (unchanged).
- All 3 new profile unit tests pass.
- No clippy warnings in modified files.
- No refactoring needed -- implementation is minimal (single `flatten().collect()`).

## Deviations

None.
