# Specifica di Feature – Inglese come lingua principale

> **Stato**: draft  
> **Data creazione**: 2026-11-18  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit (e framework nella radice del repository)  
> **Epic / Ticket**: richiesta del manutentore in chat il 2026-11-17 ("Fai in modo che l'inglese diventi la lingua principale")

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: oggi l'italiano è la lingua canonica. Vive nei percorsi originali, è il default della CLI e le regole di manutenzione partono da lì; l'inglese sta in `en/` e `dfd-kit/en/`. Il manutentore vuole che l'inglese diventi la lingua principale, così che chi arriva al repository o installa la CLI trovi l'inglese per primo.
- Scelte del manutentore (2026-11-17, in risposta a domande con opzioni):
  - "Inversione completa dei percorsi": l'inglese va nei percorsi originali, l'italiano si sposta in `it/` e `dfd-kit/it/`;
  - "Passano all'inglese": i progetti senza `.dfd/language.json` usano l'inglese, anche quelli già adottati.
- Obiettivo: inglese nei percorsi originali e come default della CLI, italiano completo e mantenuto in `it/` e `dfd-kit/it/`.
- No-go:
  - nessun contenuto perso o tradotto di nuovo: i file si spostano, il testo resta identico salvo link e riga di navigazione;
  - nessun artefatto compilato sotto `.dfd/` riscritto o tradotto;
  - nessun cambiamento ai contratti JSON (`language.json` resta `schema_version: 1`), ai comandi o alla forma del bundle installato.
- Appetite: 2 giorni.
- Metriche di successo:
  - `check_translations.py`: 56 coppie, 0 errori, con inglese nei percorsi originali e italiano in `it/`;
  - in un progetto senza `.dfd/language.json`, `dfd status` e `dfd --help` rispondono in inglese e `init` installa risorse inglesi; con `language.json` `it` o `--lang it` tutto resta italiano;
  - fmt, Clippy, test, doctor, mutanti e CI verdi.
- Componenti modificati:
  - documenti: le 56 coppie (19 del framework, 37 del kit) spostate con `git mv`, link e navigazione ricalcolati;
  - `build.rs` (sorgente dei due bundle), `src/i18n.rs` (default `En`), `tests/workflow.rs`, `mutations/gates.json`;
  - `scripts/check_translations.py` e `i18n/documents.json`, `scripts/release_notes.py` e `tests/test_release_notes.py`;
  - `AGENTS.md`, `glossario.md`, `CONTRIBUTING.md`, i README e il prompt di installazione (default `en`), `CHANGELOG.md` con nota di migrazione.
- Fuori scope:
  - invertire il catalogo dei messaggi (stringhe sorgente italiane in Rust con `i18n/en.json`): è interno e non visibile all'utente;
  - sostituire gli alias `<a id>` con slug inglesi: restano validi e i link esistenti li usano;
  - tradurre gli artefatti delle feature e del dominio `dfd-kit` sotto `.dfd/`;
  - la release (v0.3.0), decisa in pre-release.
- Nel brownfield: lo scambio riusa i meccanismi della feature 003 (bundle bilingue, `language`, navigazione rimossa dal bundle); le feature storiche già `stale-review` restano baseline.

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso light/full: architettura, business e affidabilità medium → full proporzionato.
- Motivazioni e dati ancora mancanti: nessuno; l'impatto sugli utilizzatori esistenti è una scelta espressa e va documentato.

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione:
  1. **Struttura.** Per ogni coppia, il percorso originale contiene l'inglese e `it/<stesso percorso>` (o `dfd-kit/it/...`) l'italiano. `en/` e `dfd-kit/en/` non esistono più. `AGENTS.md` resta unico e non accoppiato.
  2. **Navigazione.** Ogni documento accoppiato ha una sola riga `<!-- dfd-language --> [English](…) | [Italiano](…)`, con l'inglese per primo e i due link verso la coppia.
  3. **Link.** Ogni link relativo di un file spostato punta allo stesso documento di prima, nella stessa lingua del file (o alla risorsa non documentale di prima). Anchor e alias restano invariati. `check_translations.py` non trova link o anchor mancanti.
  4. **Default CLI.** Senza `.dfd/language.json`, senza `--lang` e fuori da un progetto, la lingua è `en`. `init`/`install` senza `--lang` salvano `en`. `--lang it` e `language.json` `it` mantengono l'italiano.
  5. **Bundle.** Il bundle inglese si costruisce dai percorsi originali, quello italiano da `it/`; i percorsi installati in `.dfd/kit/` e nelle skill restano gli stessi di oggi.
  6. **Release notes.** `release_notes.py` legge `CHANGELOG.md` (inglese) e `it/CHANGELOG.md` (italiano), con l'inglese per primo nelle note.
- Errori, failure mode e comportamenti degradati:
  - una coppia senza controparte, un link rotto o un file non spostato fanno fallire `check_translations.py` con il percorso;
  - un bundle selezionato al contrario (inglese per `it`) fa fallire i test di lingua e il mutante `language-resource-selection`;
  - un valore di lingua non valido resta un errore esplicito, ora con messaggio di default in inglese.
- API, eventi, dati e dipendenze coinvolti: nessun comando o contratto JSON nuovo. `i18n/documents.json` è un manifest di sviluppo; può cambiare forma per indicare la coppia a partire dall'inglese.
- Compatibilità e migrazioni:
  - **Incompatibile, voluto**: i progetti senza `language.json` passano all'inglese nei messaggi. Le risorse già installate (`.dfd/kit/`, skill) non cambiano finché non si esegue `dfd language en` o un nuovo `install`; gli artefatti compilati restano nella lingua originale.
  - Per restare in italiano: `dfd language it` (scrive `language.json`). Lo documentano CHANGELOG, README e glossario.
  - I link esterni ai percorsi originali mostrano l'inglese; la riga di navigazione porta all'italiano.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | applicabile | Errori di lingua e di setup con messaggi espliciti; `status` invariato nella forma. | Test di lingua red/green; suite. |
| OBS-02 | applicabile | Evidenze legate a snapshot e log reali (`dfd exec`). | `verify` della feature. |
| SEC-01 | applicabile | Nessun nuovo percorso scritto; lo script di spostamento opera solo dentro il repository. | Revisione dello script; test esistenti sui percorsi. |
| SEC-02 | applicabile | Il nuovo default non riscrive file; `language` preserva gli artefatti compilati. | Test `bilingual_project_language_resources_and_preservation` e nuovi test sul default. |
| SEC-03 | applicabile | Le scelte del manutentore sono citate; nessuna approvazione inventata. | Review di design. |
| SLO-01 | applicabile | Comandi e schemi invariati; cambio di default documentato con la migrazione. | Test; CHANGELOG; review. |
| SLO-02 | applicabile | fmt, Clippy, test, doctor, check_translations, test Python, mutanti. | Suite locale e CI. |
| SLO-03 | esclusa | Scritture atomiche e lock invariati. | — |
| COST-01 | applicabile | Binario offline con le stesse due lingue. | Build; doctor. |
| COST-02 | applicabile | Verifiche proporzionate: mutanti mirati alla selezione della lingua. | Log dei mutanti. |
| UX-01 | applicabile | Help, README, CONTRIBUTING e skill coerenti sulla nuova struttura e sul default. | check_translations; lettura dei README. |
| UX-02 | applicabile | Skill reperibili in entrambe le lingue con gli stessi percorsi installati. | doctor; test di install in `en` e `it`. |

## 5. Guardrail e decisioni

- Guardrail: nessuna perdita di contenuto; artefatti `.dfd/` intatti; contratti JSON invariati; nessuna dipendenza nuova.
- Decisioni: le due scelte del manutentore citate in §1. Le proposte da confermare in review sono in §7.
- Collegamenti: feature 003-bilingual (bundle e lingua), 011-github-releases (changelog e release).

## 6. Test e validazione prevista

- Test funzionali derivati dai criteri:
  - red/green: senza `language.json` la lingua è `en` (status, help, init);
  - `--lang it` e `language.json` `it` restano italiani;
  - il bundle `en` proviene dai percorsi originali e quello `it` da `it/`;
  - aggiornare i test che presuppongono l'italiano di default, senza indebolire le asserzioni.
- Test non funzionali: mutanti `language-resource-selection`, `language-persistence` più un nuovo mutante sul default `En`; `test_release_notes.py` red/green sui nuovi percorsi.
- Verifiche manuali: lettura di README e AGENTS.md; navigazione IT/EN su GitHub dopo il push.
- Validazione del rilascio: binario candidato provato su un progetto copiato senza `language.json` e su uno con `it`; la release v0.3.0 si decide in pre-release.

## 7. Domande aperte e review

- Proposte da confermare in review:
  - gli alias `<a id>` con slug italiani restano nei file inglesi;
  - il progetto `dfd-kit` stesso (senza `language.json`) passa ai messaggi inglesi, i suoi artefatti restano italiani;
  - il catalogo dei messaggi resta con sorgente italiana;
  - questa feature parte dal branch della PR #17 (ondata 1), così lo spostamento include quei contenuti; la sua PR va unita dopo la #17.
- Condizioni e responsabili: nessuna.
- Riferimento alla review e alla decisione umana: `design-review.md`, `review-notes.md`, decisione del manutentore.

## 8. Revisioni

- 2026-11-18: prima bozza
