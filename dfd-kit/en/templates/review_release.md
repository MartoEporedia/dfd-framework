<!-- dfd-language --> [Italiano](../../templates/review_release.md) | [English](review_release.md)

<a id="checklist-di-review-dfd--release"></a>

# DFD review checklist – Release

> **Review type**: release  
> **Date**: YYYY-MM-DD  
> **Participants**: [@team / @people]  
> **References**: [Feature specification], [Extended DoD], [Architecture Guardrails], [Process / DoR], [Rollout configuration].

<a id="0-verifica-definition-of-ready-dor--release"></a>

## 0. Definition of Ready – Release

- [ ] Applicable unit/integration/E2E tests pass according to domain local/CI policy.
- [ ] Relevant load/resilience/security tests executed.
- [ ] Rollout stages, promotion/stop/rollback, alarms/dashboards defined.
- [ ] Operational resources configured or activation plan clear.
- [ ] Support/stakeholders informed, including Ops/Security when relevant.
- [ ] Design review at least approved with conditions; no architecture/security blocker.

Otherwise complete pre-release first.

<a id="1-allineamento-alla-dod-estesa-verifica-implementazione"></a>

## 1. Extended DoD alignment

<a id="11-osservabilità"></a>

### 1.1 Observability

- [ ] Relevant OBS-xx criteria explicitly referenced implemented and verified.
- [ ] Requests/errors/latency for each new endpoint implemented and verified.
- [ ] Business custom metrics, when applicable, implemented and verified.
- [ ] Logs include tenantId, userId, operation, outcome, errorCode, traceId.
- [ ] Trace ID in logs/metrics implemented and verified.
- [ ] Relevant dashboards implemented and verified.

<a id="12-sicurezza"></a>

### 1.2 Security

- [ ] Relevant SEC-xx criteria implemented and verified.
- [ ] Access roles/policies implemented and verified.
- [ ] Data protection verified: no plaintext sensitive log data.
- [ ] Audit events implemented and verified.
- [ ] DoD standards/policies respected.

<a id="13-affidabilità--slo"></a>

### 1.3 Reliability / SLOs

- [ ] Relevant SLO-xx criteria implemented and verified.
- [ ] Latency/error-rate/availability impact assessed and within estimates.
- [ ] Dependency timeout/retry/fallback mitigations implemented and verified.
- [ ] New reliability risks and countermeasures identified.

<a id="14-costo"></a>

### 1.4 Cost

- [ ] Relevant COST-xx criteria implemented and verified.
- [ ] Monthly cost impact assessed and within estimates.
- [ ] Transaction cost impact assessed when applicable and within estimates.
- [ ] Cost alarm thresholds configured when relevant.

<a id="15-esperienza-utente--business-kpi"></a>

### 1.5 UX / business KPIs

- [ ] Affected business KPIs identified and monitorable in metrics/dashboards.
- [ ] Acceptable UX-02 deterioration thresholds clear and measurable.
- [ ] Production validation plan ready to execute.

<a id="2-guardrail-globali-verifica-finale"></a>

## 2. Global guardrails

- [ ] Architectural guardrails respected.
- [ ] Security guardrails respected.
- [ ] Observability guardrails respected.
- [ ] Cost guardrails respected.
- [ ] Exceptions explicit, justified and approved.

<a id="3-test-e-validazione-stato-attuale"></a>

## 3. Tests and validation

<a id="31-test-funzionali"></a>

### 3.1 Functional tests

- [ ] Unit tests for new/changed logic passing.
- [ ] Integration tests for new/changed APIs passing.
- [ ] E2E tests for main flows passing.

<a id="32-test-non-funzionali"></a>

### 3.2 Nonfunctional tests

- [ ] Relevant load tests executed; result: [OK / issues].
- [ ] Resilience tests for failures/timeouts/fallback executed; result: [OK / issues].
- [ ] Security tests for unauthorized access/malicious input executed; result: [OK / issues].

<a id="33-validazione-in-produzione-shift-right"></a>

### 3.3 Production validation (shift-right)

- [ ] Technical canary plan: metrics, thresholds and rollback ready to execute.
- [ ] Tenant/segment rollout plan ready to execute.

<a id="4-operatività-e-supporto"></a>

## 4. Operations and support

- [ ] Support/Customer Success informed.
- [ ] Operational alerts configured: recipients and thresholds.
- [ ] Rollback clear and tested at least through a walkthrough.
- [ ] Promotion/stop/rollback criteria clear and configured.

<a id="5-decisioni-e-aperture"></a>

## 5. Decisions and open questions

- [ ] Decisions: [list].
- [ ] Remaining questions/investigations: [list].

<a id="6-esito-della-review-di-release"></a>

## 6. Review outcome

- [ ] Release approved: production rollout may proceed.
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
