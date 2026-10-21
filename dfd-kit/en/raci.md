<!-- dfd-language --> [Italiano](../raci.md) | [English](raci.md)

<a id="modello-raci--dod-first-driven-dfd"></a>

# RACI model – DoD-First Driven (DFD)

> **Status**: draft / stable  
> **Last updated**: YYYY-MM-DD  
> **Owners**: [@team / @person]  
> **References**: [Foundations](fondamenta.md), [Process](processo-e2e.md).

R = Responsible (does the work); A = Accountable (one final owner per activity); C = Consulted (input requested); I = Informed (kept aware).

Roles: PM Product Manager/Owner; TL Tech Lead/Architect; DEV Developer; QA QA/SDET; OPS Ops/SRE; SEC Security Engineer; SUP Support/Customer Success.

<a id="1-matrice-raci-per-fase"></a>

## 1. Responsibility matrix by phase

| Phase                        | Main activity                         | PM   | TL   | DEV  | QA   | OPS  | SEC  | SUP  |
|-----------------------------|---------------------------------------------|------|------|------|------|------|------|------|
| **Idea / Epic**             | Define epic and business goals       | A/R  | C    | I    | I    | I    | I    | I    |
|                             | Approve epic                           | A    | C    | I    | I    | I    | I    | I    |
| **DFD Design**              | Write feature specification              | C    | A/R  | R    | C    | C    | C    | I    |
|                             | Identify DoD criteria                 | C    | A/R  | R    | C    | C    | C    | I    |
| **DFD Review (design)**     | Review specification and checklist            | C    | A/R  | R    | R    | C    | C    | I    |
|                             | Design approval decision               | C    | A    | R    | R    | C    | C    | I    |
| **Development (DFD TDD)**      | Design tests                          | I    | C    | A/R  | R    | I    | C    | I    |
|                             | Implement code and tests               | I    | C    | A/R  | C    | I    | I    | I    |
|                             | Observability instrumentation                | I    | C    | A/R  | I    | C    | I    | I    |
| **DFD Pre-release**         | Define rollout configuration          | I    | A/R  | R    | C    | R    | I    | I    |
|                             | Configure alarms / dashboards          | I    | C    | R    | I    | A/R  | I    | I    |
|                             | Communicate to Support / stakeholders      | C    | C    | I    | I    | I    | I    | A/R  |
| **DFD Review (release)**    | Review rollout and checklist              | C    | A/R  | R    | R    | R    | C    | C    |
|                             | Release approval decision                   | C    | A    | R    | R    | R    | C    | I    |
| **Shift-right release**    | Execute rollout (stages, canary, flags)     | I    | C    | R    | I    | A/R  | I    | I    |
|                             | Monitor technical/business metrics   | I    | C    | R    | R    | A/R  | I    | I    |
|                             | Promotion / stop / rollback decisions      | C    | A/R  | R    | R    | R    | I    | I    |
| **DFD Post-release**        | Analyze outcomes and learning                | A/R  | R    | R    | R    | R    | I    | C    |
|                             | Propose DoD / guardrail updates      | C    | A/R  | R    | R    | R    | C    | I    |
|                             | Post-mortem / internal notes   | A/R  | R    | R    | R    | R    | I    | I    |

<a id="2-descrizione-ruoli"></a>

## 2. Role descriptions

### PM (Product Manager / Product Owner)

Define business goals/KPIs, approve epics/priorities and evaluate post-release UX/business impact.

### TL (Tech Lead / Architect)

Own specification architectural consistency, global guardrail compliance and final DFD design/release approval.

### DEV (Developer)

Write specifications with TL/PM, implement code/tests through TDD and instrument logs, metrics and traces.

### QA (QA / SDET)

Design nonfunctional load/resilience/security tests, support production validation criteria and actively participate in reviews.

### OPS (Ops / SRE)

Define DoD-aligned alarm thresholds, configure/maintain dashboards/alarms and manage operational production rollout.

### SEC (Security Engineer)

Review SEC-xx criteria, validate audit/data protection and support security incident response.

### SUP (Support / Customer Success)

Stay informed about user-facing changes, collect post-release feedback/reports and contribute to UX/business KPI impact analysis.

<a id="3-note-sulluso-del-raci"></a>

## 3. Using RACI

Each activity has one Accountable person. Small teams may combine roles (TL and DEV, for example) while keeping responsibilities distinct. Adapt the matrix without removing key accountability, especially for design, release and post-release.

<a id="maturità-del-progetto-solo-sviluppo-e-preparazione-al-rilascio"></a>

## Project maturity: development only and release preparation

Maturity is independent of DFD adoption level and feature risk. Phase 0 declares mode, CI policy and justification.

| Mode | Development checks | Outcome |
|---|---|---|
| `development-only` | Light/full: design review, TDD and final local suite; rapid: relevant checks; optional CI | Development complete; no pre-release or distribution approval |
| `release-preparation` | Same checks; CI required by default; a local alternative needs risk/target justification and human review | Ready for pre-release, followed by a separate release review |

Missing CI does not block development-only. Applicable checks remain mandatory with command, date, result and real log; local tests are never labeled CI. The final suite, when required, follows green tests. Declared failed CI still blocks even when optional.

Before distribution explicitly switch to release preparation; reassess risk, criteria, CI policy and target-environment checks. Reconfirm invalidated reviews and update plans/evidence. Development-only does not weaken security/risk controls, and local completion does not replace human release approval. Local experiments can produce learning before production data exists.

<a id="responsabilità-individuali-e-collaborazione-facoltativa"></a>

## Solo responsibilities and optional collaboration

Roles may belong to one person. Solo rapid work requires a record and relevant evidence, without mandatory PRs or a second reviewer. Light/full retain required human decisions without requiring an artificial team.

Assign per-change owners and delegated reviewers according to team policy. An ordinary PR review may satisfy the relevant DFD gate once, linked to PR, revision and examined snapshot. Policy may require reviewer independence and current integrated checks before distribution. The CLI checks declarations/hashes, not identities or commits.

Use distinct IDs, Git and explicit conflict resolution. Local locks do not coordinate clones. Never resolve decision/evidence conflicts automatically by last write. Shared-rule changes identify affected features and reviews to reconfirm. See [optional conventions](templates/collaborazione.md) and [selective reviews](fondamenta.md#review-selettive-ed-evoluzione-del-setup).
