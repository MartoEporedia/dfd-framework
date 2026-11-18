# Piano di sviluppo – Ondata 2: stati non previsti e qualità della specifica nel kit

## Ambito e test

Copie del kit di fondamenta, processo e template allineate alla revisione del framework (PR #17), template di specifica del kit e skill dfd-specify, dfd-implement, dfd-review-design e dfd-verify aggiornati in inglese e italiano

Red/green di kit_resources_adopt_unforeseen_states_revision e skills_guide_unforeseen_states_and_added_criteria (risorse e skill installate in en e it); test di preservazione esistenti; suite locale con fmt, Clippy, test, doctor, check_translations e test Python; CI reale sulla PR.

## Task TDD

### kit-resources – Copie del kit e template di specifica con la revisione dell'ondata 1

Owner: manutentore del repository
Criteri: SEC-02, SLO-01, UX-01

Implementazione: fondamenta.md, processo-e2e.md e cinque template del kit allineati alla radice in en e it (riga di navigazione del kit invariata); templates/specifica_feature.md del kit con i campi nuovi; i template dei domini esistenti non vengono riscritti.

Verifica: Red/green di cargo test kit_resources_adopt_unforeseen_states_revision (init in en e it); test di preservazione esistenti.

### skills – Skill dfd-specify, dfd-implement, dfd-review-design e dfd-verify

Owner: manutentore del repository
Criteri: SEC-03, UX-02, SLO-02, COST-02

Implementazione: Istruzioni su parole del richiedente, assunzioni, domande con opzioni, stati non previsti, vincoli per tipo di cambiamento, nessun codice senza criterio, criteri aggiunti confrontati con la richiesta, verifiche osservabili; in en e it.

Verifica: Red/green di cargo test skills_guide_unforeseen_states_and_added_criteria (skill installate in en e it); suite locale, doctor, check_translations e CI reale sulla PR.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
