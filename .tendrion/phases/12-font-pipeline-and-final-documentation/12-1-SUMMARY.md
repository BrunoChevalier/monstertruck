---
phase: 12-font-pipeline-and-final-documentation
plan: 1
tags: [font, pipeline, integration-test, tdd]
key-files:
  - monstertruck-modeling/tests/font_pipeline.rs
  - monstertruck-modeling/test-fixtures/DejaVuSans.ttf
  - monstertruck-modeling/src/text.rs
decisions:
  - "Tests exercise existing font pipeline code; all 11 pass immediately (feature already implemented)"
  - "Fixed pre-existing clippy collapsible_if error in text.rs ContourCollector::move_to"
metrics:
  tests_added: 11
  tests_passed: 11
  deviations: 3
---

## What was built

- **monstertruck-modeling/test-fixtures/DejaVuSans.ttf**: Real font fixture (DejaVu Sans, permissive license) for integration tests.
- **monstertruck-modeling/tests/font_pipeline.rs**: 11 end-to-end integration tests covering the full font outline to wire/face/solid pipeline.

### Tests written

| Test | Verification |
|------|-------------|
| `glyph_o_has_hole` | 'O' produces >= 2 wires (outer + hole), all closed, >= 3 edges each |
| `glyph_b_has_two_holes` | 'B' produces >= 3 wires (outer + 2 holes), all closed |
| `glyph_8_has_two_holes` | '8' produces >= 3 wires |
| `glyph_l_has_no_holes` | 'l' produces exactly 1 closed wire |
| `glyph_profile_face_with_holes` | Wires from 'O' produce Face with matching boundary count |
| `glyph_profile_solid_extrusion` | 'O' wires extrude to geometrically consistent Solid |
| `glyph_b_solid_extrusion` | 'B' wires extrude to geometrically consistent Solid |
| `text_profile_hello` | "HO" text profile produces > 2 wires, all closed |
| `text_profile_spacing` | "II" second glyph wires have greater X coordinates (horizontal advance) |
| `text_profile_space_skipped` | "I I" has same wire count as "II" but wider spacing |
| `glyph_profile_y_flip` | Y-flip option negates Y coordinates |

## Task commits

| SHA | Message |
|-----|---------|
| `b681fce4` | chore(modeling): add DejaVuSans.ttf font fixture for integration tests |
| `876250a1` | test(modeling): add end-to-end font pipeline integration tests |
| `4a7466fe` | fix(modeling): collapse nested if in ContourCollector::move_to to satisfy clippy |

## Deviations

1. **Task 1 TDD exemption**: Font fixture copy is pure configuration with no testable behavior.
2. **Tests pass immediately**: All 11 tests exercise existing working code (glyph_profile, text_profile, attach_plane_normalized, solid_from_planar_profile).
3. **Pre-existing clippy fix**: Collapsed nested `if` in `text.rs` `ContourCollector::move_to` to satisfy `clippy::collapsible_if`.

## Self-check

- Font fixture exists and is 759,720 bytes.
- All 11 tests pass: `cargo nextest run -p monstertruck-modeling --features font --test font_pipeline`.
- Clippy clean: `cargo clippy -p monstertruck-modeling --features font --test font_pipeline -- -W warnings`.
- Test file is 210 lines, exceeding the 120-line minimum.
