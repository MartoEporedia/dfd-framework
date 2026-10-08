# Checklist di Review DFD – Design (Light)

> **Tipo di review**: design (feature piccola / basso rischio)  
> **Data**: YYYY-MM-DD  
> **Partecipanti**: [@team / @persone]  
> **Riferimenti**:
> - [Specifica di Feature (anche leggera)]
> - [DoD Estesa – Servizio X]

> **Quando usare questa versione**:  
> Feature che **non toccano** pagamento, sicurezza, SLO critici o architettura core.  
> Esempi: piccoli cambiamenti UI, testi, metriche interne, refactor non funzionali.

---

## 0. DoR – Design (Light)

- [ ] **Obiettivo chiaro** (1 frase: cosa cambia e perché).  
- [ ] **Servizio / dominio** identificato.  
- [ ] **Nessun impatto atteso** su sicurezza, SLO critici o costo significativo.  
- [ ] **OK informale** del Tech Lead per procedere con design leggero.

---

## 1. DoD Estesa (punti essenziali)

### 1.1 Osservabilità

- [ ] Nessun nuovo endpoint critico introdotto (o, se introdotto, metriche base già disponibili).  
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

## 2. Guardrail globali

- [ ] La feature non viola i guardrail architetturali.  
- [ ] La feature non viola i guardrail di sicurezza.  
- [ ] La feature non viola i guardrail di osservabilità.  
- [ ] La feature non viola i guardrail di costo.

---

## 3. Test e validazione

- [ ] Test unitari / di integrazione previsti (almeno per la logica nuova / modificata).  
- [ ] Nessun test di carico / resilienza richiesto (o già coperto da test esistenti).  
- [ ] Validazione in produzione:  
  - [ ] rollout diretto o canary leggero (se il team lo prevede).

---

## 4. Esito della review di design (Light)

- [ ] **Approvata** (può procedere allo sviluppo)  
- [ ] **Approvata con condizioni**: [condizioni da soddisfare]  
- [ ] **Da rivedere**: [motivi, cosa manca]

**Firme / approvazioni**:

- [@persona] – [ruolo]


## Convenzioni locali — DFD Kit

Usare la [DoD di dominio](../dod.md), il [catalogo](../criteria.json) e le [convenzioni di processo](../process.md). Sostituire gli esempi del servizio web con comportamenti della CLI e delle skill: integrità dei file, correttezza dei gate, compatibilità, esiti dei comandi e attriti nell’harness. Non riusare valori di esempio come evidenze.

Collegare verifiche realmente eseguite e motivare le esclusioni. Per rollout e review di release, identificare binario candidato/checksum, progetti di prova, pilota dell’harness, risorse e rollback della distribuzione; risorse non provate restano planned. Registrare dubbi nella specifica e riprendere l’iterazione senza sovrascritture. Placeholder e checklist vuote sono intenzionali in questo template.
