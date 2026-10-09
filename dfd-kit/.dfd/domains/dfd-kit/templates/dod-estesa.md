# DoD Estesa – [Prodotto / Dominio / Servizio]

> **Stato**: draft / stabile / in revisione  
> **Ultimo aggiornamento**: YYYY-MM-DD  
> **Proprietari**: [@team / @persona]  
> **Riferimenti**: [link a Architecture Guardrails, ADR, wiki, ecc.]

Questo documento definisce i criteri di **Definition of Done (DoD) estesa** per [prodotto / dominio / servizio].  
I criteri qui definiti sono usati in tutto il ciclo di vita: specifica, test, rilascio e valutazione in produzione (DoD-First Driven).

---

## 1. Osservabilità

### 1.1 Metriche obbligatorie

Per ogni endpoint / flusso critico devono essere disponibili:

- [ ] Request count (per endpoint / operazione)
- [ ] Error count (4xx, 5xx, per errorCode)
- [ ] Latency: p50, p95, p99
- [ ] Custom metric per flussi business critici:
  - [ ] [es. ordini completati]
  - [ ] [es. fallimenti per motivo X]

**Fonte delle metriche**: [infrastruttura (ALB, API Gateway, service mesh), codice (SDK CloudWatch, OpenTelemetry), altro]

**Tag / dimensioni minime**:

- [ ] `service`
- [ ] `operation`
- [ ] `env`
- [ ] `tenantId` (se rilevante)
- [ ] Altri: [specificare]

### 1.2 Log strutturati

Per ogni operazione critica devono essere prodotti log con:

- [ ] `tenantId`
- [ ] `userId` (o anonimo / pseudonimo, se richiesto)
- [ ] `operation`
- [ ] `outcome` (success / failure)
- [ ] `errorCode` (se failure)
- [ ] `traceId` / correlation ID

**Vincoli**:

- [ ] Nessun dato PII / sensibile in chiaro nei log
- [ ] Formato: [JSON / altro]
- [ ] Livelli di log minimi: [INFO per operazioni completate, ERROR per fallimenti, ecc.]

### 1.3 Trace e correlazione

- [ ] Ogni richiesta deve avere un `traceId` presente su log e metriche
- [ ] Span minimi tracciati: [es. gateway → servizio → DB]
- [ ] Sistema di tracing: [X-Ray, Jaeger, altro]

### 1.4 Dashboard

Dashboard minime richieste:

- [ ] Vista per servizio: request, error, latency, custom metric critiche
- [ ] Vista per flusso business: [es. funnel ordini, pagamenti]
- [ ] Link alle dashboard: [inserire link]

---

## 2. Sicurezza

### 2.1 Controlli di accesso

- [ ] Ogni endpoint / operazione ha ruoli / policy definiti
- [ ] Accesso minimo privilegiato (least privilege)
- [ ] Documentazione dei ruoli: [link]

### 2.2 Protezione dati

- [ ] Classificazione dati: [PII, sensibili, interni, pubblici]
- [ ] Encryption:
  - [ ] In transito (TLS)
  - [ ] A riposo (DB, storage)
- [ ] Nessun dato sensibile in chiaro nei log

### 2.3 Audit & logging di sicurezza

Eventi minimi da loggare:

- [ ] Login / logout
- [ ] Cambiamenti configurazione
- [ ] Accessi a dati sensibili
- [ ] Fallimenti di accesso / autorizzazione

**Retention**: [es. 12 mesi per audit, 30 giorni per log operativi]

### 2.4 Conformità

Riferimenti a standard / policy:

- [ ] ISO [numero]
- [ ] SOC2
- [ ] GDPR
- [ ] Policy interne: [link]

---

## 3. Affidabilità / SLO

### 3.1 SLO / SLI

- [ ] Disponibilità: [es. 99.9% mensile]
- [ ] Latenza end-to-end: [es. p95 < X ms per il flusso Y]
- [ ] Error rate: [es. < 0.1% per endpoint critici]

### 3.2 Degradazione accettabile

- [ ] Comportamento in caso di failure di dependency: [fallback, retry, circuit breaker]
- [ ] Funzionalità degradate accettabili: [descrivere]

### 3.3 Politiche di resilienza

- [ ] Timeout: [valori per tipo di chiamata]
- [ ] Retry: [politiche, backoff]
- [ ] Circuit breaker: [dove applicato]

---

## 4. Costo

### 4.1 Tagging risorse

- [ ] Tutte le risorse hanno tag:
  - [ ] `service`
  - [ ] `team`
  - [ ] `env`
  - [ ] Altri: [specificare]

### 4.2 Stima impatto sul costo

Per ogni cambiamento significativo:

- [ ] Stima variazione costo mensile del servizio: [metodo / foglio / tool]
- [ ] Stima variazione costo per transazione: [metodo approssimativo]

### 4.3 Soglie di allarme

- [ ] Variazione > [X]% del costo mensile → review
- [ ] Variazione > [Y]% del costo per transazione stimato → review

**Fonte dati costo**: [Cost Explorer, dashboard interne, altro]

---

## 5. Altri assi (opzionali)

### 5.1 Esperienza utente / business KPI

- [ ] KPI da monitorare: [conversione, churn, usage, ecc.]
- [ ] Soglie di peggioramento accettabile: [descrivere]

### 5.2 Compliance specifica

- [ ] Requisiti normativi: [descrivere]
- [ ] Controlli specifici: [descrivere]

---

## 6. Riferimenti ai criteri (codici)

Per facilitare il riferimento nelle specifiche di feature, ogni criterio ha un codice:

- **Osservabilità**: OBS-01, OBS-02, …
- **Sicurezza**: SEC-01, SEC-02, …
- **Affidabilità / SLO**: SLO-01, SLO-02, …
- **Costo**: COST-01, COST-02, …
- **Altro**: UX-01, COMP-01, …

**Elenco criteri**:

- OBS-01: [descrizione breve]
- OBS-02: [descrizione breve]
- SEC-01: [descrizione breve]
- SLO-01: [descrizione breve]
- COST-01: [descrizione breve]
- …

---

## 7. Evoluzione del documento

- **Revisioni**:
  - YYYY-MM-DD: [descrizione modifica, es. "aggiunti criteri di costo per transazione"]
  - YYYY-MM-DD: [descrizione modifica]

- **Feedback**:
  - [Come inviare feedback / proposte di modifica]


## Convenzioni locali — DFD Kit

Usare la [DoD di dominio](../dod.md), il [catalogo](../criteria.json) e le [convenzioni di processo](../process.md). Sostituire gli esempi del servizio web con comportamenti della CLI e delle skill: integrità dei file, correttezza dei gate, compatibilità, esiti dei comandi e attriti nell’harness. Non riusare valori di esempio come evidenze.

Collegare verifiche realmente eseguite e motivare le esclusioni. Per rollout e review di release, identificare binario candidato/checksum, progetti di prova, pilota dell’harness, risorse e rollback della distribuzione; risorse non provate restano planned. Registrare dubbi nella specifica e riprendere l’iterazione senza sovrascritture. Placeholder e checklist vuote sono intenzionali in questo template.

La policy lifecycle.json distingue solo sviluppo e preparazione al rilascio: verifiche locali tracciate nel primo caso; rivalutare CI e requisiti operativi prima della distribuzione.
