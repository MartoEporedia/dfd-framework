---
name: dfd-setup
description: Define or update domain DFD Phase 0: DoD, guardrails, local templates and process conventions before feature design.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-setup/SKILL.md) | [English](SKILL.md)

<a id="setup-dfd-di-dominio"></a>

# Domain DFD setup

Read `.dfd/kit/fondamenta.md`, `.dfd/kit/processo-e2e.md` and `.dfd/kit/templates/contratto_toolkit.md`. Phase 0 is the shared domain reference, comparable to an SDD constitution, and evolves with learning.

1. Use dfd-init for missing domains. For existing domains rerun init with the project's harness/mode to add missing setup files without overwriting local work.
2. Read existing policies/decisions and `.dfd/domains/<domain>/`. Refine dod.md/criteria.json iteratively, preserving IDs and separating new-work requirements from baseline gaps.
3. Complete guardrails.md: architecture/contracts, security, SLOs, observability and cost; link relevant common guardrails.
4. Adapt or retain templates and record differences in process.md alongside naming, risk, checklists, artifact placement and responsibilities.
5. Ask focused questions, never invent thresholds/decisions. Run `dfd setup --domain <domain>`. ready means structural completeness, not human approval or semantic quality.
6. Update after architectural changes, recurring patterns or incidents. Reconfirm affected reviews when relevant contracts, criteria, guardrails or lifecycle change. Descriptive templates/process prose alone do not invalidate new selective reviews; legacy reviews stay conservative.

Adoption assessment is separate: unrelated legacy debt does not automatically block setup. Once ready, continue risk/specification. Declare lifecycle.json: development-only with ci_required false, or release-preparation with justified CI policy. Read the contract for schema/legacy defaults. A final local suite avoids premature CI in development-only; pre-release remains disabled. Agree rapid/light/full and proportionate checks. Team policy is optional: no team.json means no independent/integrated checks required. Keep normative requirements in catalog/guardrails, not hidden in descriptive prose.
