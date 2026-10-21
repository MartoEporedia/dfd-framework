<!-- dfd-language --> [Italiano](../adozione.md) | [English](adozione.md)

<a id="guida-allintegrazione-di-dfd-su-prodotti-esistenti"></a>

<a id="adozione-proporzionata-individuale-o-in-team"></a>

# Introducing DFD into existing products

> **Status**: draft / stable  
> **Last updated**: 2026-10-08  
> **Owners**: [@team / @person]  
> **References**: [Foundations](fondamenta.md), [Process](processo-e2e.md), [Risk](rischio.md), [RACI](raci.md).

Introduce DoD-First Driven gradually and sustainably into an existing product/domain.

<a id="1-principi-per-lintegrazione-su-esistente"></a>

## 1. Principles for existing products

Existing products already have code, tests, deployment, monitoring, processes and established habits. Adopt DFD by layers and areas through suitable pilots. New features/services use full or light immediately; existing components improve incrementally through changes, refactoring and incident learning. Rewriting everything or imposing complete adoption at once is unnecessary.

<a id="2-fase-0--assessment-iniziale-dove-siete-ora"></a>

## 2. Phase 0 – Initial assessment

<a id="21-cosa-valutare"></a>

### 2.1 What to assess

Capture an evidence-based snapshot per service/domain:

1. **Current process:** specifications, unit/integration/E2E tests, CI/CD, canaries and tenant rollout.
2. **Observability:** requests/errors/latency metrics, structured logs and tracing.
3. **Security:** explicit access, audit, sensitive-data criteria and applicable standards/policies.
4. **Reliability/SLOs:** availability, latency, error-rate SLOs and matching alarms.
5. **Cost:** service/transaction estimates or measurements, budgets and thresholds.
6. **Culture/process:** willingness to add specifications/reviews/checklists and existing TDD, review and post-mortem practices.

| Service | Process | Observability | Security | SLOs | Cost | Culture | Notes |
|---|---|---|---|---|---|---|---|
| Checkout | ... | ... | ... | ... | ... | ... | ... |

<a id="22-identificare-candidati-dfd"></a>

### 2.2 Identify DFD candidates

Choose 1–2 technically mature, willing pilot domains where improvements (fewer incidents, better SLOs) can be observed. Candidate work includes medium/high-risk features, significant refactoring and incident follow-up that improves the DoD.

<a id="3-fase-1--pilota-dfd-su-nuovo--modifiche-significative"></a>

## 3. Phase 1 – Pilot on new or significantly changed work

<a id="31-regola-pratica"></a>

### 3.1 Practical rule

New/significantly changed work in pilot services follows DFD: medium/high-risk features and SLO/security refactoring use full; low-risk features use light. Other work continues, while the extended DoD starts guiding additional alarms, instrumentation and tests.

<a id="32-cosa-introdurre-subito"></a>

### 3.2 Introduce immediately

1. A usable domain DoD draft covering observability, security, SLOs and at least tagging/rough cost estimates. Adapt the template to observed reality.
2. Feature specifications for medium/high-risk features and architectural changes.
3. Design/release checklists as an overlay on existing reviews: assess DoD alignment, guardrails and production validation.
4. Shift-right rollout for new features: canary/tenant percentages, DoD/SLO alarms and explicit rollback criteria.

<a id="33-cosa-non-fare-in-questa-fase"></a>

### 3.3 Keep the pilot focused

Focus on 1–2 services and 3–5 significant features. Demonstrate fewer regressions, clearer responsibilities and improved observability/incident handling. Avoid rewriting historical specifications or requiring a perfect DoD across every service.

<a id="4-fase-2--estensione-graduale-allesistente"></a>

## 4. Phase 2 – Gradual extension

<a id="41-per-servizio--dominio"></a>

### 4.1 Per service / domain

| Level | Adoption |
|---|---|
| 0 – None | No formal extended DoD, informal/missing specifications, existing review/rollout practices |
| 1 – DoD and new features | At least a DoD draft, DFD specifications/reviews for medium/high-risk new features, shift-right rollout for new features |
| 2 – Systematic review | Stable DoD in use, at least light for all features, design/release checklists, DoD-aligned alarms/dashboards |
| 3 – Mature | DoD evolves from production feedback, clear respected guardrails, measured incident/MTTR improvements, integration into CI/tickets/repository |

Record the current level, a 3–6 month target and concrete steps to reach it.

<a id="42-interventi-sullesistente"></a>

### 4.2 Existing components

When modifying an endpoint/flow, check DoD-aligned metrics/logs and add missing instrumentation in scope; update the DoD if necessary. After incidents, ask which criterion could have prevented/reduced impact, then add the criterion and derived alarms/tests. Significant architectural refactoring needs a specification of the target design, review and controlled rollout. New services begin with full DFD, a DoD before code and specifications for initial features.

<a id="5-fase-3--istituzionalizzazione-dfd-come-modo-di-lavorare"></a>

## 5. Phase 3 – DFD as the working practice

<a id="51-integrazione-con-processi-esistenti"></a>

### 5.1 Existing processes

Tickets/epics carry risk and specification/review links. For medium/high risk, CI/CD can check specification, completed checklist and relevant rollout configuration. Keep a versioned DFD folder per service containing DoD, feature specifications, rollout configuration and review notes.

<a id="52-onboarding-e-formazione"></a>

### 5.2 Onboarding and training

Introduce newcomers through foundations, process and examples; optionally use AI skills/context packs or tools such as SpecKit. Guild/community sessions share use cases, incident learning and framework improvements.

<a id="53-metriche-di-efficacia-dfd"></a>

### 5.3 Effectiveness metrics

Measure incidents, regressions, MTTR, developer onboarding time, review clarity and rollout quality (rollback/hotfix frequency). Use these observations to justify and improve DFD.

<a id="6-regole-pratiche-per-non-fallire"></a>

## 6. Practical adoption rules

Start small; choose light for low risk; expand as teams observe value. Treat DFD as an internal product: collect feedback, measure usefulness and adapt.

<a id="7-schema-riassuntivo-di-integrazione"></a>

## 7. Adoption overview

| Concern | Greenfield | Brownfield |
|---|---|---|
| Extended DoD | Defined from the start | Gradual, beginning with pilots |
| Specification | Full/light from the start | Medium/high-risk features or significant refactoring initially |
| Review | Full/light from the start | Overlay first, then systematic |
| Rollout | Shift-right from the start | New features first, then extend |
| Existing code | N/A | Improve metrics/logs/tests when touched |
| Incident follow-up | N/A | Enrich DoD and alarms |

<a id="8-prossimi-passi-concreti"></a>

## 8. Concrete next steps

Assess main services; select 1–2 pilots and 3–5 features; draft a useful DoD for each; overlay full/light checklists on existing reviews; collect feedback after 1–2 months and adapt. Use [foundations](fondamenta.md), [process](processo-e2e.md), [risk](rischio.md) and [RACI](raci.md) for support.

## Proportionate adoption, solo or in a team

Organization, maturity and risk are independent. Starting DFD does not itself require CI, PRs or additional reviewers. Agree rapid/light/full routes in setup. A scoped, entirely low-risk fix uses [one record](templates/cambiamento_rapido.md); questions or greater impact require light/full. Solo is the CLI default; no team.json is required. Enable [team policy](templates/collaborazione.md) explicitly when useful.

Keep normative contracts in criteria/guardrails. New selective reviews reopen only designs affected by relevant changes; descriptive templates and unrelated baseline gaps alone do not require approval again. Legacy features retain conservative behavior. See [foundations](fondamenta.md#percorsi-proporzionati-e-lavoro-individuale-o-in-team).

<a id="maturità-del-progetto-solo-sviluppo-e-preparazione-al-rilascio"></a>

## Project maturity: development only and release preparation

Maturity is independent of DFD adoption level and feature risk. Phase 0 declares mode, CI policy and justification.

| Mode | Development checks | Outcome |
|---|---|---|
| `development-only` | Light/full: design review, TDD and final local suite; rapid: relevant checks; optional CI | Development complete; no pre-release or distribution approval |
| `release-preparation` | Same checks; CI required by default; a local alternative needs risk/target justification and human review | Ready for pre-release, followed by a separate release review |

Missing CI does not block development-only. Applicable checks remain mandatory with command, date, result and real log; local tests are never labeled CI. The final suite, when required, follows green tests. Declared failed CI still blocks even when optional.

Before distribution explicitly switch to release preparation; reassess risk, criteria, CI policy and target-environment checks. Reconfirm invalidated reviews and update plans/evidence. Development-only does not weaken security/risk controls, and local completion does not replace human release approval. Local experiments can produce learning before production data exists.
