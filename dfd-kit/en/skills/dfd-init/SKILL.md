---
name: dfd-init
description: Introduce DFD in a new or existing repository, choose pilot domains and build adoption assessment, DoD and guardrails. Use for onboarding, not individual feature risk classification.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-init/SKILL.md) | [English](SKILL.md)

<a id="adozione-di-dfd"></a>

<a id="percorso"></a>

# Adopting DFD

Resources live in `.dfd/kit/` in the adopting repository; before installation use the corresponding toolkit-root files. Read `.dfd/kit/adozione.md` for existing products and `.dfd/kit/fondamenta.md` for DoD definition.

1. Respect repository instructions. Identify harness, domain and scope; choose brownfield for existing code/processes. Reuse existing policy, tests, CI, monitoring and review, citing sources actually read.
2. Run `dfd init --harness <adapter> --domain <domain> --mode brownfield`; for another repository use `dfd --project <repo> init ...`. Use the supplied binary path if outside PATH. Adapters: claude-code, opencode, copilot, codex. Select `--lang en` for an English project; existing preferences are preserved unless explicitly changed.
3. Complete `.dfd/domains/<domain>/adoption.json`: process, observability, security, reliability, cost and culture. Status is observed/gap/unknown; evidence references real sources. Explain current/target levels in assessment.md.
4. For brownfield choose 1–2 pilot domains and 3–5 changes, keeping baseline_gaps/next_steps separate. Creating files does not demonstrate target adoption.
5. Adapt dod.md and criteria.json. Each criterion has id, statement and scope: changes for new work, baseline for existing-system improvements. Preserve OBS-01, SEC-01, SLO-01, COST-01, UX-01 and existing IDs. Clarify unknown SLOs/thresholds rather than inventing them.
6. Complete Phase 0 with dfd-setup: domain guardrails link relevant `.dfd/guardrails.md` and existing policies; process.md and local templates record conventions/adaptations. Distinguish observations, proposals and missing information; ask only what blocks a useful pilot.

Use `.dfd/kit/templates/contratto_toolkit.md` for formats/commands. Initialization is repeatable and preserves artifacts. Reviews overlay team reviews. Report pilot domain, gaps, actual `dfd setup --domain <domain>` result and first feature for dfd-assess. Agree maturity: development-only while CI/distribution are premature, release-preparation when distributing. Declare justified lifecycle.json with dfd-setup; init never silently changes existing policy.
