# Specifica di Feature – [Nome Feature]

> **Stato**: draft / in review / approvata / implementata  
> **Data creazione**: YYYY-MM-DD  
> **Proprietari**: [@team / @persona]  
> **Servizio / Dominio**: [es. Checkout, Pagamenti, Catalogo]  
> **Riferimenti**:
> - [DoD Estesa – Servizio X]
> - [Architecture Guardrails]
> - [ADR correlati]
> - [Ticket / Epic]

---

## 1. Contesto e obiettivo

**Contesto**:  
[Breve descrizione del contesto: problema da risolvere, opportunità, richiesta business, ecc.]

**Obiettivo**:  
[Cosa deve ottenere questa feature in termini di valore per l'utente / business]

**Confini**:  
- Dominio / servizio interessato: [es. Checkout]
- Servizi / sistemi coinvolti: [es. Payment Service, Inventory Service, Notification Service]
- Fuori scope: [cosa esplicitamente non è incluso]

---

## 2. Descrizione funzionale

### 2.1 Scenari principali

- **Scenario 1**: [nome scenario]
  - Attori: [es. utente, sistema]
  - Precondizioni: [es. utente autenticato, carrello non vuoto]
  - Flusso:
    1. [step 1]
    2. [step 2]
    3. [step 3]
  - Postcondizioni: [es. ordine creato, notifica inviata]

- **Scenario 2**: [nome scenario]
  - ...

### 2.2 Casi d'uso / user story

- Come [tipo di utente], voglio [obiettivo], in modo che [beneficio].
- Come [tipo di utente], voglio [obiettivo], in modo che [beneficio].

### 2.3 Interfacce e contratti

**API esposte / modificate**:

- `METHOD /path` – [breve descrizione]
  - Request: [campi principali]
  - Response: [campi principali]
  - Errori possibili: [codici e significato]

**Eventi pubblicati / consumati**:

- Evento: `[NomeEvento]`
  - Pubblicato da: [servizio]
  - Consumato da: [servizi]
  - Payload (campi principali): [elenco]

**Dipendenze esterne**:

- [Payment Service, Inventory Service, ecc.]

---

## 3. Allineamento alla DoD Estesa

Questa feature soddisfa i seguenti criteri della **DoD Estesa** del servizio [Nome Servizio]:

### 3.1 Osservabilità

- [ ] **OBS-01** – Metriche obbligatorie per endpoint  
  Implementazione: [es. "nuovo endpoint `POST /checkout/promo` esposto su ALB, metriche request/error/latency già disponibili"]
- [ ] **OBS-02** – Log strutturati con tenantId, userId, operation, outcome, errorCode, traceId  
  Implementazione: [es. "log per applicazione promo con errorCode `PROMO_INVALID`, `PROMO_EXPIRED`"]
- [ ] **OBS-03** – Trace ID presente su log e metriche  
  Implementazione: [es. "traceId X-Ray propagato a Payment Service"]
- [ ] **OBS-04** – Dashboard servizio e business  
  Implementazione: [es. "nuovo pannello 'Promo Usage' nella dashboard checkout-business"]

**Nuove metriche custom (se applicabile)**:

- [ ] `PromoApplied` (conteggio promo applicati con successo)
  - Dimensioni: `tenantId`, `promoCode`, `outcome`
- [ ] `PromoFailed` (conteggio fallimenti applicazione promo)
  - Dimensioni: `tenantId`, `errorCode`

### 3.2 Sicurezza

- [ ] **SEC-01** – Controlli di accesso IAM per endpoint e task ECS  
  Implementazione: [es. "nuova policy per ruolo ECS che limita accesso a tabella Promo"]
- [ ] **SEC-02** – Nessun dato sensibile in chiaro nei log  
  Implementazione: [es. "promoCode loggato, nessun dato PII aggiuntivo"]
- [ ] **SEC-03** – Audit log per creazione ordine e tentativo di pagamento  
  Implementazione: [es. "evento `PromoApplied` loggato con userId, tenantId, timestamp"]

**Nuovi eventi di audit (se applicabile)**:

- [ ] `PromoApplied` – loggato per audit (chi, quando, quale promo)

### 3.3 Affidabilità / SLO

- [ ] **SLO-01** – Disponibilità 99.9% mensile  
  Impatto: [es. "nessun impatto, usa stessa infrastruttura ECS esistente"]
- [ ] **SLO-02** – Latenza p95 < 2.5 s per flusso checkout  
  Impatto: [es. "latenza aggiuntiva stimata < 100 ms per applicazione promo"]
- [ ] **SLO-03** – Error rate 5xx < 0.5% su `POST /checkout`  
  Impatto: [es. "nuovi errorCode 4xx (PROMO_INVALID) non contano nel 5xx"]

**Nuovi rischi per affidabilità**:

- [ ] [es. "dipendenza da Promo Service: se down, checkout degradato"]

**Mitigazioni**:

- [ ] [es. "fallback: checkout senza promo, errore chiaro all'utente"]

### 3.4 Costo

- [ ] **COST-01** – Tagging minimo (service, team, env, costCenter)  
  Implementazione: [es. "nessuna nuova risorsa, solo codice aggiuntivo"]
- [ ] **COST-02** – Stima impatto sul costo per cambiamento significativo  
  Stima: [es. "+2% costo mensile stimato per aumento CPU dovuto a logica promo"]
- [ ] **COST-03** – Soglie di allarme costo (+15% mensile, +20% per transazione)  
  Impatto: [es. "nessun superamento soglie atteso"]

**Stima impatto economico**:

- Costo mensile aggiuntivo stimato: [X EUR / %]
- Costo per transazione aggiuntivo stimato: [Y EUR / %]

### 3.5 Esperienza utente / business KPI

- [ ] **UX-01** – Monitoraggio tasso di completamento checkout  
  Impatto: [es. "da monitorare eventuale aumento abbandono se promo fallisce"]
- [ ] **UX-02** – Nessuna riduzione > 5% del tasso di completamento su rollout  
  Piano di validazione: [es. "rollout per tenant con confronto baseline 14 giorni"]

---

## 4. Guardrail globali

Questa feature opera entro i seguenti **guardrail globali**:

- [ ] **Architetturali**: [es. "nessun accesso diretto al DB ordini da parte del Promo Service"]
- [ ] **Sicurezza**: [es. "nessun dato PII condiviso con sistemi non autorizzati"]
- [ ] **Osservabilità**: [es. "tutti i nuovi log seguono formato JSON standard"]
- [ ] **Costo**: [es. "nessun superamento budget di dominio senza approvazione"]

**Eccezioni ai guardrail (se applicabile)**:

- [ ] [descrivere eccezione, motivazione, approvazione]

---

## 5. Test e validazione

### 5.1 Test funzionali

- [ ] Test unitari per logica applicazione promo
- [ ] Test di integrazione per endpoint `POST /checkout/promo`
- [ ] Test end-to-end per flusso completo (carrello → applicazione promo → checkout)

### 5.2 Test non funzionali

- [ ] Test di carico per verificare impatto su latenza (SLO-02)
- [ ] Test di resilienza (Promo Service down → fallback)
- [ ] Test di sicurezza (accesso non autorizzato a endpoint promo)

### 5.3 Validazione in produzione (shift-right)

- [ ] Canary tecnico:
  - Metriche monitorate: error rate, latency, custom metric `PromoApplied`, `PromoFailed`
  - Soglie di rollback: [es. error rate 5xx > 1% per 5 min]
- [ ] Rollout per tenant / segmento:
  - Criteri di successo: nessun peggioramento > 5% del tasso di completamento (UX-02)
  - Piano: [es. 10% tenant → 50% → 100%]

---

## 6. Piano di rilascio

- [ ] Feature flag: [nome flag, sistema (es. AppConfig)]
- [ ] Fasi di rollout:
  1. [es. ambiente staging]
  2. [es. 10% tenant prod]
  3. [es. 50% tenant prod]
  4. [es. 100% tenant prod]
- [ ] Criteri di promozione:
  - [es. error rate < soglia, nessun impatto negativo su UX-02]
- [ ] Criteri di rollback:
  - [es. error rate > soglia, peggioramento UX-02]

---

## 7. Aperture e decisioni da prendere

- [ ] [es. "decidere se implementare circuit breaker su Promo Service"]
- [ ] [es. "valutare se aggiungere metrica per tempo medio applicazione promo"]

---

## 8. Riferimenti e allegati

- [Link a wireframe / mockup]
- [Link a documenti tecnici correlati]
- [Link a ticket / epic]

---

## 9. Storico revisioni

- YYYY-MM-DD: versione iniziale (draft)
- YYYY-MM-DD: [descrizione modifica]
