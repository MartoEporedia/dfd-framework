---
name: dfd-assess
description: Classify feature, refactoring, incident follow-up or new-service risk using all five DFD dimensions and choose rapid, light or full.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-assess/SKILL.md) | [English](SKILL.md)

<a id="rischio-di-una-feature"></a>

# Feature risk

Read `.dfd/config.json`, domain assessment/DoD, `.dfd/kit/rischio.md`, `.dfd/kit/adozione.md` for brownfield and `.dfd/kit/templates/contratto_toolkit.md`. Check for an existing quick record before creating feature artifacts.

1. Identify explicit title/scope; use dfd-feature to open/resume. If absent run `dfd feature <id> --domain <domain> --title "..." --scope "..." --kind feature`. Kinds: feature, refactor, incident, new-service.
2. Assess security, reliability/SLOs, cost, business and architecture. In risk.json record security/reliability/cost/business/architecture, each with level and rationale. Use low/medium/high; retain null for missing information and explain what is needed.
3. Assess the actual change and relevant legacy gaps; do not inherit unrelated historical debt automatically. Link post-incident work to a preventive DoD criterion.
4. Run `dfd assess <id>`. Do not assign route/state manually. All Low or one Medium gives light; at least two Medium without High gives full proportionate; any High gives full complete. This ordering resolves overlapping prose rules. New-service always uses full complete while keeping risk separate.
5. Transaction-cost increase from 5% to below 10% has no explicit framework classification: clarify unless other sufficient facts/team policy resolve it. Unknown never means Low.

Report rationale/missing information/route. Record justified escalations; no automatic override. Proceed to dfd-specify when assessment is complete. Quick IDs use only `.dfd/changes/<id>.json` and assess, without separate risk/report files. Rapid requires all Low, bounded noncritical scope, unchanged contract and no questions. One Medium requires at least light; critical/unknown data cannot be treated as negligible.
