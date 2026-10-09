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

- fasi (Idea/Epic, Design, Sviluppo, Pre-release, Rilascio, Post-release);
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
