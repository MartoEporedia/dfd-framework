<!-- dfd-language --> [Italiano](../../templates/specifica_feature.md) | [English](specifica_feature.md)

<a id="specifica-di-feature--nome-feature"></a>

# Feature specification – [Feature Name]

> **Status**: draft  
> **Created**: YYYY-MM-DD  
> **Owners**: TODO  
> **Service / Domain**: TODO  
> **Epic / Ticket**: TODO

<a id="1-contesto-obiettivo-e-ambito"></a>

## 1. Context, objective and scope

- Problem/expected value: TODO
- Success metrics: TODO
- Changed components: TODO
- Out of scope: TODO
- Brownfield processes to reuse and relevant baseline gaps: TODO

<a id="2-classificazione-del-rischio"></a>

## 2. Risk classification

- Assessment reference: TODO
- Risk and light/full route: TODO
- Rationale/missing facts: TODO

<a id="3-comportamenti-e-contratti"></a>

## 3. Behavior and contracts

- Main scenarios and acceptance criteria: TODO
- Errors, failure modes and degradation: TODO
- APIs, events, data and dependencies: TODO
- Compatibility/migration if needed: TODO

<a id="4-allineamento-alla-dod-del-dominio"></a>

## 4. Domain DoD alignment

Use actual domain IDs. Evaluate every changes criterion; justify exclusions. Link baseline gaps when they influence this change.

| DoD criterion | Applicability | Implementation or exclusion rationale | Planned verification |
|---|---|---|---|
| TODO | TODO | TODO | TODO |

<a id="5-guardrail-e-decisioni"></a>

## 5. Guardrails and decisions

- Architecture/security/observability/cost guardrails: TODO
- Justified exceptions, owners and decisions: TODO
- Existing policies/ADRs: TODO

<a id="6-test-e-validazione-prevista"></a>

## 6. Planned tests and validation

- Criterion-derived functional tests: TODO
- Risk-proportionate nonfunctional tests: TODO
- Manual checks/expected evidence: TODO
- Production validation, thresholds and rollback: TODO

<a id="7-domande-aperte-e-review"></a>

## 7. Open questions and review

- Blocking questions: TODO
- Conditions/owners: TODO
- Human review/decision reference: TODO

<a id="8-revisioni"></a>

## 8. Revisions

- YYYY-MM-DD: initial draft.

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
