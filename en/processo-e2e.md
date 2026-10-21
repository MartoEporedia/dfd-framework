<!-- dfd-language --> [Italiano](../processo-e2e.md) | [English](processo-e2e.md)

<a id="processo-end-to-end--dod-first-driven-dfd"></a>

<a id="obiettivo"></a>

<a id="artefatti-del-setup"></a>

<a id="quando-aggiornare-il-setup"></a>

<a id="relazione-con-il-processo-end-to-end"></a>

# End-to-end process – DoD-First Driven (DFD)

> **Status**: draft / stable  
> **Last updated**: 2026-10-08  
> **Owners**: [@team / @person]  
> **References**: [Foundations](fondamenta.md), [Extended DoD](templates/dod-estesa.md), [Review index](templates/review_indice.md).

This process connects product work, DoD-driven design and TDD, controlled production rollout and learning.

<a id="0-fase-0--setup-dfd-di-dominio"></a>

## 0. Phase 0 – Domain DFD setup

Before feature work, establish a domain extended DoD, global guardrails, local templates/checklists and process conventions. The end-to-end process assumes this setup exists. If missing, complete Phase 0 first. The domain references the DoD as contract, guardrails as constraints and adapted templates/checklists as working artifacts.

The setup establishes how domain DoDs and feature specifications are written, which global constraints apply and how templates/checklists are used. Adapt the [DoD template](templates/dod-estesa.md) to Checkout, Auth, Reporting or the actual domain, identifying observability, security, SLO, cost, UX and any standard service criteria. Keep stable IDs such as OBS-01, SEC-01, SLO-01 and COST-01.

Domain guardrails specify architecture (tenant isolation/end-to-end traceability), security (no plaintext PII, minimum audit), shared SLOs and minimum metrics/logs. Choose unchanged or adapted templates explicitly; document domain-specific additions and review practices, ownership, artifact locations and how this setup evolves. The feature lifecycle assumes these shared artifacts already exist.

<a id="1-panoramica"></a>

## 1. Overview

Phase 0 is the domain prerequisite; eight feature phases follow: Idea/Epic, Design, Design review, TDD Development, Pre-release, Release review, Shift-right release and Post-release. Each phase specifies inputs, activities, outputs, exit criteria and a checkpoint/gate.

<a id="2-fasi-del-processo"></a>

## 2. Process phases

### 2.1 Idea / Epic

**Objective:** define the opportunity/problem and expected value.  
**Input:** business request, product insight or user feedback.  
**Activities:** define the epic/ticket, estimate impact/complexity and identify the primary domain/service.  
**Output:** epic with problem/opportunity, business KPIs and high-level estimate.  
**Exit:** Product/PM has approved the epic and the primary domain is identified.

- [ ] Epic ready for the product backlog.

<a id="22-design-dfd"></a>

### 2.2 DFD Design

**Objective:** turn the approved epic into a DoD-aligned feature specification. The specification itself is the design artifact; no separate “DFD Design” document is required.  
**Inputs:** approved epic, domain/service extended DoD and architecture guardrails.  
**Activities:** specify functional behavior, relevant OBS-xx, SEC-xx, SLO-xx, COST-xx and UX-xx criteria, guardrails and initial tests/validation.  
**Outputs:** draft specification and ADRs/architecture notes when decisions need recording.  
**Exit:** main fields complete, relevant criteria referenced and guardrails checked with no known violation.

- [ ] Specification ready for design review.

<a id="23-review-dfd-design"></a>

### 2.3 DFD Review (design)

**Objective:** validate consistency with DFD and the DoD.  
**Inputs:** draft specification, extended DoD and guardrails.  
**Activities:** relevant Tech Lead, Product, Dev, QA, Ops and Security review using the design checklist; identify gaps, architecture/security risks and questions.  
**Outputs:** approved, approved with conditions, or changes requested specification, plus actions/clarifications.  
**Exit:** at least approved with conditions; remaining actions clear and assigned.

- [ ] Development may proceed, with or without conditions.

<a id="24-sviluppo-tdd-dfd"></a>

### 2.4 Development (DFD TDD)

**Objective:** implement through DoD-driven TDD.  
**Inputs:** approved specification and extended DoD.  
**Activities:** design unit/integration/E2E tests and relevant load/resilience/security tests; red → green code → refactor; implement structured logs, necessary custom metrics and trace IDs.  
**Outputs:** code, test suite and observability instrumentation.  
**Exit:** applicable tests pass, with a final local suite in development-only and CI according to justified domain release policy; coverage meets team standards; logs/metrics align with DoD.

- [ ] Development complete in development-only; ready for pre-release only in release-preparation.

<a id="25-pre-release-dfd"></a>

### 2.5 DFD Pre-release

**Objective:** prepare shift-right production release.  
**Inputs:** ready code/tests, specification and DoD.  
**Activities:** define rollout phases, promotion/stop/rollback criteria, alarms and dashboards; configure flags and CloudWatch/equivalent; confirm relevant nonfunctional tests and DoD-aligned thresholds.  
**Outputs:** rollout configuration, configured alarms/dashboards or activation tickets, Support/stakeholder communication plan.  
**Exit:** complete rollout configuration, ready resources or a clear activation plan, stakeholders informed.

- [ ] Ready for release review.

<a id="26-review-dfd-release"></a>

### 2.6 DFD Review (release)

**Objective:** validate release preparation against DFD.  
**Inputs:** updated specification, rollout configuration and DoD.  
**Activities:** relevant Tech Lead, Dev, Ops, Product and Security review with release checklist; verify consistency of specification/tests/rollout, alarm/rollback thresholds and SLO/cost/UX impact.  
**Outputs:** release approved, approved with conditions, or changes requested; assigned remaining actions.  
**Exit:** at least approved with conditions and clear ownership of remaining actions.

- [ ] Production rollout may start.

<a id="27-rilascio-shift-right"></a>

### 2.7 Shift-right release

**Objective:** release through controlled canary/tenant exposure.  
**Inputs:** release review approval and rollout configuration.  
**Activities:** execute technical canary and tenant/segment phases as planned; actively monitor errors/latency, completion/business KPIs and relevant cost; promote, stop or partially/fully roll back.  
**Outputs:** completed/interrupted rollout, decision log and operational/incident notes.  
**Exit:** success or documented interruption with mitigation plan.

- [ ] Feature considered released, including a documented rollback outcome.

<a id="28-post-release-dfd"></a>

### 2.8 DFD Post-release

**Objective:** consolidate learning and update DoD/guardrails.  
**Inputs:** rollout outcome and post-release metrics/feedback.  
**Activities:** compare business/SLO/cost/UX outcomes; identify successes, friction, incidents and near misses; propose DoD, guardrail and process improvements.  
**Outputs:** internal note/post-mortem where appropriate and proposed document changes.  
**Exit:** learning documented/shared and improvement actions assigned.

- [ ] Feature closed from the DFD perspective.

## 3. Definition of Ready (DoR)

Minimum entry criteria apply at three key transitions: Idea→Design, Design→Development and Pre-release→Release.

<a id="31-dor-per-entrare-in-design-dfd"></a>

### 3.1 Entry to DFD Design

- [ ] Clear business objective in 1–2 sentences.
- [ ] At least high-level success KPIs, such as checkout conversion or support ticket reduction.
- [ ] Primary domain/service identified.
- [ ] High-level complexity estimate, such as T-shirt size.
- [ ] Product backlog priority assigned.
- [ ] Epic approved by Product Manager/Owner.

Otherwise keep refining the epic before design.

<a id="32-dor-per-entrare-in-sviluppo-tdd-dfd"></a>

### 3.2 Entry to Development (DFD TDD)

- [ ] Main specification fields complete: scenarios/stories, interfaces/APIs/events and external dependencies.
- [ ] Relevant OBS-xx, SEC-xx, SLO-xx, COST-xx and UX-xx criteria referenced.
- [ ] Guardrails checked with no known violation.
- [ ] Architecture, security and reliability risks identified.
- [ ] Tech Lead/Architect approval in principle, even informally before formal review.
- [ ] Design review at least approved with conditions, with no open architecture/security blocker.

Otherwise complete design before implementation.

<a id="33-dor-per-entrare-in-rilascio-shift-right"></a>

### 3.3 Entry to Shift-right Release

- [ ] Applicable unit/integration/E2E tests pass, with local/CI evidence required by domain release policy.
- [ ] Relevant load, resilience and security tests executed.
- [ ] Rollout phases, promotion/stop/rollback criteria, alarms and dashboards defined.
- [ ] Operational resources configured or clear activation plan available.
- [ ] Support and key stakeholders informed, including Ops/Security when relevant.
- [ ] Release review at least approved with conditions, with no open technical/security blocker.

Otherwise complete pre-release before rollout.

<a id="4-rappresentazione-sintetica-del-flusso"></a>

## 4. Flow overview

```text
Idea/Epic → DFD Design → Design Review → DFD TDD Development
→ Pre-release → Release Review → Shift-right Release → Post-release
```

<a id="5-eccezioni-e-semplificazioni"></a>

## 5. Exceptions and simplifications

Small low-risk features may use light design/review and direct rollout without canary when guardrails allow. Rules must be explicit (for example excluding payment, security and critical SLO changes) and approved by Tech Lead/Architect.

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
