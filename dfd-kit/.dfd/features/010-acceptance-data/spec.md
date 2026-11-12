# Specifica di Feature – Casi di accettazione come dipendenze del design

> **Stato**: in review di design  
> **Data creazione**: 2026-11-12  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: richiesta del manutentore in chat il 2026-11-12, dopo il laboratorio dfd-gate-lab v4 (test come dati)

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso. Nel laboratorio v3 la CLI non giudica i test: F02 (test svuotato) e F11 (codice e test indeboliti insieme) restano solo SEGNALATI. Il laboratorio v4 ha provato i *test come dati* (M. Vaccari, "Tests as Data", martinfowler.com, 2026-11-11). I criteri sono verificati da file di casi in un DSL esterno, approvati con il design, e un runner li esegue. Per proteggerli, il laboratorio ha elencato i file di casi in `lifecycle.json → suite_entrypoints`, già inclusi nel fingerprint di design (issue #4). Così F02 e F11 diventano BLOCCATI. Il laboratorio ha però trovato tre lacune:
  1. `suite_entrypoints` è una lista di dominio: con più feature, cambiare un caso di una feature invalida la review di design di tutte;
  2. la Fase 0 esige che gli entrypoint esistano, mentre i casi nascono con la specifica di una feature;
  3. un file di casi che nessuno ha dichiarato non è protetto, e nulla lo segnala.
- Scelte del manutentore (2026-11-12):
  - dichiarazione **per criterio** (`criteria[].acceptance`): il file entra nel fingerprint di design e nel fingerprint del criterio;
  - file di casi non dichiarati: **avviso**, non blocco, tramite un campo di dominio `acceptance_roots`.
- Metriche di successo (laboratorio v4 rieseguito con il nuovo binario, baseline con i casi in `criteria[].acceptance` invece che in `suite_entrypoints`):
  - F02, F02r, F11, F11c restano BLOCCATI; F00, CX-codice e CX-dati restano come nella v4;
  - un file di casi nuovo non dichiarato produce un avviso;
  - cambiare un caso di una feature non invalida la review di un'altra feature dello stesso dominio;
  - con `source_roots`, cambiare un caso dopo il green richiede nuovi red/green per i task del criterio;
  - le baseline esistenti senza `acceptance` (v3-i3, v4, feature 001-009 di dfd-kit) mantengono gate e fingerprint.
- Componenti modificati:
  - `src/model.rs`: `CriterionReference`, `Lifecycle`;
  - `src/lib.rs`: validazione del design e della Fase 0, input del fingerprint di design, fingerprint per criterio;
  - `src/development.rs`: avviso sui file di casi non dichiarati;
  - test, mutanti, contratto, skill e guide IT/EN.
- Fuori scope:
  - il formato dei file di casi e il runner, che restano del progetto (la CLI non esegue né interpreta i casi);
  - record rapidi;
  - giudizio sulla qualità dei casi, che resta compito della review di design;
  - autenticazione delle decisioni (F08).

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso: architecture e reliability medium → full proporzionato.

## 3. Comportamenti e contratti

1. **`criteria[].acceptance`** (facoltativo, in `design.json`, `schema_version: 1`): lista di percorsi di file relativi alla radice del progetto. Vale solo per i criteri `applicable`. Ogni percorso deve indicare un file regolare dentro il progetto, non un symlink, senza duplicati nello stesso criterio. Lo stesso file può servire più criteri. In caso contrario la review di design è bloccata, con un errore che nomina criterio e percorso. Una lista vuota equivale al campo assente e non viene serializzata.
2. **Fingerprint di design**: per ogni file dichiarato da un criterio, il suo SHA-256 entra negli input del design con il percorso come chiave, sia nella review selettiva sia in quella legacy. Un file mancante vale `absent`. Quindi:
   - modificare, rimuovere o rinominare un file di casi manda la review in `stale-review`;
   - `changes` nomina il percorso.

   Senza `acceptance` gli input non cambiano: i fingerprint esistenti restano validi.
3. **Fingerprint per criterio** (issue #3): se il criterio dichiara `acceptance`, al calcolo si aggiunge la mappa `percorso → sha256` dei suoi file. Con `source_roots` nel dominio, la regola esistente "criterio cambiato dopo il green" richiede quindi nuovi red/green per i task del criterio quando un suo file di casi cambia. Senza `acceptance` il fingerprint del criterio non cambia.
4. **`acceptance_roots`** (facoltativo, in `lifecycle.json`): percorsi di file o directory dove il progetto tiene i file di casi. Seguono le stesse regole di `source_roots`:
   - in Fase 0 ognuno deve esistere dentro il progetto, senza duplicati;
   - la visita esclude file e directory nascosti, i symlink e i nomi di `source_excludes` (o la lista integrata).
5. **Avviso sui file di casi non dichiarati**: con `acceptance_roots`, il gate di sviluppo della feature aggiunge a `development.warnings` "File di casi non dichiarato da alcun criterio: <percorso>" per ogni file sotto le radici che nessun criterio dichiara. Contano i criteri di qualunque feature del dominio. È un avviso, non un blocco, ed è attivo con o senza `source_roots`. Un `design.json` di un'altra feature che non si legge viene ignorato: i suoi file risultano non dichiarati.

- Errori, failure mode e comportamenti degradati: un file dichiarato e illeggibile per motivi diversi dall'assenza è un errore di lettura (exit 1), non viene ignorato.
- Compatibilità e migrazioni: campi facoltativi, nessun cambio di `schema_version`. Chi sposta i casi da `suite_entrypoints` ad `acceptance` cambia `lifecycle.json` e `design.json`, quindi deve riconfermare la review di design una volta.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | applicabile | Errori di validazione, `changes` e avvisi nominano criterio e percorso | Test workflow |
| OBS-02 | applicabile | I casi dichiarati legano criterio, file e decisione di design | Test, laboratorio |
| SEC-01 | applicabile | Percorsi via Store: niente escape né symlink; radici con le regole di `source_roots` | Test con symlink e percorso esterno |
| SEC-03 | applicabile | Un caso cambiato invalida la review e, con `source_roots`, il green del criterio | Test, mutanti |
| SLO-01 | applicabile | Campi facoltativi; fingerprint invariati senza `acceptance` | Suite esistente; status invariato delle feature 001-009 e delle baseline v3-i3 e v4 |
| SLO-02 | applicabile | Red/green, suite, fmt, Clippy, doctor, mutanti | Log in `evidence/` |
| COST-01 | applicabile | Nessuna dipendenza; solo letture di file dichiarati | Test con PATH vuoto |
| UX-01 | applicabile | Contratto, guide e skill IT/EN (`dfd-setup`, `dfd-specify`, `dfd-review-design`, `dfd-implement`) | Revisione documenti, check_translations |
| SEC-02 | esclusa | Nessuna scrittura nuova: la CLI legge soltanto | — |
| SLO-03 | esclusa | Nessuna scrittura o lock nuovi | — |
| COST-02 | esclusa | Nessuna soglia di prestazioni; il costo dipende dai file dichiarati | — |
| UX-02 | esclusa | Le skill restano dodici; cambia solo il testo | — |

## 5. Guardrail e decisioni

- Guardrail:
  - la CLI non esegue né interpreta i file di casi;
  - nessuna approvazione derivata;
  - nessun effetto senza i campi nuovi;
  - `schema_version` 1.
- Alternative scartate dal manutentore:
  - dichiarazione per feature (invalida solo la review, non il green);
  - blocco sui file non dichiarati;
  - nessun controllo CLI sui file non dichiarati.

## 6. Test e validazione prevista

- Test funzionali (red prima dell'implementazione):
  - un caso dichiarato e modificato dopo l'approvazione manda la review in `stale-review`, con il percorso in `changes`, sia in review selettiva sia legacy;
  - con `source_roots`, un caso modificato dopo il green blocca il task del criterio;
  - validazione: `acceptance` su criterio escluso, file mancante, percorso esterno, symlink, duplicato;
  - `acceptance_roots`: file non dichiarato → avviso; file dichiarato da un'altra feature del dominio → nessun avviso; radice mancante → errore di Fase 0;
  - due feature nello stesso dominio: cambiare il caso dell'una non invalida l'altra;
  - design senza `acceptance`: stesso fingerprint del binario precedente.
- Verifiche manuali:
  - `dfd status` delle feature 001-009 invariato rispetto al binario `eec9dffa…`;
  - laboratorio v4 rieseguito con baseline che usa `acceptance` e `acceptance_roots: ["tests/casi"]`;
  - F00 sulle baseline v3-i3 e v4 esistenti.
- Validazione in produzione: nessun rilascio (modalità `development-only`).

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Riferimento alla review e alla decisione umana: da registrare con `dfd decide` solo dopo la decisione espressa del manutentore.

## 8. Revisioni

- 2026-11-12: prima bozza, con le scelte del manutentore.
