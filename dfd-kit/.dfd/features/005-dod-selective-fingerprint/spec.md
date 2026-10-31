# Specifica di Feature – Testo di dod.md nel fingerprint selettivo

> **Stato**: in review di design  
> **Data creazione**: 2026-10-31  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: issue #5 (MartoEporedia/dfd-framework)

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: con `selective_review` (default delle nuove feature) il design fingerprint include gli statement dei criteri selezionati di `criteria.json`, non `dod.md`. Dopo `decide --stage design` si può cambiare il testo di un criterio solo in `dod.md` (la DoD letta dalle persone) e tutti i gate restano verdi. Nel laboratorio è il fault F07b; è stata l'unica bugia sfuggita anche all'agente revisore.
- Metriche di successo: nel laboratorio v3 F07b passa da NON VISTO a design `stale-review` (sviluppo `blocked-plan`, `status` exit 2); F00 e la baseline ricostruita restano verdi; modifiche a `dod.md` che non citano i criteri selezionati non invalidano la review.
- Componenti modificati: `src/lib.rs` (`design_fingerprint` ed estrazione dei blocchi di `dod.md`), `tests/workflow.rs`, `mutations/gates.json`, contratto IT/EN, skill `dfd-review-design` IT/EN.
- Fuori scope: confronto testuale tra statement del catalogo e `dod.md` in `setup`; generazione di un file dall'altro; fingerprint legacy (`selective_review` assente o false), che include già l'intero `dod.md`.
- Nel brownfield: le review selettive già approvate (in questo repository 003, 004, oltre alle feature dei progetti che hanno adottato DFD) diventano `stale-review` una volta con il nuovo binario e vanno riconfermate. Scelta accettata dal manutentore il 2026-10-31 ("se rompe qualcosa di preesistente non è un problema").

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso light/full: security e reliability medium → full proporzionato.
- Motivazioni e dati ancora mancanti: nessuno.

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione:
  1. Per una feature con `selective_review: true`, il design fingerprint include, per ogni criterio presente in `design.json`, i blocchi di `dod.md` che citano quell'ID (`selected-dod`).
  2. Un blocco è un'unità Markdown: una voce di elenco (`-`, `*`, `+`, `1.`) con le righe che la continuano o sono più indentate (sotto-voci comprese), fino a una riga vuota, un titolo o una voce allo stesso livello o meno indentata; una riga di tabella (`|…`); un titolo; un paragrafo di righe consecutive fino a una riga vuota. Un blocco cita un criterio se contiene l'ID secondo la stessa regola di `criterion_ids`. Un titolo che cita un criterio forma anche un blocco di sezione: il titolo con tutto il contenuto fino al titolo successivo di livello pari o superiore (DoD scritte come `### OBS-01` seguito dal testo). I titoli si riconoscono come in CommonMark (fino a tre spazi iniziali, da 1 a 6 `#` seguiti da spazio o fine riga); i blocchi di codice recintati (```` ``` ```` o `~~~`) restano nel blocco che li contiene e non chiudono una sezione. Fail-closed: se nessun blocco che cita un criterio selezionato contiene almeno quattro parole di almeno tre lettere, tolti ID, destinazioni dei link e caselle `[ ]`/`[x]`, si usa l'intero `dod.md`.
  3. Il testo dei blocchi viene normalizzato comprimendo gli spazi: un a capo o un'indentazione diversa non invalidano la review; ogni cambiamento di parole, punteggiatura o markup sì.
  4. Modificare in `dod.md` il testo di un criterio selezionato rende il design `stale-review`; modificare un criterio non selezionato, un titolo o un paragrafo che non cita criteri selezionati lascia il design `approved`. Un blocco che cita più criteri (per esempio un indice) li lega tutti: in modo conservativo, modificarlo invalida le feature che ne selezionano almeno uno.
  5. Il contratto non dice più che `dod.md` è solo descrittivo per le review selettive: il testo dei criteri selezionati in `dod.md` diventa una dipendenza del design, come lo statement nel catalogo.
- Errori, failure mode e comportamenti degradati: `dod.md` assente o illeggibile resta un errore di setup come oggi; un ID selezionato non citato in `dod.md` produce una lista vuota (già bloccato da `setup`).
- API, eventi, dati e dipendenze coinvolti: formula del fingerprint selettivo. Nessuno schema JSON cambia.
- Compatibilità e migrazioni, se necessarie: nessuna migrazione dei file; le decisioni selettive esistenti diventano `stale-review` e si riconfermano con `decide`. Le review legacy non cambiano.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| SEC-03 | applicabile | Il testo DoD dei criteri selezionati invalida la review quando cambia | Test F07b riprodotto; mutante sull'inclusione nel fingerprint |
| OBS-01 | applicabile | Lo stato riporta `stale-review` e non dichiara approvata una DoD cambiata | Test workflow |
| SLO-01 | applicabile | Invalidazione una tantum delle review selettive documentata nel contratto IT/EN | Revisione documenti, `check_translations.py` |
| SLO-02 | applicabile | Red/green, suite, fmt, Clippy, doctor, mutanti mirati | Log in `evidence/` |
| UX-01 | applicabile | Contratto e skill di review di design descrivono la nuova dipendenza | Revisione documenti IT/EN |
| OBS-02 | esclusa | Nessun cambiamento alle evidenze | — |
| SEC-01 | esclusa | Si legge solo `dod.md` del dominio, già letto oggi | — |
| SEC-02 | esclusa | Nessuna scrittura nuova | — |
| SLO-03 | esclusa | Scritture e lock invariati | — |
| COST-01 | esclusa | Nessuna dipendenza | — |
| COST-02 | esclusa | Nessuna soglia di prestazioni | — |
| UX-02 | esclusa | Nessun cambiamento ai comandi chiamati dalle skill | — |

## 5. Guardrail e decisioni

- Guardrail: il fingerprint legacy resta identico; nessuna approvazione derivata; le modifiche irrilevanti di `dod.md` (altri criteri, spazi) non invalidano.
- Alternative scartate: confronto testuale statement/`dod.md` in `setup` (nel laboratorio lo statement è una sintesi del paragrafo: bloccherebbe setup legittimi); generare un file dall'altro (cambia il modo in cui si scrive la DoD); includere l'intero `dod.md` (annulla la review selettiva).
- Collegamenti a policy e ADR esistenti: [contratto](../../../../templates/contratto_toolkit.md), sezione "Fingerprint selettivi e compatibilità".

## 6. Test e validazione prevista

- Test funzionali derivati dai criteri: F07b riprodotto (testo di un criterio selezionato cambiato solo in `dod.md` → `stale-review`); modifica di un criterio non selezionato e riformattazione degli spazi → resta `approved`; sotto-voce indentata di un criterio selezionato → `stale-review`.
- Test non funzionali proporzionati al rischio: nessuno.
- Verifiche manuali e loro evidenze attese: laboratorio v3 livello 1 con il binario nuovo; baseline ricostruita verde.
- Piano di validazione in produzione, soglie e rollback: nessun rilascio (solo sviluppo).

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Condizioni e responsabili: —
- Riferimento alla review e alla decisione umana: da registrare dal manutentore con `dfd decide`.

## 8. Revisioni

- 2026-10-31: prima bozza.
- 2026-10-31: blocchi di sezione per i titoli che citano un criterio e ripiego sull'intero `dod.md` per criteri citati solo come ID (rilievo della review di sicurezza automatica sulla PR #9).
- 2026-10-31: titoli CommonMark, codice recintato e soglia fail-closed per i riferimenti senza testo (secondo rilievo della review di sicurezza automatica, commit a8539d1).
