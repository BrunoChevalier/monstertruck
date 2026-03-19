# Font Stress Corpus

Curated corpus of pathological font geometry fixtures for regression testing
the profile normalization and solid extrusion pipeline.

## Running Tests

```bash
cargo nextest run -p monstertruck-modeling --features font font_stress
```

## Corpus Entries

| Entry | Source File | Failure Mode Category | Description | Status |
|-------|------------|-----------------------|-------------|--------|
| `self_intersecting_cubic` | `self_intersecting.rs` | Self-intersecting contours | Figure-8 contour where cubic Bezier segments cross each other | pass |
| `bow_tie_contour` | `self_intersecting.rs` | Self-intersecting contours | Two triangular loops sharing a single vertex (bow-tie shape) | pass |
| `overlapping_contours` | `self_intersecting.rs` | Overlapping contours | Two separate rectangles with overlapping bounding boxes | pass |
| `near_zero_area_sliver` | `near_zero_area.rs` | Near-zero-area loops | Extremely thin rectangle (width ~1e-8) with negligible area | pass |
| `collapsed_quad_bezier` | `near_zero_area.rs` | Degenerate Bezier curves | Quadratic Beziers with control points ~1e-10 from endpoints | pass |
| `micro_feature_loop` | `near_zero_area.rs` | Micro-features | Tiny contour within 1e-6 x 1e-6 bounding box | pass |
| `deeply_nested_holes` | `deeply_nested.rs` | Deeply nested holes | 5 concentric rectangles (outer > hole > island > inner-hole > innermost) | pass |
| `high_loop_count` | `deeply_nested.rs` | High contour counts | 21 wires: 1 outer + 20 small rectangles in a 4x5 grid | pass |
| `coincident_control_points` | `degenerate.rs` | Coincident control points | Cubic Bezier edges where control points coincide with start vertex | pass |
| `reverse_wound_hole` | `degenerate.rs` | Reverse-wound holes | Outer CCW rectangle + inner CW rectangle (pre-wound hole) | pass |
| `single_point_degeneracy` | `degenerate.rs` | Single-point degeneracies | Wire with two consecutive vertices at the same point (zero-length edge) | pass |
| `@` glyph | DejaVuSans.ttf | Complex real-world glyphs | Complex nested contours from the at-sign glyph | pass |
| `&` glyph | DejaVuSans.ttf | Complex real-world glyphs | Self-intersecting-like curves from the ampersand glyph | pass |
| `%` glyph | DejaVuSans.ttf | Complex real-world glyphs | Multiple small circular contours from the percent glyph | pass |

## Failure Mode Categories

### Self-intersecting contours
Contours where edges cross, creating ambiguous interior regions. The signed-area
computation may produce unexpected results, and the containment classifier cannot
reliably determine inside/outside for self-intersecting paths.

### Overlapping contours
Separate wires with overlapping bounding boxes that are siblings (neither contains
the other). The containment classifier must correctly determine that no
parent/child relationship exists despite spatial overlap.

### Near-zero-area loops
Contours whose enclosed area approaches machine epsilon. The plane normal vector
may be poorly conditioned, and cross-product calculations may underflow to zero.

### Micro-features
Sub-pixel geometry that may be below processing thresholds. All coordinates lie
within an extremely small bounding box (e.g., 1e-6 x 1e-6), testing whether the
pipeline preserves or filters features smaller than typical tolerance values.

### Degenerate Bezier curves
Bezier curves where control points coincide with or nearly coincide with endpoints.
The resulting curves have near-zero curvature, and tangent vectors at control-point
coincidence are zero or undefined.

### Deeply nested holes
Contours with 3+ levels of containment hierarchy (outer > hole > island > ...).
Most simple classifiers only handle 2 levels (outer + hole). Deep nesting
requires recursive or multi-pass containment analysis.

### High contour counts
Many wires in a single profile, testing O(n^2) scalability of containment
classification and normalization.

### Reverse-wound holes
Holes that are already wound in the expected CW direction. The normalization
pipeline must detect existing winding and avoid double-inverting.

### Single-point degeneracies
Zero-length edges created by two consecutive vertices at the same point.
Edge direction vectors are zero, causing issues in tangent and normal computation.

### Coincident control points
Bezier edges where interior control points are placed at vertex positions,
effectively collapsing the curve to a degenerate near-linear path.

### Complex real-world glyphs
Glyphs from DejaVuSans.ttf that naturally exercise stress patterns:
`@` (deep nesting), `&` (complex curves), `%` (multiple small contours).

## Adding New Entries

1. Create the fixture function in the appropriate sub-module under
   `test-fixtures/stress-corpus/`. Return `Vec<Wire>` and include doc comments
   describing the pathological geometry, failure mode, and real-world analog.

2. Add the fixture to the `all_fixtures()` list in `mod.rs`.

3. Write a dedicated test in `tests/font_stress_corpus.rs` prefixed with
   `font_stress_`. The test should:
   - Assert wire count and closure.
   - Attempt `profile::attach_plane_normalized` and either assert success
     or document the known limitation with `// KNOWN LIMITATION:` and
     `// TODO(font-stress):` comments.

4. Update this README table with the new entry.
