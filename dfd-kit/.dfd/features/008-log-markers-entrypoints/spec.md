# Specifica di Feature – Marcatori nei log ed entrypoint della suite

> **Stato**: in review di design  
> **Data creazione**: 2026-10-31  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: issue #4 (MartoEporedia/dfd-framework)

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: la CLI non legge il contenuto dei log delle evidenze. Nel laboratorio:
  - F03: un log con `OK (skipped=1)` passa come verde;
  - F05: un log con `FAILED` ma `exit_code: 0`, ottenuto aggiungendo `|| true` in coda a `ci.sh`, passa come verde;
  - la modifica allo script della suite dopo l'approvazione non invalida nulla.
- Scelte del manutentore (2026-10-31): i marcatori producono un **avviso**, non un blocco; senza configurazione si usa una **lista integrata**; gli script della suite entrano nel **fingerprint di design**.
- Metriche di successo nel laboratorio v3:
  - F03 e F05 mostrano un avviso;
  - con `ci.sh` dichiarato come entrypoint, F05 rende il design `stale-review`, con `changes` che indica `ci.sh`;
  - F00 senza avvisi e baseline ricostruita verde.
- Componenti modificati: `src/model.rs` (`Lifecycle`), `src/lib.rs` (validazione e fingerprint), `src/development.rs` (avvisi), `src/release.rs` (avvisi nel report), test, mutanti, contratto, skill e guide IT/EN.
- Fuori scope:
  - avvisi sui record rapidi;
  - blocco del gate;
  - qualità dei test (F02, F11);
  - eseguire la suite (il contratto: la CLI non esegue comandi del progetto).

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso: architecture e reliability medium → full proporzionato.

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione:
  1. `lifecycle.json` accetta due campi facoltativi; `schema_version` resta 1 e i campi sconosciuti restano rifiutati:
     - `log_markers`: lista di stringhe;
     - `suite_entrypoints`: lista di percorsi relativi alla radice del progetto.
  2. Marcatori: senza `log_markers` vale la lista integrata `FAILED`, `skipped`, `Traceback`, `xfail`, `XFAIL`, `panicked`. La corrispondenza è una sottostringa esatta, sensibile alle maiuscole. Una lista vuota disattiva il controllo.
  3. Per ogni check di `evidence.json` di tipo `green`, `suite` o `ci` con `exit_code: 0` e log leggibile, ogni marcatore presente nel log produce un avviso. I log `red` sono esclusi, perché un fallimento lì è atteso.
  4. Dove compaiono gli avvisi:
     - `status`, nel campo `development.warnings`;
     - `verify`, nel campo `development.warnings` e nella sezione "Avvisi" di `development-review.md`;
     - `review-release`, nel campo `release.warnings` e nel report.

     Gli avvisi non cambiano né il gate né l'exit code.
  5. Entrypoint: ogni percorso di `suite_entrypoints` deve esistere come file nel progetto. Un percorso assente, duplicato o che esce dalla radice è un errore di setup (Fase 0). L'hash di ciascun file entra nel fingerprint di design, sia selettivo sia legacy, come componente `<percorso>` (visibile in `changes` di #6). Modificarlo dopo l'approvazione rende il design `stale-review`.
  6. Domini senza `suite_entrypoints` mantengono il fingerprint attuale. Aggiungere o rimuovere il campo invalida la review, perché `lifecycle.json` è già una dipendenza.
- Errori, failure mode e comportamenti degradati: un log non leggibile resta l'errore attuale ("Evidenza assente"); i marcatori non si cercano.
- API, eventi, dati e dipendenze coinvolti: `lifecycle.json` (campi facoltativi); output `warnings`.
- Compatibilità e migrazioni, se necessarie: nessuna per chi non usa i campi nuovi. Le skill `dfd-init`/`dfd-setup` suggeriscono di dichiarare gli entrypoint.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | applicabile | Avvisi su marcatori nei log verdi, senza cambiare gate ed exit code | Test su F03/F05 riprodotti |
| OBS-02 | applicabile | Il contenuto dei log delle evidenze entra nei controlli; entrypoint con hash | Test, laboratorio |
| SEC-01 | applicabile | Percorsi degli entrypoint validati da Store, niente escape | Test con percorso esterno |
| SEC-03 | applicabile | Modificare lo script della suite invalida la review di design | Test, mutante |
| SLO-01 | applicabile | Campi facoltativi; schema e rifiuto dei campi sconosciuti invariati | Test di compatibilità |
| SLO-02 | applicabile | Red/green, suite, fmt, Clippy, doctor, mutanti | Log in `evidence/` |
| UX-01 | applicabile | Contratto, guide e skill IT/EN | Revisione documenti |
| SEC-02 | esclusa | Nessuna scrittura nuova oltre al report rigenerato | — |
| SLO-03 | esclusa | Scritture e lock invariati | — |
| COST-01 | esclusa | Nessuna dipendenza | — |
| COST-02 | esclusa | Nessuna soglia di prestazioni | — |
| UX-02 | esclusa | Nessun comando delle skill cambia | — |

## 5. Guardrail e decisioni

- Guardrail: la CLI non esegue comandi; gli avvisi non approvano né bloccano; percorsi solo dentro la radice.
- Alternative scartate dal manutentore: blocco del gate; nessun marcatore di default; entrypoint solo nelle evidenze.

## 6. Test e validazione prevista

- Test funzionali:
  - F03 riprodotto: green con `OK (skipped=1)`;
  - F05 riprodotto: suite con `FAILED` ed exit 0;
  - log red con `FAILED`, che non deve produrre avvisi;
  - `log_markers: []` e lista personalizzata;
  - entrypoint modificato dopo l'approvazione, che porta a `stale-review` con `changes`;
  - entrypoint assente o fuori radice, che porta a un errore di setup;
  - lifecycle senza campi nuovi, con fingerprint invariato.
- Verifiche manuali: laboratorio v3 livello 1 con baseline che dichiara `suite_entrypoints: ["ci.sh"]`.
- Validazione in produzione: nessun rilascio.

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Riferimento alla review e alla decisione umana: da registrare con `dfd decide`.

## 8. Revisioni

- 2026-10-31: prima bozza.
