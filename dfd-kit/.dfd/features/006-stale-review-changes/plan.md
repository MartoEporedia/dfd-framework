# Piano di sviluppo – Cosa è cambiato dopo l'ultima decisione

## Ambito e test

Le decisioni registrano lo snapshot dei componenti del fingerprint; con stale-review status e i report elencano componenti e file cambiati, aggiunti o rimossi dopo la decisione (issue #6)

Red/green: design con spec.md modificata elenca spec.md; release con codice ed evidence.json aggiornati elenca il file di codice; release-notes.md creato dopo; decisione senza snapshot con changes null; record rapido; gate non stale senza changes. fmt, Clippy, suite, doctor, check_translations, mutanti.

## Task TDD

### stale-review-changes – Snapshot nelle decisioni e changes con stale-review (design, release, record rapido)

Owner: manutentore del repository
Criteri: OBS-01, SEC-03, SLO-01, SLO-02, UX-01

Implementazione: Campo facoltativo snapshot (componente → sha256) in Decision; snapshot calcolato in decide per design, release e record rapido senza cambiare le formule dei fingerprint; con stale-review l'output e i report aggiungono changes {changed, added, removed}, oppure changes null per decisioni senza snapshot; contratto e skill di review IT/EN.

Verifica: Regressione cargo test --locked --offline --test workflow stale_review_lists: red su design (spec.md), release (file di codice ed evidence.json), record rapido, decisione legacy e gate non stale; green dopo; mutanti sul confronto e sulla raccolta dei file delle evidenze.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
