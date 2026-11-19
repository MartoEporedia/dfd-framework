# Specifica di Feature – Segnaposto tra virgolette nella review di design (v0.3.1)

> **Stato**: draft  
> **Data creazione**: 2026-11-19  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: richiesta del manutentore in chat il 2026-11-19 ("Intanto correggi il bug del TODO nelle citazioni", poi "Unisci la #22 e rilascia la 0.3.1"); PR #22; origine nel record rapido `fix-todo-in-quotes`

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: dalla 0.3.0 la specifica chiede le parole del richiedente alla lettera, ma il controllo dei segnaposto di `review-design` bloccava le citazioni che contengono la parola cercata. Nel pilota su martoranamarco-site (2026-11-19) la riga del README con la parola segnaposto ha bloccato la review e l'agente ha dovuto riformularla.
- Parole del richiedente: "Intanto correggi il bug del TODO nelle citazioni" e "Unisci la #22 e rilascia la 0.3.1" (chat, 2026-11-19).
- Assunzioni: i segnaposto dei template sono sempre testo libero, mai tra virgolette o in codice.
- Obiettivo: le citazioni letterali non bloccano la review; i segnaposto nel testo libero sì; la correzione esce come release `v0.3.1`.
- No-go: nessun cambiamento ai template, alle skill, agli schemi JSON o agli altri controlli dei segnaposto (piano, rollout, setup).
- Appetite: mezza giornata, release compresa.
- Metriche di successo: test di regressione red/green; mutanti mirati intercettati; release GitHub `v0.3.1` pubblicata con CI verde, archivi e checksum verificati.
- Componenti modificati: `validate_design` e la nuova funzione `without_quotations` in `src/lib.rs`; `tests/workflow.rs`; `mutations/gates.json`; `CHANGELOG.md` e `it/CHANGELOG.md`; versione in `Cargo.toml` e `Cargo.lock`.
- Fuori scope: gli altri attriti emersi dal pilota (setup senza conferma umana, lifecycle di default silenzioso, data dell'epic), rimandati all'ondata 3.
- Nel brownfield: nessun processo nuovo; la feature nasce dal record rapido `fix-todo-in-quotes` (promosso), che contiene red, green, suite e CI.

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso light/full: tutte le dimensioni low, percorso light per la release.
- Motivazioni e dati ancora mancanti: nessuno.

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione:
  1. Una specifica con la parola segnaposto dentro virgolette doppie (`"…"` o `“…”`), in codice inline o in un blocco di codice recintato arriva a `awaiting-human-review`.
  2. La stessa specifica con un segnaposto nel testo libero resta `blocked` con l'errore sui segnaposto.
  3. La specifica del template non compilata resta `blocked` come prima.
- Errori, failure mode e comportamenti degradati: una virgoletta non chiusa sulla riga non nasconde nulla (il filtro agisce solo su coppie sulla stessa riga); un blocco di codice non chiuso nasconde il resto del file, comportamento accettato perché la review umana legge comunque la specifica.
- API, eventi, dati e dipendenze coinvolti: nessuno; `regex` è già una dipendenza.
- Compatibilità e migrazioni: nessuna; le specifiche già approvate non cambiano gate perché il fingerprint del design non dipende dal controllo.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| OBS-01 | esclusa | Messaggi ed exit code invariati. | — |
| OBS-02 | esclusa | Meccanismo delle evidenze invariato. | — |
| SEC-01 | esclusa | Nessun percorso o input nuovo. | — |
| SEC-02 | esclusa | Nessuna scrittura cambiata. | — |
| SEC-03 | applicabile | Il controllo resta attivo sul testo libero; la review umana non cambia. | Test di regressione e mutanti. |
| SLO-01 | applicabile | Comandi e contratti invariati. | Suite completa. |
| SLO-02 | applicabile | Suite, doctor, controllo traduzioni, CI. | Suite locale e CI. |
| SLO-03 | esclusa | Scritture e lock invariati. | — |
| COST-01 | esclusa | Binario offline invariato. | — |
| COST-02 | applicabile | Due mutanti mirati, nessuna verifica sproporzionata. | Report dei mutanti. |
| UX-01 | applicabile | Le parole del richiedente si citano alla lettera come chiede `dfd-specify`. | Test di regressione; CHANGELOG EN/IT. |
| UX-02 | esclusa | Skill invariate. | — |

## 5. Guardrail e decisioni

- Guardrail: nessun allentamento fuori da citazioni e codice; nessun gate nuovo.
- Eccezioni motivate, responsabili e decisioni: nessuna.
- Collegamenti: feature 013 (parole del richiedente nel template), record rapido `fix-todo-in-quotes`.

## 6. Test e validazione prevista

- Test funzionali: red/green `quoted_placeholders_do_not_block_review`; `placeholder_design_blocks_approval` resta verde.
- Test non funzionali: mutanti `spec-placeholder-quotes-ignored` e `spec-placeholder-free-text-blocks`.
- Verifiche manuali: nessuna.
- Validazione in produzione: release GitHub `v0.3.1`; il dominio è in `release-preparation`, quindi servono CI reale sulla PR, pre-release e review di release; la versione in `Cargo.toml` e la sezione `0.3.1` dei due CHANGELOG fanno parte di questa feature. Rollback: eliminare release e tag, la v0.3.0 resta disponibile.

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Condizioni e responsabili: nessuna.
- Riferimento alla review e alla decisione umana: `design-review.md`, decisione del manutentore.

## 8. Revisioni

- 2026-11-19: prima bozza, dal record rapido promosso.
