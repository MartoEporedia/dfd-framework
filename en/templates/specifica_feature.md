<!-- dfd-language --> [Italiano](../../templates/specifica_feature.md) | [English](specifica_feature.md)

<a id="specifica-di-feature--nome-feature"></a>

# Feature specification – [Feature Name]

> **Status**: draft / in review / approved / implemented  
> **Created**: YYYY-MM-DD  
> **Owners**: [@team / @person]  
> **Service / Domain**: [Checkout, Payments, Catalog]  
> **References**: [Extended DoD], [Architecture Guardrails], [Related ADRs], [Ticket / Epic].

<a id="1-contesto-e-obiettivo"></a>

## 1. Context and objective

Context: [problem/opportunity/business request]. Objective: [user/business value]. Boundaries: [primary domain, involved Payment/Inventory/Notification services, explicit exclusions].

<a id="2-descrizione-funzionale"></a>

## 2. Functional description

<a id="21-scenari-principali"></a>

### 2.1 Main scenarios

- Scenario 1: [name].
  - Actors: [user/system].
  - Preconditions: [authenticated user, nonempty cart].
  - Flow: [steps 1–3].
  - Postconditions: [order created, notification sent].
- Scenario 2: [same structure].

<a id="22-casi-duso--user-story"></a>

### 2.2 Use cases / user stories

- As [user], I want [goal], so that [benefit].
- As [user], I want [goal], so that [benefit].

<a id="23-interfacce-e-contratti"></a>

### 2.3 Interfaces and contracts

API: `METHOD /path` – [description], request/response fields and error codes/meaning. Events: [name, producer, consumers, payload]. External dependencies: [Payment/Inventory/etc.].

<a id="3-allineamento-alla-dod-estesa"></a>

## 3. Extended DoD alignment

Evaluate relevant domain criteria; values below illustrate the Checkout/promotion example and must be adapted.

<a id="31-osservabilità"></a>

### 3.1 Observability

- [ ] OBS-01: requests/errors/latency for endpoints, e.g. ALB metrics for `POST /checkout/promo`.
- [ ] OBS-02: structured `tenantId`, `userId`, `operation`, `outcome`, `errorCode`, `traceId`; e.g. `PROMO_INVALID`, `PROMO_EXPIRED`.
- [ ] OBS-03: trace ID in logs/metrics, propagated via X-Ray to Payment Service.
- [ ] OBS-04: service/business dashboards, e.g. Promo Usage in checkout-business.
- [ ] Custom `PromoApplied` count with `tenantId`, `promoCode`, `outcome`.
- [ ] Custom `PromoFailed` count with `tenantId`, `errorCode`.

<a id="32-sicurezza"></a>

### 3.2 Security

- [ ] SEC-01: IAM controls for endpoints/ECS tasks, e.g. restrict Promo table access.
- [ ] SEC-02: no plaintext sensitive log data; no additional PII.
- [ ] SEC-03: order/payment audit, e.g. PromoApplied with userId, tenantId, timestamp.
- [ ] New audit events: `PromoApplied` (who, when, which promotion).

<a id="33-affidabilità--slo"></a>

### 3.3 Reliability / SLOs

- [ ] SLO-01: 99.9% monthly availability; [impact on existing ECS infrastructure].
- [ ] SLO-02: checkout p95 < 2.5 s; [for example promotion overhead < 100 ms].
- [ ] SLO-03: `POST /checkout` 5xx < 0.5%; expected 4xx PROMO_INVALID excluded.
- [ ] New risks: [Promo Service failure degrades checkout].
- [ ] Mitigation: [checkout without promotion, clear user error].

<a id="34-costo"></a>

### 3.4 Cost

- [ ] COST-01: tags service/team/env/costCenter; [new resources or code only].
- [ ] COST-02: significant-change estimate, e.g. +2% monthly CPU cost.
- [ ] COST-03: +15% monthly/+20% transaction alarm thresholds; [expected impact].
- Estimated monthly increase: [X EUR / %].
- Estimated transaction increase: [Y EUR / %].

<a id="35-esperienza-utente--business-kpi"></a>

### 3.5 UX / business KPIs

- [ ] UX-01: completion monitoring, including abandonment if promotion fails.
- [ ] UX-02: no > 5% completion drop; compare tenant rollout to 14-day baseline.

<a id="4-guardrail-globali"></a>

## 4. Global guardrails

- [ ] Architecture: [Promo Service cannot access order DB directly].
- [ ] Security: [no PII shared with unauthorized systems].
- [ ] Observability: [standard JSON logs].
- [ ] Cost: [no domain budget overrun without approval].
- [ ] Exceptions: [description, rationale, approval].

<a id="5-test-e-validazione"></a>

## 5. Tests and validation

<a id="51-test-funzionali"></a>

### 5.1 Functional tests

- [ ] Promotion logic unit tests.
- [ ] `POST /checkout/promo` integration tests.
- [ ] Cart → promotion → checkout E2E tests.

<a id="52-test-non-funzionali"></a>

### 5.2 Nonfunctional tests

- [ ] Load/latency (SLO-02).
- [ ] Resilience: Promo Service down → fallback.
- [ ] Security: unauthorized endpoint access.

<a id="53-validazione-in-produzione-shift-right"></a>

### 5.3 Production validation (shift-right)

- [ ] Canary: errors, latency, PromoApplied/PromoFailed; [rollback, e.g. 5xx > 1% for 5 min].
- [ ] Tenant rollout: UX-02 holds; [e.g. 10% → 50% → 100%].

<a id="6-piano-di-rilascio"></a>

## 6. Release plan

- [ ] Flag: [name/system, e.g. AppConfig].
- [ ] Stages: staging → 10% production tenants → 50% → 100%.
- [ ] Promotion: errors below threshold, no negative UX-02 impact.
- [ ] Rollback: error threshold or UX-02 deterioration.

<a id="7-aperture-e-decisioni-da-prendere"></a>

## 7. Open questions and decisions

- [ ] Decide whether Promo Service needs a circuit breaker.
- [ ] Evaluate a promotion duration metric.

<a id="8-riferimenti-e-allegati"></a>

## 8. References and attachments

[Wireframes/mockups], [technical documents], [ticket/epic].

<a id="9-storico-revisioni"></a>

## 9. Revision history

- YYYY-MM-DD: initial draft.
- YYYY-MM-DD: [change].

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
