# Specifica di Feature – Exit code di status e fase effettiva

> **Stato**: in review di design  
> **Data creazione**: 2026-10-31  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: issue #7 (MartoEporedia/dfd-framework)

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: `dfd status` esce sempre 0, anche quando un gate mostrato è bloccato; uno script o un agente che guarda solo l'exit code non vede il blocco (esperimento di fault injection, fault F06: clone senza evidenze, "Evidenza assente" ×7, `status` exit 0). Il campo `phase` non segue lo stato effettivo: resta `release-reviewed` quando lo sviluppo si blocca (F01, F10), e un `verify` riuscito su una feature già in pre-release o con release approvata lo riporta a `development-verified`.
- Metriche di successo: nel laboratorio v3 F06 passa da "status exit 0" a "status exit 2" nel clone; F00 (baseline pulita) e una baseline ricostruita restano con `status` exit 0; `effective_phase` coerente con i gate in tutti i fault di livello 1.
- Componenti modificati: `src/lib.rs` (status, exit code), `src/development.rs` (transizioni di fase in verify), test in `tests/workflow.rs`, mutazioni in `mutations/gates.json`, contratto, skill e guide IT/EN.
- Fuori scope: cambiare il significato dei gate o i loro valori; scrivere `state.json` da `status` (resta senza lock e in sola lettura); fasi dei record rapidi; issue #3-#6.
- Nel brownfield: le feature storiche del dominio (001, 002) e il record rapido `fix-template-specifica-scambiato` risultano oggi bloccati perché file condivisi sono cambiati dopo le loro evidenze. Con questa modifica `dfd status` senza ID in questo repository uscirà 2: è il comportamento scelto, non un falso allarme.

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso light/full: una sola dimensione medium (reliability: cambia il contratto di exit code) → light.
- Motivazioni e dati ancora mancanti: nessuno. Scelte confermate dal manutentore il 2026-10-31: exit 2 sempre con un gate bloccato (anche senza ID); `phase` resta storico e `status` aggiunge `effective_phase`.

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione:
  1. `status <feature>` esce 2 se almeno uno tra setup del dominio (`blocked`), design (`blocked`), sviluppo (`blocked-plan`, `blocked-evidence`) e release (`blocked`) è in stato bloccante; altrimenti 0. L'output aggiunge `blocking`: elenco dei gate bloccanti (`setup`, `design`, `development`, `release`), vuoto se nessuno.
  2. `status <record-rapido>` esce 2 con `gate: blocked`, 0 negli altri casi; `blocking` è `["change"]` o vuoto.
  3. `status` senza ID esce 2 se un setup, una feature o un record rapido è bloccante; l'output aggiunge `blocking` con voci `setup:<dominio>`, `feature:<id>/<gate>`, `change:<id>`.
  4. Stati di attesa umana o non bloccanti (`awaiting-human-review`, `stale-review`, `changes-requested`, `not-started`, `not-applicable`, gate positivi) non cambiano l'exit code: il campo `gate` resta l'autorità sul percorso.
  5. `status <feature>` aggiunge `effective_phase`, derivata dai gate correnti: `idea` (rischio non classificato), `assessed` (specifica assente), `design` (design non approvato o da riconfermare), `design-reviewed` (sviluppo non iniziato), `development` (sviluppo bloccato), `development-verified` (sviluppo completo o pronto e pre-release non iniziata), `pre-release` (release in corso o da riconfermare), `release-reviewed` (release approvata o approvata con condizioni). `phase` resta l'ultimo passaggio registrato in `state.json`.
  6. `verify` riuscito non regredisce `phase` se è già `pre-release` o `release-reviewed`; `verify` fallito riporta `phase` a `development` da `development-verified`, `pre-release` o `release-reviewed`.
- Errori, failure mode e comportamenti degradati: errori di lettura o JSON non valido restano exit 1, come oggi; exit 2 non sostituisce gli errori.
- API, eventi, dati e dipendenze coinvolti: output JSON di `status` (campi additivi `blocking`, `effective_phase`), exit code di `status`, transizioni di `phase` in `verify`. Nessuno schema di input cambia.
- Compatibilità e migrazioni, se necessarie: nessuna migrazione dei file. Cambio di contratto documentato: chi usa `status` in script deve trattare 2 come "gate bloccato" e non come errore di esecuzione (1).

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | applicabile | Exit code e `blocking` descrivono il blocco; `effective_phase` dallo stato effettivo | Test workflow su feature, record rapido e status globale |
| SEC-03 | applicabile | `effective_phase` non deriva né dichiara approvazioni: segue i gate calcolati | Test con release approvata e sviluppo poi bloccato |
| SLO-01 | applicabile | Cambio di contratto documentato in contratto, guide e skill IT/EN | `check_translations.py`, revisione dei documenti |
| SLO-02 | applicabile | Regressione red/green, suite, fmt, Clippy, doctor, mutanti sull'exit code e sulla fase | Log in `evidence/` |
| UX-01 | applicabile | Help, contratto e skill descrivono exit 2 e `effective_phase` | Revisione documenti IT/EN |
| UX-02 | applicabile | Le skill che chiamano `status` trattano exit 2 come gate bloccato | Revisione skill, `doctor` |
| OBS-02 | esclusa | Nessun cambiamento a evidenze o hash | — |
| SEC-01 | esclusa | Nessun nuovo accesso a percorsi | — |
| SEC-02 | esclusa | `status` resta in sola lettura; nessuna scrittura nuova | — |
| SLO-03 | esclusa | Scritture e lock invariati | — |
| COST-01 | esclusa | Nessuna dipendenza | — |
| COST-02 | esclusa | Nessuna soglia di prestazioni | — |

## 5. Guardrail e decisioni

- Guardrail: `status` non scrive file e non prende il lock; i gate non cambiano valore; nessuna decisione umana derivata.
- Eccezioni motivate, responsabili e decisioni: scelte del manutentore del 2026-10-31 in chat (exit 2 sempre; `effective_phase` derivata).
- Collegamenti a policy e ADR esistenti: [contratto](../../../../templates/contratto_toolkit.md), righe su exit code, `phase` e `verify`.

## 6. Test e validazione prevista

- Test funzionali derivati dai criteri: un test di regressione che riproduce F06 (evidenze rimosse dopo la release approvata: `status` deve uscire 2 con `blocking: ["development"]`, `effective_phase: development`) e la regressione di fase (`verify` dopo release approvata non riporta `phase` indietro). Test per record rapido bloccato e per `status` senza ID. Aggiornamento dei test esistenti che chiamano `status` su feature bloccate.
- Test non funzionali proporzionati al rischio: nessuno.
- Verifiche manuali e loro evidenze attese: livello 1 del laboratorio rieseguito in `~/lavori/dfd-gate-lab/v3/` con il binario nuovo; F00 e baseline ricostruita restano exit 0.
- Piano di validazione in produzione, soglie e rollback: nessun rilascio (solo sviluppo).

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Condizioni e responsabili: —
- Riferimento alla review e alla decisione umana: da registrare dal manutentore con `dfd decide`.

## 8. Revisioni

- 2026-10-31: prima bozza, con le scelte del manutentore su exit code e fase.
