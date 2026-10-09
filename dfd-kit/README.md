# DFD Kit

DFD usa la Definition of Done come contratto continuo tra specifica, test e validazione in produzione. **DFD Kit** porta il percorso dentro Claude Code, OpenCode, GitHub Copilot e Codex tramite skill condivise e artefatti nel repository.

## Sviluppare questo toolkit con DFD

Il dominio `dfd-kit` è configurato in modalità brownfield nella directory `dfd-kit/`, con skill Codex installate. Per ogni sviluppo riprendere il [setup del dominio](.dfd/domains/dfd-kit/process.md), la [DoD](.dfd/domains/dfd-kit/dod.md) e l’[assessment di adozione](.dfd/domains/dfd-kit/assessment.md), seguendo [CONTRIBUTING.md](CONTRIBUTING.md). Il setup è strutturalmente pronto in solo sviluppo; la CI è rinviata alla valutazione del rilascio e il pilota reale degli harness resta da verificare.

## Provare il toolkit

La CLI `dfd` è un eseguibile Rust con skill e template incorporati. Per usarlo non servono Python, Node, jq o altri runtime. La versione 0.1 copre adozione, setup di dominio, rischio, specifica, review di design, sviluppo TDD, pre-release e review di release. Esecuzione del rollout e apprendimento post-release saranno fasi successive.

La prima build locale è disponibile in `dist/dfd-macos-arm64`. La configurazione CI in `ci/github-actions.yml` prepara le build per gli altri sistemi; per attivarla, copiarla in `.github/workflows/` nella radice del repository. Le build non sono ancora pubblicate come release. Dopo aver ottenuto il binario per il proprio sistema, posizionarlo nel PATH con nome `dfd`, oppure invocarlo tramite il suo percorso.

```sh
dfd doctor
dfd --project /percorso/prodotto init --harness codex --domain checkout --mode brownfield
# Completare DoD, guardrail e convenzioni nella cartella del dominio
dfd --project /percorso/prodotto setup --domain checkout
dfd --project /percorso/prodotto feature 001-paypal --domain checkout --title "PayPal" --scope "Nuovo metodo di pagamento"
```

Su macOS/Linux il binario deve avere il permesso di esecuzione (`chmod +x dfd`); su Windows usare `dfd.exe`. Per compilare da sorgente e contribuire, vedere [CONTRIBUTING.md](CONTRIBUTING.md).

`init` prepara `.dfd/` e installa dodici skill: `dfd-init`, `dfd-setup`, `dfd-feature`, `dfd-assess`, `dfd-specify`, `dfd-review-design`, `dfd-plan`, `dfd-implement`, `dfd-verify`, `dfd-pre-release`, `dfd-review-release`, `dfd-status`. La Fase 0 è il riferimento condiviso del dominio, paragonabile alla constitution di SDD: definisce DoD, guardrail, template locali e convenzioni di processo. Compilare l'assessment di adozione e gli artefatti in `.dfd/domains/<dominio>/`, poi verificare con `dfd setup --domain <dominio>`. `specify` richiede un setup strutturalmente pronto; idea e rischio possono essere raccolti prima. Le lacune legacy fuori ambito restano nel piano di adozione. Le skill guidano l'agente dell'harness; la CLI esegue i controlli deterministici.

| Harness | Opzione | Directory di installazione |
|---|---|---|
| Claude Code | `--harness claude-code` | `.claude/skills/` |
| OpenCode | `--harness opencode` | `.opencode/skills/` |
| GitHub Copilot | `--harness copilot` | `.github/skills/` |
| Codex | `--harness codex` | `.agents/skills/` |

Invocare le skill con la sintassi del proprio harness, per esempio `/dfd-init` in Claude Code o `$dfd-init` in Codex. L'installazione dei file è verificata dai test; discovery e comportamento dell'agente richiedono ancora un pilota negli harness reali.

È attivo un adapter per repository: `dfd install --harness claude-code` lo cambia senza perdere le feature. I file modificati dal team causano un conflitto esplicito, senza sovrascrittura. Le istruzioni esistenti (`AGENTS.md`, `CLAUDE.md`, Copilot) vengono preservate.

Nel brownfield partire da 1–2 domini pilota e 3–5 interventi, come descritto in [adozione.md](adozione.md). Le lacune baseline fuori ambito restano nel piano di adozione. Il gate distingue controlli automatici e decisione umana: una review superata diventa da riconfermare quando cambiano gli artefatti verificati.

Per formati JSON, comandi, regole di rischio e limiti del gate, vedere [il contratto del toolkit](templates/contratto_toolkit.md).

La specifica si costruisce in modo iterativo: `dfd specify <id>` crea gli artefatti oppure li riprende senza sovrascriverli, mostrando `open_questions` da `design.json`. Chiarire i dubbi con domande mirate e aggiornare insieme `spec.md` e `design.json`; i dubbi aperti impediscono l'approvazione del design, ma non la revisione della bozza. Il setup evolve allo stesso modo: modifiche a criteri pertinenti, guardrail o lifecycle richiedono riconferma delle review interessate; template e convenzioni descrittivi non invalidano le nuove review selettive. Le feature legacy mantengono il fingerprint conservativo.

Per domini creati con la versione precedente, rieseguire `init` con lo stesso dominio e modalità: aggiunge gli artefatti mancanti preservando quelli esistenti. I guardrail comuni in `.dfd/guardrails.md` restano disponibili; collegarli nei guardrail del dominio quando pertinenti.

## Apertura e sviluppo delle feature

`dfd-feature` guida la creazione di una feature, chiarendo titolo, dominio, ambito e tipo prima di chiamare `dfd feature`. Riprende gli interventi già esistenti; rischio e specifica seguono con le rispettive skill.

Dopo una review di design approvata e attuale:

```sh
dfd plan 001-paypal
# Raffinare plan.md e plan.json; usare dfd-implement nell'harness
# Eseguire il ciclo TDD, salvare log e compilare evidence.json
dfd verify 001-paypal
dfd status 001-paypal
```

`dfd-plan` deriva task e test dai criteri DoD applicabili. `dfd-implement` esegue lo sviluppo nell'harness: test rosso, codice, test verde, refactor e strumentazione. `dfd-verify` controlla le evidenze prima della pre-release. La CLI verifica copertura dei criteri, riferimenti al design/piano, esiti e hash dei log e dei file; non esegue comandi del progetto e non certifica l'autenticità dei risultati dichiarati. Una suite CI ancora da eseguire lascia lo sviluppo aperto solo se richiesta dalla policy del dominio; in development-only una suite finale locale tracciata consente development-complete.

`status` espone separatamente gate di design e gate di sviluppo e indica la prossima skill. Quando design, piano, codice o log cambiano, le evidenze interessate devono essere aggiornate dopo nuove verifiche. Dopo una nuova approvazione del design, `dfd plan <id> --refresh` riallinea il piano preservando i task. `ready-for-pre-release` significa che si può preparare il rilascio; non lo approva.

Per installare le nuove skill in un progetto già inizializzato, usare il binario aggiornato ed eseguire `dfd install --harness <adapter>`.

## Pre-release e review di release

Con sviluppo verificato e attuale, usare `dfd-pre-release` per preparare artefatto, revisione, fasi di esposizione, criteri di promozione/stop, rollback, on-call, allarmi e dashboard. La skill usa il template locale e permette di chiarire i dubbi iterativamente. Registrare evidenze dei test non funzionali pertinenti e riferimenti alle comunicazioni realmente effettuate.

```sh
dfd pre-release 001-paypal
# Completare rollout.md e rollout.json con la skill dfd-pre-release
dfd review-release 001-paypal
# Dopo la decisione del responsabile umano:
dfd decide 001-paypal --stage release --decision approved --reviewer "TL" --role tech-lead --note "Review di release effettuata" --human-confirmed
dfd status 001-paypal
```

`dfd-review-release` usa la checklist light/full del dominio. Un controllo strutturale positivo attende ancora la decisione umana. Le risorse ancora da attivare possono avere un piano esplicito, ma richiedono `approved-with-conditions`; `status` ne mostra le attivazioni e le condizioni. Il gate di release vive in `release.gate`, separato da design e sviluppo, con uno storico di decisioni dedicato.

Dopo cambiamenti allo sviluppo, rieseguire le verifiche precedenti e `dfd pre-release <id> --refresh`, che conserva il piano e aggiorna i riferimenti. La review di release va riconfermata dopo modifiche agli artefatti verificati. La CLI prepara e controlla i documenti: configurazione dei servizi, comunicazioni e rollout si svolgono negli strumenti del team con l'autorizzazione pertinente.

## Organizzazione

Il toolkit è contenuto in `dfd-kit/`; nella radice del repository restano i documenti originali del framework. I documenti DFD, i template e gli esempi in questa directory sono le copie usate dal toolkit, con gli adattamenti necessari al workflow. La build li incorpora nel binario.

## Sviluppo prima della CI

Il dominio di questo toolkit usa [lifecycle.json](.dfd/domains/dfd-kit/lifecycle.json) in modalità solo sviluppo. La suite finale locale tracciata consente di chiudere lo sviluppo senza CI simulata. La preparazione al rilascio richiede una transizione esplicita, rivalutazione della scelta CI e riconferma delle review invalidate. Vedere il [contratto](templates/contratto_toolkit.md#policy-di-maturità-del-dominio).

## Fix rapidi, singolo sviluppatore e team

Per un fix circoscritto usare `dfd quick <id> --domain <dominio> --title "..." --owner "..."`: un solo record, rischio motivato e test di regressione pertinenti. Le modifiche editoriali richiedono una verifica utile, senza red artificiale. Il singolo dev non deve aprire PR o ottenere una review indipendente; il [contesto team](templates/collaborazione.md) è facoltativo. Cambiamenti critici o dubbi impongono light/full; `dfd promote <id> --to <feature-id>` conserva l’origine. Per comandi, gate e prove integrate vedere il [contratto](templates/contratto_toolkit.md#percorso-rapido).
