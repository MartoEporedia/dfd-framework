# Guida all’integrazione di DFD su prodotti esistenti

> **Stato**: draft / stabile  
> **Ultimo aggiornamento**: 2026-10-08  
> **Proprietari**: [@team / @persona]  
> **Riferimenti**:
> - [Fondamenta DFD](fondamenta.md)
> - [Processo End-to-End](processo-e2e.md)
> - [Criteri di Rischio](rischio.md)
> - [Modello RACI](raci.md)

Questa guida descrive come introdurre **DoD-First Driven (DFD)** in un prodotto o dominio **già esistente**, in modo graduale e sostenibile.

---

## 1. Principi per l’integrazione su esistente

Partiamo da alcuni vincoli realistici:

- Il prodotto esiste già, con:
  - codice, test, deploy, monitoring;
  - processi (più o meno formali);
  - team abituati a un certo modo di lavorare.
- Non ha senso:
  - riscrivere tutto;
  - imporre DFD al 100% da subito;
  - trattare feature piccole come se fossero greenfield.

Quindi:

- DFD va introdotto **per strati** e **per aree**;  
- Si parte da **pilot** su feature / servizi adatti;  
- Si usa una distinzione esplicita tra:
  - **Nuovo** (feature, servizi nuovi) → DFD full o light da subito;  
  - **Esistente** → DFD applicato in modo incrementale (refactor, nuove feature, incidenti).

---

## 2. Fase 0 – Assessment iniziale (dove siete ora)

Prima di integrare DFD, fai un “snapshot” della situazione.

### 2.1 Cosa valutare

Per ogni servizio / dominio, valuta:

1. **Processo attuale**
   - Come si scrivono le specifiche (se esistono)?
   - Come si fanno i test (unitari, integrazione, E2E)?
   - Come si rilascia (CI/CD, canary, rollout per tenant)?

2. **Osservabilità**
   - Ci sono metriche (request, error, latency)?
   - I log sono strutturati?
   - C’è tracing?

3. **Sicurezza**
   - Ci sono criteri espliciti di sicurezza (accesso, audit, dati sensibili)?
   - Ci sono standard / policy di riferimento?

4. **Affidabilità / SLO**
   - Ci sono SLO definiti (disponibilità, latenza, error rate)?
   - Ci sono allarmi basati su questi SLO?

5. **Costo**
   - Si misura / stima il costo per servizio / transazione?
   - Ci sono budget o soglie?

6. **Cultura / processi**
   - Il team è aperto a introdurre più struttura (spec, review, checklist)?
   - Ci sono già pratiche TDD / review / post-mortem?

Puoi farlo con un semplice foglio o tabella:

| Servizio | Processo | Osservabilità | Sicurezza | SLO | Costo | Cultura | Note |
|----------|----------|---------------|-----------|-----|-------|---------|------|
| Checkout | ...      | ...           | ...       | ... | ...   | ...     | ...  |

### 2.2 Identificare “candidati DFD”

Sulla base di questo assessment, identifica:

- **1–2 servizi / domini** dove:
  - c’è abbastanza maturità tecnica;
  - c’è volontà di sperimentare;
  - l’impatto di DFD può essere visibile (es. riduzione incidenti, miglior SLO).

- **Tipologie di lavoro** su cui applicare DFD:
  - nuove feature medium/high risk;
  - refactor significativi;
  - risposta a incidenti (post-mortem → nuova DoD).

---

## 3. Fase 1 – Pilota DFD su nuovo / modifiche significative

In questa fase, l’obiettivo è **dimostrare valore** senza sconvolgere tutto.

### 3.1 Regola pratica

- **Tutto ciò che è nuovo o significativamente modificato** in un servizio pilota segue DFD:
  - nuove feature medium/high risk → DFD full;
  - feature low risk → DFD light;
  - refactor che toccano SLO / sicurezza → DFD full.

- **Il resto** continua come prima, ma:
  - si inizia a usare la **DoD Estesa** come riferimento per:
    - nuovi allarmi;
    - nuovi log / metriche;
    - nuovi test.

### 3.2 Cosa introdurre subito

Nel servizio pilota:

1. **DoD Estesa (almeno bozza)**  
   - Non serve perfetta: basta una versione “good enough” con:
     - osservabilità (metriche, log, trace);
     - sicurezza (accesso, audit, dati sensibili);
     - SLO (disponibilità, latenza, error rate);
     - costo (almeno tagging e stima grossolana).
   - Usa il template DFD e adattalo alla realtà.

2. **Specifica di Feature DFD** per:
   - nuove feature medium/high risk;
   - cambiamenti architetturali.

3. **Review DFD (design + release)**  
   - Usa le checklist DFD (full o light) come “overlay” sulle review che già fate.
   - Non cambiare tutto il processo di review, aggiungi solo:
     - verifica allineamento DoD;
     - verifica guardrail;
     - verifica piano di validazione in produzione.

4. **Rollout shift-right**  
   - Per le feature nuove:
     - definisci fasi di rollout (canary, percentuali, tenant);
     - allarmi basati su SLO e DoD;
     - criteri di rollback espliciti.

### 3.3 Cosa NON fare in questa fase

- Non cercare di:
  - riscrivere tutte le specifiche esistenti in formato DFD;
  - imporre DFD a tutti i servizi;
  - avere una DoD Estesa perfetta e completa per tutto.

- Concentrati su:
  - 1–2 servizi pilota;
  - 3–5 feature significative;
  - dimostrare che DFD aiuta a:
    - ridurre regressioni;
    - chiarire responsabilità;
    - migliorare osservabilità e gestione incidenti.

---

## 4. Fase 2 – Estensione graduale all’esistente

Una volta che il pilota ha mostrato valore, puoi espandere DFD in modo graduale.

### 4.1 Per servizio / dominio

Per ogni servizio, definisci un **livello di adozione DFD**:

- **Livello 0 – Nessuna adozione**  
  - Nessuna DoD Estesa formale;  
  - Specifiche informali o assenti;  
  - Review e rollout come prima.

- **Livello 1 – DoD Estesa + nuove feature**  
  - DoD Estesa definita (almeno bozza);  
  - Nuove feature medium/high risk usano specifica DFD e review DFD;  
  - Rollout shift-right per feature nuove.

- **Livello 2 – DoD + review sistematiche**  
  - DoD Estesa stabile e usata;  
  - Tutte le feature (anche low risk) usano almeno DFD light;  
  - Review design e release usano checklist DFD;  
  - Allarmi e dashboard allineati a DoD.

- **Livello 3 – DFD “maturo”**  
  - DoD Estesa evoluta con feedback di produzione;  
  - Guardrail globali chiari e rispettati;  
  - Metriche di efficacia DFD (riduzione incidenti, MTTR, ecc.);  
  - DFD integrato nel flusso (CI, ticket, repo).

Per ogni servizio, decidi:

- qual è il livello attuale;  
- qual è il livello target nei prossimi 3–6 mesi;  
- quali passi concreti servono per salire di livello.

### 4.2 Interventi sull’esistente

Sul codice / servizi esistenti, DFD si integra in modo opportunistico:

1. **Quando si tocca un componente**  
   - Se stai modificando un endpoint / flusso:
     - verifica se ci sono metriche / log coerenti con la DoD;
     - se mancano, aggiungili come parte del lavoro;
     - aggiorna la DoD Estesa se necessario.

2. **Post-incidente / post-mortem**  
   - Dopo un incidente:
     - chiedi: “Quale criterio DoD avrebbe potuto prevenire / ridurre l’impatto?”;
     - aggiungi quel criterio alla DoD;
     - implementa allarmi / test derivati da quel criterio.

3. **Refactor significativi**  
   - Quando rifai una parte di architettura:
     - scrivi una specifica DFD per il “to-be”;
     - usa review DFD;
     - definisci rollout shift-right per il nuovo design.

4. **Nuovi servizi / moduli**  
   - Per nuovi servizi:
     - parti con DFD full da subito;
     - DoD Estesa definita prima di scrivere codice;
     - specifica DFD per le prime feature.

---

## 5. Fase 3 – Istituzionalizzazione (DFD come “modo di lavorare”)

Qui DFD diventa parte del modo di lavorare dell’organizzazione.

### 5.1 Integrazione con processi esistenti

- **Ticket / epic**:
  - campo “Rischio DFD” (low/medium/high);
  - link alla specifica DFD (se richiesta);
  - link alla checklist di review.

- **CI / CD**:
  - per feature medium/high risk:
    - check che esista una specifica DFD;
    - check che esista una checklist di review compilata;
    - eventualmente, check che esista una configurazione rollout.

- **Repo**:
  - cartella `/dfd` per servizio con:
    - DoD Estesa;
    - specifiche di feature;
    - configurazioni rollout;
    - note di review.

### 5.2 Onboarding e formazione

- Onboarding nuovi arrivati:
  - sessione su DFD (fondamenta, processo, esempi);
  - uso delle skill AI (context pack + skill stateless o tool come SpecKit).

- Gilda / community:
  - sessioni periodiche su:
    - casi d’uso DFD;
    - incidenti dove DFD ha aiutato;
    - miglioramenti al framework.

### 5.3 Metriche di efficacia DFD

Per giustificare e affinare DFD, misura:

- riduzione di:
  - incidenti in produzione;
  - regressioni;
  - MTTR (mean time to repair).
- miglioramento di:
  - tempo di onboarding nuovi sviluppatori;
  - chiarezza nelle review (meno “ma questo era stato pensato?”);
  - qualità dei rollout (meno rollback, meno hotfix).

---

## 6. Regole pratiche per non fallire

Per evitare che DFD diventi “un altro processo pesante”:

- **Parti piccolo**: 1–2 servizi pilota, poche feature.  
- **Usa DFD light** dove ha senso (feature low risk).  
- **Non imporre DFD su tutto subito**: lascia che i team vedano valore e chiedano di più.  
- **Tratta DFD come prodotto interno**:
  - raccogli feedback;
  - misura efficacia;
  - adatta il framework.

---

## 7. Schema riassuntivo di integrazione

| Cosa                   | Nuovo (greenfield)                 | Esistente (brownfield)                        |
|------------------------|------------------------------------|-----------------------------------------------|
| DoD Estesa             | Definita da subito                 | Definita gradualmente (partendo da servizi pilota) |
| Specifica di feature   | DFD full o light da subito         | Solo per feature medium/high risk o refactor significativi |
| Review                 | DFD full/light da subito           | Overlay sulle review esistenti, poi sistematico |
| Rollout                | Shift-right da subito              | Introdotto per feature nuove, poi esteso      |
| Codice esistente       | N/A                                | DFD applicato quando si tocca (metriche, log, test) |
| Post-incidente         | N/A                                | Usato per arricchire DoD e allarmi            |

---

## 8. Prossimi passi concreti

Per mettere in pratica questa guida:

1. **Fai l’assessment** (Sezione 2) per i tuoi servizi principali.  
2. **Scegli 1–2 servizi pilota** e 3–5 feature su cui applicare DFD.  
3. **Definisci una DoD Estesa “good enough”** per ciascun servizio pilota.  
4. **Usa le checklist DFD** (full / light) come overlay sulle review che già fate.  
5. **Raccogli feedback** dopo 1–2 mesi e adatta il processo.

Per supporto:

- [Fondamenta DFD](fondamenta.md) – per rivedere principi e artefatti.  
- [Processo End-to-End](processo-e2e.md) – per allineare fasi e gate.  
- [Criteri di Rischio](rischio.md) – per classificare feature e scegliere full vs light.  
- [Modello RACI](raci.md) – per chiarire ruoli e responsabilità.
