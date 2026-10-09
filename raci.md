# Modello RACI – DoD-First Driven (DFD)

> **Stato**: draft / stabile  
> **Ultimo aggiornamento**: YYYY-MM-DD  
> **Proprietari**: [@team / @persona]  
> **Riferimenti**:
> - [Documento “vivo” DFD]
> - [Processo End-to-End DFD]

Questo documento definisce **ruoli e responsabilità** per le fasi del processo DFD, usando una matrice RACI:

- **R (Responsible)**: chi esegue il lavoro.  
- **A (Accountable)**: chi ha la responsabilità finale (uno solo per attività).  
- **C (Consulted)**: chi viene consultato (input richiesti).  
- **I (Informed)**: chi viene informato (per trasparenza).

**Ruoli considerati**:

- **PM** – Product Manager / Product Owner  
- **TL** – Tech Lead / Architect  
- **DEV** – Developer  
- **QA** – QA / SDET  
- **OPS** – Ops / SRE  
- **SEC** – Security Engineer  
- **SUP** – Support / Customer Success  

---

## 1. Matrice RACI per fase

| Fase                        | Attività principale                         | PM   | TL   | DEV  | QA   | OPS  | SEC  | SUP  |
|-----------------------------|---------------------------------------------|------|------|------|------|------|------|------|
| **Idea / Epic**             | Definizione epic e obiettivi business       | A/R  | C    | I    | I    | I    | I    | I    |
|                             | Approvazione epic                           | A    | C    | I    | I    | I    | I    | I    |
| **Design DFD**              | Scrittura Specifica di Feature              | C    | A/R  | R    | C    | C    | C    | I    |
|                             | Identificazione criteri DoD                 | C    | A/R  | R    | C    | C    | C    | I    |
| **Review DFD (design)**     | Review specifica e checklist DFD            | C    | A/R  | R    | R    | C    | C    | I    |
|                             | Decisione approvazione design               | C    | A    | R    | R    | C    | C    | I    |
| **Sviluppo (TDD DFD)**      | Progettazione test                          | I    | C    | A/R  | R    | I    | C    | I    |
|                             | Implementazione codice + test               | I    | C    | A/R  | C    | I    | I    | I    |
|                             | Strumentazione osservabilità                | I    | C    | A/R  | I    | C    | I    | I    |
| **Pre-release DFD**         | Definizione Configurazione Rollout          | I    | A/R  | R    | C    | R    | I    | I    |
|                             | Configurazione allarmi / dashboard          | I    | C    | R    | I    | A/R  | I    | I    |
|                             | Comunicazione a Supporto / stakeholder      | C    | C    | I    | I    | I    | I    | A/R  |
| **Review DFD (release)**    | Review rollout e checklist DFD              | C    | A/R  | R    | R    | R    | C    | C    |
|                             | Decisione OK a rilasciare                   | C    | A    | R    | R    | R    | C    | I    |
| **Rilascio shift-right**    | Esecuzione rollout (fasi, canary, flag)     | I    | C    | R    | I    | A/R  | I    | I    |
|                             | Monitoraggio metriche tecniche e business   | I    | C    | R    | R    | A/R  | I    | I    |
|                             | Decisioni promozione / stop / rollback      | C    | A/R  | R    | R    | R    | I    | I    |
| **Post-release DFD**        | Analisi risultati e learning                | A/R  | R    | R    | R    | R    | I    | C    |
|                             | Proposte aggiornamento DoD / guardrail      | C    | A/R  | R    | R    | R    | C    | I    |
|                             | Documentazione post-mortem / note interne   | A/R  | R    | R    | R    | R    | I    | I    |

---

## 2. Descrizione ruoli

### PM (Product Manager / Product Owner)

- Responsabile di:
  - definire obiettivi business e KPI;
  - approvare epic e priorità;
  - valutare impatto su UX e business KPI post-release.

### TL (Tech Lead / Architect)

- Responsabile di:
  - coerenza architetturale della specifica;
  - rispetto dei guardrail globali;
  - approvazione finale di design e release in ottica DFD.

### DEV (Developer)

- Responsabile di:
  - scrivere specifica (in collaborazione con TL e PM);
  - implementare codice e test (TDD DFD);
  - strumentare osservabilità (log, metriche, trace).

### QA (QA / SDET)

- Responsabile di:
  - progettare test non funzionali (carico, resilienza, sicurezza);
  - supportare la definizione dei criteri di validazione in produzione;
  - partecipare attivamente alle review DFD.

### OPS (Ops / SRE)

- Responsabile di:
  - definire soglie di allarme coerenti con la DoD;
  - configurare e mantenere dashboard e allarmi;
  - gestire il rollout operativo in produzione.

### SEC (Security Engineer)

- Responsabile di:
  - review dei criteri di sicurezza (SEC-xx);
  - validazione di audit e protezione dati;
  - supporto in caso di incidenti di sicurezza.

### SUP (Support / Customer Success)

- Responsabile di:
  - essere informato sui cambiamenti che impattano gli utenti;
  - raccogliere feedback e segnalazioni post-release;
  - contribuire all’analisi di impatto su UX / business KPI.

---

## 3. Note sull’uso del RACI

- Per ogni attività, **c’è un solo Accountable (A)**.  
- I ruoli possono essere ricoperti dalla stessa persona in team piccoli (es. TL + DEV), ma la responsabilità resta distinta.  
- Il RACI va adattato al contesto del team, ma senza eliminare le responsabilità chiave (specialmente A su design, release e post-release).

## Maturità del progetto: solo sviluppo e preparazione al rilascio

La maturità del progetto è distinta dal livello di adozione DFD e dal rischio della feature. La Fase 0 dichiara una modalità, la scelta sulla CI e la relativa motivazione:

| Modalità | Verifiche di sviluppo | Uscita |
|---|---|---|
| Solo sviluppo (`development-only`) | Light/full: review di design, TDD e suite finale locale; rapido: verifiche pertinenti. CI facoltativa | Sviluppo completato; nessuna pre-release o autorizzazione a distribuire |
| Preparazione al rilascio (`release-preparation`) | Stessi controlli, con CI richiesta per default; un’alternativa locale richiede motivazione rispetto a rischio e destinazione e review umana | Prontezza alla pre-release, poi review di release distinta |

L’assenza della CI non è una lacuna bloccante in solo sviluppo. Restano obbligatorie le verifiche applicabili e la tracciabilità di comando, data, risultato e log: un test locale non viene dichiarato CI. Quando prevista, la suite finale deve seguire i test green. Una CI dichiarata fallita continua a bloccare anche quando facoltativa.

Prima di distribuire, passare esplicitamente alla preparazione al rilascio: rivalutare rischio, criteri, policy CI e verifiche nell’ambiente di destinazione. Riconfermare review e aggiornare piano ed evidenze invalidati. La modalità solo sviluppo non attenua controlli di sicurezza o rischio; il completamento locale non sostituisce l’approvazione umana del rilascio. Il learning può provenire anche dagli esperimenti locali, prima di disporre di dati di produzione.

## Responsabilità individuali e collaborazione facoltativa

DFD vale anche per il singolo sviluppatore: i ruoli possono coincidere nella stessa persona. Il rapido individuale richiede record ed evidenze pertinenti, senza PR o secondo reviewer obbligatori. Light/full mantengono le decisioni umane previste; non richiedono di creare un team artificiale.

In un team assegnare owner per intervento e reviewer delegati secondo policy; la review ordinaria della PR può soddisfare il gate DFD pertinente senza un secondo giro equivalente. Riferire decisioni a PR, revisione e snapshot esaminato. La policy può richiedere indipendenza dall’owner e prove della versione integrata corrente prima della distribuzione. La CLI controlla dichiarazioni e hash, senza autenticare identità o commit.

Usare ID distinti, Git e gestione esplicita dei conflitti. Il lock locale non coordina cloni diversi; non risolvere conflitti fra decisioni o evidenze scegliendo automaticamente l’ultima scrittura. Chi cambia le regole comuni identifica feature coinvolte e review da riconfermare. Vedere le [convenzioni facoltative](templates/collaborazione.md) e le [review selettive](fondamenta.md#review-selettive-ed-evoluzione-del-setup).
