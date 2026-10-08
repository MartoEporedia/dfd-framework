# Checklist di Review DFD – Release (Light)

> **Tipo di review**: release (feature piccola / basso rischio)  
> **Data**: YYYY-MM-DD  
> **Partecipanti**: [@team / @persone]  
> **Riferimenti**:
> - [Specifica di Feature]
> - [DoD Estesa – Servizio X]
> - [Configurazione Rollout (anche leggera)]

> **Quando usare questa versione**:  
> Feature che **non toccano** pagamento, sicurezza, SLO critici o architettura core.  
> Esempi: piccoli cambiamenti UI, testi, metriche interne, refactor non funzionali.

---

## 0. DoR – Release (Light)

- [ ] **Test verdi** (unitari / integrazione principali).  
- [ ] **Nessun test non funzionale** richiesto (o già eseguito in passato per casi simili).  
- [ ] **Piano di rilascio semplice**:
  - [ ] rollout diretto o canary leggero;
  - [ ] nessun feature flag complesso (o già esistente).
- [ ] **Supporto / stakeholder** informati (almeno per awareness, se il cambiamento è visibile agli utenti).

---

## 1. DoD Estesa (verifica leggera)

### 1.1 Osservabilità

- [ ] Nessun nuovo allarme critico richiesto.  
- [ ] Log coerenti con lo standard del servizio (se ci sono nuovi log).  
- [ ] Nessun impatto negativo su dashboard esistenti.

### 1.2 Sicurezza

- [ ] Nessun cambiamento su controlli di accesso / autorizzazioni.  
- [ ] Nessun nuovo dato sensibile trattato o loggato.

### 1.3 Affidabilità / SLO

- [ ] Nessun impatto atteso su latenza, error rate o disponibilità.  
- [ ] Nessuna nuova dipendenza esterna critica.

### 1.4 Costo

- [ ] Nessun impatto significativo sul costo del servizio.

---

## 2. Guardrail globali (verifica rapida)

- [ ] La feature non viola i guardrail architetturali.  
- [ ] La feature non viola i guardrail di sicurezza.  
- [ ] La feature non viola i guardrail di osservabilità.  
- [ ] La feature non viola i guardrail di costo.

---

## 3. Test e rilascio

- [ ] Test principali verdi in CI.  
- [ ] Piano di rilascio chiaro (diretto o canary leggero).  
- [ ] Procedura di rollback chiara (anche semplice: “disabilita flag / revert PR”).

---

## 4. Esito della review di release (Light)

- [ ] **OK a rilasciare** (può procedere il rollout in produzione)  
- [ ] **OK con condizioni**: [condizioni da soddisfare prima o durante il rollout]  
- [ ] **Da rivedere**: [motivi, cosa manca]

**Firme / approvazioni**:

- [@persona] – [ruolo]


## Convenzioni locali — DFD Kit

Usare la [DoD di dominio](../dod.md), il [catalogo](../criteria.json) e le [convenzioni di processo](../process.md). Sostituire gli esempi del servizio web con comportamenti della CLI e delle skill: integrità dei file, correttezza dei gate, compatibilità, esiti dei comandi e attriti nell’harness. Non riusare valori di esempio come evidenze.

Collegare verifiche realmente eseguite e motivare le esclusioni. Per rollout e review di release, identificare binario candidato/checksum, progetti di prova, pilota dell’harness, risorse e rollback della distribuzione; risorse non provate restano planned. Registrare dubbi nella specifica e riprendere l’iterazione senza sovrascritture. Placeholder e checklist vuote sono intenzionali in questo template.
