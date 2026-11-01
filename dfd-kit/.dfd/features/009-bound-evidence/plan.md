# Piano di sviluppo – Evidenze legate allo stato dei file

## Ambito e test

Snapshot dei file sotto source_roots nei check; blocco se l'ultima suite non corrisponde ai file attuali; fingerprint di design per criterio nei red/green; comando dfd exec facoltativo che esegue e registra i check (issue #3)

Red/green: F01b, F09, F12, F13 e F04 riprodotti con source_roots; file non dichiarato come avviso; exec red/green/suite con log, check, snapshot e recorded_by; programma inesistente; radici con symlink, file nascosti, percorso esterno; dominio senza source_roots invariato. fmt, Clippy, suite, doctor, check_translations, mutanti.

## Task TDD

### source-roots-snapshot – source_roots, snapshot nei check e blocco sulle divergenze dall'ultima suite

Owner: manutentore del repository
Criteri: OBS-01, OBS-02, SEC-01, SLO-01, SLO-02, UX-01

Implementazione: Campo source_roots in Lifecycle con validazione di setup; visita sicura dell'albero in Store (no nascosti, no symlink); campo snapshot in TestEvidence; regole del gate: suite senza snapshot, snapshot divergente (blocco), file non dichiarati (avviso).

Verifica: Regressione cargo test --locked --offline --test workflow source_roots: red su F01b, F04, F09, F12 riprodotti (gate pronto), green con blocco e avviso; radici con symlink, nascosti, percorso esterno; dominio senza radici invariato; mutanti.

### criteria-fingerprints – Fingerprint di design per criterio nei red/green

Owner: manutentore del repository
Criteri: OBS-01, OBS-02, SEC-03, SLO-01, SLO-02, UX-01

Implementazione: Fingerprint per criterio (voce design.json, catalogo, testo DoD) esposto in status; campo criteria_fingerprints in TestEvidence; con source_roots l'ultimo green di ogni task deve corrispondere ai fingerprint attuali dei suoi criteri.

Verifica: Regressione cargo test --locked --offline --test workflow criteria_fingerprints: red su F13 riprodotto (riferimenti aggiornati, gate pronto), green con blocco del task; spec.md cambiata senza nuovi red/green; mutante.

### dfd-exec – Comando dfd exec che esegue e registra red, green e suite

Owner: manutentore del repository
Criteri: OBS-02, SEC-01, SEC-02, SLO-03, COST-01, UX-01

Implementazione: Sottocomando exec in cli.rs; esecuzione senza shell con lock, log numerato in evidence/, check aggiunto con snapshot, criteria_fingerprints e recorded_by dfd; validazione di kind e task; exit 0 se registrato, 1 se il programma non parte.

Verifica: Regressione cargo test --locked --offline --test workflow dfd_exec: red (comando assente), green con red/green/suite registrati, log e snapshot coerenti, verify pronto; task mancante o vietato; programma inesistente; mutanti.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
