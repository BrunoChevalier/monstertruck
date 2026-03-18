---
phase: 9-boolean-repair-and-tolerance-foundation
plan: 2
type: execute
wave: 1
depends_on: []
files_modified:
  - monstertruck-solid/src/transversal/faces_classification/mod.rs
  - monstertruck-solid/src/transversal/integrate/mod.rs
  - monstertruck-solid/src/transversal/mod.rs
  - monstertruck-solid/src/transversal/edge_cases.rs
autonomous: true
must_haves:
  truths:
    - "Boolean AND of overlapping cubes produces a closed shell with no boundary loops"
    - "Boolean OR of overlapping cubes produces a closed shell with no singular vertices"
    - "Face classification handles empty boundary wires without panicking"
    - "integrate_by_component gracefully handles components with empty boundaries by leaving them Unknown"
    - "The unknown-face classification loop does not fail the entire boolean when a single face cannot be classified"
    - "Coincident face detection is wired as diagnostic logging behind MT_BOOL_DEBUG_COINCIDENT"
    - "heal_shell_if_needed returns the best available shell instead of None when extract fails"
  artifacts:
    - path: "monstertruck-solid/src/transversal/faces_classification/mod.rs"
      provides: "Robust face classification with empty-boundary guard and majority-edge scoring"
      min_lines: 60
      contains: "integrate_by_component"
    - path: "monstertruck-solid/src/transversal/integrate/mod.rs"
      provides: "Boolean pipeline with resilient unknown-face classification, diagnostic coincident logging, improved healing fallback, and better diagnostics"
      min_lines: 500
      contains: "process_one_pair_of_shells"
    - path: "monstertruck-solid/src/transversal/edge_cases.rs"
      provides: "Edge case detection utilities for coincident and tangent faces"
      min_lines: 200
      contains: "detect_coincident_faces"
  key_links:
    - from: "monstertruck-solid/src/transversal/faces_classification/mod.rs"
      to: "monstertruck-solid/src/transversal/integrate/mod.rs"
      via: "FacesClassification consumed by process_one_pair_of_shells"
      pattern: "FacesClassification"
    - from: "monstertruck-solid/src/transversal/edge_cases.rs"
      to: "monstertruck-solid/src/transversal/integrate/mod.rs"
      via: "detect_coincident_faces called as diagnostic logging only"
      pattern: "edge_cases::detect_coincident_faces"
---

<objective>
Fix boolean operation bugs by hardening the face classification pipeline, making the unknown-face classification loop resilient to single-face failures, wiring coincident-face detection as diagnostic-only logging, and improving shell healing robustness with a clearly ordered fallback strategy. Tangent face handling is out of scope for this plan.
</objective>

<execution_context>
@AGENTS.md
</execution_context>

<context>
@monstertruck-solid/src/transversal/integrate/mod.rs
@monstertruck-solid/src/transversal/faces_classification/mod.rs
@monstertruck-solid/src/transversal/edge_cases.rs
@monstertruck-solid/src/transversal/mod.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Harden face classification and enable edge_cases module</name>
  <files>monstertruck-solid/src/transversal/faces_classification/mod.rs, monstertruck-solid/src/transversal/mod.rs, monstertruck-solid/src/transversal/edge_cases.rs</files>
  <action>
**Part A: Fix integrate_by_component to handle empty boundaries.**

In `monstertruck-solid/src/transversal/faces_classification/mod.rs`, the `integrate_by_component` method at line 52 does `boundary[0][0].id()` which panics if `boundary` is empty or `boundary[0]` is empty. Also, the method only checks a single edge, which can misclassify components that share edges with both and/or regions.

Replace the body of `integrate_by_component` (lines 42-73) with a robust implementation:

```rust
pub fn integrate_by_component(&mut self) {
    let [and, or, unknown] = self.and_or_unknown();
    let and_boundary = and.extract_boundaries();
    let or_boundary = or.extract_boundaries();
    let and_edge_ids: rustc_hash::FxHashSet<_> = and_boundary
        .iter()
        .flat_map(|wire| wire.edge_iter().map(|e| e.id()))
        .collect();
    let or_edge_ids: rustc_hash::FxHashSet<_> = or_boundary
        .iter()
        .flat_map(|wire| wire.edge_iter().map(|e| e.id()))
        .collect();
    let components = unknown.connected_components();
    for comp in components {
        let boundary = comp.extract_boundaries();
        let comp_edge_ids: Vec<_> = boundary
            .iter()
            .flat_map(|wire| wire.edge_iter().map(|e| e.id()))
            .collect();
        if comp_edge_ids.is_empty() {
            continue; // Cannot classify; leave as Unknown.
        }
        let and_matches = comp_edge_ids.iter().filter(|id| and_edge_ids.contains(id)).count();
        let or_matches = comp_edge_ids.iter().filter(|id| or_edge_ids.contains(id)).count();
        if and_matches > 0 && and_matches >= or_matches {
            comp.iter().for_each(|face| {
                *self.status.get_mut(&face.id())
                    .expect("face id missing from status map") = ShapesOpStatus::And;
            })
        } else if or_matches > 0 && or_matches > and_matches {
            comp.iter().for_each(|face| {
                *self.status.get_mut(&face.id())
                    .expect("face id missing from status map") = ShapesOpStatus::Or;
            })
        }
        // else: tie or no matches -- leave as Unknown
    }
}
```

Add `FxHashSet` to the existing `rustc_hash` import. The file already imports `FxHashMap as HashMap` on line 3. Change it to:
```rust
use rustc_hash::{FxHashMap as HashMap, FxHashSet};
```
Then use `FxHashSet` directly (not qualified) in the code above, or keep the qualified `rustc_hash::FxHashSet` — either works. Be consistent.

**Part B: Enable the edge_cases module.**

In `monstertruck-solid/src/transversal/mod.rs`:
1. Remove the `#[allow(dead_code)]` attribute from `mod edge_cases;` (line 2).
2. Change `mod edge_cases;` to `pub(crate) mod edge_cases;`.

In `monstertruck-solid/src/transversal/edge_cases.rs`:
- Verify all public functions compile without dead_code warnings now that the module is pub(crate).
- If `handle_degenerate_intersection` or `detect_tangent_faces` cause unused warnings, add `#[allow(dead_code)]` on the specific unused items only, NOT on the whole module. The module being `pub(crate)` means items are accessible. Only the boolean pipeline will call `detect_coincident_faces` directly (in Task 2); the other functions are diagnostic utilities that may be used later.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --lib -E 'test(faces_classification)' --no-fail-fast` and `cargo nextest run -p monstertruck-solid --lib -E 'test(edge_cases)' --no-fail-fast`. Run `cargo clippy -p monstertruck-solid --all-targets -- -W warnings` to confirm no warnings on the edge_cases module.</verify>
  <done>Face classification handles empty boundaries with majority-edge scoring; edge_cases module enabled without dead_code suppression on the whole module.</done>
</task>

<task type="auto">
  <name>Task 2: Make unknown-face classification resilient and wire coincident detection as diagnostic logging</name>
  <files>monstertruck-solid/src/transversal/integrate/mod.rs</files>
  <action>
**Design rationale:** Previous review rounds attempted to use coincident-face detection as a pre-classification mechanism, mapping original face indices to post-divide_faces identities. This approach is fundamentally flawed because divide_faces creates new face identities that cannot be reliably mapped back. Instead, this task takes a simpler approach: make the unknown-face classification loop resilient to individual failures, and use coincident detection as diagnostic logging only.

**Change 1: Add diagnostic coincident-face logging.**

In `process_one_pair_of_shells`, after line 338 (`return Err(ShapeOpsError::InvalidTolerance);`) and before line 340 (`let poly_tol = ...`), add:

```rust
// Diagnostic: detect coincident faces on original shell geometry.
// This is logging-only — it does NOT feed into classification.
let debug_coincident = std::env::var("MT_BOOL_DEBUG_COINCIDENT").is_ok();
if debug_coincident {
    let coincident_pairs = super::edge_cases::detect_coincident_faces(shell0, shell1, tol);
    if !coincident_pairs.is_empty() {
        eprintln!(
            "debug coincident_pairs count={} pairs={:?}",
            coincident_pairs.len(),
            coincident_pairs,
        );
    }
}
```

This is behind an env var check so there is zero runtime cost when not debugging. It does NOT influence classification in any way.

**Change 2: Make the unknown-face classification loop resilient.**

Currently (lines 378-385), when `classify_unknown_face` returns `None` for any face from shell0, the entire boolean fails with `UnknownClassificationFailed`. The `None` comes from `sample_points_on_face` returning `None` (face has no triangulatable vertices) or from `classify_inside_with_polyshell` returning `None` (ray-cast ambiguity, especially for coincident/near-coincident faces).

Replace lines 378-385:
```rust
let mut unknown_faces = Vec::new();
unknown0
    .into_iter()
    .try_for_each(|face| {
        unknown_faces.push((face.clone(), classify_unknown_face(&poly_shell1, &face)?));
        Some(())
    })
    .ok_or(ShapeOpsError::UnknownClassificationFailed { shell_index: 0 })?;
```

With:
```rust
let mut unknown_faces = Vec::new();
let mut shell0_classify_failures = 0usize;
for face in unknown0.into_iter() {
    match classify_unknown_face(&poly_shell1, &face) {
        Some(is_inside) => {
            unknown_faces.push((face.clone(), is_inside));
        }
        None => {
            // Classification failed for this face. Use a conservative default
            // (false = outside = OR) rather than failing the entire boolean.
            // For AND operations this is safe: an incorrectly-classified face
            // will be rejected by the shell quality optimizer.
            shell0_classify_failures += 1;
            unknown_faces.push((face.clone(), false));
        }
    }
}
if debug_bool && shell0_classify_failures > 0 {
    eprintln!(
        "debug classify shell0: {shell0_classify_failures} faces fell back to default (outside)"
    );
}
```

Apply the same pattern to the shell1 classification loop (lines 396-402). Replace:
```rust
unknown1
    .into_iter()
    .try_for_each(|face| {
        unknown_faces.push((face.clone(), classify_unknown_face(&poly_shell0, &face)?));
        Some(())
    })
    .ok_or(ShapeOpsError::UnknownClassificationFailed { shell_index: 1 })?;
```

With:
```rust
let mut shell1_classify_failures = 0usize;
for face in unknown1.into_iter() {
    match classify_unknown_face(&poly_shell0, &face) {
        Some(is_inside) => {
            unknown_faces.push((face.clone(), is_inside));
        }
        None => {
            shell1_classify_failures += 1;
            unknown_faces.push((face.clone(), false));
        }
    }
}
if debug_bool && shell1_classify_failures > 0 {
    eprintln!(
        "debug classify shell1: {shell1_classify_failures} faces fell back to default (outside)"
    );
}
```

**Why `false` (outside) is the right default:** The optimizer that follows this loop evaluates all 2^N (or greedy) assignments and picks the one that produces the best shell quality score. A wrong initial assignment for a failed face will be corrected by the optimizer if flipping it improves quality. Using `false` (outside/OR) as default is conservative — for AND operations it removes material rather than adding it, which is safer for topology.

Note: The `UnknownClassificationFailed` error variant is no longer returned by this code path. Do NOT remove it from the enum — other code paths or future changes may use it, and removing it would be a breaking API change.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` to verify all boolean tests pass. Run `cargo clippy -p monstertruck-solid --all-targets -- -W warnings` to verify no warnings.</verify>
  <done>Unknown-face classification loop is resilient to individual face failures using conservative defaults; coincident face detection is wired as diagnostic logging only behind MT_BOOL_DEBUG_COINCIDENT.</done>
</task>

<task type="auto">
  <name>Task 3: Improve shell healing with clearly ordered fallback strategy</name>
  <files>monstertruck-solid/src/transversal/integrate/mod.rs</files>
  <action>
Replace the `heal_shell_if_needed` function (lines 235-245) with a clearly ordered fallback strategy. The current implementation has a single path that can return None and fail the entire boolean. The new implementation tries multiple candidates and picks the best:

```rust
fn heal_shell_if_needed<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    shell: Shell<Point3, C, S>,
    tol: f64,
) -> Option<Shell<Point3, C, S>> {
    // Stage 0: If already valid, return immediately.
    if shell.shell_condition() == ShellCondition::Closed
        && shell.singular_vertices().is_empty()
    {
        return Some(shell);
    }

    let original_quality = shell_quality(&shell);
    let debug_heal = std::env::var("MT_BOOL_DEBUG_HEAL").is_ok();

    // Stage 1: Compress + robust heal + extract.
    let mut compressed = shell.clone().compress();
    compressed.robust_split_closed_edges_and_faces(tol);
    let healed = Shell::extract(compressed).ok();

    if let Some(ref h) = healed {
        let q = shell_quality(h);
        if debug_heal {
            eprintln!("debug heal stage1 quality={q:?} original={original_quality:?}");
        }
        if q <= original_quality {
            return healed;
        }
    }

    // Stage 2: Compress without heal + extract (in case healing made it worse).
    let compressed_no_heal = shell.clone().compress();
    let unhealed = Shell::extract(compressed_no_heal).ok();

    if let Some(ref u) = unhealed {
        let q = shell_quality(u);
        if debug_heal {
            eprintln!("debug heal stage2 quality={q:?} original={original_quality:?}");
        }
        if q <= original_quality {
            return unhealed;
        }
    }

    // Stage 3: Pick the best candidate among healed, unhealed, and original.
    let candidates: Vec<Shell<Point3, C, S>> = [healed, unhealed]
        .into_iter()
        .flatten()
        .chain(std::iter::once(shell))
        .collect();
    if debug_heal {
        eprintln!(
            "debug heal stage3 candidates={}",
            candidates.len(),
        );
    }
    candidates
        .into_iter()
        .min_by_key(|s| shell_quality(s))
}
```

Key properties of this implementation:
- **Never returns None** if the input shell is non-empty. The original shell is always a fallback candidate.
- Distinguishes three states: `original`, `compressed_without_heal`, and `compressed_after_heal`.
- Uses `shell_quality` (already defined in the module at line 256) to pick the best candidate.
- Early-returns when a candidate is better than or equal to the original.
- Debug logging behind `MT_BOOL_DEBUG_HEAL` for diagnosing healing regressions.
- The `shell.clone()` calls are necessary because `.compress()` consumes the shell.

Also improve `try_cap_shell_with_existing_surfaces` (starting at line 264):
- Add an early return if the shell is already closed. Insert at the very beginning of the function body, before the existing `let debug_cap` line:
```rust
// Early exit: already closed shells don't need capping.
if shell.shell_condition() == ShellCondition::Closed {
    return shell;
}
```

And in `try_build_solid` (starting at line 511), at line 571 where `valid_boundaries.len() < boundaries.len()`, add diagnostic logging:
```rust
let output_boundaries = if valid_boundaries.len() < boundaries.len() {
    if std::env::var("MT_BOOL_DEBUG_COMPONENTS").is_ok() {
        boundaries.iter().enumerate()
            .filter(|(_, shell)| !is_valid(shell))
            .for_each(|(i, shell)| {
                eprintln!(
                    "debug build_solid dropping shell[{i}] condition={:?} boundary={} singular={}",
                    shell.shell_condition(),
                    shell.extract_boundaries().len(),
                    shell.singular_vertices().len(),
                );
            });
    }
    valid_boundaries
} else {
    boundaries
};
```
  </action>
  <verify>Run `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` to verify all boolean tests pass. Run `cargo clippy -p monstertruck-solid --all-targets -- -W warnings` to confirm no warnings.</verify>
  <done>Shell healing uses a clearly ordered 3-stage fallback (healed, unhealed, original) that never returns None; capping skips already-closed shells; build_solid logs dropped boundaries.</done>
</task>

</tasks>

<verification>
1. `cargo nextest run -p monstertruck-solid --lib --no-fail-fast` passes all boolean unit tests.
2. `cargo nextest run -p monstertruck-solid --test boolean_edge_cases --no-fail-fast` passes all edge case integration tests.
3. `cargo clippy -p monstertruck-solid --all-targets -- -W warnings` produces no warnings.
4. The `#[allow(dead_code)]` on `edge_cases` module is removed.
5. `integrate_by_component` no longer panics on empty boundaries (uses majority-edge scoring with FxHashSet).
6. The unknown-face classification loop falls back to a conservative default instead of failing the entire boolean.
7. Coincident face detection is wired as diagnostic logging only (behind MT_BOOL_DEBUG_COINCIDENT), NOT as a classification mechanism.
8. `heal_shell_if_needed` never returns None for non-empty input shells.
9. `try_cap_shell_with_existing_surfaces` early-returns for already-closed shells.
</verification>

<success_criteria>
- Face classification does not panic on empty boundaries
- Unknown-face classification is resilient: individual failures use conservative defaults instead of aborting
- Coincident face detection is diagnostic-only logging behind MT_BOOL_DEBUG_COINCIDENT (not a classification mechanism)
- Shell healing has a 3-stage fallback: healed > unhealed > original (never returns None)
- All existing boolean tests continue to pass
- No dead_code warnings on edge_cases module
- Tangent face handling is explicitly out of scope for this plan
- Addresses requirement BOOL-01
</success_criteria>

<output>
After completion, create `.tendrion/phases/9-boolean-repair-and-tolerance-foundation/9-2-SUMMARY.md`
</output>
