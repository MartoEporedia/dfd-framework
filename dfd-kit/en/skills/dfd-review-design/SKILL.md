---
name: dfd-review-design
description: Review a DFD specification with the light/full checklist and prepare the responsible human decision, including solo or delegated reviewers.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-review-design/SKILL.md) | [English](SKILL.md)

<a id="review-di-design"></a>

# Design review

Read feature/domain artifacts: DoD, guardrails, conventions and local templates. Select review_design_light.md or review_design.md from `.dfd/domains/<domain>/templates/` according to state.json.route. Read `.dfd/kit/raci.md` and `.dfd/kit/templates/contratto_toolkit.md`.

1. Run `dfd review-design <id>`. design-review.md records structural checks/current gate; passing checks still require semantic review and a human decision.
2. Assess objectives, metrics, scope, risk, criteria and tests. Separate blockers, conditions and suggestions. For brownfield focus on in-scope changes and relevant legacy gaps, overlaying team reviews.
3. Save semantic findings in review-notes.md separately from regenerable reports. Do not record your report as a human decision or approve from a completed checklist.
4. Present the proposed outcome. Record approved, approved-with-conditions or changes-requested only after the responsible human expresses it. Reuse a relevant decision already expressed in the conversation without asking again.
5. Run `dfd decide <id> --decision approved --reviewer "..." --role tech-lead --note "..." --human-confirmed`. For conditions use approved-with-conditions and repeated `--condition "text|owner|stage"`. Declared reviewer/role are not authenticated identities; never self-approve.
6. Rerun status. Specification/risk/relevant criteria (catalog or dod.md text)/guardrail changes invalidate reviews. New features use selective fingerprints; legacy features stay conservative. With stale-review read changes (components changed, added or removed since the last decision) and review those items; changes null means a decision recorded by an earlier CLI. Preserve history and reconfirm relevant decisions only.

With current approved design continue dfd-plan/TDD. Design approval does not authorize deployment. Report outcome, conditions and next step. Solo rapid records use verify without a separate design/new approval; independent review applies only if configured. Verify problem/scope/tests in the record; reuse an actual applicable PR decision once, never invent one.
