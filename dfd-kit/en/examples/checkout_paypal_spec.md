<!-- dfd-language --> [Italiano](../../examples/checkout_paypal_spec.md) | [English](checkout_paypal_spec.md)

<a id="specifica-di-feature--paypal-sul-checkout"></a>

# Feature specification – PayPal in Checkout

> **Status**: illustrative example, not approved  
> **Last updated**: 2026-10-08  
> **Domain**: Checkout  
> **References**: [Checkout DoD](checkout_dod.md), [Risk](../rischio.md), [Adoption](../adozione.md).

This example illustrates a pilot in an existing product. Checks are planned, not evidence of executed tests or production outcomes.

<a id="1-obiettivo-e-ambito"></a>

## 1. Objective and scope

Add PayPal while preserving other payment methods/existing checkout. Monitor completion, failures and payment abandonment. Scope includes payment initiation, confirmation, errors and provider timeouts; excludes wholesale legacy flow/SLO redesign. DFD review overlays the team's technical review.

<a id="2-classificazione-del-rischio"></a>

## 2. Risk classification

| Dimension | Level | Rationale |
|---|---|---|
| Security | High | New payment flows and authorized callbacks |
| Reliability / SLOs | High | New external checkout dependency |
| Cost | Medium | Estimate fees/operational cost |
| User / business | High | Critical flow and conversion KPIs |
| Architecture | High | Provider integration/confirmation |

Route: full complete. Cost estimates remain to be completed; they are not observed measurements.

<a id="3-scenari-e-contratti"></a>

## 3. Scenarios and contracts

- Initiation: user selects PayPal; associate payment session with order and present next action.
- Confirmation: verify notification authenticity/correlation before updating order state.
- Duplicates: repeated confirmation must not duplicate orders/charges.
- Failure/timeout: understandable error, consistent state and another payment choice.

Define actual provider contracts, states, idempotency, timeouts and retry policies. Retries must not duplicate financial effects.

<a id="4-criteri-dod-e-verifiche-previste"></a>

## 4. DoD criteria and planned verification

| Criterion | Change application | Planned verification |
|---|---|---|
| OBS-01 | Requests/errors/latency for new flow | Metric emission test |
| OBS-02 | Correlate order/operation/outcome/trace | Integration fields/redaction test |
| SEC-01 | Restrict payment operations | Authorization/invalid callback tests |
| SEC-02 | No sensitive log data | Success/failure log checks |
| SLO-02 | Evaluate domain latency target | Load/slow dependency tests |
| SLO-03 | Observe error regressions | Failure tests/canary plan |
| COST-02 | Monthly/transaction estimate | Documented volumes/assumptions |
| UX-02 | Completion versus baseline | Segmented rollout analysis |

The real specification must evaluate all other required domain criteria and justify exclusions. Unrelated legacy gaps stay in adoption planning.

<a id="5-review-e-rilascio-previsti"></a>

## 5. Planned review and release

Request QA, Security, Ops and Product input per RACI. Tech Lead records the design decision after clarifying contracts/risks/tests. Plan flag, gradual exposure, alarms and rollback aligned to domain DoD. Release requires a separate review.

Open questions: final provider/contracts, costs, alarm thresholds, operations responsibilities and epic approval. This example does not automatically pass a CLI gate.

<a id="modalità-dellesempio"></a>

## Example maturity mode

This is an illustrative production-bound checkout service using release-preparation with risk-appropriate CI and operational requirements. A local prototype may use development-only and a traced final local suite; prepare rollout/release review when distribution is planned, reassessing requirements first.

<a id="applicazione-individuale-o-in-team"></a>

## Solo or team application

Payment contracts retain light/full even with a solo developer. Responsibilities may coincide; PRs/independence depend on project policy. A later bounded unchanged-contract fix with all Low risk may use the [quick record](../templates/cambiamento_rapido.md) and a defect regression.
