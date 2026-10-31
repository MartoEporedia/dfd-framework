# Specifica di Feature – Cosa è cambiato dopo l'ultima decisione

> **Stato**: in review di design  
> **Data creazione**: 2026-10-31  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: issue #6 (MartoEporedia/dfd-framework)

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: quando una review diventa da riconfermare il report dice solo `stale-review`. In un incremento legittimo è lo stato normale. Nel laboratorio (livello 2) 14 branch, 10 con una bugia e 4 puliti, finiscono tutti `stale-review` con zero errori, e il revisore deve ricostruire da solo cosa è cambiato.
- Metriche di successo: nel laboratorio v3 ogni fault che rende una review `stale-review` mostra l'elenco di ciò che è cambiato dopo la decisione: F01b, F02, F03, F04, F05, F11 e F13 sulla release; F07, F07b e F08 sul design. F00 non elenca nulla.
- Componenti modificati: `src/model.rs` (campo `snapshot` nelle decisioni), `src/lib.rs`, `src/release.rs`, `src/quick.rs` (snapshot al momento di `decide` e confronto in `status`/report), test, mutanti, contratto e skill IT/EN.
- Fuori scope: "file cambiati dopo l'ultima suite" e file fuori da `evidence.files` (issue #3, `source_roots`); cambiare quando una review diventa `stale-review`.
- Nel brownfield: le decisioni già registrate non hanno snapshot; per loro l'elenco non è disponibile e il report lo dice.

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso light/full: una sola dimensione medium (architecture) → light.
- Motivazioni e dati ancora mancanti: nessuno.

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione:
  1. `decide` (design, release, record rapido) salva nella decisione il campo facoltativo `snapshot`: una mappa `componente → sha256` degli input del fingerprint in quel momento. Il fingerprint aggregato e la sua formula non cambiano.
  2. Componenti del design: i file della feature (`spec.md`, `design.json`, `risk.json`, `review-notes.md`), i guardrail del dominio e comuni, `lifecycle.json` e il contesto (titolo, ambito, tipo, rischio, percorso, modalità di progetto). Per le review selettive anche `criteria.json#<ID>` e `dod.md#<ID>` per ogni criterio selezionato; per le legacy i file di dominio fingerprintati (`dod.md`, `criteria.json`, `adoption.json`, file di setup, `config.json`).
  3. Componenti della release: `rollout.md`, `rollout.json`, `release-notes.md`, `plan.md`, `plan.json`, `evidence.json`, la decisione di design corrente (`design-decision`) e, singolarmente, ogni file di `evidence.files` e ogni log dei check di `evidence.json`, con il loro hash attuale. Così un file di codice cambiato dopo la decisione compare per nome anche se `evidence.json` è stato aggiornato.
  4. Componenti del record rapido: i campi del record (`record#<campo>`), ogni file e log dichiarato, i criteri selezionati, i guardrail e le policy.
  5. Quando il gate è `stale-review` e l'ultima decisione ha uno snapshot, l'output aggiunge `changes: {"changed": [...], "added": [...], "removed": [...]}` (in `status`, nell'output di `review-design`/`review-release`/`decide` e in `release` dentro `status`). I report `design-review.md` e `release-review.md` elencano le stesse voci. Senza snapshot, `changes` è `null` e il report dice che l'elenco non è disponibile per una decisione registrata con una CLI precedente.
  6. Negli altri gate `changes` è assente: l'informazione serve solo quando la review va riconfermata.
- Errori, failure mode e comportamenti degradati: un file elencato e poi rimosso compare come `absent` nell'hash e quindi tra i `changed`; un componente nuovo (per esempio `release-notes.md` creato dopo) cambia da `absent`.
- API, eventi, dati e dipendenze coinvolti: campo `snapshot` facoltativo nelle decisioni (`state.json`, `release-decisions.json`, `.dfd/changes/<id>.json`); campo `changes` nell'output.
- Compatibilità e migrazioni, se necessarie: additivo. Le decisioni esistenti restano valide; i file restano a `schema_version: 1`; i campi sconosciuti restano rifiutati.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | applicabile | `changes` e report dicono cosa ha reso la review da riconfermare | Test workflow su design, release, record rapido e decisione legacy |
| SEC-03 | applicabile | Lo snapshot è informativo: gate e fingerprint invariati, nessuna approvazione derivata | Test esistenti invariati, mutante |
| SLO-01 | applicabile | Campo facoltativo documentato; decisioni precedenti leggibili | Test con decisione senza snapshot |
| SLO-02 | applicabile | Red/green, suite, fmt, Clippy, doctor, mutanti | Log in `evidence/` |
| UX-01 | applicabile | Contratto e skill di review IT/EN descrivono `changes` | Revisione documenti |
| OBS-02 | esclusa | Le evidenze non cambiano formato | — |
| SEC-01 | esclusa | Nessun percorso nuovo oltre a quelli già letti dai gate | — |
| SEC-02 | esclusa | Nessuna scrittura nuova oltre al campo nella decisione | — |
| SLO-03 | esclusa | Scritture e lock invariati | — |
| COST-01 | esclusa | Nessuna dipendenza | — |
| COST-02 | esclusa | Nessuna soglia di prestazioni | — |
| UX-02 | esclusa | Nessun comando delle skill cambia | — |

## 5. Guardrail e decisioni

- Guardrail: lo snapshot non entra nel fingerprint e non decide alcun gate; nessuna decisione esistente viene riscritta.
- Alternative scartate: salvare solo l'elenco dei file senza i componenti logici (non spiegherebbe cambi di catalogo, DoD selettiva o contesto); confrontare con la cronologia git (contratto: la CLI non interroga git).
- Collegamenti a policy e ADR esistenti: [contratto](../../../../templates/contratto_toolkit.md).

## 6. Test e validazione prevista

- Test funzionali derivati dai criteri: design approvato poi `spec.md` modificato → `changes.changed` contiene `spec.md`; release approvata poi file di codice e `evidence.json` aggiornati → `changes.changed` contiene il file di codice; `release-notes.md` creato dopo → elencato; decisione senza snapshot → `changes: null`; record rapido; gate non stale → nessun `changes`.
- Test non funzionali proporzionati al rischio: nessuno.
- Verifiche manuali e loro evidenze attese: laboratorio v3 livello 1 (baseline rimigrata con il binario nuovo, perché le decisioni della baseline devono avere uno snapshot).
- Piano di validazione in produzione, soglie e rollback: nessun rilascio.

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Condizioni e responsabili: —
- Riferimento alla review e alla decisione umana: da registrare dal manutentore con `dfd decide`.

## 8. Revisioni

- 2026-10-31: prima bozza.
