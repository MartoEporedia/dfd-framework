# Specifica di Feature – Evidenze legate allo stato dei file

> **Stato**: in review di design  
> **Data creazione**: 2026-10-31  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: issue #3 (MartoEporedia/dfd-framework)

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: le evidenze non sono legate allo stato dei file su cui la suite è stata eseguita. Nel laboratorio:
  - F01b: codice cambiato dopo `verify`, hash ricalcolati a mano;
  - F04: log scritto a mano con hash coerenti;
  - F09: file nuovo fuori da `evidence.files`;
  - F12: regressione in un file esistente non elencato;
  - F13: design cambiato e riferimenti copiati da `status` senza nuovi red/green.

  In tutti e cinque i casi i gate restano verdi.
- Scelte del manutentore (2026-10-31):
  - comando `dfd exec` **facoltativo**;
  - divergenza dall'ultima suite = **blocco**;
  - fingerprint di design **per criterio** incluso;
  - campi **facoltativi in `schema_version: 1`**, con requisiti attivati da `source_roots`.
- Metriche di successo (laboratorio v3, baseline che dichiara `source_roots` con suite registrata da `dfd exec`):
  - F01b, F09, F12 e F13 diventano BLOCCATI;
  - F04 diventa BLOCCATO (suite registrata a mano senza snapshot);
  - F00 e la baseline ricostruita da zero restano verdi;
  - i domini senza `source_roots` non cambiano comportamento.
- Componenti modificati:
  - `src/model.rs`: `TestEvidence`, `Lifecycle`;
  - `src/cli.rs`: comando `exec`;
  - `src/development.rs`: snapshot, regole del gate, `exec`;
  - `src/lib.rs`: fingerprint per criterio, validazione delle radici;
  - `src/store.rs`: visita sicura delle radici;
  - test, mutanti, contratto, skill e guide IT/EN.
- Fuori scope:
  - record rapidi;
  - esecuzione della CI (`dfd exec` non accetta `--kind ci`);
  - autenticazione delle evidenze: chi falsifica deliberatamente può scrivere anche lo snapshot (l'issue lo dichiara);
  - qualità dei test (F02, F11).

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso: architecture, reliability e security medium → full proporzionato.

## 3. Comportamenti e contratti

1. **`source_roots`** (facoltativo, in `lifecycle.json`): percorsi di file o directory relativi alla radice. Ognuno deve esistere dentro il progetto, senza duplicati, altrimenti la Fase 0 è bloccata. L'albero di una radice comprende tutti i file regolari sotto di essa, esclusi:
   - le directory e i file nascosti (nome che inizia con `.`, quindi `.dfd` e `.git`);
   - i symlink, che non vengono seguiti.

   - i nomi elencati in `source_excludes` a qualunque livello; senza il campo vale la lista integrata `__pycache__`, `node_modules`, `target`, perché i file generati mancano in un clone e farebbero divergere lo snapshot (lista vuota: nessuna esclusione).

   Lo snapshot è la mappa `percorso → sha256` di questi file.
2. **Campi facoltativi dei check** in `evidence.json`, sempre a `schema_version: 1`; i campi sconosciuti restano rifiutati:
   - `snapshot`: mappa `percorso → sha256` al momento dell'esecuzione;
   - `criteria_fingerprints`: mappa `criterio → fingerprint` per i check red e green di un task;
   - `recorded_by`: vale `"dfd"` quando il check è registrato da `dfd exec`. È una dichiarazione, non un'autenticazione.
3. **Fingerprint di design per criterio**: è lo SHA-256 di tre componenti:
   - la voce del criterio in `design.json`;
   - la voce del catalogo;
   - il testo DoD del criterio (unità di `dod.md` della review selettiva, oppure l'intero `dod.md` per le review legacy).

   `status` lo espone in `development.criteria_fingerprints`. Le modifiche a `spec.md`, `risk.json` o al contesto richiedono la review di design ma non nuovi red/green.
4. **Regole del gate di sviluppo**, attive solo se il dominio dichiara `source_roots`:
   1. La suite finale più recente (check `suite` o `ci` valido con exit 0) deve avere `snapshot`. Se manca: errore "Suite finale senza snapshot dei file".
   2. Lo snapshot di quella suite deve coincidere con l'albero attuale sotto le radici. Altrimenti: errore "Suite eseguita su una versione diversa dei file", con i file modificati, aggiunti o rimossi.
   3. Per ogni task, l'ultimo green deve avere `criteria_fingerprints` uguali ai fingerprint attuali di tutti i criteri del task. Altrimenti: errore "Task X: criterio C cambiato dopo il green: servono nuovi red/green".
   4. I file sotto le radici non presenti in `evidence.files` producono un avviso (`development.warnings`), non un blocco.
5. **`dfd exec <id> --kind red|green|suite [--task <task>] -- <programma> [argomenti…]`**:
   - esegue il programma senza shell, dalla radice del progetto, con l'ambiente ereditato, tenendo il lock del progetto;
   - salva stdout e stderr in `.dfd/features/<id>/evidence/<NN>-<kind>[-<task>].log`, con comando, data di fine (UTC) ed exit code;
   - aggiunge il check a `evidence.json`, con `snapshot` (albero sotto `source_roots`, oppure i file di `evidence.files` se le radici non sono dichiarate), `criteria_fingerprints` (per red e green con task) e `recorded_by: "dfd"`.

   Richiede il piano (`evidence.json` presente). `--task` è obbligatorio per red e green e vietato per suite. `dfd exec` restituisce 0 quando il check è registrato, qualunque sia l'exit code del programma, riportato nell'output. Restituisce 1 se il programma non parte. Il comando registrato non deve invocare comandi `dfd` che prendono il lock sullo stesso progetto.
6. **Compatibilità**: senza `source_roots` nessuna regola nuova. `dfd exec` è usabile anche senza radici, ma lo snapshot di una suite registrata a mano si può sempre scrivere secondo il contratto. Chi dichiara `source_roots` riesegue una volta la suite e i green.

- Errori, failure mode e comportamenti degradati: un file illeggibile sotto le radici è un errore di lettura (exit 1) e non viene ignorato in silenzio.
- Compatibilità e migrazioni: come al punto 6. Nessun cambio di `schema_version`.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | applicabile | Errori e avvisi nominano i file e i criteri divergenti | Test workflow |
| OBS-02 | applicabile | Check legati allo snapshot dei file e ai fingerprint dei criteri; `recorded_by` | Test, laboratorio |
| SEC-01 | applicabile | Radici e log passano da Store; symlink e file nascosti esclusi; nessuna shell | Test con symlink e percorso esterno |
| SEC-02 | applicabile | `exec` aggiunge un check e un log nuovo, senza sovrascrivere | Test di preservazione |
| SEC-03 | applicabile | Un design cambiato richiede nuovi red/green per i criteri toccati | Test F13, mutante |
| SLO-01 | applicabile | Campi facoltativi; nessun effetto senza `source_roots` | Suite esistente invariata |
| SLO-02 | applicabile | Red/green, suite, fmt, Clippy, doctor, mutanti | Log in `evidence/` |
| SLO-03 | applicabile | Lock del progetto durante `exec`; scrittura atomica di log ed `evidence.json` | Test |
| COST-01 | applicabile | Nessuna dipendenza; `exec` esegue solo il programma indicato | Test con PATH vuoto |
| UX-01 | applicabile | Contratto, guide e skill (`dfd-implement`, `dfd-verify`, `dfd-setup`) IT/EN | Revisione documenti |
| COST-02 | esclusa | Nessuna soglia di prestazioni; il costo dell'hash dipende dalle radici dichiarate | — |
| UX-02 | esclusa | Le skill restano dodici; cambia solo il testo | — |

## 5. Guardrail e decisioni

- Guardrail:
  - la CLI esegue un comando del progetto solo con `dfd exec` richiesto esplicitamente;
  - niente shell, niente rete aggiunta;
  - nessuna approvazione derivata;
  - nessuna regola nuova senza `source_roots`.
- Alternative scartate dal manutentore:
  - solo snapshot dichiarato;
  - `dfd exec` obbligatorio;
  - avviso al posto del blocco;
  - rinvio del fingerprint per criterio;
  - `schema_version: 2`.

## 6. Test e validazione prevista

- Test funzionali:
  - F01b riprodotto: codice cambiato dopo la suite con hash aggiornati, risultato blocco;
  - F09: file nuovo sotto le radici, blocco;
  - F12: file non dichiarato modificato, blocco;
  - F13: criterio del design cambiato e riferimenti aggiornati, blocco per il task;
  - suite senza snapshot (F04), blocco;
  - file non dichiarato, avviso;
  - `exec` per red, green e suite: log, check, snapshot, `recorded_by`, exit code del programma e programma inesistente;
  - radici con symlink, file nascosti e percorso esterno;
  - dominio senza `source_roots` invariato.
- Verifiche manuali: laboratorio v3 livello 1, con baseline che dichiara `source_roots: ["mensa", "tests", "ci.sh"]` e suite e green rieseguiti con `dfd exec`.
- Validazione in produzione: nessun rilascio.

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Riferimento alla review e alla decisione umana: da registrare con `dfd decide`.

## 8. Revisioni

- 2026-10-31: prima bozza, con le scelte del manutentore.
- 2026-11-01: `source_excludes` con lista integrata, dopo il falso allarme trovato su `__pycache__` (scelta del manutentore).
