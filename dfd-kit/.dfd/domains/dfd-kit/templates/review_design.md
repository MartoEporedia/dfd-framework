# Checklist di Review DFD – Design

> **Tipo di review**: design  
> **Data**: YYYY-MM-DD  
> **Partecipanti**: [@team / @persone]  
> **Riferimenti**:
> - [Specifica di Feature]
> - [DoD Estesa – Servizio X]
> - [Architecture Guardrails]
> - [Processo End-to-End DFD – Definition of Ready]

---

## 0. Verifica Definition of Ready (DoR) – Design

Prima di procedere con la review di design, verificare che l’epic soddisfi i seguenti criteri:

- [ ] **Obiettivo business chiaro** (1–2 frasi: che problema risolve / che opportunità sfrutta).  
- [ ] **KPI / metriche di successo** almeno ad alto livello (es. “aumento conversione checkout”, “riduzione ticket supporto”).  
- [ ] **Dominio / servizio principale** identificato (es. Checkout, Pagamenti, Catalogo).  
- [ ] **Stima ad alto livello** di complessità (T-shirt size o simile).  
- [ ] **Priorità** definita nel backlog di prodotto.  
- [ ] **Epic approvata** dal Product Manager / Product Owner.

Se questi criteri non sono soddisfatti, la review di design non dovrebbe procedere; si completa prima l’epic.

---

## 1. Allineamento alla DoD Estesa

### 1.1 Osservabilità

- [ ] La feature cita esplicitamente i criteri **OBS-xx** rilevanti.
- [ ] Sono definite le metriche obbligatorie (request, error, latency) per ogni nuovo endpoint.
- [ ] Sono definite le custom metric business (se applicabile).
- [ ] I log includono: `tenantId`, `userId`, `operation`, `outcome`, `errorCode`, `traceId`.
- [ ] È previsto l’uso del trace ID su log e metriche.
- [ ] Sono identificate le dashboard da aggiornare / creare.

### 1.2 Sicurezza

- [ ] La feature cita esplicitamente i criteri **SEC-xx** rilevanti.
- [ ] Sono definiti i controlli di accesso (ruoli, policy).
- [ ] È verificata la protezione dati (nessun dato sensibile in chiaro nei log).
- [ ] Sono definiti gli eventi di audit da loggare.
- [ ] Sono rispettati gli standard / policy indicati nella DoD.

### 1.3 Affidabilità / SLO

- [ ] La feature cita esplicitamente i criteri **SLO-xx** rilevanti.
- [ ] È stimato l’impatto su latenza, error rate, disponibilità.
- [ ] Sono definite le mitigazioni per dipendenze esterne (timeout, retry, fallback).
- [ ] Sono identificati i nuovi rischi per affidabilità e le relative contromisure.

### 1.4 Costo

- [ ] La feature cita esplicitamente i criteri **COST-xx** rilevanti.
- [ ] È stimato l’impatto sul costo mensile del servizio.
- [ ] È stimato l’impatto sul costo per transazione (se applicabile).
- [ ] È verificato il rispetto delle soglie di allarme (COST-03).

### 1.5 Esperienza utente / business KPI

- [ ] Sono identificati i KPI business impattati (es. tasso di completamento).
- [ ] Sono definite le soglie di peggioramento accettabile (UX-02).
- [ ] È definito un piano di validazione in produzione (rollout per tenant, esperimento).

---

## 2. Guardrail globali

- [ ] La feature non viola i guardrail architetturali.
- [ ] La feature non viola i guardrail di sicurezza.
- [ ] La feature non viola i guardrail di osservabilità.
- [ ] La feature non viola i guardrail di costo.
- [ ] Eventuali eccezioni sono esplicite, motivate e approvate.

---

## 3. Test e validazione

### 3.1 Test funzionali

- [ ] Piano di test unitari per la logica nuova / modificata.
- [ ] Piano di test di integrazione per le nuove / modificate API.
- [ ] Piano di test end-to-end per i flussi principali.

### 3.2 Test non funzionali

- [ ] Piano di test di carico (se rilevante per SLO).
- [ ] Piano di test di resilienza (dipendenze down, timeout, fallback).
- [ ] Piano di test di sicurezza (accesso non autorizzato, input malevoli).

### 3.3 Validazione in produzione (shift-right)

- [ ] È definita un’idea di piano di canary tecnico (metriche, soglie, rollback).
- [ ] È definita un’idea di piano di rollout per tenant / segmento.

---

## 4. Operatività e supporto

- [ ] Sono stati identificati i team da informare prima del rilascio (Support, Ops, Security).
- [ ] Sono stati identificati gli alert operativi principali da configurare in pre-release.

---

## 5. Decisioni e aperture

- [ ] Decisioni prese in review: [elenco sintetico]
- [ ] Aperture residue (da decidere / investigare): [elenco]

---

## 6. Esito della review di design

- [ ] **Approvata** (può procedere allo sviluppo)
- [ ] **Approvata con condizioni**: [condizioni da soddisfare]
- [ ] **Da rivedere**: [motivi, cosa manca]

**Firme / approvazioni**:

- [@persona1] – [ruolo]
- [@persona2] – [ruolo]


## Convenzioni locali — DFD Kit

Usare la [DoD di dominio](../dod.md), il [catalogo](../criteria.json) e le [convenzioni di processo](../process.md). Sostituire gli esempi del servizio web con comportamenti della CLI e delle skill: integrità dei file, correttezza dei gate, compatibilità, esiti dei comandi e attriti nell’harness. Non riusare valori di esempio come evidenze.

Collegare verifiche realmente eseguite e motivare le esclusioni. Per rollout e review di release, identificare binario candidato/checksum, progetti di prova, pilota dell’harness, risorse e rollback della distribuzione; risorse non provate restano planned. Registrare dubbi nella specifica e riprendere l’iterazione senza sovrascritture. Placeholder e checklist vuote sono intenzionali in questo template.
