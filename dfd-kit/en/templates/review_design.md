<!-- dfd-language --> [Italiano](../../templates/review_design.md) | [English](review_design.md)

<a id="checklist-di-review-dfd--design"></a>

# DFD review checklist – Design

> **Review type**: design  
> **Date**: YYYY-MM-DD  
> **Participants**: [@team / @people]  
> **References**: [Feature specification], [Extended DoD], [Architecture Guardrails], [Process / DoR].

<a id="0-verifica-definition-of-ready-dor--design"></a>

## 0. Definition of Ready – Design

- [ ] Clear business objective (1–2 sentences).
- [ ] At least high-level KPIs (conversion/support tickets, for example).
- [ ] Primary domain/service identified.
- [ ] High-level complexity estimate.
- [ ] Product backlog priority defined.
- [ ] Epic approved by Product Manager/Owner.

Otherwise complete the epic first.

<a id="1-allineamento-alla-dod-estesa"></a>

## 1. Extended DoD alignment

<a id="11-osservabilità"></a>

### 1.1 Observability

- [ ] Relevant OBS-xx criteria explicitly referenced defined in the specification.
- [ ] Requests/errors/latency for each new endpoint defined in the specification.
- [ ] Business custom metrics, when applicable, defined in the specification.
- [ ] Logs include tenantId, userId, operation, outcome, errorCode, traceId.
- [ ] Trace ID in logs/metrics defined in the specification.
- [ ] Relevant dashboards defined in the specification.

<a id="12-sicurezza"></a>

### 1.2 Security

- [ ] Relevant SEC-xx criteria defined in the specification.
- [ ] Access roles/policies defined in the specification.
- [ ] Data protection verified: no plaintext sensitive log data.
- [ ] Audit events defined in the specification.
- [ ] DoD standards/policies respected.

<a id="13-affidabilità--slo"></a>

### 1.3 Reliability / SLOs

- [ ] Relevant SLO-xx criteria defined in the specification.
- [ ] Latency/error-rate/availability impact assessed for the proposed design.
- [ ] Dependency timeout/retry/fallback mitigations defined in the specification.
- [ ] New reliability risks and countermeasures identified.

<a id="14-costo"></a>

### 1.4 Cost

- [ ] Relevant COST-xx criteria defined in the specification.
- [ ] Monthly cost impact assessed for the proposed design.
- [ ] Transaction cost impact assessed when applicable for the proposed design.
- [ ] COST-03 thresholds checked against the estimate.

<a id="15-esperienza-utente--business-kpi"></a>

### 1.5 UX / business KPIs

- [ ] Affected business KPIs identified for the design.
- [ ] Acceptable UX-02 deterioration thresholds clear and measurable.
- [ ] Production validation plan outlined.

<a id="2-guardrail-globali"></a>

## 2. Global guardrails

- [ ] Architectural guardrails respected.
- [ ] Security guardrails respected.
- [ ] Observability guardrails respected.
- [ ] Cost guardrails respected.
- [ ] Exceptions explicit, justified and approved.

<a id="3-test-e-validazione"></a>

## 3. Tests and validation

<a id="31-test-funzionali"></a>

### 3.1 Functional tests

- [ ] Unit tests for new/changed logic planned.
- [ ] Integration tests for new/changed APIs planned.
- [ ] E2E tests for main flows planned.

<a id="32-test-non-funzionali"></a>

### 3.2 Nonfunctional tests

- [ ] Relevant load tests planned.
- [ ] Resilience tests for failures/timeouts/fallback planned.
- [ ] Security tests for unauthorized access/malicious input planned.

<a id="33-validazione-in-produzione-shift-right"></a>

### 3.3 Production validation (shift-right)

- [ ] Technical canary plan: metrics, thresholds and rollback outlined.
- [ ] Tenant/segment rollout plan outlined.

<a id="4-operatività-e-supporto"></a>

## 4. Operations and support

- [ ] Teams to inform before release identified (Support, Ops, Security).
- [ ] Main operational alerts to configure at pre-release identified.

<a id="5-decisioni-e-aperture"></a>

## 5. Decisions and open questions

- [ ] Decisions: [list].
- [ ] Remaining questions/investigations: [list].

<a id="6-esito-della-review-di-design"></a>

## 6. Review outcome

- [ ] Design approved: development may proceed.
- [ ] Approved with conditions: [conditions and owners].
- [ ] Changes requested: [reasons/gaps].

Approvals: [@person1 – role], [@person2 – role].

<a id="maturità-e-policy-delle-verifiche"></a>

## Maturity and verification policy

- Domain mode: development-only / release-preparation.
- CI required: yes / no; rationale based on risk and target.
- Final suite: command, environment, date, result and reference to the real log.
- Questions/requirements deferred to pre-release: owner and activation condition.

In development-only, finish design and proportionate checks without prematurely requiring CI, rollout, on-call or release communications. Release checklists apply when preparing distribution. Before release reassess risk/policy, reconfirm reviews and refresh evidence. Deferred work must not be marked executed.

<a id="proporzionalità-e-collaborazione"></a>

## Proportionality and collaboration

First consider rapid for scoped noncritical fixes, using one [quick change record](cambiamento_rapido.md) rather than completing this entire template. Match light/full detail to risk. Owner and checks are required; PRs, independence and integrated checks depend on [optional team conventions](collaborazione.md). Solo developers do not simulate roles or communications. Reuse applicable reviews; test failure effects and important regressions.
