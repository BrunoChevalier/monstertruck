You are a Tendrion plan reviewer. Review the plan file and produce a REVIEW.md.

## Instructions

Read the review protocol template:
- /home/ubuntu/.claude/plugins/cache/local-embedded/td/1.3.0/templates/codex-plan-review.md

Read the review context file:
- /home/ubuntu/claude_code/monstertruck/.tendrion/phases/7-integration-mode/7-1-review-context-planning.md

Read the plan under review:
- /home/ubuntu/claude_code/monstertruck/.tendrion/phases/7-integration-mode/7-1-PLAN.md

Read the roadmap for phase requirements:
- /home/ubuntu/claude_code/monstertruck/.tendrion/ROADMAP.md

If needed for cross-plan analysis, sibling plans are at:
- /home/ubuntu/claude_code/monstertruck/.tendrion/phases/7-integration-mode/7-2-PLAN.md

## Review Parameters

- plan_id: 7-1
- phase_number: 7
- round: 1
- max_rounds: 3
- review_type: planning
- plan_path: /home/ubuntu/claude_code/monstertruck/.tendrion/phases/7-integration-mode/7-1-PLAN.md

## Structural Validation

Run this command to validate plan structure:
```
node "/home/ubuntu/.claude/plugins/cache/local-embedded/td/1.3.0/bin/td-tools.cjs" verify plan-structure "/home/ubuntu/claude_code/monstertruck/.tendrion/phases/7-integration-mode/7-1-PLAN.md"
```

## Output

Write the review to:
/home/ubuntu/claude_code/monstertruck/.tendrion/phases/7-integration-mode/7-1-REVIEW-planning.md

Follow the review protocol template exactly. Include YAML frontmatter with verdict, findings organized by tier (Blockers, Suggestions, Nits), and a summary section.

Today's date: 2026-03-16
