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
   - Usa il [template DoD Estesa](templates/dod-estesa.md).  
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

- Tutti i test applicabili verdi, con suite finale locale in solo sviluppo e CI secondo la policy motivata del dominio in preparazione al rilascio.
- Copertura test adeguata (secondo standard del team).
- Log e metriche allineati alla DoD.

**Gate**:

- [ ] Sviluppo completato in solo sviluppo; feature pronta per pre-release DFD solo in preparazione al rilascio.

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

- [ ] **Tutti i test applicabili verdi**, con evidenze locali/CI secondo la policy di rilascio del dominio (unitari, integrazione, E2E).  
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

## Maturità del progetto: solo sviluppo e preparazione al rilascio

La maturità del progetto è distinta dal livello di adozione DFD e dal rischio della feature. La Fase 0 dichiara una modalità, la scelta sulla CI e la relativa motivazione:

| Modalità | Verifiche di sviluppo | Uscita |
|---|---|---|
| Solo sviluppo (`development-only`) | Light/full: review di design, TDD e suite finale locale; rapido: verifiche pertinenti. CI facoltativa | Sviluppo completato; nessuna pre-release o autorizzazione a distribuire |
| Preparazione al rilascio (`release-preparation`) | Stessi controlli, con CI richiesta per default; un’alternativa locale richiede motivazione rispetto a rischio e destinazione e review umana | Prontezza alla pre-release, poi review di release distinta |

L’assenza della CI non è una lacuna bloccante in solo sviluppo. Restano obbligatorie le verifiche applicabili e la tracciabilità di comando, data, risultato e log: un test locale non viene dichiarato CI. Quando prevista, la suite finale deve seguire i test green. Una CI dichiarata fallita continua a bloccare anche quando facoltativa.

Prima di distribuire, passare esplicitamente alla preparazione al rilascio: rivalutare rischio, criteri, policy CI e verifiche nell’ambiente di destinazione. Riconfermare review e aggiornare piano ed evidenze invalidati. La modalità solo sviluppo non attenua controlli di sicurezza o rischio; il completamento locale non sostituisce l’approvazione umana del rilascio. Il learning può provenire anche dagli esperimenti locali, prima di disporre di dati di produzione.

## Percorsi proporzionati e lavoro individuale o in team

DFD si applica sia al singolo sviluppatore sia a un team. La maturità del progetto (solo sviluppo / preparazione al rilascio), il rischio dell’intervento e l’organizzazione del lavoro sono scelte distinte. Il team non è un prerequisito.

| Percorso | Quando | Minimo utile |
|---|---|---|
| Rapido | Fix che ripristina un contratto noto o modifica editoriale; ambito circoscritto, rischio interamente low, nessun contratto/guardrail cambiato, nessun comportamento critico o dubbio aperto | Un record con problema, ambito, owner, rischio motivato, criteri pertinenti e verifiche reali |
| Light | Funzionalità contenuta, fix non ammissibile al rapido o dubbi da chiarire, senza condizioni full | Specifica breve, criteri applicabili, review pertinente ed evidenze |
| Full | Almeno una dimensione high, almeno due medium o nuovo servizio | Analisi, design e verifiche approfonditi secondo il rischio; preparazione operativa quando si distribuisce |

Il rapido è una scelta esplicita per interventi ammissibili, non il risultato del numero di righe modificate. Una sola medium esclude il rapido e porta almeno a light. Rischio ignoto blocca la scelta; dubbi o variazioni di contratto richiedono chiarimento o promozione. Un comportamento critico richiede full completo. La CLI propone il percorso con i motivi; il manutentore può correggere la valutazione documentandone i fatti, senza override silenziosi dei gate.

### Fix rapidi e test che prevengono danni

Per un fix comportamentale serve una regressione che riproduca il bug, fallisca per il requisito atteso prima della correzione e passi dopo. Un errore di compilazione o ambiente non vale come red. Per una modifica editoriale basta una verifica pertinente di contenuto, link o output: nessun test artificiale. Collegare i criteri realmente coinvolti e motivare il perimetro delle verifiche; non imporre suite completa o mutation test a ogni correzione.

Dare priorità a perdita di dati, autorizzazioni, pagamenti, compatibilità, scritture parziali, retry e rollback. Verificare anche gli effetti del fallimento: un’operazione rifiutata deve preservare i dati; retry e richieste duplicate non devono duplicare effetti. Gli interventi critici escono dal rapido anche se minuscoli. Usare mutanti mirati sugli invarianti importanti per verificare che una protezione rimossa venga rilevata. Numero di test e copertura percentuale da soli non dimostrano protezione dai bug gravi.

Ogni bug importante sfuggito in produzione deve produrre una verifica capace di intercettarlo al livello efficace (unitario, integrazione o end-to-end); aggiornare i criteri DFD quando il problema è sistemico. Il learning degli esperimenti locali resta utile prima della produzione.

### Un solo record e riuso delle decisioni

Il rapido non richiede specifica, piano e report di design separati: usare il [record rapido](templates/cambiamento_rapido.md). Una PR può semplicemente riferirsi a quel record, senza duplicarlo. Con la CLI il record macchina è `.dfd/changes/<id>.json`; log e file verificati sono evidenze di supporto, non nuovi documenti di processo. Riprendere il design già approvato indicato nel contratto noto; non riaprire il design per un ripristino circoscritto.

Il singolo dev può completare il rapido con record ed evidenze, senza PR, secondo reviewer o nuovo gate di approvazione del design. `change-verified` è un esito tecnico, non una decisione umana simulata. Se il team richiede review indipendente, la review ordinaria della PR soddisfa la review del cambiamento: registrarla una volta, con persona, riferimento, revisione e snapshot. Una review registrata volontariamente resta comunque vincolata a ciò che è stato esaminato.

### Collaborazione facoltativa e integrazione

Versionare DoD, guardrail e policy comuni; assegnare ID distinti e owner per intervento. I riferimenti a issue, branch, PR e revisione sono opzionali per il lavoro individuale e richiesti quando la policy del team li rende necessari. I ruoli sono assegnabili per intervento: il reviewer può essere un collega delegato secondo la policy, senza dipendere sempre dalla stessa persona. Una review di PR può soddisfare il gate DFD pertinente: non chiedere due approvazioni equivalenti.

Gli artefatti delle feature restano separati: evitare uno stato centrale da modificare per ogni fix. Il lock della CLI protegge una cartella locale, non coordina cloni diversi. Usare Git e review dei conflitti; non risolvere automaticamente conflitti tra decisioni o evidenze scegliendo l’ultima scrittura. Dopo il merge, rivalutare snapshot e prove interessate.

Prima della distribuzione, la policy team può richiedere review indipendente e prove sulla versione integrata corrente: i test verdi di due branch isolati non dimostrano che i due cambiamenti funzionino insieme. La verifica deve coprire i file dell’intervento, avere revisione coerente, comandi, esiti, date e log reali. Un hash non autentica una persona o un commit: la review semantica verifica origine, completezza dei test e corrispondenza della revisione dichiarata.

### Review selettive ed evoluzione del setup

Per le nuove feature, i contratti normativi dei criteri sono nel catalogo strutturato e i vincoli nei guardrail e nella policy lifecycle. Il fingerprint considera specifica, rischio, criteri valutati e vincoli operativi pertinenti; correzioni ai template, convenzioni descrittive, aggiunta di domini o lacune baseline estranee non invalidano da sole il design. Il catalogo e i guardrail devono contenere tutte le prescrizioni operative: non introdurre obblighi soltanto nel testo di processo escluso dal fingerprint. Nuovi criteri changes da valutare o modifiche ai contratti pertinenti riaprono il design.

Cambiare le regole comuni tramite una modifica revisionata, indicando le feature interessate e le eventuali azioni. I guardrail comuni restano conservativi: una modifica normativa può coinvolgere tutto il dominio. La CLI non distingue semanticamente un refuso da una modifica di sicurezza nei file normativi. Le feature legacy conservano i fingerprint precedenti; non migrare approvazioni automaticamente.

Il rapido conclude lo sviluppo e non autorizza una distribuzione: quando si decide di rilasciare, riprendere/promuovere una feature light/full che riferisce il record rapido, rivalutando policy e integrazione prima della review di release.
