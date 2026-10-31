# Piano di sviluppo – Marcatori nei log ed entrypoint della suite

## Ambito e test

Avvisi per marcatori (FAILED, skipped, Traceback...) nei log verdi delle evidenze; script della suite dichiarati in lifecycle.json e inclusi nel fingerprint di design (issue #4)

Red/green: F03 (skipped) e F05 (FAILED exit 0) con avvisi; red con FAILED senza avvisi; log_markers vuoto e personalizzato; entrypoint modificato dopo l'approvazione con stale-review e changes; entrypoint assente o esterno come errore di setup; compatibilità senza campi. fmt, Clippy, suite, doctor, check_translations, mutanti.

## Task TDD

### log-markers – Avvisi per marcatori nei log green/suite/ci con exit 0

Owner: manutentore del repository
Criteri: OBS-01, OBS-02, SLO-01, SLO-02, UX-01

Implementazione: Campo facoltativo log_markers in Lifecycle con lista integrata di default; warnings in development::status, nel report di verify e in review-release; documentazione IT/EN.

Verifica: Regressione cargo test --locked --offline --test workflow log_markers: red su F03 e F05 riprodotti (nessun avviso), green dopo; red esclusi, lista vuota e personalizzata; mutanti su esclusione dei red e lista di default.

### suite-entrypoints – Script della suite dichiarati e inclusi nel fingerprint di design

Owner: manutentore del repository
Criteri: OBS-02, SEC-01, SEC-03, SLO-01, SLO-02, UX-01

Implementazione: Campo facoltativo suite_entrypoints in Lifecycle; validazione in setup (assente, duplicato, fuori radice); hash di ciascun file in design_inputs; documentazione IT/EN.

Verifica: Regressione cargo test --locked --offline --test workflow suite_entrypoints: red su script modificato dopo l'approvazione (resta approved), green con stale-review e changes; errori di setup; compatibilità senza campo; mutante sull'inclusione nel fingerprint.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
