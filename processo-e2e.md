# Processo End-to-End – DoD-First Driven (DFD)

> **Stato**: draft / stabile  
> **Ultimo aggiornamento**: 2026-10-08  
> **Proprietari**: [@team / @persona]  
> **Riferimenti**:
> - [Documento “vivo” DFD]
> - [DoD Estesa – Servizio X]
> - [Checklist di Review DFD]

Questo documento descrive il **processo end-to-end** per sviluppare e rilasciare feature con DFD, dalla nascita dell’idea alla valutazione post-release.

---

## 0. Fase 0 – Setup DFD di dominio

Prima di applicare DFD a un prodotto / dominio, è necessario definire un **setup DFD locale**.  
Questa non è una fase di una singola feature, ma un **prerequisito di dominio**.

### Obiettivo

Definire gli “strumenti del gioco” per usare DFD in modo coerente in questo dominio:

- come sono fatte le DoD Estese;
- come sono fatte le Specifiche di Feature;
- quali guardrail globali valgono;
- come si usano template e checklist nel processo.

### Artefatti del setup

Almeno:

1. **DoD Estesa di dominio (bozza iniziale)**  
   - Usa il [template DoD Estesa](templates/dod_estesa.md).  
   - Adattalo al tuo dominio (es. Checkout, Auth, Reporting).  
   - Definisci:
     - assi rilevanti (Osservabilità, Sicurezza, SLO, Costo, UX, ecc.);
     - convenzioni di naming dei criteri (OBS-01, SEC-01, SLO-01, COST-01, …);
     - eventuali criteri “standard” per questo tipo di servizio.

2. **Guardrail globali di dominio**  
   - Principi architetturali (es. isolamento tenant, tracciabilità end-to-end).  
   - Vincoli di sicurezza (es. nessun PII in chiaro nei log, audit minimo).  
   - SLO trasversali (es. disponibilità minima per servizi critici).  
   - Linee guida di osservabilità (metriche minime, log minimi).

3. **Adattamento template DFD**  
   - Decidi se:
     - usi i template così come sono;
     - o li adattate (es. aggiungi campi specifici al tuo contesto).  
   - Documenta eventuali differenze rispetto al framework centrale.

4. **Convenzioni di processo**  
   - Come si classificano le feature (low / medium / high risk).  
   - Quali checklist usare (full / light) in base al rischio.  
   - Dove vivono gli artefatti DFD (repo, wiki, cartelle per servizio).

### Quando aggiornare il setup

Il setup non è un documento “una tantum”. Va aggiornato quando:

- cambi il dominio / architettura in modo significativo;  
- emergono pattern ricorrenti non coperti;  
- incidenti / learning mostrano che alcuni criteri DoD mancano o sono sbagliati;  
- il team vuole semplificare / rafforzare il processo.

### Relazione con il processo end-to-end

Una volta definito il setup:

- ogni **epic / feature** del dominio usa:
  - la DoD Estesa di dominio come riferimento;
  - i guardrail globali come vincoli;
  - i template e le checklist adattati.

Il processo end-to-end (Idea → Post-release) **presuppone** che il setup esista.  
Se manca, la prima attività da fare è proprio la Fase 0.

## 1. Panoramica

Il processo è suddiviso in **8 fasi**:

0. Setup DFD di dominio (prerequisito di dominio) 
1. Idea / Epic  
2. Design DFD  
3. Review DFD (design)  
4. Sviluppo (TDD DFD)  
5. Pre-release DFD  
6. Review DFD (release)  
7. Rilascio shift-right  
8. Post-release DFD  

Ogni fase ha:

- **Input**: artefatti necessari per iniziare.  
- **Attività**: cosa viene fatto.  
- **Output**: artefatti prodotti.  
- **Criteri di uscita**: condizioni per considerare la fase chiusa.  
- **Gate**: punto di controllo (spesso con checklist DFD).

---

## 2. Fasi del processo

### 2.1 Idea / Epic

**Obiettivo**: definire l’opportunità / problema e il valore atteso.

**Input**:

- Richiesta business / insight prodotto / feedback utenti.

**Attività**:

- Definizione dell’epic (o equivalente nel tool di gestione lavoro).
- Stima ad alto livello di impatto e complessità.
- Identificazione del dominio / servizio principale coinvolto.

**Output**:

- Epic / ticket con:
  - descrizione del problema / opportunità;
  - obiettivi business (KPI);
  - stima ad alto livello.

**Criteri di uscita**:

- Epic approvata dal Product / PM.
- Dominio / servizio principale identificato.

**Gate**:

- [ ] Epic pronta per entrare nel backlog di prodotto.

---

### 2.2 Design DFD

**Obiettivo**: tradurre l’epic in una **specifica di feature** allineata alla DoD.

> **Nota**: non esiste un documento separato di “Design DFD”.  
> Il design è contenuto nella **Specifica di Feature**, che funge da artefatto principale di design in ottica DFD.

**Input**:

- Epic approvata.
- DoD Estesa del servizio / dominio.
- Architecture Guardrails.

**Attività**:

- Scrittura della **Specifica di Feature** (template DFD):
  - descrizione funzionale;
  - allineamento ai criteri DoD (OBS-xx, SEC-xx, SLO-xx, COST-xx, UX-xx);
  - identificazione di guardrail coinvolti;
  - primi abbozzi di test e validazione.

**Output**:

- Specifica di Feature (draft).
- Eventuali ADR / note architetturali (se servono decisioni).

**Criteri di uscita**:

- Specifica completa nei campi principali.
- Criteri DoD rilevanti identificati e citati.
- Guardrail verificati (nessuna violazione nota).

**Gate**:

- [ ] Specifica pronta per review DFD (design).

---

### 2.3 Review DFD (design)

**Obiettivo**: validare che la specifica sia coerente con DFD e DoD.

**Input**:

- Specifica di Feature (draft).
- DoD Estesa.
- Architecture Guardrails.

**Attività**:

- Review collettiva (Tech Lead, Product, Dev, QA, Ops, Security come necessario).
- Uso della **Checklist di Review DFD** (sezione design).
- Identificazione di:
  - lacune nella specifica;
  - rischi architetturali / di sicurezza;
  - punti aperti da chiarire.

**Output**:

- Specifica di Feature:
  - approvata;
  - approvata con condizioni;
  - da rivedere.
- Lista di azioni / chiarimenti richiesti.

**Criteri di uscita**:

- Specifica almeno “approvata con condizioni”.
- Azioni residue chiare e assegnate.

**Gate**:

- [ ] OK a procedere con lo sviluppo (con o senza condizioni).

---

### 2.4 Sviluppo (TDD DFD)

**Obiettivo**: implementare la feature seguendo TDD, con i test guidati dalla DoD.

**Input**:

- Specifica di Feature approvata.
- DoD Estesa.

**Attività**:

- Progettazione dei test:
  - unitari, di integrazione, end-to-end;
  - test non funzionali (carico, resilienza, sicurezza) se rilevanti.
- Ciclo TDD:
  - test rosso → codice verde → refactor.
- Implementazione della strumentazione:
  - log strutturati;
  - metriche custom (se necessarie);
  - trace ID.

**Output**:

- Codice implementato.
- Suite di test (unitari, integrazione, ecc.).
- Strumentazione osservabilità (log, metriche, trace).

**Criteri di uscita**:

- Tutti i test verdi in CI.
- Copertura test adeguata (secondo standard del team).
- Log e metriche allineati alla DoD.

**Gate**:

- [ ] Feature pronta per pre-release DFD.

---

### 2.5 Pre-release DFD

**Obiettivo**: preparare il rilascio in produzione in ottica shift-right.

**Input**:

- Codice e test pronti.
- Specifica di Feature.
- DoD Estesa.

**Attività**:

- Definizione della **Configurazione Rollout**:
  - fasi di rollout;
  - criteri di promozione / stop / rollback;
  - allarmi e dashboard.
- Configurazione di:
  - feature flag (se usati);
  - allarmi CloudWatch (o equivalente);
  - dashboard di monitoraggio.
- Verifica che:
  - i test non funzionali siano stati eseguiti (carico, resilienza, sicurezza);
  - le soglie di allarme siano coerenti con la DoD.

**Output**:

- Configurazione Rollout (documento / file di config).
- Allarmi e dashboard configurati (o ticket per configurarli).
- Piano di comunicazione a Supporto / stakeholder.

**Criteri di uscita**:

- Configurazione Rollout completa.
- Allarmi e dashboard pronti (o piano chiaro per attivarli).
- Supporto / stakeholder informati.

**Gate**:

- [ ] Feature pronta per review DFD (release).

---

### 2.6 Review DFD (release)

**Obiettivo**: validare che il rilascio sia preparato in modo coerente con DFD.

**Input**:

- Specifica di Feature (aggiornata).
- Configurazione Rollout.
- DoD Estesa.

**Attività**:

- Review collettiva (Tech Lead, Dev, Ops, Product, Security come necessario).
- Uso della **Checklist di Review DFD** (sezione release).
- Verifica di:
  - coerenza tra specifica, test e configurazione rollout;
  - soglie di allarme e criteri di rollback;
  - impatto su SLO, costo, UX.

**Output**:

- Decisione:
  - OK a rilasciare;
  - OK con condizioni;
  - da rivedere.
- Eventuali azioni residue.

**Criteri di uscita**:

- Almeno “OK con condizioni”.
- Azioni residue chiare e assegnate.

**Gate**:

- [ ] OK a iniziare il rollout in produzione.

---

### 2.7 Rilascio shift-right

**Obiettivo**: rilasciare la feature in produzione con canary e rollout controllati.

**Input**:

- Approvazione dalla review DFD (release).
- Configurazione Rollout.

**Attività**:

- Esecuzione del rollout secondo le fasi definite:
  - canary tecnico (se previsto);
  - rollout per tenant / segmento.
- Monitoraggio attivo di:
  - metriche tecniche (error rate, latency);
  - metriche business (tasso di completamento, KPI);
  - costo (se rilevante).
- Decisioni di:
  - promozione alla fase successiva;
  - stop;
  - rollback parziale o totale.

**Output**:

- Rollout eseguito (completato o interrotto).
- Decision log (promozioni, stop, rollback).
- Eventuali incidenti / note operative.

**Criteri di uscita**:

- Rollout completato con successo **oppure**
- Rollout interrotto con decisione documentata e piano di mitigazione.

**Gate**:

- [ ] Feature considerata “rilasciata” (anche se con rollback).

---

### 2.8 Post-release DFD

**Obiettivo**: consolidare i learning e aggiornare DoD / guardrail se necessario.

**Input**:

- Esito del rollout.
- Metriche e feedback post-release.

**Attività**:

- Analisi dei risultati:
  - rispetto agli obiettivi business;
  - rispetto a SLO, costo, UX.
- Identificazione di:
  - cosa ha funzionato bene;
  - cosa ha creato attrito;
  - eventuali incidenti / near-miss.
- Proposte di:
  - aggiornamento della DoD Estesa;
  - nuovi guardrail;
  - miglioramenti al processo DFD.

**Output**:

- Nota interna / post-mortem (se necessario).
- Proposte di modifica a:
  - DoD Estesa;
  - Architecture Guardrails;
  - Processo DFD (se rilevante).

**Criteri di uscita**:

- Learning documentati e condivisi.
- Azioni di miglioramento identificate (con owner).

**Gate**:

- [ ] Feature chiusa dal punto di vista DFD.

---

## 3. Definition of Ready (DoR)

Le **Definition of Ready (DoR)** definiscono i criteri minimi perché un lavoro possa entrare in una fase successiva.  
Si applicano solo ai tre passaggi chiave: **Idea→Design**, **Design→Sviluppo**, **Pre-release→Rilascio**.

---

### 3.1 DoR per entrare in Design DFD

Prima di iniziare il design di una feature, l’epic deve soddisfare i seguenti criteri minimi:

- [ ] **Obiettivo business chiaro** (1–2 frasi: che problema risolve / che opportunità sfrutta).  
- [ ] **KPI / metriche di successo** almeno ad alto livello (es. “aumento conversione checkout”, “riduzione ticket supporto”).  
- [ ] **Dominio / servizio principale** identificato (es. Checkout, Pagamenti, Catalogo).  
- [ ] **Stima ad alto livello** di complessità (T-shirt size o simile).  
- [ ] **Priorità** definita nel backlog di prodotto.  
- [ ] **Epic approvata** dal Product Manager / Product Owner.

Se questi criteri non sono soddisfatti, l’epic resta in raffinazione prodotto e non entra in Design DFD.

---

### 3.2 DoR per entrare in Sviluppo (TDD DFD)

Prima di iniziare lo sviluppo di una feature, la Specifica di Feature deve soddisfare i seguenti criteri minimi:

- [ ] **Specifica di Feature** compilata nei campi principali:
  - scenari / user story;
  - interfacce / API / eventi;
  - dipendenze esterne.
- [ ] **Criteri DoD rilevanti** identificati e citati (OBS-xx, SEC-xx, SLO-xx, COST-xx, UX-xx).  
- [ ] **Guardrail globali** verificati (nessuna violazione nota).  
- [ ] **Rischi principali** identificati (architetturali, sicurezza, affidabilità).  
- [ ] **Approvazione in linea di principio** da parte del Tech Lead / Architect (anche informale, prima della review formale).  
- [ ] **Review DFD (design)** almeno “approvata con condizioni” (nessun blocco architetturale / di sicurezza aperto).

Se questi criteri non sono soddisfatti, la feature non entra in sviluppo; si completa prima il design.

---

### 3.3 DoR per entrare in Rilascio shift-right

Prima di iniziare un rollout in produzione, la feature deve soddisfare i seguenti criteri minimi:

- [ ] **Tutti i test verdi** in CI (unitari, integrazione, E2E).  
- [ ] **Test non funzionali** eseguiti (se rilevanti: carico, resilienza, sicurezza).  
- [ ] **Configurazione Rollout** definita:
  - fasi di rollout;
  - criteri di promozione / stop / rollback;
  - allarmi e dashboard.
- [ ] **Allarmi e dashboard** configurati (o piano chiaro per attivarli).  
- [ ] **Supporto / stakeholder** informati (almeno i team chiave: Support, Ops, Security se rilevante).  
- [ ] **Review DFD (release)** almeno “OK con condizioni” (nessun blocco tecnico / di sicurezza aperto).

Se questi criteri non sono soddisfatti, la feature non entra in rollout; si completa prima la pre-release.

---

## 4. Rappresentazione sintetica del flusso

```text
Idea/Epic
   ↓
Design DFD
   ↓
Review DFD (design)
   ↓
Sviluppo (TDD DFD)
   ↓
Pre-release DFD
   ↓
Review DFD (release)
   ↓
Rilascio shift-right
   ↓
Post-release DFD
```

---

## 5. Eccezioni e semplificazioni

Per feature piccole / a basso rischio, il processo può essere **semplificato**:

- Design DFD e review DFD (design) possono essere leggeri (checklist ridotta).
- Rollout può essere diretto (senza canary), se il rischio è basso e i guardrail lo consentono.

Le regole di semplificazione devono essere:

- esplicite (es. “feature che non toccano pagamento, sicurezza o SLO critici”);
- approvate dal Tech Lead / Architect.
