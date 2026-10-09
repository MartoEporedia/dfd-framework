# DoD Estesa – Servizio Checkout (AWS ECS)

> **Stato**: draft  
> **Ultimo aggiornamento**: 2026-10-08  
> **Proprietari**: @team-checkout  
> **Riferimenti**: [Architecture Guardrails], [ADR-001: Isolamento tenant], [Wiki: Osservabilità]

Questo documento definisce i criteri di **Definition of Done (DoD) estesa** per il servizio **Checkout**, eseguito su **AWS ECS** (Fargate), con ALB come load balancer e CloudWatch per monitoring.  
I criteri qui definiti sono usati in tutto il ciclo di vita: specifica, test, rilascio e valutazione in produzione (DoD-First Driven).

---

## 1. Osservabilità

### 1.1 Metriche obbligatorie

Per ogni endpoint / flusso critico devono essere disponibili:

- [x] Request count (per endpoint / operazione) – da ALB / ECS
- [x] Error count (4xx, 5xx, per errorCode) – da ALB / CloudWatch Logs Insights
- [x] Latency: p50, p95, p99 – da ALB Target Metrics
- [x] Custom metric per flussi business critici:
  - [x] `CheckoutCompleted` (conteggio ordini completati)
  - [x] `CheckoutFailed` (conteggio fallimenti, con `errorCode`)
  - [x] `PaymentProcessingTime` (durata elaborazione pagamento)

**Fonte delle metriche**:

- Infrastruttura: ALB (request, error, latency), ECS (CPU, memory)
- Codice: SDK CloudWatch (custom metric per checkout e pagamento)

**Tag / dimensioni minime**:

- [x] `service = checkout`
- [x] `operation = createOrder | processPayment | confirmOrder`
- [x] `env = prod | staging`
- [x] `tenantId` (per custom metric business)
- [x] `outcome = success | failure` (per custom metric)

### 1.2 Log strutturati

Per ogni operazione critica devono essere prodotti log con:

- [x] `tenantId`
- [x] `userId` (pseudonimizzato se PII)
- [x] `operation` (es. `createOrder`, `processPayment`)
- [x] `outcome` (success / failure)
- [x] `errorCode` (se failure, es. `PAYMENT_DECLINED`, `INVENTORY_UNAVAILABLE`)
- [x] `traceId` (X-Ray Trace ID)

**Vincoli**:

- [x] Nessun dato PII / sensibile in chiaro nei log (no numeri di carta, no indirizzo completo)
- [x] Formato: JSON
- [x] Livelli di log minimi:
  - `INFO` per operazioni completate (checkout successo)
  - `ERROR` per fallimenti (pagamento rifiutato, errore inventory)
  - `WARN` per situazioni anomale non bloccanti (retry pagamento)

**Esempio log**:

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
  "message": "Pagamento rifiutato dall'issuer"
}
```

### 1.3 Trace e correlazione

- [x] Ogni richiesta HTTP ha un `traceId` (X-Ray) presente su log e metriche
- [x] Span minimi tracciati:
  - ALB → ECS (servizio checkout)
  - Checkout → Payment Service
  - Checkout → Inventory Service
- [x] Sistema di tracing: AWS X-Ray

### 1.4 Dashboard

Dashboard minime richieste (CloudWatch):

- [x] **Checkout – Servizio**:
  - Request count (ALB)
  - Error rate (4xx, 5xx)
  - Latency p50/p95/p99 (ALB)
  - CPU / memory ECS
- [x] **Checkout – Flusso business**:
  - `CheckoutCompleted` (conteggio)
  - `CheckoutFailed` (conteggio, per `errorCode`)
  - `PaymentProcessingTime` (p50/p95)
- [x] Link alle dashboard:
  - `cloudwatch://dashboard/checkout-service`
  - `cloudwatch://dashboard/checkout-business`

---

## 2. Sicurezza

### 2.1 Controlli di accesso

- [x] Ogni endpoint ha ruoli / policy IAM definiti (per chiamate tra servizi)
- [x] Accesso minimo privilegiato (least privilege) per task ECS e Lambda collegate
- [x] Documentazione dei ruoli: [link a wiki IAM]

**Endpoint principali**:

- `POST /checkout` – crea ordine e avvia pagamento
- `POST /checkout/{orderId}/confirm` – conferma ordine dopo pagamento
- `GET /checkout/{orderId}/status` – stato checkout

### 2.2 Protezione dati

- [x] Classificazione dati:
  - PII: `userId`, indirizzo, email
  - Sensibili: dati di pagamento (gestiti da Payment Service esterno)
  - Interni: `orderId`, `tenantId`
- [x] Encryption:
  - In transito: TLS 1.2+ per tutte le chiamate (ALB, service-to-service)
  - A riposo: RDS / DynamoDB con encryption abilitata
- [x] Nessun dato sensibile in chiaro nei log (no PAN, no CVV, no indirizzo completo)

### 2.3 Audit & logging di sicurezza

Eventi minimi da loggare:

- [x] Creazione ordine (`createOrder`)
- [x] Tentativo di pagamento (`processPayment`) con outcome
- [x] Fallimenti di autorizzazione (403) su endpoint checkout
- [x] Cambiamenti configurazione critica (es. feature flag di checkout)

**Retention**:

- [x] 12 mesi per log di audit (creazione ordine, pagamento)
- [x] 30 giorni per log operativi dettagliati (debug)

### 2.4 Conformità

Riferimenti a standard / policy:

- [x] GDPR (gestione PII, diritto alla cancellazione)
- [x] PCI DSS (nessun dato di carta gestito direttamente da Checkout; delegato a Payment Service certificato)
- [x] Policy interne: [link a Security Policy v3]

---

## 3. Affidabilità / SLO

### 3.1 SLO / SLI

- [x] Disponibilità: **99.9% mensile** (misurata su ALB target health)
- [x] Latenza end-to-end (flusso completo checkout):
  - p95 < **2.5 s** (da `POST /checkout` a risposta finale)
  - p99 < **4 s**
- [x] Error rate:
  - < **0.5%** per errori 5xx su `POST /checkout`
  - < **1%** per errori business (es. `PAYMENT_DECLINED` esclusi, perché attesi)

### 3.2 Degradazione accettabile

- [x] In caso di failure del Payment Service:
  - [x] Checkout ritorna errore chiaro (`PAYMENT_SERVICE_UNAVAILABLE`)
  - [x] Nessun ordine creato in stato inconsistente
- [x] In caso di failure dell'Inventory Service:
  - [x] Checkout blocca la creazione ordine con errore `INVENTORY_UNAVAILABLE`
  - [x] Log dell'evento con `errorCode`

**Funzionalità degradate accettabili**:

- [x] In modalità degradata, il checkout può:
  - [ ] Posticipare aggiornamenti non critici (es. notifiche email)
  - [x] Ma non può creare ordini senza conferma inventory / pagamento

### 3.3 Politiche di resilienza

- [x] Timeout:
  - Payment Service: **3 s**
  - Inventory Service: **2 s**
- [x] Retry:
  - Payment Service: max 1 retry con backoff esponenziale (solo per errori transienti 5xx)
  - Inventory Service: nessun retry (fallimento = blocco checkout)
- [x] Circuit breaker:
  - [ ] Da implementare su Payment Service se error rate > 5% per 5 min

---

## 4. Costo

### 4.1 Tagging risorse

- [x] Tutte le risorse hanno tag:
  - [x] `service = checkout`
  - [x] `team = checkout-team`
  - [x] `env = prod | staging`
  - [x] `costCenter = ecommerce`

**Risorse interessate**:

- Task definition ECS
- Cluster ECS
- ALB
- RDS / DynamoDB
- CloudWatch Logs / X-Ray

### 4.2 Stima impatto sul costo

Per ogni cambiamento significativo (nuovo flusso, nuova integrazione, cambio architetturale):

- [x] Stima variazione costo mensile del servizio:
  - Metodo: [Cost Explorer per servizio + stima traffico aggiuntivo]
  - Foglio / tool: [link a foglio stima costi checkout]
- [x] Stima variazione costo per transazione:
  - Metodo approssimativo:  
    `costo mensile servizio / numero mensile di checkout completati`

### 4.3 Soglie di allarme

- [x] Variazione > **15%** del costo mensile (su finestra 7 giorni) → review architetturale
- [x] Variazione > **20%** del costo per transazione stimato → review

**Fonte dati costo**:

- AWS Cost Explorer (filtrato per tag `service=checkout`)
- Dashboard interna: [link]

---

## 5. Altri assi (opzionali)

### 5.1 Esperienza utente / business KPI

- [x] KPI da monitorare:
  - Tasso di completamento checkout (`CheckoutCompleted / CheckoutInitiated`)
  - Tasso di abbandono a step di pagamento
- [x] Soglie di peggioramento accettabile:
  - Nessuna riduzione > **5%** del tasso di completamento su rollout per tenant (rispetto a baseline 14 giorni)

### 5.2 Compliance specifica

- [x] Requisiti normativi:
  - Conservazione dati ordine: **10 anni** (fatturazione)
  - Cancellazione PII su richiesta utente (entro 30 giorni)
- [x] Controlli specifici:
  - Audit trail per ogni ordine creato / modificato

---

## 6. Riferimenti ai criteri (codici)

Per facilitare il riferimento nelle specifiche di feature, ogni criterio ha un codice:

- **Osservabilità**: OBS-01, OBS-02, …
- **Sicurezza**: SEC-01, SEC-02, …
- **Affidabilità / SLO**: SLO-01, SLO-02, …
- **Costo**: COST-01, COST-02, …
- **Esperienza utente**: UX-01, UX-02, …

**Elenco criteri**:

- OBS-01: Metriche obbligatorie per endpoint (request, error, latency)
- OBS-02: Log strutturati con tenantId, userId, operation, outcome, errorCode, traceId
- OBS-03: Trace ID presente su log e metriche (X-Ray)
- OBS-04: Dashboard servizio e business per checkout
- SEC-01: Controlli di accesso IAM per endpoint e task ECS
- SEC-02: Nessun dato sensibile in chiaro nei log
- SEC-03: Audit log per creazione ordine e tentativo di pagamento
- SLO-01: Disponibilità 99.9% mensile
- SLO-02: Latenza p95 < 2.5 s per flusso checkout
- SLO-03: Error rate 5xx < 0.5% su `POST /checkout`
- COST-01: Tagging minimo (service, team, env, costCenter)
- COST-02: Stima impatto sul costo per cambiamento significativo
- COST-03: Soglie di allarme costo (+15% mensile, +20% per transazione)
- UX-01: Monitoraggio tasso di completamento checkout
- UX-02: Nessuna riduzione > 5% del tasso di completamento su rollout

---

## 7. Evoluzione del documento

- **Revisioni**:
  - 2026-10-08: versione iniziale (checkout su ECS, AWS)
  - [future revisioni da aggiungere]

- **Feedback**:
  - Inviare PR / MR su repo `checkout-dod` o ping su Slack `#checkout-team`

## Maturità e policy delle verifiche

- Modalità del dominio: solo sviluppo / preparazione al rilascio.
- CI richiesta: sì / no; motivazione rispetto a rischio e destinazione.
- Suite finale: comando, ambiente, data, esito e riferimento al log reale.
- Dubbi o requisiti rinviati alla pre-release: owner e condizione di attivazione.

In solo sviluppo, completare design e verifiche proporzionate senza richiedere CI, rollout, on-call o comunicazioni di rilascio anticipatamente. Le checklist di release si applicano quando si prepara una distribuzione. Per passare al rilascio rivalutare rischio e policy, riconfermare review e aggiornare evidenze; non segnare come eseguite attività rinviate.

## Proporzionalità e collaborazione

Valutare prima il percorso rapido per fix circoscritti e non critici: in quel caso usare un solo [record rapido](cambiamento_rapido.md), senza compilare questo template integralmente. Per light/full mantenere il dettaglio utile al rischio. Owner e verifiche sono necessari; PR, reviewer indipendente e prove integrate dipendono dalle [convenzioni facoltative del team](collaborazione.md). Il singolo dev non deve simulare ruoli o comunicazioni. Riutilizzare review già pertinenti; testare gli effetti del fallimento e le regressioni importanti.
