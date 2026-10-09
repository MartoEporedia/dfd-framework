# Sviluppo senza CI prematura

- Data: 2026-10-09
- Owner: Marco Martorana
- Rischio: medium, full proporzionato

## Decisione e ambito

Il manutentore ha approvato nella conversazione la distinzione tra solo sviluppo e preparazione al rilascio, chiedendo implementazione e aggiornamento dei documenti. Questa feature applica tale decisione a framework, template, CLI Rust, skill e setup del dominio; non pubblica release né esegue rollout.

## Contratto

`lifecycle.json` opzionale nel dominio: schema_version 1, mode development-only oppure release-preparation, ci_required booleano, rationale non vuota. File assente equivale a release-preparation con CI obbligatoria, senza cambiare i fingerprint legacy. In development-only ci_required deve essere false; errori e valori ignoti non ripiegano sul default.

La suite finale usa kind suite per esecuzioni locali; kind ci resta distinto. In solo sviluppo, evidenze red/green e suite finale valida danno development-complete; pre-release resta vietata. In preparazione al rilascio, CI obbligatoria per default; la policy può escluderla motivando l’alternativa locale, da valutare nella review umana. Ogni check dichiarato fallito o alterato blocca, anche se opzionale. Una suite finale deve seguire tutti i green; se CI richiesta anche questa deve seguire i green. Status non propone rilascio in solo sviluppo.

Cambi di policy invalidano review, piano ed evidenze; la transizione richiede rivalutazione esplicita. Manteniamo i comandi esistenti e nessuna migrazione obbligatoria; il nuovo tipo suite e file opzionale richiedono la CLI aggiornata. Livello di adozione, rischio light/full e maturità sono assi distinti. Non cambiano responsabilità umane o controlli filesystem.

## Criteri

- OBS-01: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- OBS-02: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- SEC-01: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- SEC-02: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- SEC-03: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- SLO-01: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- SLO-02: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- SLO-03: Nessuna modifica al meccanismo di persistenza o ai budget; verifiche esistenti preservate.
- COST-01: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- COST-02: Nessuna modifica al meccanismo di persistenza o ai budget; verifiche esistenti preservate.
- UX-01: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.
- UX-02: Test comportamentali lifecycle e suite Rust; mutanti mirati; verifica link e risorse incorporate.

## Verifica

Test comportamentali su fixture: assenza CI locale ammessa solo con policy e suite finali valide, esiti/timestamp/hash corretti; default legacy stretto; policy malformate rifiutate; cambi policy invalidano decisioni; pre-release impossibile in solo sviluppo anche con CI. Verificare formato, Clippy, suite completa, doctor, documentazione e mutanti pertinenti. Le prove su fixture non sono evidenze della CI del progetto.
