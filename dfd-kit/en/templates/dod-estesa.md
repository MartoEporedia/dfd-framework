<!-- dfd-language --> [Italiano](../../templates/dod-estesa.md) | [English](dod-estesa.md)

<a id="dod-estesa--prodotto--dominio--servizio"></a>

# Extended Definition of Done – [Product / Domain / Service]

> **Status**: draft / stable / under review  
> **Last updated**: YYYY-MM-DD  
> **Owners**: [@team / @person]  
> **References**: [Architecture Guardrails, ADRs, wiki]

These extended DoD criteria apply throughout specification, testing, release and production evaluation.

<a id="1-osservabilità"></a>

## 1. Observability

<a id="11-metriche-obbligatorie"></a>

### 1.1 Required metrics

For each critical endpoint/flow:

- [ ] Requests per endpoint/operation.
- [ ] Errors: 4xx, 5xx and errorCode.
- [ ] Latency: p50, p95, p99.
- [ ] Critical business metrics: [completed orders, failures by reason].

Metric source: [ALB, API Gateway, service mesh, CloudWatch SDK, OpenTelemetry, other].

Minimum tags/dimensions:

- [ ] `service`
- [ ] `operation`
- [ ] `env`
- [ ] `tenantId`, when relevant.
- [ ] Others: [specify].

<a id="12-log-strutturati"></a>

### 1.2 Structured logs

Every critical operation emits:

- [ ] `tenantId`
- [ ] `userId`, anonymous/pseudonymous when required.
- [ ] `operation`
- [ ] `outcome`: success/failure.
- [ ] `errorCode` on failure.
- [ ] `traceId` / correlation ID.
- [ ] No plaintext PII/sensitive data.
- [ ] Format: [JSON/other].
- [ ] Minimum levels: [INFO success, ERROR failure, other].

<a id="13-trace-e-correlazione"></a>

### 1.3 Tracing and correlation

- [ ] Each request has a traceId in logs/metrics.
- [ ] Minimum spans: [gateway → service → database].
- [ ] Tracing system: [X-Ray, Jaeger, other].

<a id="14-dashboard"></a>

### 1.4 Dashboards

- [ ] Service view: requests, errors, latency, critical custom metrics.
- [ ] Business flow view: [order/payment funnel].
- [ ] Dashboard links: [insert].

<a id="2-sicurezza"></a>

## 2. Security

<a id="21-controlli-di-accesso"></a>

### 2.1 Access controls

- [ ] Roles/policies defined per endpoint/operation.
- [ ] Least privilege.
- [ ] Role documentation: [link].

<a id="22-protezione-dati"></a>

### 2.2 Data protection

- [ ] Classification: [PII, sensitive, internal, public].
- [ ] Encryption in transit (TLS).
- [ ] Encryption at rest (database/storage).
- [ ] No plaintext sensitive data in logs.

<a id="23-audit--logging-di-sicurezza"></a>

### 2.3 Audit and security logging

- [ ] Login/logout.
- [ ] Configuration changes.
- [ ] Sensitive-data access.
- [ ] Authentication/authorization failures.

Retention: [for example 12 months audit, 30 days operational logs].

<a id="24-conformità"></a>

### 2.4 Compliance

- [ ] ISO [number].
- [ ] SOC2.
- [ ] GDPR.
- [ ] Internal policies: [links].

<a id="3-affidabilità--slo"></a>

## 3. Reliability / SLOs

### 3.1 SLO / SLI

- [ ] Availability: [for example 99.9% monthly].
- [ ] End-to-end latency: [p95 < X ms for flow Y].
- [ ] Error rate: [< 0.1% for critical endpoints].

<a id="32-degradazione-accettabile"></a>

### 3.2 Acceptable degradation

- [ ] Dependency failure behavior: [fallback/retry/circuit breaker].
- [ ] Acceptable degraded functionality: [describe].

<a id="33-politiche-di-resilienza"></a>

### 3.3 Resilience policies

- [ ] Timeouts: [per call type].
- [ ] Retries: [policy/backoff].
- [ ] Circuit breaker: [where applied].

<a id="4-costo"></a>

## 4. Cost

<a id="41-tagging-risorse"></a>

### 4.1 Resource tagging

- [ ] Every resource has `service`, `team`, `env` and [other] tags.

<a id="42-stima-impatto-sul-costo"></a>

### 4.2 Cost impact estimate

For every significant change:

- [ ] Estimated monthly service cost variation: [method/sheet/tool].
- [ ] Estimated transaction cost variation: [approximate method].

<a id="43-soglie-di-allarme"></a>

### 4.3 Alarm thresholds

- [ ] Monthly cost variation > [X]% triggers review.
- [ ] Estimated transaction cost variation > [Y]% triggers review.

Source: [Cost Explorer, internal dashboards, other].

<a id="5-altri-assi-opzionali"></a>

## 5. Other concerns (optional)

<a id="51-esperienza-utente--business-kpi"></a>

### 5.1 UX / business KPIs

- [ ] KPIs: [conversion/churn/usage].
- [ ] Acceptable deterioration thresholds: [describe].

<a id="52-compliance-specifica"></a>

### 5.2 Specific compliance

- [ ] Regulatory requirements: [describe].
- [ ] Specific controls: [describe].

<a id="6-riferimenti-ai-criteri-codici"></a>

## 6. Criterion identifiers

Stable identifiers allow references from feature specifications: OBS-01, OBS-02; SEC-01, SEC-02; SLO-01, SLO-02; COST-01, COST-02; UX-01, COMP-01, and subsequent IDs.

- OBS-01: [short description].
- OBS-02: [short description].
- SEC-01: [short description].
- SLO-01: [short description].
- COST-01: [short description].

<a id="7-evoluzione-del-documento"></a>

## 7. Document evolution

- YYYY-MM-DD: [revision, such as adding transaction cost criteria].
- YYYY-MM-DD: [revision].
- Feedback/proposals: [how to submit].

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
