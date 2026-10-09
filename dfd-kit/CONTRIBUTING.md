# Sviluppare DFD Kit

La CLI è scritta in Rust; gli utilizzatori eseguono un binario e non devono installare Rust, interpreti o pacchetti aggiuntivi. Lo sviluppo richiede la toolchain Rust stable con Cargo, rustfmt e Clippy.

## Processo di sviluppo obbligatorio

Dal 2026-10-09 ogni sviluppo della CLI e delle skill segue DFD, inclusi bugfix e refactor. La regola persistente è in [AGENTS.md](../AGENTS.md#dfd-obbligatorio-per-cli-e-skill).

Il progetto adottato è questa directory (`dfd-kit/`), con dominio `dfd-kit`, modalità brownfield e harness Codex. Il [setup operativo](.dfd/domains/dfd-kit/process.md), la [DoD](.dfd/domains/dfd-kit/dod.md) e l’[assessment](.dfd/domains/dfd-kit/assessment.md) sono i riferimenti per il prossimo cambiamento. Le skill installate vivono in `.agents/skills/`; le sorgenti da sviluppare restano in `skills/`.

```sh
./dist/dfd-macos-arm64 setup --domain dfd-kit
./dist/dfd-macos-arm64 status
```

Prima di lavorare al cambiamento, riprendere gli artefatti e verificare il setup del dominio. Se manca, completare la Fase 0. Classificare il rischio e scegliere rapido/light/full. Il rapido usa un solo record; light/full richiedono una feature con specifica iterativa.

Per light/full, dopo la review di design attuale derivare il piano. Raccogliere evidenze reali: regressione red/green per fix comportamentali, verifica pertinente per modifiche editoriali e mutation test sugli invarianti importanti. Preparare pre-release e review di release per i cambiamenti destinati al rilascio. Le decisioni umane già espresse e ancora pertinenti si riusano; i controlli strutturali non conferiscono approvazioni.

Usare le skill del toolkit e la CLI secondo il [contratto degli artefatti](templates/contratto_toolkit.md). Conservare setup, specifiche, decisioni e risultati nel repository; aggiornarli quando cambia il lavoro e riconfermare i gate invalidati.

## Struttura

- `src/cli.rs`: comandi e opzioni.
- `src/model.rs`: contratti JSON versionati e tipi del dominio.
- `src/lib.rs`: installazione, adozione, rischio e gate di design.
- `src/development.rs`: piano TDD, evidenze e gate di sviluppo.
- `src/quick.rs`: record rapido, verifiche ed escalation a light/full.
- `src/collaboration.rs`: contesto facoltativo e policy team con prove integrate.
- `src/release.rs`: preparazione operativa, rollout e review di release.
- `src/store.rs`: accesso ai file, lock e scritture atomiche.
- `skills/`: dodici workflow canonici, condivisi dagli adapter.
- `templates/`: artefatti riusabili e contratto del toolkit.
- `tests/workflow.rs`: test della CLI su repository temporanei.
- `scripts/mutation_test.py`: runner isolato di mutation test.
- `mutations/gates.json`: mutazioni mirate ai gate e riferimenti ai test che le intercettano.
- `build.rs`: incorpora documenti, template e skill nel binario.

## Comandi

Eseguire questi comandi dalla directory `dfd-kit/` (`cd dfd-kit` dalla radice del repository).

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked --release
./target/release/dfd doctor
```

Se tutte le dipendenze sono già in cache, aggiungere `--offline` ai comandi Cargo. Conservare `Cargo.lock`: la CLI deve avere build riproducibili rispetto alle versioni delle dipendenze.

## Convenzioni e verifiche

Usare rustfmt e nomi `snake_case`; le skill usano nomi `dfd-<azione>` in kebab-case. Mantenere in italiano la documentazione di DFD e i workflow. Quando cambia un contratto JSON, aggiornare modelli, guida e test; la prima versione accetta solo `schema_version: 1` e rifiuta campi sconosciuti.

Verificare comportamenti osservabili: preservazione dei file, idempotenza, gestione dei dati mancanti, isolamento delle feature, invalidazione delle decisioni e adozione proporzionata. I test eseguono il binario senza programmi nel PATH. `doctor` verifica le skill incorporate; non sostituisce una prova reale nell'harness.

La configurazione `ci/github-actions.yml` configura build e test per Linux x86_64 con musl, macOS arm64/x86_64 e Windows x86_64. Produce archivi del binario con checksum come artifact, senza pubblicare automaticamente release. I test locali su un sistema non certificano gli altri sistemi. Per attivare la CI, copiare il file in `.github/workflows/` nella radice del repository; il workflow esegue i comandi dalla directory `dfd-kit/`.

## Mutation test

Il runner usa Python 3.9+ e Cargo, senza pacchetti Python aggiuntivi o installazione di `cargo-mutants`. I 31 mutanti curati coprono rischio, review umana, validità dei fingerprint, hash delle evidenze, ordine TDD, CI, rollout, preparazione operativa, comunicazioni e versione degli schemi JSON, oltre a percorso rapido, autonomia individuale, review selettive e snapshot integrati del team.

```sh
python3 -m unittest discover -s tests -p 'test_mutation_runner.py'
python3 scripts/mutation_test.py
# Esempio: ripetere solo la verifica sull'integrità delle evidenze
python3 scripts/mutation_test.py --only evidence-file-integrity
```

Per default Cargo lavora offline; con dipendenze non ancora in cache usare `--online`. `--timeout 180` imposta il limite per ciascuna invocazione Cargo. Il runner copia gli input di build sotto `target/`, applica un mutante alla volta e ripristina la copia dopo il test. Non modifica i sorgenti originali né i binari distribuiti. Su macOS/Linux un timeout termina anche i processi figli del comando Cargo. La cache di build è separata in `target/mutation-build/`.

Prima verifica la suite completa senza modifiche e l'esistenza dei test selezionati. Ogni mutante deve sostituire esattamente un frammento: se il sorgente cambia rendendo il frammento assente o ambiguo, il runner fallisce esplicitamente. Esegue prima i test mirati; se passano, esegue l'intera suite per confermare la sopravvivenza del mutante.

Il report JSON e i log sono in `target/mutation-results/`; cambiare directory con `--output <directory>`. Il report include gli hash dei sorgenti/test e del manifest, gli esiti e il punteggio. `killed` significa che un test ha fallito; `survived` significa che anche la suite completa è passata. Errori di compilazione, test non trovati e timeout sono distinti e **non** contano come mutanti intercettati. Il comando restituisce `0` solo se tutti i mutanti selezionati sono intercettati; baseline fallita o run incompleto rendono il risultato non valido. Un lock protegge la directory del report dalle esecuzioni concorrenti; dopo un'interruzione forzata rimuoverlo solo se nessun runner è attivo.

Il punteggio riguarda questo insieme mirato, non tutte le mutazioni possibili nel programma. Per aggiungere una mutazione, inserire in `mutations/gates.json` ID, file, frammento originale/sostituito, comportamento alterato e almeno un test. Preferire mutanti compilabili con un effetto osservabile; verificare che il test fallisca per quel comportamento. Non rimuovere mutanti sopravvissuti per alzare il punteggio: rafforzare i test oppure documentare perché un mutante è equivalente.

Il job `mutations` in `ci/github-actions.yml` esegue il runner e conserva i risultati come artifact anche in caso di fallimento. Come gli altri job, si attiva copiando il workflow nella radice del repository; non è stato eseguito su GitHub in questa sessione.

## Contributi

Descrivere problema, comportamento risultante e verifiche svolte. Collegare le issue pertinenti e aggiornare template, skill e percorsi di lettura insieme alle modifiche al framework. Usare messaggi concisi, per esempio `feat: add brownfield adoption assessment` o `fix: invalidate stale design reviews`. Non è disponibile una cronologia Git locale da cui dedurre una convenzione precedente.

## Documenti del framework

Le copie locali di fondamenta, processo, rischio, RACI e adozione sono risorse del bundle. I documenti canonici del framework rimangono nella radice del repository. Aggiornare le copie in modo esplicito quando il toolkit adotta una revisione del framework; le correzioni ai template del toolkit non devono sovrascrivere gli originali della radice.

## Sviluppo prima della CI

Il dominio di questo toolkit usa [lifecycle.json](.dfd/domains/dfd-kit/lifecycle.json) in modalità solo sviluppo. La suite finale locale tracciata consente di chiudere lo sviluppo senza CI simulata. La preparazione al rilascio richiede una transizione esplicita, rivalutazione della scelta CI e riconferma delle review invalidate. Vedere il [contratto](templates/contratto_toolkit.md#policy-di-maturità-del-dominio).

## Fix rapidi, singolo sviluppatore e team

Per un fix circoscritto usare `dfd quick <id> --domain <dominio> --title "..." --owner "..."`: un solo record, rischio motivato e test di regressione pertinenti. Le modifiche editoriali richiedono una verifica utile, senza red artificiale. Il singolo dev non deve aprire PR o ottenere una review indipendente; il [contesto team](templates/collaborazione.md) è facoltativo. Cambiamenti critici o dubbi impongono light/full; `dfd promote <id> --to <feature-id>` conserva l’origine. Per comandi, gate e prove integrate vedere il [contratto](templates/contratto_toolkit.md#percorso-rapido).
