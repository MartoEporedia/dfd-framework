<!-- dfd-language --> [Italiano](../../templates/configurazione_rollout.md) | [English](configurazione_rollout.md)

<a id="configurazione-rollout--nome-feature"></a>

# Rollout configuration – [Feature Name]

> **Status**: draft / in review / active / completed  
> **Created**: YYYY-MM-DD  
> **Owners**: [@team / @person]  
> **Service / Domain**: [Checkout]  
> **References**: [Feature specification], [Extended DoD], [Architecture Guardrails].

<a id="1-panoramica"></a>

## 1. Overview

Feature: [for example new PayPal payment method]. Environment: [staging / production].

- [ ] Technical canary (traffic fraction).
- [x] Tenant/segment rollout (feature flag).
- [ ] Combined canary and tenant rollout.

<a id="2-feature-flag--configurazione"></a>

## 2. Feature flag / configuration

System: [AWS AppConfig, LaunchDarkly, internal flag]. Example:

```yaml
flag:
  name: checkout.paypal.enabled
  type: boolean
  scope: tenant
  default: false
tenants:
  - id: internal
    enabled: true
    percentage: 100
  - id: market-de
    enabled: true
    percentage: 100
  - id: market-it
    enabled: true
    percentage: 100
  - id: market-fr
    enabled: false
    percentage: 0
```

<a id="3-fasi-di-rollout"></a>

## 3. Rollout stages

| Stage | Description | Tenant/segment | Traffic | Minimum duration | Promotion |
|---|---|---|---|---|---|
| 1 | Staging | Synthetic tenants | 100% | 24h | No critical errors; E2E passing |
| 2 | Production, internal | internal | 5% | 48h | Section 4 |
| 3 | Production, Germany | market-de | 20% | 72h | Section 4 |
| 4 | Production, Italy | market-it | 50% | 72h | Section 4 |
| 5 | Production, enabled tenants | All enabled=true | 100% | – | Section 4 |

These are illustrative values. For structured CLI rollout stages, describe final-environment exposure; staging is separate evidence.

<a id="4-criteri-di-promozione--stop--rollback"></a>

## 4. Promotion / stop / rollback criteria

<a id="41-criteri-tecnici-canary"></a>

### 4.1 Technical criteria (canary)

Monitor CloudWatch/equivalent: `POST /checkout` 5xx < 1% for 5 min; paymentMethod=PAYPAL p95 < 3 s for 10 min; PayPal CheckoutFailed < 10%, including expected failures.

- [ ] On threshold breach stop promotion.
- [ ] At advanced exposure consider partial rollback by disabling affected tenant flags.

<a id="42-criteri-business--ux"></a>

### 4.2 Business / UX criteria

Completion globally and per paymentMethod: compare to the prior 14 days without PayPal; no > 5% decline. Support reports: no significant PayPal-related rise (> X% versus average).

- [ ] Stop rollout if completion drops > 5%.
- [ ] Analyze by tenant/paymentMethod.
- [ ] Roll back affected flags if necessary.

<a id="43-criteri-di-costo"></a>

### 4.3 Cost criteria

Monthly service cost variation < 15% on a 7-day window (COST-03); estimated transaction cost variation < 20%.

- [ ] Stop rollout on breach.
- [ ] Open architectural/economic review.

<a id="5-allarmi-e-dashboard"></a>

## 5. Alarms and dashboards

- [ ] Checkout service: requests/errors/latency.
- [ ] Checkout business: CheckoutCompleted/CheckoutFailed by paymentMethod.
- [ ] Checkout cost: monthly/transaction cost.
- [ ] Checkout-5xx-ErrorRate: 1% for 5 min.
- [ ] Checkout-PayPal-Latency-p95: 3 s for 10 min.
- [ ] Checkout-CompletionRate-Drop: > 5% versus baseline.
- [ ] Checkout-Cost-Monthly-Increase: > 15% versus 30-day mean.

<a id="6-piano-di-rollback"></a>

## 6. Rollback plan

Triggers: 5xx > 1% for 5 min; completion deterioration > 5%; COST-03 breach.

1. Disable checkout.paypal.enabled for affected/all tenants.
2. Verify errors return to normal and completion stabilizes.
3. Document incident/post-mortem/internal note.
4. Revise rollout plan when necessary.

<a id="7-responsabilità-e-comunicazione"></a>

## 7. Communication and support

- [ ] Inform Support/Customer Success, Ops/SRE and relevant stakeholders before rollout.
- [ ] Record operational contacts/on-call and escalation channels.
- [ ] Communicate rollout status, pauses and rollback decisions.
- [ ] Update [dashboard / status page].

<a id="8-storico-esecuzioni"></a>

## 8. Execution history

| Date | Stage | Outcome | Notes |
|---|---|---|---|
| YYYY-MM-DD | 1 | completed | No critical errors |
| YYYY-MM-DD | 2 | completed | Slight latency increase within threshold |
| YYYY-MM-DD | 3 | in progress | Monitoring ongoing |

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
