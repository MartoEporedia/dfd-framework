---
name: dfd-plan
description: Derive a TDD development plan from the current approved DFD specification, mapping tasks, criteria, tests and instrumentation before implementation.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-plan/SKILL.md) | [English](SKILL.md)

<a id="piano-di-sviluppo-dfd"></a>

# DFD development plan

Read `.dfd/kit/templates/contratto_toolkit.md`, domain setup and feature artifacts. Run status: design must be approved or approved-with-conditions and current. Resolve conditions blocking start; never self-approve.

Run `dfd plan <id>` to create/resume plan.md, plan.json and evidence.json without overwriting. Refine criterion-derived tasks into executable steps with owner/implementation/verification. Cover every applicable criterion, without reintroducing exclusions. Align JSON/Markdown with functional/nonfunctional tests and risk-appropriate instrumentation.

Keep questions in plan.json.open_questions, clarify iteratively and preserve resolutions. If scope/criteria/design decisions change, update specification and reconfirm review first. After approval use `dfd plan <id> --refresh`, preserving tasks and regenerating affected evidence. Present tasks/conditions, then dfd-implement. A plan is not execution evidence.

Read lifecycle.json and plan the required final local/CI suite. Mode transitions invalidate review/plan/evidence. Rapid uses one record and no separate plan. PRs/independent review apply only under explicit policy.
