# Piano di sviluppo – Testo di dod.md nel fingerprint selettivo

## Ambito e test

Il fingerprint di design selettivo include, per ogni criterio selezionato, il testo di dod.md che lo cita; una modifica solo in dod.md rende la review da riconfermare (issue #5)

Red/green su F07b riprodotto (criterio selezionato cambiato solo in dod.md: stale-review) e sui casi da non invalidare (criterio non selezionato, spazi) e sulle sotto-voci. Test selettivo esistente invariato. fmt, Clippy, suite, doctor, check_translations, mutanti.

## Task TDD

### dod-fingerprint – Blocchi di dod.md dei criteri selezionati nel fingerprint selettivo

Owner: manutentore del repository
Criteri: SEC-03, OBS-01, SLO-01, SLO-02, UX-01

Implementazione: Estrarre da dod.md i blocchi Markdown (voce di elenco con continuazioni e sotto-voci, riga di tabella, titolo, paragrafo) che citano ciascun criterio selezionato, normalizzarne gli spazi e aggiungerli al fingerprint selettivo come selected-dod; aggiornare contratto e skill dfd-review-design IT/EN.

Verifica: Regressione cargo test --locked --offline --test workflow selective_review_tracks_dod: red su F07b riprodotto (criterio selezionato cambiato solo in dod.md resta approved), green dopo la correzione; casi non invalidanti (criterio non selezionato, spazi) e sotto-voce; mutanti sull'inclusione e sull'estensione dei blocchi.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
