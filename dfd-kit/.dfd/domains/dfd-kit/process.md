# Convenzioni DFD del dominio DFD Kit

- Data: 2026-10-09
- Owner: manutentore del repository
- Modalità: brownfield, pilota; livello osservato 0, obiettivo 1.

## 1. Ambito e responsabilità

Il dominio unico copre CLI e skill con i loro artefatti di supporto. Il manutentore è owner di dominio e reviewer umano; l’agente cura analisi, bozze, implementazione ed evidenze. I ruoli possono coincidere senza trasformare una verifica automatica in approvazione. Il codice preesistente resta baseline: non ricostruire retroattivamente approvazioni o cicli TDD.

## 2. Radice e comandi

Operare da `dfd-kit/`, radice adottata. Le feature vivono in `.dfd/features/`; il setup in `.dfd/domains/dfd-kit/`. Il binario distribuito è il bootstrap; ricompilare quello candidato quando cambiano sorgenti o risorse incorporate, mantenendo disponibile la versione funzionante e il relativo checksum.

```sh
cd dfd-kit
./dist/dfd-macos-arm64 setup --domain dfd-kit
./dist/dfd-macos-arm64 status
./dist/dfd-macos-arm64 feature <id> --domain dfd-kit --title '<titolo>' --scope '<ambito>' --kind feature
```

Usare la variante di piattaforma appropriata o `./target/release/dfd` dopo la build. I dettagli dei comandi e degli artefatti sono nel [contratto](../../../templates/contratto_toolkit.md). Le [skill installate](../../../.agents/skills/dfd-setup/SKILL.md) guidano il lavoro; modificarne le sorgenti canoniche, non le copie installate.

## 3. Rischio e specifiche iterative

Applicare l’algoritmo di [rischio centrale](../../../../rischio.md): almeno un high implica full completo; almeno due medium implicano full proporzionato; altrimenti light. Un nuovo servizio usa full completo. Il rapido è una scelta esplicita per fix di contratto noto o editoriali: tutte le dimensioni low, ambito circoscritto, nessun contratto cambiato, dubbio o comportamento critico. Dati ignoti non diventano low.

Valutare le dimensioni canoniche rispetto al toolkit: perdita o sovrascrittura di file, escape dei percorsi, false approvazioni e migrazioni incompatibili possono essere high; nuovi comandi e contratti condivisi richiedono analisi dell’impatto; correzioni editoriali isolate possono essere low se non alterano comportamento o istruzioni operative. Questi esempi non sostituiscono la valutazione per dimensione.

Per light/full aprire o riprendere la feature; eseguire assess e specify. Per il rapido usare `quick` e il solo record in `.dfd/changes/`, senza specifica o piano separati. Registrare i dubbi aperti e fare domande mirate; ripetere specify e aggiornare la bozza senza sovrascrivere il lavoro. Collegare criteri, test e vincoli. Passare alla review di design solo con una specifica verificabile; registrare con decide la decisione umana effettivamente espressa. Il setup ready certifica completezza strutturale, non approvazione del design.

## 4. Sviluppo ed evidenze

Dopo design approvato e attuale, derivare plan, sviluppare con TDD e raccogliere evidenze red/green per task, hash e log. Eseguire le verifiche pertinenti di [CONTRIBUTING.md](../../../CONTRIBUTING.md), inclusi mutanti interessati dai gate modificati; giustificare esclusioni per cambiamenti documentali. verify valuta ciò che è registrato e non esegue i test.

La policy in `lifecycle.json` dichiara `development-only`, con CI non richiesta e motivazione esplicita. Light/full richiedono evidenze red/green e una suite finale locale valida; il rapido richiede regressione red/green per fix o verifica pertinente green per editoriale, senza seconda approvazione individuale; nessun risultato locale viene etichettato come CI. Il gate `development-complete` conclude lo sviluppo senza aprire la pre-release. Per distribuire, passare esplicitamente a `release-preparation`, rivalutare la policy CI e riconfermare design, piano ed evidenze invalidati.

## 5. Pre-release, rilascio e learning

Preparare pre-release, rollout e review di release con soglie motivate, rollback, risorse e comunicazioni reali. Per un toolkit locale, rollout significa prova del binario candidato su fixture/copied project, pilota dell’harness e distribuzione progressiva; gli indicatori sono esiti dei comandi, preservazione degli artefatti e completamento del workflow. Dashboard e allarmi devono riferirsi a report o strumenti realmente disponibili, con risorse planned finché non provate. Non inventare reperibilità, comunicazioni o infrastruttura web.

Motivare esecuzione o esclusione di load, resilienza e sicurezza secondo rischio e contratto. La decisione di release è distinta da quella di design; risorse pending richiedono la decisione condizionata prevista. I comandi attuali preparano e validano il rilascio: l’esecuzione del rollout e il learning post-release non sono ancora automatizzati. Documentarne gli esiti reali senza simulare comandi inesistenti.

## 6. Template e manutenzione

Gli otto [template locali](templates/specifica_feature.md) mantengono la struttura centrale e aggiungono istruzioni per il toolkit. I placeholder sono intenzionali nei template e vanno risolti negli artefatti concreti. Usare il [catalogo](criteria.json) per selezionare i criteri e la [DoD](dod.md) per le condizioni verificabili.

Rivedere il setup dopo variazioni del contratto, regressioni o learning e dopo i primi tre cicli feature, anticipando la retrospettiva se il pilota mostra blocchi. Registrare tempi delle verifiche, dubbi risolti, regressioni e attriti dell’harness. Alzare il livello solo con evidenze dell’adozione effettiva.

## 7. Lavoro individuale e policy facoltative

Il dominio è gestito da un singolo manutentore: team.json assente, senza PR, reviewer indipendente o verifiche integrate obbligatori. Le policy team si attivano solo con scelta esplicita; i ruoli possono coincidere. Le nuove feature usano fingerprint selettivi, con prescrizioni normative nel catalogo e nei guardrail. Template e processo descrittivi non introducono vincoli nascosti; le review legacy restano conservative. I template facoltativi di cambiamento rapido e collaborazione affiancano gli otto template di setup.
