<!-- dfd-language --> [Italiano](../fondamenta.md) | [English](fondamenta.md)

<a id="dod-first-driven-dfd--fondamenta"></a>

# DoD-First Driven (DFD) – Foundations

> **Status**: draft / stable  
> **Last updated**: 2026-10-08  
> **Owners**: [@team / @person]  
> **References**: [End-to-end process](processo-e2e.md), [Risk criteria](rischio.md), [RACI](raci.md), [Extended DoD](templates/dod-estesa.md), [Feature specification](templates/specifica_feature.md), [Rollout](templates/configurazione_rollout.md), [Checkout example](examples/checkout_dod.md), [PayPal example](examples/checkout_paypal_spec.md).

DFD makes the extended Definition of Done guide feature specifications (Spec-Driven Design), test design and execution (Test-Driven Development), and production release evaluation (shift-right, canaries and controlled rollout). This living document evolves through experiments, feedback and production data.

<a id="1-idea-centrale"></a>

## 1. Core idea

The Definition of Done is a **continuous contract**, used before, during and after release. It explicitly prioritizes observability (metrics, logs and traces), security (access, audit and data protection), reliability / SLOs, and cost per service, transaction or tenant. These connect what we build and its constraints, how we verify it, and how we validate production behavior without damaging the wider system. See the [process](processo-e2e.md).

<a id="2-principi"></a>

## 2. Principles

<a id="21-dod-come-contratto"></a>

### 2.1 DoD as a contract

Done criteria are explicit, referenceable and used throughout every phase. Each feature explains how it meets relevant criteria: functional behavior and tests, observability, security, reliability and sustainable cost.

<a id="22-specifica-prima-del-codice"></a>

### 2.2 Specification before code

Start with a clear specification of behavior, interfaces and constraints. Reference applicable DoD criteria explicitly and stay within product guardrails for architecture, security, observability and cost.

<a id="23-test-derivati-dalla-dod"></a>

### 2.3 Tests derived from the DoD

Every relevant criterion has at least one associated automated test. Tests verify compliance with the DoD as well as functional behavior. Criteria guide the test → code → refactor cycle.

<a id="24-validazione-continua-shift-right"></a>

### 2.4 Continuous validation (shift-right)

Use the DoD to evaluate production canaries, tenant rollout and experiments. Derive release alarms and metrics from criteria. Treat release as a controlled experiment.

<a id="25-guardrail-globali-spec-locali"></a>

### 2.5 Global guardrails, local specifications

The global architectural, security, observability and cost framework defines boundaries. Local specifications refer to it and must respect it. Document and justify exceptions explicitly.

### 2.6 Feedback loop

Production metrics, errors, cost and feedback improve the living DoD. Experiments also improve DFD itself.

<a id="3-artefatti-principali"></a>

## 3. Main artifacts

<a id="31-dod-estesa"></a>

### 3.1 Extended DoD

The central reference defines product/domain Done criteria: observability (including dashboards), security, reliability/SLOs, cost and additional concerns such as UX or compliance. Use the [template](templates/dod-estesa.md) and [Checkout example](examples/checkout_dod.md).

<a id="32-specifica-di-feature"></a>

### 3.2 Feature specification

For each significant feature or service, describe scenarios, use cases, architectural boundaries and dependencies. Reference criteria (OBS-xx, SEC-xx, SLO-xx, COST-xx, UX-xx) and justify any guardrail exceptions. See the [template](templates/specifica_feature.md) and [PayPal example](examples/checkout_paypal_spec.md).

<a id="33-suite-di-test"></a>

### 3.3 Test suite

Include unit, integration, security and observability tests, with a mapping to criteria. Add load/stress tests for relevant SLOs and synthetic/end-to-end tests for critical flows.

<a id="34-configurazione-di-rilascio"></a>

### 3.4 Release configuration

Define canary thresholds, duration and rollback; tenant/segment exposure through feature flags or AppConfig; alarms for errors, latency, security and cost; promotion and stop criteria. Use the [rollout template](templates/configurazione_rollout.md).

<a id="35-quadro-globale-guardrail"></a>

### 3.5 Global framework (guardrails)

Document architectural principles, global API/event/data contracts, shared SLOs and constraints, observability and security guidance. Local specifications explicitly refer to these documents.

<a id="36-setup-dfd-di-dominio"></a>

### 3.6 Domain DFD setup

Maintain a domain DoD (initial draft and evolution), domain guardrails, local template adaptations and process conventions. Setup evolves with the domain and production learning. See [Phase 0](processo-e2e.md#0-fase-0--setup-dfd-di-dominio).

<a id="4-processo-e-ruoli"></a>

## 4. Process and roles

<a id="41-processo-end-to-end"></a>

### 4.1 End-to-end process

The [process](processo-e2e.md) defines Idea/Epic, Design, Development, Pre-release, Release and Post-release, with design/release review gates and key Definitions of Ready.

<a id="42-ruoli-e-responsabilità"></a>

### 4.2 Roles and responsibilities

See [RACI](raci.md). Product owns objectives, KPIs and priorities; Tech Lead/Architect owns architectural consistency, guardrails and design/release approval; developers own specification, code, tests and instrumentation; QA/SDET owns nonfunctional tests and validation; Ops/SRE owns alarms, dashboards and rollout support; Security owns security review, audit and data protection; Support supplies user feedback and post-release reports.

<a id="5-dfd-full-vs-dfd-light"></a>

## 5. DFD full versus DFD light

Choose by [risk](rischio.md): low → light design and release checklists; medium → proportionate full; high → complete full with broader review. Use the [full design](templates/review_design.md), [light design](templates/review_design_light.md), [full release](templates/review_release.md) or [light release](templates/review_release_light.md) checklist.

<a id="6-cosa-dfd-non-è"></a>

## 6. What DFD is not

DFD evolves with its DoD and guardrails. Criteria need to be clear, referenceable and used; they need not all be executable or machine-readable. AI can accelerate work but is optional. DFD complements Domain-Driven Design, Agile and DevOps by connecting them through the DoD.

<a id="7-evoluzione-del-documento"></a>

## 7. Document evolution

Track significant changes in changelogs, ADRs or revision notes. Guild experiments, tools and pilots feed revisions; the latest version is the reference for articles, talks and internal initiatives. Propose changes through an issue/MR or the [risk exceptions process](rischio.md#6-eccezioni).

<a id="maturità-del-progetto-solo-sviluppo-e-preparazione-al-rilascio"></a>

## Project maturity: development only and release preparation

Maturity is independent of DFD adoption level and feature risk. Phase 0 declares mode, CI policy and justification.

| Mode | Development checks | Outcome |
|---|---|---|
| `development-only` | Light/full: design review, TDD and final local suite; rapid: relevant checks; optional CI | Development complete; no pre-release or distribution approval |
| `release-preparation` | Same checks; CI required by default; a local alternative needs risk/target justification and human review | Ready for pre-release, followed by a separate release review |

Missing CI does not block development-only. Applicable checks remain mandatory with command, date, result and real log; local tests are never labeled CI. The final suite, when required, follows green tests. Declared failed CI still blocks even when optional.

Before distribution explicitly switch to release preparation; reassess risk, criteria, CI policy and target-environment checks. Reconfirm invalidated reviews and update plans/evidence. Development-only does not weaken security/risk controls, and local completion does not replace human release approval. Local experiments can produce learning before production data exists.

<a id="percorsi-proporzionati-e-lavoro-individuale-o-in-team"></a>

## Proportionate routes and solo or team work

DFD supports solo developers and teams. Maturity, change risk and work organization are independent; a team is not a prerequisite.

| Route | Eligibility | Useful minimum |
|---|---|---|
| Rapid | Scoped known-contract restoration/editorial change, all Low, no changed contract/guardrail, critical behavior or open question | One record: problem, scope, owner, justified risk, relevant criteria and real checks |
| Light | Contained feature, ineligible fix or questions needing clarification, without full conditions | Brief specification, applicable criteria, relevant review and evidence |
| Full | Any High, at least two Medium or a new service | Risk-appropriate analysis, design and checks; operational preparation for distribution |

Rapid eligibility is explicit, never based on line count. One Medium excludes rapid; unknown risk blocks choice. Questions or contract changes need clarification/promotion. Critical behavior requires full complete. The maintainer can correct assessments with documented facts; gates have no silent override.

<a id="fix-rapidi-e-test-che-prevengono-danni"></a>

### Quick fixes and tests that prevent harm

A behavioral regression must reproduce the bug, fail for the expected requirement before the fix and pass afterwards. Compilation/environment failures are not red evidence. Editorial changes need relevant content/link/output checks without artificial tests. Match suites and mutation tests to actual risk.

Prioritize data loss, authorization, payments, compatibility, partial writes, retry and rollback. Rejected operations must preserve data; retries/duplicates must not duplicate effects. Critical changes leave rapid even when tiny. Target mutants at important invariants; test count/coverage alone does not establish protection. Every serious escaped production bug yields an effective unit/integration/E2E regression; systemic issues update DoD criteria. Local learning is useful too.

<a id="un-solo-record-e-riuso-delle-decisioni"></a>

### One record and reused decisions

Rapid uses the [quick change record](templates/cambiamento_rapido.md), without separate specification, plan or design report. A PR can reference it. The CLI record is `.dfd/changes/<id>.json`; logs/files support evidence. Reuse the approved design referenced by the known contract rather than reopening design for scoped restoration.

A solo developer can finish rapid without a PR, second reviewer or new design approval. `change-verified` is a technical result. If team policy requires independence, record the real ordinary PR review once with person, reference, revision and snapshot. Voluntarily recorded reviews still bind to the examined material.

<a id="collaborazione-facoltativa-e-integrazione"></a>

### Optional collaboration and integration

Version shared DoD, guardrails and policy; assign distinct IDs/owners. Issue/branch/PR/revision references are optional solo and required only by explicit team policy. Delegate reviewers per change. One applicable PR review can satisfy the DFD gate without duplicate approvals.

Keep feature artifacts separate. A CLI lock protects one local directory, not separate clones. Use Git and explicit conflict review; never settle decision/evidence conflicts automatically with last-write-wins. Reassess affected snapshots and evidence after merge.

Before distribution, team policy may require independence and checks of the current integrated revision. Passing isolated branches does not prove integration. Checks cover change files and consistent revision, command, result, dates and real logs. Hashes authenticate neither people nor commits; semantic review assesses origin, completeness and declared revision.

<a id="review-selettive-ed-evoluzione-del-setup"></a>

### Selective reviews and setup evolution

For new features, normative criterion contracts live in the structured catalog and constraints in guardrails/lifecycle. Fingerprints include specification, risk, assessed criteria and relevant operational constraints. Template fixes, descriptive conventions, added domains and unrelated baseline gaps alone do not invalidate design. Keep all operational requirements in catalog/guardrails rather than hiding them in excluded process prose. New changes criteria or relevant contract changes reopen design.

Review shared-rule changes and identify affected features/actions. Shared guardrails stay conservative and can affect the whole domain. The CLI cannot semantically distinguish a typo from a security change in normative files. Legacy features retain previous fingerprints; never migrate approvals automatically.

Rapid completes development, not distribution. To release, resume/promote a light/full feature referencing the quick change and reassess policy/integration before release review.
