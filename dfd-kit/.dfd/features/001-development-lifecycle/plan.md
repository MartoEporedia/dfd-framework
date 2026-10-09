# Piano — Sviluppo senza CI prematura

## Task task-1

Owner: Marco Martorana

Criteri: OBS-01, OBS-02, SEC-01, SEC-02, SEC-03, SLO-01, SLO-02, COST-01, UX-01, UX-02

Un singolo intervento coerente applica il contratto lifecycle a setup, fingerprint, verifiche e release, documentazione e skill. I test comportamentali coprono isolamento e compatibilità della policy oltre al successo locale.

## Verifiche ed evidenze

Il primo tentativo red.log aveva un errore nel nome della fixture e non vale come prova TDD. La riproduzione comportamentale red-baseline.log usa i nuovi scenari contro il binario distribuito precedente, senza modificarlo: tutti falliscono per i comportamenti mancanti. È stata riprodotta dopo la prima implementazione, prima della suite finale; non si dichiara un ciclo red/green ideale retroattivamente. I log green.log e suite.log conservano le verifiche del candidato. Clippy, fmt, doctor, test Python e mutanti completano la verifica proporzionata. Nessuna CI eseguita o dichiarata.

## Refactor e limiti

Nessuna modifica del protocollo di scrittura; il nuovo file è opzionale e i domini legacy restano stretti. Nessuna distribuzione, review di release o rollout richiesto in solo sviluppo. Il manutentore ha approvato la distinzione funzionale in conversazione; la CLI verifica struttura e integrità degli artefatti.
