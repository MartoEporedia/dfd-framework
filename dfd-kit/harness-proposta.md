# DFD Kit – Integrazione negli harness di coding

> **Stato**: MVP 0.1 implementato; da validare con un progetto pilota  
> **Data**: 2026-10-08  
> **Riferimenti**: [Fondamenta](fondamenta.md), [Processo](processo-e2e.md), [Rischio](rischio.md), [RACI](raci.md)

## 1. Obiettivo e primo utilizzatore

Aiutare un team ad applicare DFD nel proprio repository: definire la DoD del dominio, classificare una feature, produrre una specifica, derivare test e verifiche, preparare il rilascio e registrare i risultati.

Il primo utilizzatore ipotizzato è un Tech Lead o developer che introduce DFD in un progetto esistente. Il percorso deve funzionare anche quando una persona ricopre più ruoli, mantenendo esplicite le responsabilità del RACI.

La distribuzione scelta è un toolkit installabile negli harness già usati dal team: Claude Code, OpenCode, GitHub Copilot e Codex. L’harness esegue il lavoro con i propri modelli, strumenti e permessi; DFD Kit fornisce workflow, template e verifiche. Non serve un runtime autonomo né un servizio LLM aggiuntivo.

## 2. Ipotesi di adozione

**Rischio di adozione: medio**, come valutazione preliminare del prodotto: mancano evidenze di utilizzo e il framework richiede nuove abitudini di review.

L'ipotesi principale è che un workflow guidato riduca il lavoro di coordinamento più di quanto aumenti il lavoro documentale. La validazione minima è accompagnare una feature reale, misurando tempo impiegato, interventi manuali, lacune individuate e uso effettivo degli artefatti in review.

Rinviare dashboard, integrazioni con ticketing, orchestrazione multi-agente e deploy automatici finché il percorso base non dimostra utilità.

## 3. Esperienza proposta

La CLI 0.1 implementa init, install, setup, feature, assess, specify, review-design, decide, plan, verify, pre-release, review-release, status e doctor. Lo sviluppo TDD è guidato da `dfd-implement` nell’harness; rollout e learn descrivono l’evoluzione prevista. Ogni fase sarà una skill con nome stabile, per esempio `dfd-assess` e `dfd-specify`. L’invocazione esplicita segue la sintassi dell’harness; l’installer e il validatore potranno offrire una CLI distinta.

| Operazione | Risultato |
|---|---|
| `dfd init` | Installazione e scaffolding del dominio |
| `dfd setup` | Fase 0: verifica DoD, guardrail, template e convenzioni del dominio |
| `dfd feature` | Apertura della feature guidata dalla skill dedicata |
| `dfd assess` | Rischio per dimensione, motivazioni, dati mancanti e percorso suggerito |
| `dfd specify` | Specifica collegata ai criteri DoD applicabili |
| `dfd review-design` | Controlli di coerenza, criticità e proposta di esito |
| `dfd plan` | Task, test e osservabilità derivati dalla specifica approvata |
| `dfd-implement` (skill) | Ciclo TDD, codice, refactor e raccolta delle evidenze |
| `dfd verify` | Evidenze disponibili e criteri ancora scoperti |
| `dfd pre-release` | Piano di rollout, soglie, rollback e preparazione operativa |
| `dfd review-release` | Controlli e gate della review di release |
| `dfd learn` | Risultati post-release e proposte di revisione della DoD |
| `dfd status` | Fase corrente, condizioni aperte e prossima azione |

Il passaggio dall'idea al design conserva obiettivi, KPI e riferimento all'approvazione dell'epic. Le operazioni possono essere ripetute per aggiornare gli artefatti senza perdere decisioni precedenti.

## 4. Nucleo comune e adapter

Il nucleo contiene skill `SKILL.md`, template versionati, regole di rischio, gestione degli artefatti e controlli deterministici. Una sola definizione di ogni workflow alimenta tutti gli adapter; nomi e descrizioni delle skill restano coerenti. Le risorse vengono caricate per fase, evitando di inserire tutto il framework nel contesto iniziale.

L’installer colloca skill e risorse nei percorsi riconosciuti dall’harness, mantiene un manifest dei file gestiti e rileva conflitti durante gli aggiornamenti. La CLI installa e valida artefatti senza chiamare un modello. Le skill chiedono all’agente dell’harness di svolgere il lavoro.

| Harness | Percorso di skill di progetto documentato | Fonte |
|---|---|---|
| Claude Code | `.claude/skills/<nome>/SKILL.md` | [Claude Code](https://code.claude.com/docs/en/skills) |
| Codex | `.agents/skills/<nome>/SKILL.md` | [OpenAI](https://learn.chatgpt.com/docs/build-skills) |
| OpenCode | `.opencode/skills/<nome>/SKILL.md`; supporta anche `.agents/skills` e `.claude/skills` | [OpenCode](https://docs.opencode.ai/docs/skills/) |
| GitHub Copilot | `.github/skills/<nome>/SKILL.md`; supporta anche `.agents/skills` e `.claude/skills` | [GitHub](https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/customize-cloud-agent/add-skills) |

Questi percorsi costituiscono una base documentata, non una certificazione dell’integrazione. L’installer deve evitare copie duplicate della stessa skill nei percorsi di compatibilità. Ogni adapter richiede prove di discovery, invocazione e accesso alle risorse nelle versioni e superfici supportate. La prima versione installa un adapter attivo per repository per evitare duplicazioni nelle directory di compatibilità. Il cambio di adapter preserva lo stato; hook e packaging specifico sono evoluzioni future.

La struttura proposta del toolkit è:

```text
skills/dfd-feature/SKILL.md
skills/dfd-assess/SKILL.md
skills/dfd-specify/SKILL.md
skills/dfd-review-design/SKILL.md
src/cli.rs
src/model.rs
src/store.rs
Cargo.toml
templates/
examples/
```

La CLI è implementata in Rust e incorpora skill, template e documenti nel binario: l’utilizzatore non deve installare runtime o pacchetti aggiuntivi. Gli adapter sono descritti in `src/cli.rs`; gli schemi versionati sono tipi Rust in `src/model.rs`. Preservare `AGENTS.md`, `CLAUDE.md` e le istruzioni Copilot già presenti nel progetto adottante. Eventuali riferimenti di raccordo devono essere aggiunte circoscritte e riconoscibili, senza sostituire le policy del team.

L'agente propone contenuti e interpreta evidenze. Il validatore controlla identificatori, riferimenti, campi obbligatori e prerequisiti dei gate. I responsabili umani registrano le approvazioni previste dal RACI. Un report dell'agente non equivale a un'approvazione umana.

La struttura proposta nel repository adottante è:

```text
.dfd/
  config.json
  domains/checkout/
    dod.md
    criteria.json
    guardrails.md
    process.md
    templates/
    adoption.json
    assessment.md
  guardrails.md
  features/001-paypal/
    state.json
    risk.md
    spec.md
    design-review.md
    plan.md
    plan.json
    development-review.md
    evidence.json
    rollout.md
    rollout.json
    release-review.md
    release-decisions.json
    release-notes.md
    learning.md
```

Markdown conserva il contesto leggibile; JSON versionato rappresenta stato e riferimenti verificabili. Ogni feature identifica dominio, versione della DoD, percorso, fase e decisioni. Il formato della prima versione è descritto nel [contratto del toolkit](templates/contratto_toolkit.md).

## 5. Tracciabilità e gate

La relazione centrale è **criterio DoD → specifica → task/test → evidenza → verifica del rilascio**. Ogni criterio applicabile deve avere una verifica, oppure un'esclusione motivata sottoposta alla review pertinente.

Un'evidenza registra origine, comando o procedura, esito, data e revisione verificata. Distinguere evidenze automatiche, manuali e assenti. Un collegamento a un test non dimostra che il test sia stato eseguito; una checklist compilata non dimostra che un requisito sia soddisfatto.

Quando cambia la specifica o la revisione verificata, segnalare le evidenze e le review da riconfermare. Le condizioni di approvazione restano visibili e devono indicare responsabile e fase entro cui risolverle.

Nel primo MVP i gate guidano il workflow e producono esiti espliciti. Il blocco tecnico di merge o deploy richiede un'integrazione CI successiva: le sole istruzioni all'agente non lo garantiscono.

## 6. MVP e criteri di accettazione

Prima iterazione: inizializzazione, assessment, specifica, review di design e stato. I quattro adapter condividono lo stesso percorso; il primo pilota operativo valida una singola combinazione di harness e progetto, poi si verifica la portabilità sugli altri. La review di design è il primo traguardo; piano, sviluppo TDD e verifica delle evidenze sono implementati. Pre-release e review di release sono implementate; esecuzione del rollout e apprendimento seguono nelle iterazioni successive.

- L'inizializzazione preserva file e istruzioni esistenti e si può ripetere senza duplicazioni.
- Due feature mantengono artefatti e stato separati.
- La stessa feature si può riprendere in un altro harness leggendo `.dfd/`, senza dipendere dalla cronologia della chat.
- L’installer non duplica le skill nei percorsi di compatibilità e segnala conflitti sui file modificati dal team.
- La classificazione è riproducibile e tratta informazioni mancanti come da chiarire.
- La specifica cita criteri esistenti nella DoD del dominio.
- I prerequisiti mancanti impediscono al workflow di dichiarare un gate superato.
- Le approvazioni umane e le proposte dell'agente sono distinguibili.
- Il pilota completa una review usando gli artefatti prodotti e registra gli interventi manuali.

## 7. Chiarimenti del framework prima dell'automazione

In [rischio.md](rischio.md), una sola dimensione Medio e nessuna Alto soddisfa sia la regola Low sia la regola Medium. La CLI applica in ordine le regole: almeno una Alto → full completo; almeno due Medio → full proporzionato; altrimenti light. La regola esplicita Low prevale quindi sulla definizione generica Medium. Le escalation restano decisioni motivate del team; l’MVP non implementa override automatici.

La fascia di aumento del costo per transazione tra 5% e meno del 10% non ha una classificazione esplicita. Definire una regola prima di automatizzarla; nel frattempo restituire “da chiarire”.

I riferimenti alla DoD sono stati allineati a `templates/dod-estesa.md`. Il template di feature è stato corretto e separato dalla DoD Checkout; l’esempio PayPal è illustrativo e non costituisce un’evidenza reale di test o produzione.

## 8. Riferimento esterno

[Spec Kit](https://github.com/github/spec-kit) mostra un modello di toolkit con processi, template e integrazioni negli agenti. DFD Kit adotta come punto di partenza questa modalità di distribuzione e sviluppa la propria catena di DoD, rischio, evidenze e validazione post-release. Riuso diretto o estensione di Spec Kit restano opzioni da valutare rispetto ai requisiti DFD e alla manutenzione degli adapter.
