---
phase: 32-i-o-validation-and-migration-docs
plan: 2
tags: [documentation, migration, deprecation]
key-files:
  - docs/MIGRATION.md
decisions:
  - "Corrected plan inaccuracy: deprecated function is `cone` not `revolve` (confirmed via source at builder.rs:1533 and review file 32-2-REVIEW-planning.md)"
  - "Added `interpole` -> `interpolate` rename (discovered during codebase scan, not listed in plan)"
  - "Documented `PCurve` -> `ParameterCurve` rename in monstertruck-geometry decorators (discovered during scan)"
metrics:
  tasks: 2
  tasks_completed: 2
  deviations: 1
  tdd_exempt: true
  tdd_exempt_reason: "Pure documentation file -- no runtime code"
---

## What Was Built

- **`docs/MIGRATION.md`** (320 lines): Comprehensive migration guide for v0.5.2 to v0.5.3 covering:
  - Complete deprecation tables across all 7 crates (monstertruck-core, monstertruck-topology, monstertruck-gpu, monstertruck-step, monstertruck-geometry, monstertruck-traits, monstertruck-modeling)
  - 7 before/after code example pairs for major API change patterns
  - 8 numbered version upgrade steps
  - New API patterns: ruled surfaces, Gordon from intersection grid, geometry healing, GPU skip, I/O validation
  - Compatibility notes and v0.6.0 removal timeline

## Deprecation Inventory (33 items total)

- 1 builder function rename (`cone` -> `revolve_wire`)
- 5 surface constructor migrations (`skin`, `sweep_rail`, `birail1`, `birail2`, `gordon` -> `try_*` variants)
- 2 curve method renames (`interpole` -> `interpolate`, `try_interpole` -> `try_interpolate`)
- 5 ID type renames (`ID`, `VertexID`, `EdgeID`, `FaceID`, `RenderID`)
- 11 STEP loader type renames (BSpline/PCurve casing)
- 4 geometry type renames (`BSplineCurve`, `BSplineSurface`, `KnotVec`, `PCurve`)
- 2 search parameter hint renames (`SPHint1D`, `SPHint2D`)
- 3 new surface APIs documented (`try_ruled`, `try_gordon_from_network`, `try_gordon_verified`)

## Deviations

- Plan listed `revolve` as the deprecated function; actual deprecated function is `cone` (builder.rs:1533). Corrected in final document. This was also noted in the planning review (32-2-REVIEW-planning.md).

## Verification

- [x] `docs/MIGRATION.md` exists with 320 lines (minimum: 100)
- [x] Document contains complete deprecated -> replacement mapping tables
- [x] Document contains 7 before/after code example pairs
- [x] Document contains 8 numbered version upgrade steps
- [x] All deprecated type names match actual `#[deprecated]` annotations in source
- [x] Document covers new v0.5.3 capabilities (ruled surface, Gordon from network, healing, I/O validation)
