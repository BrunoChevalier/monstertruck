---
target: "12-1"
type: planning
round: 1
max_rounds: 3
reviewer: claude-opus-4-6
stage: planning
date: 2026-03-19
verdict: PASS
---

# Planning Review: Plan 12-1

**Reviewer:** claude-opus-4-6
**Round:** 1 of 3
**Stage:** Planning
**Date:** 2026-03-19

## Verdict

**PASS**

No blockers found. The plan is well-structured, feasible, and covers the FONT-01 requirement from ROADMAP.md. The test strategy is thorough, covering single glyph hole preservation, face/solid construction, multi-character text profiles, spacing, and edge cases. The DOC-02 requirement is correctly deferred to sibling plan 12-2 (wave 2).

## Findings

### Blockers

None

### Suggestions

#### S1: Integration test may not have direct access to ttf_parser crate [confidence: 62]
- **Confidence:** 62
- **File:** 12-1-PLAN.md, Task 2 (line 96-98)
- **Issue:** The test file uses `ttf_parser::Face::parse(...)` directly. While integration tests can generally access dependencies of the crate under test, optional dependencies behind a feature flag may require adding `ttf-parser` explicitly to `[dev-dependencies]` to guarantee availability in integration test binaries. Task 1 notes "no changes needed" but this may not be correct.
- **Impact:** If `ttf_parser` is not linkable from the integration test, all tests fail to compile.
- **Suggested fix:** The implementer should verify compilation works as-is. If not, add `ttf-parser = { workspace = true }` to `[dev-dependencies]` in Cargo.toml. The plan's Task 1 already discusses this but concludes no changes are needed -- the implementer should validate this assumption early.

#### S2: Binary font fixture should be in .gitattributes or .gitignore considerations [confidence: 58]
- **Confidence:** 58
- **File:** 12-1-PLAN.md, Task 1 (line 64)
- **Issue:** Copying a ~700KB binary TTF file into the repo as a test fixture is reasonable, but the plan does not mention ensuring it is tracked properly (e.g., marking as binary in `.gitattributes` to prevent line-ending transformations).
- **Impact:** On Windows or with certain git configs, line-ending normalization could corrupt the binary file.
- **Suggested fix:** Consider adding `monstertruck-modeling/test-fixtures/*.ttf binary` to `.gitattributes`, or note that this is handled by git's auto-detection.

#### S3: Plan does not mention the `text_profile` cursor offset scaling detail [confidence: 71]
- **Confidence:** 71
- **File:** 12-1-PLAN.md, Task 2 (line 152-154)
- **Issue:** The `text_profile_spacing` test verifies that the second character's wires have greater X coordinates than the first. However, the actual `text_profile` implementation applies cursor offset in font units divided by scale (`c.start_x + c.offset_x / scale`), which is a non-obvious transform. If the test does not account for this correctly, assertions may be fragile or fail due to floating-point scaling nuances.
- **Impact:** Test may pass but not actually verify the correct spacing behavior, or may be brittle.
- **Suggested fix:** The implementer should verify that the spacing test's assertions account for the actual offset calculation in `text_profile`. Comparing relative X positions of sampled vertices between the two glyphs is sufficient if done correctly.

### Nits

#### N1: Duplicate closing `</output>` tag in plan [confidence: 96]
- **Confidence:** 96
- **File:** 12-1-PLAN.md, line 201
- **Issue:** The plan ends with two `</output>` tags (lines 200-201). The second appears to be a copy-paste artifact.

#### N2: Task 1 action step 3 is self-contradictory [confidence: 88]
- **Confidence:** 88
- **File:** 12-1-PLAN.md, Task 1 (lines 70-72)
- **Issue:** Step 3 says "Update Cargo.toml to add ttf-parser to [dev-dependencies]" then immediately says "no changes needed since ttf-parser is already a workspace dependency used via the font feature." The instruction contradicts itself. It would be clearer to simply state: "No Cargo.toml changes needed; the integration test will use `#![cfg(feature = "font")]` and ttf-parser is available through the font feature."

## Summary

Plan 12-1 is a well-designed integration test plan that comprehensively covers the FONT-01 requirement. It tests the complete pipeline from font loading through glyph extraction, wire topology verification (including hole preservation for O, B, 8), face construction, and solid extrusion. The two-task structure is appropriately sized. Wave 1 placement with no dependencies is correct since this plan only creates new test files and a fixture. The only area of mild concern is whether `ttf_parser` will be directly importable from integration tests without an explicit dev-dependency entry, but the implementer can resolve this at compile time.
