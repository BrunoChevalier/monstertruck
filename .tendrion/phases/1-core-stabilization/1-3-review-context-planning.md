# Review Context: Plan 1-3 (Planning Review, Round 2 of 2)

## Plan Under Review

- Plan ID: `1-3`
- Plan path: `.tendrion/phases/1-core-stabilization/1-3-PLAN.md`
- Previous review: `.tendrion/phases/1-core-stabilization/1-3-REVIEW-planning.md`
- Stage: `planning`
- Round: `2 of 2`

## Dispatch Focus (Blockers From Round 1)

- `B1`: Plan allowed keeping all `unwrap()` calls with `// SAFETY:` comments and had no numeric reduction target.
- `B2`: Safety invariant in `integrate` was incorrect (`Solid::try_new` does not reject empty boundaries), leaving a panic path.

## Evidence Summary

### B1 Status: RESOLVED

- The plan now carries explicit numeric targets in `must_haves` (`16 -> <=8` for `monstertruck-solid`, `25 -> <=12` for `monstertruck-meshing`) at `.tendrion/phases/1-core-stabilization/1-3-PLAN.md:27-28`.
- The objective also states baseline and target explicitly at `.tendrion/phases/1-core-stabilization/1-3-PLAN.md:49`.
- Phase-level reduction framing is explicitly restated in success criteria (`41 -> <=20`) at `.tendrion/phases/1-core-stabilization/1-3-PLAN.md:224`.

### B2 Status: RESOLVED

- The plan now explicitly states the old invariant is incorrect and calls out that `Solid::try_new` does not validate non-empty boundaries at `.tendrion/phases/1-core-stabilization/1-3-PLAN.md:77-79`.
- It prescribes replacing `iter.next().unwrap()` with `ok_or_else(...)?` in both `and()` and `or()` paths at `.tendrion/phases/1-core-stabilization/1-3-PLAN.md:80-91`.
- Verification criteria require error return (not panic) for empty-boundary solids at `.tendrion/phases/1-core-stabilization/1-3-PLAN.md:200-202` and `.tendrion/phases/1-core-stabilization/1-3-PLAN.md:220`.
- Underlying source check confirms `Solid::try_new` iterates shells but accepts empty `boundaries` vectors (`for shell in &boundaries { ... }` then `Ok(Solid::new_unchecked(boundaries))`) at `monstertruck-topology/src/solid.rs:18-30`.

## Additional Observations (Non-Blocking)

- Meshing baseline arithmetic in the plan appears off by `3` (`25` claimed vs `22` production occurrences under the plan's own exclusions) because there are `8` rustdoc `unwrap()` lines (not `7`) and `2` `unwrap()` calls inside a `#[test]` function in `triangulation.rs`.
- This does not block the revised plan because Task 2's stated implementation target is stronger (`25 -> 0`), but count reporting should be corrected for audit clarity.

## Sibling Plans

| Plan | Wave | Objective |
|------|------|-----------|
| 1-1 | 1 | Replace all 9 unimplemented!() arms for Curve::IntersectionCurve in geometry.rs |
| 1-2 | 1 | Replace the deprecated proc-macro-error dependency in monstertruck-derive with proc-macro-error2 |
| 1-4 | 2 | Add criterion-based benchmarking infrastructure with CI integration |
