# Contratto degli artefatti – DFD Kit 0.1

## Comandi locali

Usare il binario `dfd` per il proprio sistema, senza runtime o pacchetti aggiuntivi. Dalla radice del prodotto usare `dfd <comando>`; per un altro repository `dfd --project <repo> <comando>`. `--project` può essere specificato anche dopo il sottocomando. Se il binario non è nel PATH, usare il suo percorso. Nessun comando chiama un LLM. Per sviluppare il toolkit servono Rust e Cargo.

| Comando | Funzione |
|---|---|
| `init --harness codex --domain checkout --mode brownfield` | Installazione e scaffolding del dominio pilota |
| `setup --domain checkout` | Controllo strutturale della Fase 0 del dominio |
| `install --harness claude-code` | Cambio dell'adapter, preservando stato e contenuti del team |
| `feature 001-paypal --domain checkout --title "PayPal" --scope "Nuovo metodo di pagamento"` | Creazione di una feature |
| `assess 001-paypal` | Classificazione da `risk.json` |
| `specify 001-paypal` | Creazione o ripresa iterativa della specifica, senza sovrascrittura |
| `review-design 001-paypal` | Report dei controlli e gate corrente |
| `plan 001-paypal [--refresh]` | Piano TDD derivato dal design approvato e attuale |
| `verify 001-paypal` | Verifica delle evidenze di sviluppo e report pre-release |
| `pre-release 001-paypal [--refresh]` | Preparazione o ripresa del rollout dopo lo sviluppo verificato |
| `review-release 001-paypal` | Controlli strutturali, attivazioni e gate di release |
| `decide 001-paypal --stage release ... --human-confirmed` | Decisione umana di release, separata dal design |
| `doctor` | Verifica skill e risorse incorporate nel binario |
| `status [001-paypal]` | Ripresa del lavoro e lacune rilevate |
| `decide 001-paypal --decision approved --reviewer "TL" --role tech-lead --note "Review effettuata" --human-confirmed` | Registrazione dichiarata di una decisione umana |

Output JSON; exit code `0` per un comando riuscito, `1` per un errore di uso/lettura o un conflitto, `2` per assessment, setup incompleto o controlli di design o sviluppo falliti. `status` restituisce `0` anche quando mostra un gate bloccato. `review-design` può riuscire mentre attende ancora la review umana: il campo `gate` è l'autorità sullo stato del percorso, non il solo exit code.

È attivo un adapter per installazione, per evitare skill duplicate nei percorsi di compatibilità. `install` cambia adapter senza perdere le feature. Le skill canoniche sono nel toolkit; `.dfd/kit/` è una copia gestita. Non modificare la copia: gli aggiornamenti segnalano conflitti invece di sovrascriverla. Le istruzioni `AGENTS.md`, `CLAUDE.md` e Copilot preesistenti sono preservate.

## Dominio e adozione

`config.json` dichiara `mode` (`brownfield`/`greenfield`) e domini. `adoption.json` contiene `pilot`, `current_level`, `target_level` (0–3), sei aree (`process`, `observability`, `security`, `reliability`, `cost`, `culture`), lacune `baseline_gaps` e `next_steps`. Ogni area dichiara `status` (`unknown`, `gap`, `observed`), `evidence` (lista di riferimenti) e `notes`. `assessment.md` spiega priorità, motivazioni e interventi pilota. Lo schema versionato è `schema_version: 1` per ogni JSON.

`dod.md` è la DoD leggibile del dominio; `criteria.json` ne rappresenta gli identificatori verificabili. Mantenere entrambi coerenti. Esempio di catalogo:

```json
{
  "schema_version": 1,
  "criteria": [
    {"id": "OBS-01", "statement": "Metriche di errore sul flusso modificato", "scope": "changes"},
    {"id": "SLO-01", "statement": "Definire SLO per i flussi legacy", "scope": "baseline"}
  ]
}
```

Tutti i criteri `changes` devono essere valutati nella feature, come applicabili o esclusi con motivazione. Un criterio `baseline` si collega quando pertinente; una lacuna storica fuori ambito non blocca automaticamente il cambiamento. Per avanzare il design serve almeno un criterio applicabile.

## Fase 0 – Setup DFD di dominio

`init` prepara la Fase 0, ma non la dichiara completata. In `.dfd/domains/<dominio>/` mantenere:

- `dod.md` e `criteria.json`: DoD Estesa e catalogo coerenti, con identificatori stabili;
- `guardrails.md`: principi architetturali, contratti, sicurezza, SLO, osservabilità e costo;
- `process.md`: naming, rischio, checklist full/light, collocazione degli artefatti, responsabilità e differenze rispetto ai template centrali;
- `templates/`: copie locali degli otto template del framework, riusabili così come sono oppure adattate al dominio.

`dfd setup --domain <dominio>` restituisce `phase: domain-setup`, `gate: blocked` o `ready`, `errors` e `next`. Verifica documenti presenti, contenuto non vuoto, placeholder di setup e criteri validi, univoci e coerenti tra DoD e catalogo. I placeholder dei template riusabili sono intenzionali. Il controllo non valuta la semantica delle policy, non conferisce approvazione umana e non certifica il livello di adozione. `status` senza ID mantiene `domains` come lista dei nomi e aggiunge `setups`; con ID include `setup` e propone `dfd-setup` prima delle altre azioni quando la Fase 0 è incompleta.

Il setup è un prerequisito del design condiviso dalle feature, paragonabile alla constitution di SDD. `feature` e `assess` permettono di raccogliere idea e rischio; `specify` richiede la Fase 0 pronta e usa il template locale `templates/specifica_feature.md`. Il gate di design controlla nuovamente il setup. Aggiornarlo quando cambiano architettura, pattern ricorrenti o learning di produzione. Il fingerprint include anche guardrail, convenzioni e tutti i template del dominio: le modifiche invalidano le review delle feature interessate, conservando lo storico.

Per domini preesistenti rieseguire `init` con gli stessi parametri: aggiunge file di setup mancanti e preserva gli artefatti locali. `.dfd/guardrails.md` resta un riferimento comune del progetto; i guardrail di dominio possono richiamarlo.

## Feature e rischio

La feature vive in `.dfd/features/<id>/`. `state.json` registra dominio, tipo di intervento, ambito, fase, rischio, percorso e storico delle decisioni. La CLI lo gestisce: non editarlo per superare gate.

`risk.json` richiede tutte le dimensioni `security`, `reliability`, `cost`, `business`, `architecture`, ciascuna con `level` (`low`/`medium`/`high`/`null`) e `rationale`. Anche un rischio low richiede una motivazione. `assess` genera `risk.md` e registra un fingerprint dell'input. Se cambia l'input, rieseguire il comando.

Regole ordinate: almeno una high → high/full completo; almeno due medium senza high → medium/full proporzionato; altrimenti → low/light. I dati incompleti restano da chiarire. Per `kind: new-service`, la guida di adozione richiede full completo anche se il rischio è low. Non è ancora previsto un override automatico delle regole.

## Design e decisioni

La definizione della specifica è iterativa, soprattutto per dipanare dubbi. `specify` crea la bozza oppure, se entrambi gli artefatti esistono, restituisce `resumed: true` e le `open_questions` senza modificare contenuti, fase o decisioni. Se esiste un solo artefatto, segnala il conflitto e preserva il file. Aggiornare direttamente entrambi i documenti dopo domande e risposte; mantenere i dubbi in `open_questions` fino al loro chiarimento. Le domande aperte bloccano l'approvazione del design, non l'iterazione della specifica.

`spec.md` contiene la specifica leggibile. `design.json` ne rappresenta i prerequisiti e i collegamenti:

```json
{
  "schema_version": 1,
  "owner": "team-checkout",
  "objective": "Consentire il nuovo metodo di pagamento",
  "business_metrics": ["Tasso di completamento del checkout"],
  "epic": {"reference": "ticket-123", "approved_by": "PM", "approved_at": "2026-01-01T09:00:00Z"},
  "criteria": [
    {"id": "OBS-01", "applicability": "applicable", "implementation": "Contatore errori", "verification": "Test dell'emissione della metrica"}
  ],
  "test_plan": "Test unitari e integrazione del flusso",
  "guardrails": "Usare le policy esistenti di gestione pagamenti",
  "open_questions": []
}
```

Per esclusioni usare `applicability: excluded` e `justification`. Le date di approvazione dell'epic devono essere ISO 8601 con timezone e non future. Il controllo verifica presenza e riferimenti, non autenticità dell'approvazione, qualità semantica o esecuzione dei test.

`design-review.md` è rigenerabile; salvare note semantiche in `review-notes.md`. `decide` registra una decisione umana solo dopo la sua espressione: il flag `--human-confirmed` è una dichiarazione, non autenticazione. In questa versione i ruoli ammessi sono `tech-lead` e `architect` secondo il RACI. Per approvazione condizionata ripetere `--condition "testo|responsabile|fase"`; le condizioni restano aperte e visibili. Le condizioni dovute entro lo sviluppo richiedono una nuova decisione umana dopo la risoluzione; quelle per `pre-release`, `release`, `rollout` e `post-release` restano visibili e non bloccano la chiusura dello sviluppo. Fasi di scadenza diverse sono trattate conservativamente come dovute entro lo sviluppo. Le condizioni di design dovute entro la release devono essere risolte e la decisione riconfermata prima di approvare la release.

Il fingerprint include specifica, design strutturato, rischio, DoD, catalogo, adozione, guardrail, configurazione, note di review e contesto della feature. Modifiche successive rendono la decisione `stale-review`; nuovi problemi strutturali danno `blocked`. Lo storico resta disponibile. I file locali sono modificabili: l'MVP non garantisce un audit resistente a manomissioni né blocca merge/deploy.

## Sviluppo (TDD DFD)

Le skill `dfd-feature`, `dfd-plan`, `dfd-implement` e `dfd-verify` coprono apertura della feature, pianificazione, lavoro nell'harness e verifica. La CLI non chiama LLM e non esegue comandi del progetto. Le skill sono condivise dai quattro adapter.

`plan <id>` richiede `gate: approved` o `approved-with-conditions` attuale e crea `plan.md`, `plan.json` ed `evidence.json`, portando la fase a `development`. Il piano iniziale deriva un task per criterio applicabile e riusa owner, implementazione e verifica dal design. Raffinarlo iterativamente e mantenere Markdown e JSON coerenti:

```json
{
  "schema_version": 1,
  "design_fingerprint": "<fingerprint del design approvato>",
  "tasks": [
    {
      "id": "task-1",
      "owner": "team-checkout",
      "description": "Contare gli errori PayPal",
      "criteria": ["OBS-01"],
      "implementation": "Emettere il contatore errori",
      "verification": "Test di integrazione della metrica"
    }
  ],
  "open_questions": []
}
```

Tutti i criteri applicabili del design devono essere coperti. ID dei task univoci, owner, descrizione, implementazione e verifica sono richiesti; criteri esclusi o sconosciuti sono rifiutati. I dubbi impediscono di dichiarare il piano pronto, ma si possono chiarire aggiornando gli artefatti. La CLI controlla presenza e riferimenti, non equivalenza semantica di JSON e Markdown.

Ripetere `plan` preserva i file. Artefatti parziali causano un errore senza sovrascrittura. Se cambia il design, prima riconfermare la review umana, poi `plan <id> --refresh`: aggiorna solo il riferimento al design e la fase, preservando task, Markdown ed evidenze. Rivalutare i task coinvolti; le vecchie evidenze restano archiviate ma non valide per il nuovo piano.

### Evidenze di esecuzione

L'harness esegue test e implementazione, salva i log reali e compila `evidence.json`. Esempio di **struttura**, con hash e riferimenti da calcolare sui file reali:

```json
{
  "schema_version": 1,
  "design_fingerprint": "<fingerprint corrente>",
  "plan_hash": "<hash corrente del piano>",
  "files": [
    {"path": "src/checkout.rs", "sha256": "<SHA-256 del file verificato>"},
    {"path": "tests/checkout.rs", "sha256": "<SHA-256 del test verificato>"}
  ],
  "checks": [
    {
      "task": "task-1", "kind": "red", "command": "cargo test checkout_metrics",
      "executed_at": "2026-01-01T10:00:00Z", "exit_code": 1,
      "log": {"path": ".dfd/features/001-paypal/evidence/red.log", "sha256": "<SHA-256 del log>"}
    },
    {
      "task": "task-1", "kind": "green", "command": "cargo test checkout_metrics",
      "executed_at": "2026-01-01T10:01:00Z", "exit_code": 0,
      "log": {"path": ".dfd/features/001-paypal/evidence/green.log", "sha256": "<SHA-256 del log>"}
    },
    {
      "task": null, "kind": "ci", "command": "cargo test --locked (CI)",
      "executed_at": "2026-01-01T10:02:00Z", "exit_code": 0,
      "log": {"path": ".dfd/features/001-paypal/evidence/ci.log", "sha256": "<SHA-256 del log CI>"}
    }
  ],
  "refactor_notes": "Refactor eseguito; contatore implementato e suite rieseguita"
}
```

I percorsi sono relativi alla radice del progetto. Registrare file di codice, test e strumentazione coinvolti; log non vuoti, hash SHA-256 esatti, comando e data ISO 8601 con timezone non futura. Per ogni task serve un red con exit code positivo e un green con exit code zero; la suite completa CI deve essere verde e successiva ai green. Il fallimento red deve dipendere dal requisito atteso: un errore di ambiente non è una prova TDD. Documentare refactor e osservabilità, oppure motivarne la non necessità. Per la CI scaricare il log reale: un test locale non va dichiarato come CI.

`status <id>` espone `development.design_fingerprint` e `development.plan_hash` quando il piano è valido. `plan_hash` è SHA-256 della serializzazione JSON compatta dei due hash SHA-256 di `plan.json` e `plan.md` con chiavi `json` e `markdown`. Usare i valori restituiti dalla CLI; dopo cambiamenti al piano o al design aggiornare i riferimenti solo dopo le nuove verifiche. Cambiamenti ai file o ai log invalidano gli hash registrati. La CLI verifica i file elencati, non scopre automaticamente file omessi.

### Gate di sviluppo

`development.gate` distingue `not-started`, `blocked-plan`, `blocked-evidence`, `ready-for-pre-release`. Il gate di design resta in `gate`. `status` calcola entrambi dagli artefatti correnti; la fase in `state.json` registra l'ultimo passaggio eseguito e non sostituisce il gate. Le decisioni umane e le condizioni restano nello stato della feature.

`verify <id>` produce il report rigenerabile `development-review.md`. Solo con evidenze complete e attuali registra `phase: development-verified`; se una verifica successiva fallisce, riporta la fase a `development`. Restituisce `0` per prontezza alla pre-release e `2` per lacune, senza approvare il rilascio. Gli hash controllano integrità locale: non autenticano i log, non provano l'esecuzione dichiarata e non certificano qualità semantica, copertura o implementazione. Dopo la verifica positiva proseguire con `pre-release` e `review-release`.

## Pre-release

`pre-release <id>` richiede sviluppo strutturalmente pronto e attuale, senza dipendere dalla sola fase registrata. Crea `rollout.md` dal template locale del dominio e `rollout.json`, portando la fase a `pre-release`. Ripetere il comando riprende il lavoro senza modificarlo; un solo artefatto presente causa un conflitto preservando il contenuto. `--refresh` aggiorna il riferimento allo sviluppo corrente e la fase, preservando il piano; rivalutare le sezioni coinvolte.

La preparazione è iterativa: registrare dubbi in `open_questions`, chiarirli e aggiornare insieme Markdown e JSON. Nessun comando invia comunicazioni, configura risorse esterne o esegue deploy. La CLI verifica presenza, riferimenti e coerenza strutturale; la review umana valuta soglie, rischio e adeguatezza del piano.

Esempio di struttura per una release light; i valori sono illustrativi, non evidenze di un rilascio reale:

```json
{
  "schema_version": 1,
  "development_fingerprint": "<valore creato da pre-release>",
  "owner": "team-checkout",
  "artifact": "registry/checkout:revision-123",
  "revision": "revision-123",
  "environment": "production",
  "strategy": "direct",
  "strategy_rationale": "Intervento limitato entro i guardrail del dominio",
  "steps": [
    {"id": "production", "audience": "Tenant previsti", "percentage": 100, "duration_minutes": 30,
     "promotion": "OBS-01 entro soglia per 30 minuti", "stop": "Stop se aumenta il numero di errori"}
  ],
  "criteria": [
    {"id": "OBS-01", "signal": "Error count del nuovo flusso", "threshold": "Entro baseline per 30 minuti",
     "verification": "Dashboard e test della metrica"}
  ],
  "alarms": [
    {"name": "Allarmi", "owner": "", "status": "excluded", "reference": "", "activation_plan": "",
     "justification": "Light: allarmi esistenti sufficienti per il cambiamento", "evidence": []}
  ],
  "dashboards": [
    {"name": "Dashboard", "owner": "", "status": "excluded", "reference": "", "activation_plan": "",
     "justification": "Light: nessuna modifica alle dashboard esistenti", "evidence": []}
  ],
  "feature_flag": {"name": "Controllo esposizione", "owner": "", "status": "excluded", "reference": "",
    "activation_plan": "", "justification": "Rollout diretto con rollback all'artefatto precedente", "evidence": []},
  "rollback_trigger": "Superamento della soglia OBS-01",
  "rollback_procedure": "Ripristinare l'artefatto precedente",
  "rollback_verification": "Walkthrough effettuato e procedura di ritorno alla baseline definita",
  "on_call": "checkout-oncall",
  "communications": [
    {"audience": "Support e Ops", "owner": "PM", "informed_at": "2026-01-01T12:00:00Z", "reference": "ticket-123"}
  ],
  "non_functional_tests": [
    {"area": "load", "applicability": "excluded", "justification": "Nessun impatto sulla capacità secondo policy del dominio", "execution": null},
    {"area": "resilience", "applicability": "excluded", "justification": "Nessuna nuova dipendenza o failure mode", "execution": null},
    {"area": "security", "applicability": "excluded", "justification": "Nessuna modifica a controlli o trattamento dati", "execution": null}
  ],
  "open_questions": []
}
```

### Rollout e preparazione operativa

- `owner`, `artifact`, `revision`, `environment`, motivazione della strategia, on-call e piano di rollback sono richiesti. La corrispondenza della revisione alle evidenze CI deve essere verificata semanticamente: la CLI non interroga registry o repository remoti.
- Strategie: `direct`, `canary`, `segmented`, `combined`. Le fasi descrivono esposizione nell'ambiente finale; staging resta una prova separata. ID univoci, audience, durata positiva, promozione e stop sono richiesti. La percentuale deve essere da 1 a 100, non decrescente, fino al 100% dell'audience prevista. Diretto usa una fase; canary/combinato almeno due, con esposizione iniziale parziale.
- Ogni criterio applicabile del design richiede `signal`, `threshold`, `verification`, e citazione nel Markdown. Esclusioni del design non vanno reintrodotte; soglie e baseline derivano dal dominio o dalla review. La CLI non inventa valori e non certifica la loro adeguatezza.
- Risorse (`alarms`, `dashboards`, `feature_flag`): `planned` richiede owner, riferimento e piano di attivazione; `ready` richiede owner, riferimento ed evidenze locali `{path, sha256}` attuali; `excluded` richiede motivazione e nessuna evidenza. Allarmi e dashboard devono essere dichiarati; nel full non possono essere esclusi. Il controllo dell'esposizione può essere escluso motivando il meccanismo alternativo, anche senza un flag dedicato.
- Valutare esattamente `load`, `resilience`, `security`. Ogni test è applicabile con `execution` oppure escluso con motivazione e `execution: null`. Un'esecuzione contiene `command`, `executed_at`, `exit_code: 0`, `log: {path, sha256}`: data con timezone non futura, log non vuoto e hash attuale. Le esclusioni devono essere compatibili con rischio e policy nella review semantica.
- `communications` registra almeno una comunicazione realmente avvenuta, con audience, owner, data valida e riferimento. Il toolkit non invia messaggi e non autentica i riferimenti. L'assenza di comunicazioni resta una lacuna da completare negli strumenti autorizzati del team.

## Review di release e decisioni

`review-release <id>` rigenera `release-review.md`; note semantiche e rilievi vivono in `release-notes.md`. `status` aggiunge `release` senza cambiare il significato del gate di design (`gate`) e di sviluppo (`development`). Il campo `release.checklist` indica `review_release_light.md` o `review_release.md` del dominio. `pending_activation` mostra le risorse ancora pianificate.

`release.gate` distingue `not-started`, `blocked`, `awaiting-human-review`, `stale-review`, `changes-requested`, `approved`, `approved-with-conditions`. `review-release` restituisce `2` se preparazione assente o lacune strutturali; altrimenti `0`, anche se attende una decisione umana. Il gate e le condizioni sono l'autorità sul percorso.

`decide` usa `--stage design` come default, preservando la compatibilità dei comandi precedenti. Con `--stage release` registra la decisione in `release-decisions.json` (`schema_version: 1`, `decisions`), con lo stesso contratto di reviewer, ruolo dichiarato, nota, condizioni e conferma umana. Il RACI mantiene Tech Lead/Architect come responsabile della decisione; partecipanti e rilievi degli altri ruoli vanno nelle note. Nessuna approvazione automatica deriva da una checklist compilata.

Le approvazioni richiedono preparazione strutturalmente valida. Le risorse ancora pianificate impediscono `approved` e richiedono `approved-with-conditions` con attivazioni assegnate. Le condizioni di design dovute entro pre-release/release vanno risolte e riconfermate prima della decisione di release. `changes-requested` può essere registrato su una bozza incompleta, conservando il gate bloccato e lo storico.

La decisione di release registra `phase: release-reviewed` senza modificare lo storico di design. `status.next` propone `dfd-pre-release`, `dfd-review-release`, `resolve-release-conditions`, `rollout-with-conditions` o `rollout`. Attivazioni pendenti o condizioni dovute prima del rollout propongono la risoluzione; condizioni previste durante il rollout/post-release rimangono visibili. Dopo la risoluzione registrare la nuova decisione umana pertinente. La CLI non chiude condizioni automaticamente e non esegue il rollout.

Il riferimento allo sviluppo include fingerprint del design, ultima decisione di design, piano e `evidence.json`. La review di release include anche `rollout.md`, `rollout.json` e note semantiche; le evidenze locali vengono rilette e verificate tramite hash. Modifiche a sviluppo, risorse, test o rollout possono bloccare la review o renderla da riconfermare. `pre-release --refresh` preserva lo storico, che non vale automaticamente per la nuova preparazione. Report rigenerabili e fase registrata non fanno parte dei fingerprint, evitando invalidazioni dovute ai soli controlli. Gli hash non autenticano evidenze né approvazioni; l'esecuzione del rollout e il learning post-release restano fasi successive.

## Scritture concorrenti

I comandi che modificano il progetto acquisiscono `.dfd/lock` e scrivono ogni file tramite sostituzione atomica. In caso di interruzione forzata può restare il lock: rimuoverlo solo dopo aver verificato che non ci siano comandi DFD attivi. Una singola scrittura è atomica; l’installazione di più file non è una transazione unica. Gli errori di preflight preservano i file gestiti.
