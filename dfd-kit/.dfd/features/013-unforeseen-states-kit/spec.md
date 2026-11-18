# Specifica di Feature – Ondata 2: stati non previsti e qualità della specifica nel kit

> **Stato**: draft  
> **Data creazione**: 2026-11-18  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: richiesta del manutentore in chat il 2026-11-18 ("Ondata 2"), sul piano concordato in chat il 2026-11-16/17

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: la PR #17 (ondata 1) ha aggiunto al framework la reazione agli stati non previsti, i vincoli per tipo di cambiamento, i criteri aggiunti in sviluppo e la qualità della specifica. Il kit non li usa ancora: le sue copie di fondamenta, processo e template sono ferme alla revisione precedente, e le skill non guidano l'agente su questi punti. Nasce dall'esperienza d'uso di herdy2: elencare casi ha prodotto scappatoie; hanno funzionato il fallimento esplicito negli stati non previsti e i criteri aggiunti dal coder ma confrontati con la richiesta originale.
- Parole del richiedente: "Ondata 2" (2026-11-18), dopo il piano: "ondata 2, skill: `dfd-implement` propone i criteri nuovi invece di aggiungere codice non coperto; `dfd-review-design` e `dfd-verify` confrontano i criteri aggiunti con le parole del richiedente" e "riallineare `dfd-kit/templates/`" attraverso il percorso DFD.
- Obiettivo: il kit installa la revisione dell'ondata 1 e le skill la applicano.
- No-go:
  - nessun nuovo controllo automatico nella CLI (è l'ondata 3);
  - nessuna riscrittura dei template già copiati nei domini esistenti;
  - nessun catalogo vivo del comportamento (parcheggiato).
- Appetite: 1 giorno.
- Metriche di successo: con un `init` nuovo, in inglese e in italiano, le risorse installate contengono la revisione; le quattro skill contengono le istruzioni nuove; suite, doctor, controllo traduzioni e CI verdi.
- Componenti modificati:
  - copie del kit in inglese e italiano: `fondamenta.md`, `processo-e2e.md`, `templates/{dod-estesa, cambiamento_rapido, review_design, review_design_light, configurazione_rollout}.md`, allineate alla radice (riga di navigazione del kit invariata);
  - `templates/specifica_feature.md` del kit (distinto da quello della radice): parole del richiedente, assunzioni, appetite, no-go, comportamento esteso, stati non previsti, criteri aggiunti in sviluppo, verifiche osservabili;
  - skill `dfd-specify`, `dfd-implement`, `dfd-review-design`, `dfd-verify` in inglese e italiano;
  - `tests/workflow.rs`, `i18n/documents.json`, `CHANGELOG.md`.
- Fuori scope: classificazione delle cause nel post-release (nessuna skill post-release esiste oggi); convenzione obbligatoria sui nomi dei test.
- Nel brownfield: i domini già inizializzati mantengono i loro template; chi vuole i campi nuovi li adotta esplicitamente, come per ogni revisione dei template.

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso light/full: business medium, il resto low → light.
- Motivazioni e dati ancora mancanti: nessuno.

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione:
  1. `init` in `en` e in `it` installa in `.dfd/kit/` le copie con la revisione: §3.4 e §6 della DoD estesa, stati non previsti in fondamenta e processo, qualità della specifica nelle review, punti di uso nel rollout, parole del richiedente nel record rapido.
  2. Il template di specifica del dominio creato da `init` ha i campi nuovi; `dfd specify` li copia in `spec.md` come per gli altri campi.
  3. `dfd-specify` chiede di riportare alla lettera le parole del richiedente, separare le assunzioni, fare al massimo tre domande con opzioni (default per primo), definire la reazione agli stati non previsti e applicare i vincoli per tipo di cambiamento; le verifiche descrivono un risultato osservabile.
  4. `dfd-implement` non aggiunge codice che nessun criterio o scenario giustifica: propone il criterio nella specifica, che richiede la riconferma del design.
  5. `dfd-review-design` controlla la qualità della specifica e confronta ogni criterio aggiunto con le parole del richiedente: in conflitto non entra.
  6. `dfd-verify` segnala il codice senza un criterio o scenario che lo giustifichi.
- Errori, failure mode e comportamenti degradati:
  - uno stato non previsto, per le skill, va reso esplicito e osservabile, mai ignorato;
  - un campo nuovo lasciato come segnaposto blocca la review strutturale come gli altri (regola esistente).
- API, eventi, dati e dipendenze coinvolti: nessuno; solo risorse incorporate.
- Compatibilità e migrazioni: nessuna migrazione. I domini esistenti non cambiano; `dfd language` e `install` aggiornano le risorse gestite (`.dfd/kit/`, skill) come oggi, senza toccare i template del dominio.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | esclusa | Nessuna diagnostica, exit code o stato cambiato. | — |
| OBS-02 | esclusa | Meccanismo delle evidenze invariato. | — |
| SEC-01 | esclusa | Nessun percorso o input nuovo. | — |
| SEC-02 | applicabile | Le risorse nuove non sovrascrivono i template dei domini esistenti. | Test di preservazione esistenti; test nuovo sulle risorse installate. |
| SEC-03 | applicabile | Le skill confrontano i criteri aggiunti con le parole del richiedente; nessuna approvazione implicita. | Test sul contenuto delle skill; review. |
| SLO-01 | applicabile | Comandi e contratti invariati; template nuovi solo per i domini nuovi. | Suite. |
| SLO-02 | applicabile | Suite, doctor, controllo traduzioni, CI. | Suite locale e CI. |
| SLO-03 | esclusa | Scritture e lock invariati. | — |
| COST-01 | esclusa | Binario offline invariato. | — |
| COST-02 | applicabile | Verifiche proporzionate: test sul contenuto installato, nessun mutante nuovo (nessun gate cambia). | Log della suite. |
| UX-01 | applicabile | Copie, template e skill coerenti con il framework in entrambe le lingue. | Test sulle risorse installate; controllo traduzioni. |
| UX-02 | applicabile | Le quattro skill hanno le stesse istruzioni in tutti gli adapter e in entrambe le lingue. | Test sulle skill installate; doctor. |

## 5. Guardrail e decisioni

- Guardrail: testo delle copie identico alla radice salvo la riga di navigazione; nessuna riscrittura dei template dei domini; nessun gate nuovo.
- Eccezioni motivate, responsabili e decisioni: nessuna.
- Collegamenti: PR #17 (ondata 1), feature 012 (struttura con l'inglese primario); questa feature parte dal branch della 012.

## 6. Test e validazione prevista

- Test funzionali derivati dai criteri:
  - red/green `kit_resources_adopt_unforeseen_states_revision`: risorse installate in `en` e `it` con la revisione, template di specifica del dominio con i campi nuovi;
  - red/green `skills_guide_unforeseen_states_and_added_criteria`: le quattro skill installate, in `en` e `it`, contengono le istruzioni nuove.
- Sono test di regressione sul contenuto: proteggono da una sincronizzazione futura che perda la revisione, non verificano il comportamento dell'agente.
- Test non funzionali: nessuno; nessun gate o percorso di codice cambia.
- Verifiche manuali: lettura delle skill nelle due lingue.
- Validazione: un pilota reale nell'harness resta la lacuna baseline UX-03.
- Validazione in produzione: release GitHub `v0.3.0`, che contiene anche la feature 012 (inglese primario, cambiamento incompatibile). Il dominio è in `release-preparation`, quindi servono CI reale sulla PR, pre-release e review di release; la versione in `Cargo.toml` e la sezione `0.3.0` di `CHANGELOG.md` e `it/CHANGELOG.md` fanno parte di questa feature.

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Condizioni e responsabili: nessuna.
- Riferimento alla review e alla decisione umana: `design-review.md`, decisione del manutentore.

## 8. Revisioni

- 2026-11-18: prima bozza
- 2026-11-18: su richiesta del manutentore ("Rilascia il lavoro fatto fino ad ora") la feature porta la release `v0.3.0` (versione e changelog).
