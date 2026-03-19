---
phase: 13-api-polish-and-surface-operations
plan: 2
type: tdd
wave: 2
depends_on: ["13-1"]
files_modified:
  - monstertruck-geometry/src/nurbs/bspline_surface.rs
  - monstertruck-geometry/src/nurbs/nurbs_surface.rs
  - monstertruck-geometry/tests/bspsurface.rs
autonomous: true
must_haves:
  truths:
    - "User calls split_at_u(t) on a BsplineSurface and gets a (left, right) tuple where left covers [u_start, t] and right covers [t, u_end]"
    - "User calls split_at_v(t) on a BsplineSurface and gets a (bottom, top) tuple"
    - "User calls sub_patch(u_range, v_range) to extract a rectangular sub-region of a surface"
    - "User calls split_at_u/split_at_v/sub_patch on a NurbsSurface and gets the same operations via delegation"
    - "Evaluating the sub-patch at any interior point matches the original surface at the corresponding parameter"
    - "Edge cases (splitting at domain boundary, sub_patch covering full domain) return correct results"
  artifacts:
    - path: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      provides: "split_at_u, split_at_v, sub_patch methods on BsplineSurface"
      min_lines: 2950
      contains: "fn split_at_u"
    - path: "monstertruck-geometry/src/nurbs/nurbs_surface.rs"
      provides: "Delegating split_at_u, split_at_v, sub_patch on NurbsSurface"
      min_lines: 520
      contains: "fn split_at_u"
    - path: "monstertruck-geometry/tests/bspsurface.rs"
      provides: "Integration tests for split and sub_patch operations"
      min_lines: 30
      contains: "split_at_u"
  key_links:
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/src/nurbs/nurbs_surface.rs"
      via: "NurbsSurface delegates to inner BsplineSurface and wraps results"
      pattern: "non_rationalized"
    - from: "monstertruck-geometry/src/nurbs/bspline_surface.rs"
      to: "monstertruck-geometry/tests/bspsurface.rs"
      via: "Integration tests exercise split_at_u, split_at_v, sub_patch"
      pattern: "split_at_u"
---

<objective>
Implement surface split-at-parameter and sub-patch extraction operations on BsplineSurface and NurbsSurface, building on the existing cut_u/cut_v infrastructure to provide non-mutating split and rectangular extraction workflows.
</objective>

<execution_context>
@skills/state-management/SKILL.md
</execution_context>

<context>
@.tendrion/ROADMAP.md
@monstertruck-geometry/src/nurbs/bspline_surface.rs
@monstertruck-geometry/src/nurbs/nurbs_surface.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write tests for split and sub_patch operations (TDD red phase)</name>
  <files>monstertruck-geometry/tests/bspsurface.rs, monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Add tests to `monstertruck-geometry/tests/bspsurface.rs` (or inline `#[cfg(test)]` module in bspline_surface.rs) that define the expected behavior:

```rust
#[test]
fn test_split_at_u_preserves_evaluation() {
    // Create a degree-2 surface with non-trivial knots.
    let knot_u = KnotVector::uniform_knot(2, 2);
    let knot_v = KnotVector::uniform_knot(2, 2);
    let cp = /* 4x4 grid of distinct Point3 values */;
    let surface = BsplineSurface::new((knot_u, knot_v), cp);
    let (left, right) = surface.split_at_u(0.5);
    // left covers [0, 0.5], right covers [0.5, 1]
    for i in 0..=10 {
        for j in 0..=10 {
            let u = 0.5 * i as f64 / 10.0;
            let v = j as f64 / 10.0;
            assert_near2!(left.subs(u, v), surface.subs(u, v));
        }
    }
    for i in 0..=10 {
        for j in 0..=10 {
            let u = 0.5 + 0.5 * i as f64 / 10.0;
            let v = j as f64 / 10.0;
            assert_near2!(right.subs(u, v), surface.subs(u, v));
        }
    }
}

#[test]
fn test_split_at_v_preserves_evaluation() {
    // Similar structure for v-direction split.
}

#[test]
fn test_sub_patch_preserves_evaluation() {
    // Extract a sub-patch from u in [0.25, 0.75] and v in [0.3, 0.8].
    // Verify evaluation matches original at all sample points within the sub-region.
}

#[test]
fn test_split_at_boundary_u() {
    // Splitting at u_start should give a degenerate left and full right.
    // Splitting at u_end should give full left and degenerate right.
}

#[test]
fn test_sub_patch_full_domain() {
    // sub_patch covering the entire domain should match the original.
}
```

These tests will fail initially (the methods don't exist yet). This is the TDD red phase.
  </action>
  <verify>Tests fail to compile because split_at_u, split_at_v, sub_patch do not exist yet. This confirms the test harness is correct.</verify>
  <done>TDD red-phase tests written for split and sub_patch operations.</done>
</task>

<task type="auto">
  <name>Task 2: Implement split_at_u, split_at_v, and sub_patch on BsplineSurface</name>
  <files>monstertruck-geometry/src/nurbs/bspline_surface.rs</files>
  <action>
Add the following methods to `BsplineSurface<P>` (alongside the existing `cut_u`/`cut_v`):

```rust
/// Splits the surface at parameter `u`, returning `(left, right)` without mutating `self`.
///
/// `left` covers `[u_start, u]` and `right` covers `[u, u_end]`.
/// Both halves evaluate identically to `self` at corresponding parameters.
pub fn split_at_u(&self, u: f64) -> (BsplineSurface<P>, BsplineSurface<P>)
where P: Clone {
    let mut left = self.clone();
    let right = left.cut_u(u);
    (left, right)
}

/// Splits the surface at parameter `v`, returning `(bottom, top)` without mutating `self`.
pub fn split_at_v(&self, v: f64) -> (BsplineSurface<P>, BsplineSurface<P>)
where P: Clone {
    let mut bottom = self.clone();
    let top = bottom.cut_v(v);
    (bottom, top)
}

/// Extracts a rectangular sub-patch from the surface over `[u0, u1] x [v0, v1]`.
///
/// The returned surface evaluates identically to `self` at parameters within
/// the specified range.
///
/// # Panics
///
/// Panics if `u0 >= u1` or `v0 >= v1`, or if the range is outside the surface domain.
pub fn sub_patch(&self, u_range: (f64, f64), v_range: (f64, f64)) -> BsplineSurface<P>
where P: Clone {
    let (u0, u1) = u_range;
    let (v0, v1) = v_range;
    // Cut in u: keep the part [u0, u_end], then cut off [u1, u_end].
    let (_, right_of_u0) = self.split_at_u(u0);
    let (middle, _) = right_of_u0.split_at_u(u1);
    // Cut in v: keep [v0, v_end], then cut off [v1, v_end].
    let (_, above_v0) = middle.split_at_v(v0);
    let (patch, _) = above_v0.split_at_v(v1);
    patch
}
```

IMPORTANT: The existing `cut_u` method mutates `self` (modifies it to be the left portion and returns the right). Verify this carefully:
- `cut_u(u)` mutates `self` to `[u_start, u]` and returns `[u, u_end]`.
- So `split_at_u` clones, calls `cut_u`, and returns `(left=mutated_clone, right=returned)`.

Make sure `sub_patch` correctly handles the parameter domain after successive splits. The cut operations preserve absolute parameter values, so nested cuts should work correctly.

Add comprehensive doc-comments with `# Examples` sections showing usage.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry -E 'test(split_at)' -E 'test(sub_patch)'` to verify all TDD tests pass (green phase).</verify>
  <done>split_at_u, split_at_v, and sub_patch implemented on BsplineSurface with all tests passing.</done>
</task>

<task type="auto">
  <name>Task 3: Delegate split/extract operations to NurbsSurface</name>
  <files>monstertruck-geometry/src/nurbs/nurbs_surface.rs</files>
  <action>
Add delegating methods to `NurbsSurface<V>`:

```rust
/// Splits the NURBS surface at parameter `u`, returning `(left, right)`.
pub fn split_at_u(&self, u: f64) -> (NurbsSurface<V>, NurbsSurface<V>)
where V: Clone {
    let (left, right) = self.0.split_at_u(u);
    (NurbsSurface::new(left), NurbsSurface::new(right))
}

/// Splits the NURBS surface at parameter `v`, returning `(bottom, top)`.
pub fn split_at_v(&self, v: f64) -> (NurbsSurface<V>, NurbsSurface<V>)
where V: Clone {
    let (bottom, top) = self.0.split_at_v(v);
    (NurbsSurface::new(bottom), NurbsSurface::new(top))
}

/// Extracts a rectangular sub-patch over `[u0, u1] x [v0, v1]`.
pub fn sub_patch(&self, u_range: (f64, f64), v_range: (f64, f64)) -> NurbsSurface<V>
where V: Clone {
    NurbsSurface::new(self.0.sub_patch(u_range, v_range))
}
```

Add doc-comments and inline tests or doc-tests for the NurbsSurface versions.
  </action>
  <verify>Run `cargo nextest run -p monstertruck-geometry --lib` to verify all tests pass. Run `cargo clippy -p monstertruck-geometry -- -W warnings`.</verify>
  <done>NurbsSurface has delegating split_at_u, split_at_v, and sub_patch methods with passing tests.</done>
</task>

</tasks>

<verification>
1. `split_at_u(t)` returns two surfaces whose union covers the original domain, with identical evaluations
2. `split_at_v(t)` returns two surfaces whose union covers the original domain, with identical evaluations
3. `sub_patch(u_range, v_range)` returns a surface that evaluates identically to the original within the specified rectangle
4. Edge cases (boundary splits, full-domain sub_patch) are handled correctly
5. NurbsSurface delegates correctly to BsplineSurface
6. `cargo nextest run -p monstertruck-geometry --lib` passes all tests
7. No clippy warnings
</verification>

<success_criteria>
- SURF-03: Patch split/extract workflows are complete with split_at_u, split_at_v, and sub_patch on both BsplineSurface and NurbsSurface
- All operations preserve geometric fidelity (evaluation at corresponding parameters matches)
- TDD methodology followed: tests written before implementation
</success_criteria>

<output>
After completion, create `.tendrion/phases/13-api-polish-and-surface-operations/13-2-SUMMARY.md`
</output>
