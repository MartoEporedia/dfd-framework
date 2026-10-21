<!-- dfd-language --> [Italiano](../../templates/review_release_light.md) | [English](review_release_light.md)

<a id="checklist-di-review-dfd--release-light"></a>

# DFD review checklist – Release (Light)

> **Review type**: release, small low-risk feature  
> **Date**: YYYY-MM-DD  
> **Participants**: [@team / @people]  
> **References**: [Feature specification], [Extended DoD], [Rollout configuration when releasing].

Use for changes outside payments, security, critical SLOs and core architecture: small UI/text changes, internal metrics and nonfunctional refactoring.

## 0. DoR – Release (Light)

- [ ] Main unit/integration tests pass.
- [ ] Nonfunctional tests unnecessary or already executed for similar cases.
- [ ] Simple direct/light-canary plan; no complex flag, or existing one reused.
- [ ] Support/stakeholders informed when needed.

<a id="1-dod-estesa-verifica-leggera"></a>

## 1. Extended DoD essentials

<a id="11-osservabilità"></a>

### 1.1 Observability

- [ ] No new critical endpoint, or base metrics already available.
- [ ] New logs follow service standards.
- [ ] Existing dashboards unaffected.

<a id="12-sicurezza"></a>

### 1.2 Security

- [ ] Access/authorization unchanged.
- [ ] No new sensitive data handled/logged.

<a id="13-affidabilità--slo"></a>

### 1.3 Reliability / SLOs

- [ ] No expected latency, error-rate or availability impact.
- [ ] No new critical external dependency.

<a id="14-costo"></a>

### 1.4 Cost

- [ ] No significant service cost impact.

<a id="2-guardrail-globali-verifica-rapida"></a>

## 2. Global guardrails

- [ ] Architecture respected.
- [ ] Security respected.
- [ ] Observability respected.
- [ ] Cost respected.

<a id="3-test-e-rilascio"></a>

## 3. Tests and validation

- [ ] New/changed logic covered by unit/integration tests that pass.
- [ ] Load/resilience tests unnecessary or existing coverage adequate.
- [ ] Production validation: direct rollout or light canary, according to team policy.

- [ ] Simple, clear rollback procedure.
- [ ] Relevant existing alarms/dashboards sufficient.

<a id="4-esito-della-review-di-release-light"></a>

## 4. Release review outcome (Light)

- [ ] Approved: rollout may proceed.
- [ ] Approved with conditions: [conditions].
- [ ] Changes requested: [reasons/gaps].

Approval: [@person – role].

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
