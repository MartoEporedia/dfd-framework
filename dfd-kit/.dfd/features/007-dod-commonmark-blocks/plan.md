# Piano di sviluppo – Blocchi di dod.md da un parser CommonMark

## Ambito e test

I blocchi e le sezioni di dod.md del fingerprint selettivo si ricavano da pulldown-cmark; casi ambigui e criteri senza definizione propria legano l'intero dod.md (seguito di #5, rilievi della review di sicurezza su a5a71bb)

Red/green: recinto lungo con recinto corto interno, titolo setext, titolo in blockquote, HTML, indice descrittivo con testo non citato; test di 005 invariati. fmt, Clippy, suite, doctor, check_translations, mutanti.

## Task TDD

### commonmark-blocks – Unità di dod.md da pulldown-cmark con regola di definizione e ripiego su HTML

Owner: manutentore del repository
Criteri: SEC-03, SLO-01, SLO-02, COST-01, UX-01

Implementazione: Aggiungere pulldown-cmark 0.13 senza feature di default; ricavare voci, paragrafi, righe di tabella, titoli e sezioni dagli eventi con offset sorgente; regola di definizione e soglia di quattro parole; intero dod.md con HTML o senza definizione; rimuovere il parser a mano; aggiornare contratto IT/EN e mutanti.

Verifica: Regressione cargo test --locked --offline --test workflow selective_review_dod_commonmark: red sui cinque casi segnalati (recinto lungo, setext, blockquote, HTML, indice descrittivo) che restano approved, green dopo; test di 005 invariati; mutanti sulla regola di definizione, sul ripiego HTML e sui confini delle sezioni.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
