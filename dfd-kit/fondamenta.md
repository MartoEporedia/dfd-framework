# DoD-First Driven (DFD) – Fondamenta

> **Stato**: draft / stabile  
> **Ultimo aggiornamento**: 2026-10-08  
> **Proprietari**: [@team / @persona]  
> **Riferimenti**:
> - [Processo End-to-End](processo-e2e.md)
> - [Criteri di Rischio](rischio.md)
> - [Modello RACI](raci.md)
> - [Template DoD Estesa](templates/dod-estesa.md)
> - [Template Specifica di Feature](templates/specifica_feature.md)
> - [Template Configurazione Rollout](templates/configurazione_rollout.md)
> - [Esempio DoD – Checkout AWS ECS](examples/checkout_dod.md)
> - [Esempio Specifica – PayPal](examples/checkout_paypal_spec.md)

Questo documento definisce le **fondamenta** di **DoD-First Driven (DFD)**, un approccio in cui i criteri di **Definition of Done (DoD) estesa** guidano:

- la specifica delle feature (Spec-Driven Design);
- la progettazione e l’esecuzione dei test (Test-Driven Development);
- la valutazione delle release in produzione (shift-right, canary, rollout controllati).

Il documento è **vivo**: evolve in base agli esperimenti, ai feedback e ai dati di produzione.

---

## 1. Idea centrale

DFD si basa su un’idea semplice:

> La **Definition of Done** non è una checklist finale, ma un **contratto continuo** usato in tre momenti: prima, durante e dopo il rilascio.

In DFD, la DoD è **estesa** rispetto alla forma classica: include in modo esplicito e prioritario:

- **Osservabilità** (metriche, log, trace);
- **Sicurezza** (accesso, audit, protezione dati);
- **Affidabilità / SLO**;
- **Costo** (per servizio, per transazione, eventualmente per tenant).

Questi assi diventano il filo conduttore tra:

- **Specifica** (cosa costruiamo e con quali vincoli);
- **Test** (come verifichiamo che sia fatto “bene”);
- **Rilascio** (come validiamo che funzioni in produzione senza rompere il sistema globale).

Per il processo dettagliato, vedi [Processo End-to-End](processo-e2e.md).

---

## 2. Principi

### 2.1 DoD come contratto

- I criteri di Done sono **espliciti, riferibili e usati** in tutte le fasi.
- Ogni feature deve mostrare come soddisfa i criteri DoD rilevanti.
- La DoD è “estesa”: non solo “funziona e ha i test”, ma anche “è osservabile, sicura, affidabile e sostenibile in termini di costo”.

### 2.2 Specifica prima del codice

- Si parte da una **specifica chiara** (comportamenti, interfacce, vincoli).
- La specifica cita esplicitamente i **criteri DoD** che intende soddisfare.
- La specifica opera entro i **guardrail globali** del prodotto (architettura, sicurezza, osservabilità, costo).

### 2.3 Test derivati dalla DoD

- Ogni criterio DoD rilevante ha almeno un **test automatico** associato.
- I test non verificano solo “funziona”, ma “rispetta la DoD”.
- Il ciclo TDD (test → codice → refactor) è guidato dai criteri DoD.

### 2.4 Validazione continua (shift-right)

- La DoD guida anche la **valutazione in produzione**: canary, rollout per tenant, esperimenti.
- Gli **allarmi** e le metriche di rilascio sono derivati dai criteri DoD.
- Il rilascio è un **esperimento controllato**, non un “speriamo non esploda”.

### 2.5 Guardrail globali, spec locali

- Esiste un **quadro globale** (architettura, sicurezza, osservabilità, costo) che definisce i confini.
- Le spec locali si muovono dentro questi guardrail e **non possono violarli**.
- Eventuali eccezioni devono essere **esplicite e documentate**.

### 2.6 Feedback loop

- I dati di produzione (metriche, errori, costo, feedback) alimentano il **miglioramento della DoD**.
- La DoD è **viva**: si evolve in base a ciò che si impara in produzione.
- Il framework stesso (DFD) evolve in base agli esperimenti.

---

## 3. Artefatti principali

### 3.1 DoD Estesa

Documento (o sezione) che definisce i criteri di Done per il prodotto / dominio, includendo almeno:

- **Osservabilità** (metriche, log, trace, dashboard);
- **Sicurezza** (accesso, audit, protezione dati);
- **Affidabilità / SLO**;
- **Costo** (per servizio, per transazione);
- Eventuali altri assi (UX, compliance, ecc.).

La DoD Estesa è il **riferimento centrale** di DFD.

- Template: [DoD Estesa](templates/dod-estesa.md)
- Esempio: [Checkout AWS ECS](examples/checkout_dod.md)

### 3.2 Specifica di Feature

Per ogni feature o servizio significativo:

- Descrizione funzionale (scenari, casi d’uso);
- Confini architetturali e dipendenze;
- Riferimenti espliciti ai **criteri DoD** soddisfatti (OBS-xx, SEC-xx, SLO-xx, COST-xx, UX-xx);
- Eventuali eccezioni ai guardrail globali (con motivazione).

- Template: [Specifica di Feature](templates/specifica_feature.md)
- Esempio: [PayPal su Checkout](examples/checkout_paypal_spec.md)

### 3.3 Suite di test

- Test unitari, di integrazione, sicurezza, osservabilità.
- Mappatura (anche informale) tra **criteri DoD** e test.
- Eventuali test di carico / stress per SLO.
- Eventuali test sintetici / end-to-end per flussi critici.

### 3.4 Configurazione di rilascio

- Policy di **canary** (soglie, durata, criteri di rollback).
- Rollout per **tenant / segmento** (feature flag, AppConfig, ecc.).
- Allarmi derivati dalla DoD (error rate, latency, errori di sicurezza, variazioni di costo).
- Criteri di promozione / stop del rollout.

- Template: [Configurazione Rollout](templates/configurazione_rollout.md)

### 3.5 Quadro globale (guardrail)

Documento (o insieme di documenti) che definisce:

- **Principi architetturali**;
- **Contratti globali** (API, eventi di dominio, schemi dati chiave);
- **SLO e vincoli trasversali**;
- **Linee guida su osservabilità e sicurezza**.

Le spec locali devono **riferirsi esplicitamente** a questo quadro.

### 3.6 Setup DFD di dominio

Oltre agli artefatti per singola feature (DoD Estesa, Specifica, Rollout), DFD prevede un **setup di dominio**:

- una DoD Estesa di dominio (bozza iniziale + evoluzione);  
- guardrail globali specifici del dominio;  
- eventuale adattamento dei template DFD al contesto locale;  
- convenzioni di processo (come si usa DFD in questo dominio).

Il setup non è un documento “una tantum”: evolve con il dominio e con i learning di produzione.  
Per i dettagli, vedi [Processo End-to-End – Fase 0](processo-e2e.md#0-fase-0--setup-dfd-di-dominio).

---

## 4. Processo e ruoli

### 4.1 Processo End-to-End

Il processo DFD è descritto in dettaglio in:

- [Processo End-to-End](processo-e2e.md)

Include:

- fasi (Setup DFD di dominio, Idea/Epic, Design, Sviluppo, Pre-release, Rilascio, Post-release);
- gate di review (design, release);
- **Definition of Ready (DoR)** per i passaggi chiave.

### 4.2 Ruoli e responsabilità

I ruoli e le responsabilità sono definiti in:

- [Modello RACI](raci.md)

In sintesi:

- **PM**: obiettivi business, KPI, priorità.
- **Tech Lead / Architect**: coerenza architetturale, rispetto guardrail, approvazione design/release.
- **Dev**: specifica, codice, test, strumentazione osservabilità.
- **QA / SDET**: test non funzionali, validazione.
- **Ops / SRE**: allarmi, dashboard, supporto al rollout.
- **Security**: review sicurezza, audit, protezione dati.
- **Support**: feedback utenti, segnalazioni post-release.

---

## 5. DFD full vs DFD light

Non tutte le feature hanno lo stesso rischio. DFD prevede due “modalità”:

- **DFD full**: per feature medium / high risk.
- **DFD light**: per feature low risk.

I criteri per scegliere sono definiti in:

- [Criteri di Rischio](rischio.md)

In sintesi:

- **Low Risk** → checklist light (design + release).
- **Medium Risk** → DFD full, applicato in modo proporzionato.
- **High Risk** → DFD full completo, con review allargata.

Template di review:

- [Checklist di Review DFD – Design](templates/review_design.md)
- [Checklist di Review DFD – Design (Light)](templates/review_design_light.md)
- [Checklist di Review DFD – Release](templates/review_release.md)
- [Checklist di Review DFD – Release (Light)](templates/review_release_light.md)

---

## 6. Cosa DFD non è

- Non è un processo **rigido**: la DoD e i guardrail evolvono.
- Non richiede che tutto sia **eseguibile** in forma machine-readable: basta che i criteri siano chiari, riferibili e usati.
- Non è specifico per **AI**: l’AI può essere un acceleratore, ma il framework vale anche in contesti tradizionali.
- Non è un sostituto di **Domain-Driven Design**, **Agile**, **DevOps**: è un layer sopra, che usa la DoD come filo conduttore.

---

## 7. Evoluzione del documento

Questo documento è **vivo**:

- Le modifiche significative vengono tracciate (es. changelog, ADR, note di revisione).
- Gli esperimenti (gilda, tool, progetti pilota) producono feedback che alimentano revisioni.
- La versione più aggiornata è il riferimento per articoli, talk e iniziative interne.

Per proporre modifiche al framework:

- aprire una issue / MR su questo repo;
- o seguire il processo definito in [Criteri di Rischio](rischio.md) (sezione “Eccezioni”).
