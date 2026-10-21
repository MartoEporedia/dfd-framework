<!-- dfd-language --> [Italiano](../rischio.md) | [English](rischio.md)

<a id="criteri-di-rischio-feature--dod-first-driven-dfd"></a>

<a id="classificazione-del-rischio-dfd"></a>

# Feature risk criteria – DoD-First Driven (DFD)

> **Status**: draft / stable  
> **Last updated**: YYYY-MM-DD  
> **Owners**: [@team / @person]  
> **References**: [Process](processo-e2e.md), [full design](templates/review_design.md), [full release](templates/review_release.md), [light design](templates/review_design_light.md), [light release](templates/review_release_light.md).

<a id="1-obiettivo"></a>

## 1. Objective

Match process rigor to feature risk while protecting security, reliability/SLOs, cost, users and business. Classification selects full or light design and release checklists.

<a id="2-dimensioni-di-rischio"></a>

## 2. Risk dimensions

Assess **all five dimensions** as Low, Medium or High: security, reliability/SLOs, cost, user/business impact and architectural complexity. Document the facts supporting each assessment.

<a id="21-sicurezza"></a>

### 2.1 Security

- **Low:** no changes to access/authorization, sensitive data (PII/payment data), audit/security logging, encryption, secrets or IAM; no new externally exposed endpoint.
- **Medium:** small changes to security logging, such as new audit events; additional cases within the existing authorization model; new internal endpoint with no direct external exposure.
- **High:** new access/authorization controls; new sensitive data handled or logged; encryption, secrets or critical IAM policy changes; new public endpoint handling sensitive data.

<a id="22-affidabilità--slo"></a>

### 2.2 Reliability / SLOs

- **Low:** no expected latency, error-rate or availability impact; no new critical external dependency; UI, text or noncritical internal logic changes.
- **Medium:** slight latency/error impact within comfortable SLO margins and acceptable fallback/degradation; new noncritical dependency, such as an established internal service.
- **High:** significant latency (approaching/exceeding SLO), error-rate or availability impact; new critical dependency such as a payment provider/core service; changes to retry, timeout, circuit breaker or failure handling for critical dependencies.

<a id="23-costo"></a>

### 2.3 Cost

- **Low:** no significant service cost impact or new intensive CPU, memory, I/O or storage usage.
- **Medium:** estimated increase < 5% monthly service cost or < 5% per transaction; moderate new usage such as additional custom metrics/logs.
- **High:** estimated increase ≥ 5% monthly service cost or ≥ 10% per transaction; intensive new workloads, heavy batches, data pipelines or substantial storage.

<a id="24-impatto-su-utenti--business"></a>

### 2.4 User / business impact

- **Low:** invisible internal refactoring/logging/metrics; small UI/text changes outside critical flows; no main conversion, churn or revenue KPI impact.
- **Medium:** visible changes confined to noncritical areas with limited KPI impact; optional feature for a subset of users, not enabled for everyone by default.
- **High:** checkout, login, payment or onboarding changes; default enablement for all users/tenants; potentially significant positive or negative business KPI impact.

<a id="25-complessità-architetturale"></a>

### 2.5 Architectural complexity

- **Low:** local to one service/module; no public API, shared domain event or key data schema changes; no new architectural pattern.
- **Medium:** 2–3 services/modules; small backward-compatible API additions or new domain events without breaking existing consumers.
- **High:** many services/modules; public API breaking changes or regression risks; critical events/key schemas; significant new communication patterns or domain boundaries.

<a id="3-regole-di-classificazione"></a>

## 3. Classification rules

<a id="31-feature-low-risk--dfd-light"></a>

### 3.1 Low Risk → DFD Light

All dimensions Low, or at most one Medium and none High. Typical examples: internal refactoring, minor UI/text changes, internal metrics/logs without significant performance/cost impact, or noncritical scoped fixes. Use light design and release checklists.

<a id="32-feature-medium-risk--dfd-full-semplificato"></a>

### 3.2 Medium Risk → DFD Full (proportionate)

The original generic Medium rule includes at least one Medium and no High, overlapping the explicit Low rule. Apply the documented precedence: one Medium follows light; **at least two Medium dimensions** select proportionate full. Apply full checklists in proportion to affected dimensions, with a smaller/shorter review when appropriate. Examples include optional features for some tenants, internal dashboards and integrations with established internal services.

<a id="33-feature-high-risk--dfd-full-completo"></a>

### 3.3 High Risk → DFD Full (complete)

Any High dimension requires complete, detailed full checklists and all relevant reviewers (Tech Lead, Security, Ops, Product). Examples: new payment methods, authorization/sensitive data changes, substantial architecture/core service changes or default enablement of critical flows. New services also require full complete.

<a id="4-processo-di-classificazione"></a>

## 4. Classification process

<a id="41-chi-classifica"></a>

### 4.1 Who classifies

Tech Lead and lead developer assess initially; Product contributes user/business impact. Consult Security, Ops/SRE or Architect when uncertain, especially on security, SLOs or cost.

<a id="42-quando-classificare"></a>

### 4.2 When to classify

Before design, ideally during epic creation/refinement. Update when design reveals additional risks.

<a id="43-dove-documentare"></a>

### 4.3 Where to document

Record Low/Medium/High in the epic/ticket and/or specification, including per-dimension justification. Example: security Low, reliability Medium, cost Low, business Medium, architecture Low → Medium Risk, full proportionate.

<a id="5-esempi-pratici"></a>

## 5. Practical examples

<a id="51-esempio-1--refactor-interno"></a>

### 5.1 Internal refactoring

All five dimensions Low → light (or rapid when explicitly eligible).

<a id="52-esempio-2--nuova-dashboard-interna"></a>

### 5.2 New internal dashboard

Security Medium (new audit events), reliability Low, cost Medium (queries/database cost), business Low (internal users), architecture Medium (two services) → full proportionate.

<a id="53-esempio-3--nuovo-metodo-di-pagamento-paypal"></a>

### 5.3 New payment method (PayPal)

Security High (new payment data/flows), reliability High (critical provider/latency/errors), cost Medium (transaction cost), business High (checkout/conversion), architecture High (provider/end-to-end flows) → full complete.

<a id="6-eccezioni"></a>

## 6. Exceptions

Tech Lead/Architect may escalate Medium to High for prudence or strengthen an already High route with more reviews, tests or a slower rollout. Record exceptions in the specification and share relevant changes with the team.

<a id="percorsi-proporzionati-rapido-light-e-full"></a>

## Proportionate routes: rapid, light and full

Rapid is an explicit option for a scoped fix restoring a known contract or an editorial change: every dimension Low, no changed contract/guardrail, no critical behavior or open question. Line count does not determine eligibility. One Medium excludes rapid; unknown risk blocks route selection. Any High, two Medium or a new service requires full. Critical behavior always requires full complete, however small the patch.

Behavior fixes require a relevant red/green regression; editorial changes require a useful check without artificial red. Match suites/mutants to data loss, authorization, payments, compatibility, retry, partial writes and rollback. Check that rejected operations preserve data and retries/duplicates do not repeat effects.

Solo developers follow the same routes without mandatory PRs or a second reviewer. Optional team policy does not reduce risk. See [routes and checks](fondamenta.md#percorsi-proporzionati-e-lavoro-individuale-o-in-team) and the [quick change record](templates/cambiamento_rapido.md).
