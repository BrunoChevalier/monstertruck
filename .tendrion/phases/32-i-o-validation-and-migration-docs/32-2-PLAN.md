---
phase: 32-i-o-validation-and-migration-docs
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - docs/MIGRATION.md
autonomous: true
must_haves:
  truths:
    - "User opens docs/MIGRATION.md and finds a complete guide covering v0.5.2 to v0.5.3 API changes"
    - "User finds deprecated function names with their replacement equivalents in a reference table"
    - "User finds before/after code examples for each major API change pattern"
    - "User finds version upgrade steps explaining how to update Cargo.toml and resolve breaking changes"
  artifacts:
    - path: "docs/MIGRATION.md"
      provides: "Migration guidance document for v0.5.3 covering deprecated functions, new API patterns, and upgrade steps"
      min_lines: 100
      contains: "Migration"
  key_links:
    - from: "docs/MIGRATION.md"
      to: "monstertruck-modeling/src/builder.rs"
      via: "Documents deprecated revolve function replacement"
      pattern: "revolve_wire"
    - from: "docs/MIGRATION.md"
      to: "monstertruck-step/src/load/mod.rs"
      via: "Documents RFC 430 type renames (C-CASE convention)"
      pattern: "BsplineCurveForm"
    - from: "docs/MIGRATION.md"
      to: "monstertruck-topology/src/lib.rs"
      via: "Documents topology ID type renames"
      pattern: "VertexId"
---

<objective>
Create a comprehensive migration guide (docs/MIGRATION.md) for users upgrading from v0.5.2 to v0.5.3, documenting all deprecated function replacements, new API patterns introduced across milestone phases 24-32, and step-by-step upgrade instructions.
</objective>

<execution_context>
@skills/execution-tracking/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@CHANGELOG.md
@monstertruck-modeling/src/builder.rs (deprecated revolve function)
@monstertruck-step/src/load/mod.rs (deprecated type aliases)
@monstertruck-topology/src/lib.rs (deprecated ID types)
@monstertruck-core/src/id.rs (deprecated ID type)
@monstertruck-traits/src/traits/search_parameter.rs (deprecated hint types)
@monstertruck-gpu/src/lib.rs (deprecated RenderId)
@monstertruck-geometry/src/lib.rs (deprecated geometry types)
@monstertruck-geometry/src/nurbs/mod.rs (deprecated NURBS types)
</context>

<tasks>

<task type="auto">
  <name>Task 1: Research all deprecated items and API changes in v0.5.3</name>
  <files>docs/MIGRATION.md</files>
  <action>
Before writing, scan the codebase for all `#[deprecated]` annotations to build a complete inventory. The known deprecations are:

**monstertruck-modeling/src/builder.rs:**
- `revolve` (deprecated) -> `revolve_wire` (new, takes explicit origin parameter)

**monstertruck-step/src/load/mod.rs (RFC 430 C-CASE renames):**
- `BSPLINE_CURVE_FORM` -> `BsplineCurveForm`
- `BSPLINE_CURVE_WITH_KNOTS` -> `BsplineCurveWithKnots`
- `NON_RATIONAL_BSPLINE_CURVE` -> `NonRationalBsplineCurve`
- `RATIONAL_BSPLINE_CURVE` -> `RationalBsplineCurve`
- `BSPLINE_CURVE_ANY` -> `BsplineCurveAny`
- `BSPLINE_SURFACE_ANY` -> `BsplineSurfaceAny`
- `BSPLINE_SURFACE_FORM` -> `BsplineSurfaceForm`
- `BSPLINE_SURFACE_WITH_KNOTS` -> `BsplineSurfaceWithKnots`
- `NON_RATIONAL_BSPLINE_SURFACE` -> `NonRationalBsplineSurface`
- `RATIONAL_BSPLINE_SURFACE` -> `RationalBsplineSurface`
- `PCURVE` -> `Pcurve`

**monstertruck-topology/src/lib.rs (RFC 430 C-CASE renames):**
- `VertexID` -> `VertexId`
- `EdgeID` -> `EdgeId`
- `FaceID` -> `FaceId`

**monstertruck-core/src/id.rs:**
- `ID` -> `Id`

**monstertruck-traits/src/traits/search_parameter.rs:**
- `SearchParameterHint` (1D) -> `SearchParameterHint1D`
- `SearchParameterHint` (2D) -> `SearchParameterHint2D`

**monstertruck-gpu/src/lib.rs:**
- `RenderID` -> `RenderId`

**monstertruck-geometry (scan all files for #[deprecated]):**
- Run `grep -rn '#\[deprecated' monstertruck-geometry/src/` to find all deprecated items
- Include any deprecated type aliases, function names, or struct renames in the inventory
- These may include geometry-level deprecated items (NURBS types, surface constructors, etc.)

Also document new capabilities added in v0.5.3 phases:
- Phase 24: GPU test reliability (skip on headless CI)
- Phase 25: Dependency updates (nom v7+, quick-xml v0.30+)
- Phase 26-29: Expanded test coverage across all crates
- Phase 30: New surface constructors (ruled surface, loft surface, geometry healing)
- Phase 31: Intersection-grid Gordon surfaces, improved trim tessellation
- Phase 32: I/O validation tests
  </action>
  <verify>All deprecated items from the codebase are accounted for in the inventory.</verify>
  <done>Complete inventory of all deprecated items and API changes compiled.</done>
</task>

<task type="auto">
  <name>Task 2: Write the migration guide document</name>
  <files>docs/MIGRATION.md</files>
  <action>
Create `docs/MIGRATION.md` with the following structure:

```markdown
# Migration Guide

## Upgrading to v0.5.3

### Overview
Summary of what changed in v0.5.3 and why users should upgrade.

### Breaking Changes
(None in v0.5.3 -- all changes are additive or deprecation-based)

### Deprecated Function Replacements

#### Builder API Changes

| Deprecated | Replacement | Crate | Notes |
|-----------|-------------|-------|-------|
| `revolve(...)` | `revolve_wire(...)` | monstertruck-modeling | Now takes explicit origin parameter |

**Before:**
```rust
// Old API (deprecated)
let solid = builder::revolve(&face, axis, angle);
```

**After:**
```rust
// New API
let solid = builder::revolve_wire(&wire, origin, axis, angle, segments);
```

#### Type Renames (RFC 430 C-CASE Convention)

Table of all SCREAMING_CASE -> CamelCase renames across crates.

| Deprecated | Replacement | Crate |
|-----------|-------------|-------|
| `ID` | `Id` | monstertruck-core |
| `VertexID` | `VertexId` | monstertruck-topology |
| `EdgeID` | `EdgeId` | monstertruck-topology |
| `FaceID` | `FaceId` | monstertruck-topology |
| `RenderID` | `RenderId` | monstertruck-gpu |
| `BSPLINE_CURVE_FORM` | `BsplineCurveForm` | monstertruck-step |
| (... all others ...) | | |

**Before:**
```rust
use monstertruck_topology::VertexID;
use monstertruck_core::ID;
```

**After:**
```rust
use monstertruck_topology::VertexId;
use monstertruck_core::Id;
```

#### Search Parameter Hints

| Deprecated | Replacement | Crate |
|-----------|-------------|-------|
| `SearchParameterHint` (1D context) | `SearchParameterHint1D` | monstertruck-traits |
| `SearchParameterHint` (2D context) | `SearchParameterHint2D` | monstertruck-traits |

### New API Patterns

#### Surface Constructors (Phase 30)
- Ruled surface: `BsplineSurface::try_ruled(curve1, curve2)`
- Loft surface: `BsplineSurface::try_loft(profiles, options)`
- Geometry healing: gap detection and edge repair utilities

#### Gordon Surface from Intersection Grid (Phase 31)
- `BsplineSurface::try_gordon_from_network(curves)` -- auto-computes intersection grid

#### I/O Validation (Phase 32)
- STEP round-trip tests with bounding-box matching
- OBJ/STL format validation tests

### Version Upgrade Steps

1. Update `Cargo.toml` to use v0.5.3 dependencies
2. Run `cargo build` -- deprecated warnings will appear for old names
3. Find-and-replace deprecated type names (use the table above)
4. Replace `builder::revolve` calls with `builder::revolve_wire`
5. Replace `SearchParameterHint` with `SearchParameterHint1D` or `SearchParameterHint2D`
6. Run `cargo clippy` to verify no remaining deprecation warnings

### Compatibility Notes
- All deprecated items still compile in v0.5.3 with warnings
- Deprecated items may be removed in v0.6.0
- GPU/render tests now skip gracefully on headless machines
```

Ensure the document is self-contained, practical, and actionable. Use real type names and real crate paths. Check the actual deprecated annotations in the source code to ensure accuracy of the before/after examples.
  </action>
  <verify>Verify `docs/MIGRATION.md` exists with at least 100 lines. Verify it contains sections for "Deprecated", "Before", "After", "Upgrade Steps". Verify referenced type names match actual deprecated annotations in the codebase.</verify>
  <done>Migration guide document created at docs/MIGRATION.md covering all deprecated function replacements, new API patterns, and version upgrade steps.</done>
</task>

</tasks>

<verification>
1. `docs/MIGRATION.md` exists and contains at least 100 lines of content
2. Document contains a complete table of all deprecated -> replacement mappings
3. Document contains before/after code examples for major API changes
4. Document contains numbered version upgrade steps
5. All deprecated type names referenced in the document match actual `#[deprecated]` annotations in source code
6. Document covers new capabilities added in v0.5.3 (surface constructors, Gordon surfaces, I/O validation)
</verification>

<success_criteria>
- Migration guidance document exists at docs/MIGRATION.md (DOC-01)
- Document covers new API patterns introduced in v0.5.3
- Document covers deprecated function replacements with before/after examples
- Document includes step-by-step version upgrade instructions
- All deprecated items from the codebase are accurately documented
</success_criteria>

<output>
After completion, create `.tendrion/phases/32-i-o-validation-and-migration-docs/32-2-SUMMARY.md`
</output>
