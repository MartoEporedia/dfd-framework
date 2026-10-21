<!-- dfd-language --> [Italiano](../../examples/checkout_dod.md) | [English](checkout_dod.md)

<a id="dod-estesa--servizio-checkout-aws-ecs"></a>

# Extended DoD – Checkout service (AWS ECS)

> **Status**: draft  
> **Last updated**: 2026-10-08  
> **Owners**: @team-checkout  
> **References**: [Architecture Guardrails], [ADR-001: Tenant isolation], [Observability wiki].

Worked example for Checkout on AWS ECS Fargate, ALB and CloudWatch. Checked items illustrate the example, not independently verified production evidence. Criteria guide specification, tests, release and production evaluation.

<a id="1-osservabilità"></a>

## 1. Observability

<a id="11-metriche-obbligatorie"></a>

### 1.1 Required metrics

- [x] Requests per endpoint/operation: ALB/ECS.
- [x] Errors: 4xx, 5xx and errorCode through ALB/CloudWatch Logs Insights.
- [x] Latency p50/p95/p99: ALB Target Metrics.
- [x] CloudWatch SDK custom metrics: CheckoutCompleted, CheckoutFailed with errorCode, PaymentProcessingTime.
- [x] Infrastructure CPU/memory: ECS.
- [x] Dimensions: service=checkout; operation=createOrder|processPayment|confirmOrder; env=prod|staging; tenantId for business metrics; outcome=success|failure for custom metrics.

<a id="12-log-strutturati"></a>

### 1.2 Structured logs

- [x] tenantId, userId (pseudonymous if PII), operation, outcome, errorCode on failure, X-Ray traceId.
- [x] No plaintext sensitive data, card numbers or full address.
- [x] JSON; INFO for completed operations, ERROR for failures, WARN for nonblocking anomalies such as retries.

```json
{
  "timestamp": "2026-10-08T12:34:56Z",
  "level": "ERROR",
  "service": "checkout",
  "operation": "processPayment",
  "tenantId": "tenant-123",
  "userId": "u-abc",
  "outcome": "failure",
  "errorCode": "PAYMENT_DECLINED",
  "traceId": "1-65432109-abcdef0123456789",
  "message": "Payment declined by issuer"
}
```

<a id="13-trace-e-correlazione"></a>

### 1.3 Tracing and correlation

- [x] X-Ray traceId in HTTP logs/metrics.
- [x] Spans ALB → ECS Checkout; Checkout → Payment Service; Checkout → Inventory Service.
- [x] AWS X-Ray tracing.

<a id="14-dashboard"></a>

### 1.4 Dashboards

- [x] Checkout service: requests, 4xx/5xx errors, p50/p95/p99 latency, ECS CPU/memory.
- [x] Business: CheckoutCompleted; CheckoutFailed by errorCode; PaymentProcessingTime p50/p95.
- [x] References: `cloudwatch://dashboard/checkout-service`, `cloudwatch://dashboard/checkout-business`.

<a id="2-sicurezza"></a>

## 2. Security

<a id="21-controlli-di-accesso"></a>

### 2.1 Access controls

- [x] IAM roles/policies per service endpoint; least privilege for ECS tasks/related Lambdas; [IAM wiki].

Endpoints: `POST /checkout` creates order/starts payment; `POST /checkout/{orderId}/confirm` confirms after payment; `GET /checkout/{orderId}/status` reads state.

<a id="22-protezione-dati"></a>

### 2.2 Data protection

- [x] PII: userId/address/email; sensitive payment data handled by external Payment Service; internal orderId/tenantId.
- [x] TLS 1.2+ on ALB/service-to-service calls; RDS/DynamoDB encryption at rest.
- [x] No PAN, CVV, full address or other plaintext sensitive log data.

<a id="23-audit--logging-di-sicurezza"></a>

### 2.3 Audit and security logging

- [x] createOrder, processPayment with outcome, checkout authorization failures (403), critical configuration changes such as flags.
- [x] Audit retention: 12 months for order/payment; detailed operational/debug retention: 30 days.

<a id="24-conformità"></a>

### 2.4 Compliance

- [x] GDPR: PII handling/erasure.
- [x] PCI DSS: Checkout handles no card data directly, delegating to certified Payment Service.
- [x] Internal Security Policy v3: [link].

<a id="3-affidabilità--slo"></a>

## 3. Reliability / SLOs

### 3.1 SLO / SLI

- [x] Availability 99.9% monthly, measured on ALB target health.
- [x] End-to-end checkout p95 < 2.5 s and p99 < 4 s, POST /checkout to final response.
- [x] POST /checkout 5xx < 0.5%; business errors < 1%, excluding expected PAYMENT_DECLINED.

<a id="32-degradazione-accettabile"></a>

### 3.2 Acceptable degradation

- [x] Payment failure: clear PAYMENT_SERVICE_UNAVAILABLE; no inconsistent order.
- [x] Inventory failure: block creation with INVENTORY_UNAVAILABLE and log errorCode.
- [ ] Noncritical email notification postponement is a possible degradation.
- [x] Never create orders without inventory/payment confirmation.

<a id="33-politiche-di-resilienza"></a>

### 3.3 Resilience policies

- [x] Payment timeout 3 s; Inventory timeout 2 s.
- [x] Payment: at most one retry with exponential backoff for transient 5xx only.
- [x] Inventory: no retry; failure blocks checkout.
- [ ] Payment circuit breaker to implement if errors > 5% for 5 min.

<a id="4-costo"></a>

## 4. Cost

<a id="41-tagging-risorse"></a>

### 4.1 Resource tagging

- [x] service=checkout, team=checkout-team, env=prod|staging, costCenter=ecommerce on ECS task definitions/clusters, ALB, RDS/DynamoDB and CloudWatch/X-Ray.

<a id="42-stima-impatto-sul-costo"></a>

### 4.2 Cost impact estimate

For significant flows/integrations/architecture changes:

- [x] Monthly estimate: service-filtered Cost Explorer plus traffic forecast; [checkout cost sheet].
- [x] Approximate transaction cost = monthly service cost / monthly completed checkouts.

<a id="43-soglie-di-allarme"></a>

### 4.3 Alarm thresholds

- [x] Monthly cost change > 15% over a 7-day window → architectural review.
- [x] Estimated transaction cost change > 20% → review.

Sources: AWS Cost Explorer filtered service=checkout; [internal dashboard].

<a id="5-altri-assi-opzionali"></a>

## 5. Other concerns (optional)

<a id="51-esperienza-utente--business-kpi"></a>

### 5.1 UX / business KPIs

- [x] Completion = CheckoutCompleted / CheckoutInitiated; abandonment at payment step.
- [x] No > 5% completion decline during tenant rollout versus 14-day baseline.

<a id="52-compliance-specifica"></a>

### 5.2 Specific compliance

- [x] Order data retention: 10 years for invoicing in this example.
- [x] PII erasure on request within 30 days in this example.
- [x] Audit trail for every created/changed order.

<a id="6-riferimenti-ai-criteri-codici"></a>

## 6. Criterion identifiers

- OBS-01: per-endpoint requests/errors/latency.
- OBS-02: tenantId/userId/operation/outcome/errorCode/traceId structured logs.
- OBS-03: X-Ray trace ID in logs/metrics.
- OBS-04: service/business dashboards.
- SEC-01: IAM endpoint/ECS access controls.
- SEC-02: no plaintext sensitive log data.
- SEC-03: order/payment-attempt audit logs.
- SLO-01: 99.9% monthly availability.
- SLO-02: checkout p95 < 2.5 s.
- SLO-03: POST /checkout 5xx < 0.5%.
- COST-01: service/team/env/costCenter tagging.
- COST-02: significant-change cost estimate.
- COST-03: +15% monthly / +20% transaction alarms.
- UX-01: checkout completion monitoring.
- UX-02: no > 5% completion decline during rollout.

<a id="7-evoluzione-del-documento"></a>

## 7. Document evolution

- 2026-10-08: initial ECS/AWS example.
- [Add future revisions].
- Feedback: PR/MR to checkout-dod or #checkout-team Slack channel.

<a id="modalità-dellesempio"></a>

## Example maturity mode

This is an illustrative production-bound checkout service using release-preparation with risk-appropriate CI and operational requirements. A local prototype may use development-only and a traced final local suite; prepare rollout/release review when distribution is planned, reassessing requirements first.
