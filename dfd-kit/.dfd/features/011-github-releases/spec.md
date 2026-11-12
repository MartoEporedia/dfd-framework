# Specifica di Feature – Release GitHub con changelog

> **Stato**: in review di design  
> **Data creazione**: 2026-11-12  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: richiesta del manutentore in chat il 2026-11-12 ("utilizzare le release di GitHub, anche per avere un changelog")

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso:
  - il binario esiste solo in `dfd-kit/dist/`, che è escluso da git;
  - il prompt di installazione del README dice di usare `dfd-kit/dist/dfd-macos-arm64` dopo il clone, ma chi clona non lo trova;
  - la configurazione CI (`ci/github-actions.yml`) costruisce i binari dei quattro sistemi, ma non è attiva;
  - non esiste un changelog, né tag o release.
- Scelte del manutentore (2026-11-12):
  - release GitHub al **push di un tag `v*`**;
  - **`CHANGELOG.md` IT/EN** nel repository, il cui corpo diventa la nota della release;
  - prima release **`v0.1.0` sull'attuale `main`** (senza la PR #15), poi `v0.2.0` con la #15;
  - il dominio `dfd-kit` passa a **`release-preparation` con `ci_required: true`**.
- Metriche di successo:
  - la release `v0.1.0` contiene i quattro archivi (Linux x86_64 musl, macOS arm64, macOS x86_64, Windows x86_64), un `SHA256SUMS` complessivo e le note della versione prese da `CHANGELOG.md` e `en/CHANGELOG.md`;
  - un tag che non corrisponde alla versione di `Cargo.toml`, o che non ha una sezione nel changelog, non produce alcuna release;
  - la CI è verde sulla PR;
  - il prompt di installazione scarica il binario dalla release.
- Componenti modificati:
  - `.github/workflows/dfd-kit.yml`: la CI attuale, spostata da `dfd-kit/ci/github-actions.yml`, con `workflow_call`;
  - `.github/workflows/release.yml`: workflow nuovo;
  - `dfd-kit/scripts/release_notes.py` e il suo test;
  - `dfd-kit/CHANGELOG.md` ed `en/CHANGELOG.md`;
  - README, CONTRIBUTING e prompt di installazione, IT/EN;
  - `lifecycle.json` e `process.md` del dominio `dfd-kit`.
- Fuori scope:
  - firma dei binari e attestazioni di provenienza;
  - distribuzione via package manager;
  - aggiornamento automatico della CLI;
  - cambiamenti al comportamento della CLI.

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso: architecture e security medium → full proporzionato.

## 3. Comportamenti e contratti

1. **CI** (`.github/workflows/dfd-kit.yml`): si attiva su push a `main`, su pull request, a mano e come workflow riutilizzabile (`workflow_call`). I job (`quality`, `mutations`, `binaries`) sono quelli dell'attuale `ci/github-actions.yml`. Il timeout dei mutanti è commisurato alla batteria completa. Permessi: `contents: read`.
2. **Release** (`.github/workflows/release.yml`), al push di un tag `v<major>.<minor>.<patch>`:
   1. **verifica**: `release_notes.py --check <tag>` controlla che il tag corrisponda a `version` in `dfd-kit/Cargo.toml` e che `CHANGELOG.md` ed `en/CHANGELOG.md` abbiano una sezione non vuota per quella versione. Se il controllo fallisce, il workflow si ferma senza creare nulla;
   2. **CI completa** tramite `workflow_call`; un job fallito ferma la release;
   3. **pubblicazione**, unico job con `contents: write`: scarica i quattro archivi, scrive un `SHA256SUMS` complessivo e crea la release con le note generate da `release_notes.py <tag>`, cioè la sezione italiana seguita da quella inglese. Se la release esiste già, il job fallisce senza sovrascriverla.
3. **`scripts/release_notes.py`** (solo libreria standard Python):
   - `release_notes.py <tag>` stampa le note;
   - `release_notes.py --check <tag>` esce 0 o 1 con un messaggio che nomina il problema: formato del tag, versione diversa da `Cargo.toml`, sezione assente o vuota.

   La sezione di una versione va da `## [<versione>]` fino al titolo `## [` successivo.
4. **Changelog** (formato Keep a Changelog, versionamento semantico): una sezione `[Unreleased]` e, per ogni versione, le categorie `Aggiunto`/`Added`, `Modificato`/`Changed`, `Corretto`/`Fixed`. La sezione `0.1.0` riassume il lavoro delle PR #1-#14 e di questa feature.
5. **Lifecycle**: `release-preparation` con `ci_required: true` e la motivazione aggiornata. Una release richiede la pre-release e la review di release DFD della feature che la porta, con un log CI reale registrato come `kind: ci`.
6. **Installazione**: il prompt del README scarica l'archivio del proprio sistema dall'ultima release e lo verifica con `SHA256SUMS`; la compilazione con Cargo resta il fallback.

- Errori, failure mode e comportamenti degradati: un tag sbagliato non pubblica nulla. Una release sbagliata si elimina e si ripubblica con un nuovo tag di patch, non sovrascrivendo gli archivi.
- Compatibilità e migrazioni: il passaggio a `release-preparation` cambia `lifecycle.json` e invalida le review di design delle feature aperte. La feature 010, aperta nella PR #15, va riconfermata dopo il rebase.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | applicabile | `release_notes.py --check` nomina il problema; i job falliti fermano la release | Test unitari dello script |
| OBS-02 | applicabile | Log CI reali della PR registrati come `kind: ci`; release legata al tag e al commit | Evidenze, review di release |
| SEC-01 | applicabile | Permessi minimi: `contents: read` per la CI, `contents: write` solo nella pubblicazione; azioni a versione fissata | Revisione dei workflow |
| SEC-02 | applicabile | Una release esistente non viene sovrascritta; `dist/` locale invariato | Revisione del workflow, prima release |
| SEC-03 | applicabile | Release solo dopo pre-release e review di release umana; tag spinto dal manutentore o su sua richiesta esplicita | Decisioni registrate |
| SLO-01 | applicabile | CLI invariata; contratti invariati | Suite completa |
| SLO-02 | applicabile | Test dello script, suite locale, CI reale | Log in `evidence/` |
| SLO-03 | esclusa | Nessuna scrittura della CLI cambia | — |
| SLO-04 | applicabile | CI attivata e richiesta per la release | CI della PR verde |
| COST-01 | applicabile | Binario senza dipendenze nuove; script con la sola libreria standard | Revisione |
| COST-02 | applicabile | Timeout dei mutanti misurato sulla durata reale | Durata del job nella CI |
| UX-01 | applicabile | README, CONTRIBUTING, prompt di installazione e changelog IT/EN coerenti | Revisione, `check_translations` |
| UX-02 | esclusa | Skill e adapter invariati | — |
| UX-03 | esclusa | Nessun cambiamento alle skill | — |

## 5. Guardrail e decisioni

- Guardrail:
  - nessuna release senza CI verde, sezione del changelog e versione coerente;
  - nessuna sovrascrittura di release esistenti;
  - token con i permessi minimi;
  - il tag lo crea il manutentore o lo chiede esplicitamente.
- Alternative scartate dal manutentore:
  - note solo automatiche di GitHub;
  - release manuale con `workflow_dispatch`;
  - restare `development-only`;
  - una sola `v0.1.0` dopo la #15.

## 6. Test e validazione prevista

- Test funzionali (red prima dell'implementazione): `tests/test_release_notes.py`, con:
  - estrazione della sezione IT e EN;
  - tag non conforme;
  - versione diversa da `Cargo.toml`;
  - sezione assente o vuota;
  - ultima sezione del file;
  - `[Unreleased]` non selezionabile come versione.
- Verifiche manuali:
  - CI reale sulla PR, con log scaricati;
  - `actionlint`, se disponibile, oppure revisione dei workflow;
  - dopo il merge, release `v0.1.0` con quattro archivi e checksum verificati scaricandoli.
- Validazione in produzione: la prima release è il rollout. Rollback: eliminare la release e il tag, poi ripubblicare con una patch.

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Riferimento alla review e alla decisione umana: da registrare con `dfd decide` dopo la decisione espressa del manutentore.

## 8. Revisioni

- 2026-11-12: prima bozza, con le scelte del manutentore.
