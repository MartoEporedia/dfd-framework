---
name: dfd-specify
description: Write or update an iterative DFD feature specification after risk assessment, linking goals, DoD criteria, tests and guardrails.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-specify/SKILL.md) | [English](SKILL.md)

<a id="specifica-guidata-dalla-dod"></a>

# DoD-driven specification

Read feature state/risk and domain dod.md, criteria.json, guardrails.md, process.md and templates. Use `.dfd/kit/templates/contratto_toolkit.md` for structured formats and `.dfd/kit/processo-e2e.md` only to clarify prerequisites.

1. Verify Phase 0 with `dfd setup --domain <domain>` and risk with `dfd status <id>`. Complete setup using dfd-setup; update risk.json/assess when risk changes.
2. Run `dfd specify <id>` to create/resume without overwriting; existing questions are returned. Use the domain template.
3. Iterate: record questions/assumptions/alternatives in design.json.open_questions; ask focused questions and update both artifacts. Continue independent sections, reuse existing answers, and never turn assumptions into facts. Remove questions only when clarified, preserving their resolution. Questions permit drafting but block approval.
4. Complete spec.md with context, objective, scope, scenarios, contracts, risk and tests. Light needs proportionate detail; full covers dependencies, failures, security, SLOs, cost and future release validation within guardrails.
5. Keep design.json coherent: owner, objective, metrics, epic reference, criteria applicability/exclusions, tests/guardrails. Use actual domain IDs, evaluate every changes criterion and justify exclusions. Unrelated baseline gaps stay in adoption planning.
6. Describe implementation and planned verification per applicable criterion. Plans are not executed tests. Do not invent epic approval, production metrics or CI results.
7. Run `dfd review-design <id>` for structural gaps; update both artifacts when design changes.

Present specification/questions for review. Completed documents do not approve the design.
