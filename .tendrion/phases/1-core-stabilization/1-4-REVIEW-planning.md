# Planning Review: Plan 1-4 (Round 2 of 2)

## Verdict
`CHANGES_REQUESTED`

## Blocker Status

1. B1 (`must_haves` claimed CI detection, but no CI task existed): **Not resolved**.
2. B2 (verification used `cargo bench`): **Resolved**.

## Findings (Ordered by Severity)

1. **High -- CI integration is still not wired to the repository's active CI system.**
   - Evidence: Plan modifies GitHub Actions path [`.tendrion/phases/1-core-stabilization/1-4-PLAN.md:15`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:15) and defines Task 3 around `.github/workflows/ci.yml` ([`1-4-PLAN.md:232`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:232), [`1-4-PLAN.md:236`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:236), [`1-4-PLAN.md:251`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:251)).
   - Evidence: Repository CI is currently GitLab-based at [`.gitlab-ci.yml:1`](/home/ubuntu/claude_code/monstertruck/.gitlab-ci.yml:1), with existing jobs already defined there (for example [`.gitlab-ci.yml:6`](/home/ubuntu/claude_code/monstertruck/.gitlab-ci.yml:6), [`.gitlab-ci.yml:67`](/home/ubuntu/claude_code/monstertruck/.gitlab-ci.yml:67)).
   - Impact: Even though a CI task now exists in the plan, it is targeted at a different CI platform and will not enforce benchmark checks in the current pipeline.
   - Required fix: Move CI integration to the active pipeline (`.gitlab-ci.yml` and/or existing `cargo make` CI tasks), then keep the benchmark compilation gate there.

2. **Medium -- Verification still omits the mandatory non-benchmark `cargo test` gate.**
   - Evidence: Verification only lists benchmark-targeted tests ([`1-4-PLAN.md:279`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:279), [`1-4-PLAN.md:280`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:280), [`1-4-PLAN.md:281`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:281)) plus clippy ([`1-4-PLAN.md:282`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:282)).
   - Evidence: Repo policy requires always running `cargo test` and `cargo clippy --all-targets -- -W warnings` before commit ([`AGENTS.md:96`](/home/ubuntu/claude_code/monstertruck/AGENTS.md:96)).
   - Impact: Regular unit/integration tests outside benchmark targets can be skipped.
   - Required fix: Add explicit `cargo test` verification (workspace-wide or clearly scoped package set) alongside existing benchmark checks.

## Notes

- `KnotVec` guidance issue from Round 1 is resolved; plan now consistently requires `KnotVector` ([`1-4-PLAN.md:98`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:98), [`1-4-PLAN.md:211`](/home/ubuntu/claude_code/monstertruck/.tendrion/phases/1-core-stabilization/1-4-PLAN.md:211)).
- `cargo bench` is no longer used as a verification command in tasks/global verification; this addresses the core B2 command conflict.
