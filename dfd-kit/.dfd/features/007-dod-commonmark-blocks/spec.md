# Specifica di Feature – Blocchi di dod.md da un parser CommonMark

> **Stato**: in review di design  
> **Data creazione**: 2026-10-31  
> **Proprietari**: manutentore del repository  
> **Servizio / Dominio**: dfd-kit  
> **Epic / Ticket**: seguito di issue #5 (PR #9); rilievi della review di sicurezza automatica sul commit a5a71bb

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: il fingerprint selettivo (feature 005) estrae da `dod.md` i blocchi che citano i criteri selezionati con un parser scritto a mano. La review di sicurezza automatica ha trovato tre volte divergenze rispetto a CommonMark che lasciano fuori dal fingerprint il testo di un criterio:
  - recinti di codice chiusi da un recinto più corto o seguito da testo;
  - titoli setext (`OBS-01` sottolineato da `===`), titoli dentro un `>` o in HTML;
  - una riga indice descrittiva ("Criteri del flusso: OBS-01, SLO-01") che evita il ripiego anche se il testo vero del criterio sta in un paragrafo che non cita l'ID.
- Scelta del manutentore (2026-10-31): usare un parser CommonMark (`pulldown-cmark`).
- Metriche di successo: i casi segnalati producono `stale-review`; F00 e F07b del laboratorio invariati (verde e bloccato); baseline ricostruita verde; nessuna dipendenza a runtime per l'utilizzatore.
- Componenti modificati: `Cargo.toml`/`Cargo.lock` (dipendenza), `src/lib.rs` (estrazione dei blocchi al posto di `dod_blocks`, `atx_level`, `code_lines`), test, mutanti, contratto IT/EN.
- Fuori scope: fingerprint legacy; #6.

## 2. Classificazione del rischio

- Riferimento all'assessment: [risk.md](risk.md).
- Rischio e percorso light/full: architecture, reliability e security medium → full proporzionato.

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione:
  1. `dod.md` si analizza con `pulldown-cmark` 0.13 (CommonMark più le tabelle). Le unità sono ricavate dagli eventi del parser, con il testo sorgente corrispondente:
     - una voce di elenco con tutto il suo contenuto (sotto-voci e codice compresi);
     - un paragrafo fuori da una voce di elenco;
     - una riga di tabella;
     - un titolo, ATX o setext, anche dentro un `>`;
     - la sezione di un titolo che cita un criterio, fino al titolo successivo di livello pari o superiore secondo il parser.
  2. Una unità cita un criterio se il suo testo sorgente contiene l'ID. Per ogni criterio selezionato, il fingerprint include le unità che lo citano, con gli spazi normalizzati.
  3. **Regola di definizione**: ogni criterio selezionato deve avere un'unità di definizione propria, cioè una di queste:
     - voce di elenco o paragrafo il cui testo inizia con l'ID;
     - riga di tabella la cui prima cella contiene l'ID;
     - sezione sotto un titolo che lo cita.

     L'unità di definizione deve avere almeno quattro parole di almeno tre lettere, esclusi ID, destinazioni dei link e caselle. Altrimenti diventa dipendenza l'intero `dod.md`. Una riga indice non basta più.
  4. **Fail-closed sui casi ambigui**: se `dod.md` contiene HTML (blocchi o inline), diventa dipendenza l'intero `dod.md` per tutti i criteri selezionati.
  5. La formula cambia rispetto a 005: le review selettive approvate diventano `stale-review` una volta.
- Errori, failure mode e comportamenti degradati: un `dod.md` che il parser legge comunque (CommonMark non ha errori di sintassi) non blocca; i casi non classificabili ricadono sull'intero file.
- API, eventi, dati e dipendenze coinvolti: dipendenza `pulldown-cmark` senza feature di default; nessuno schema JSON cambia.
- Compatibilità e migrazioni, se necessarie: invalidazione una tantum delle review selettive, come per 005.

## 4. Allineamento alla DoD del dominio

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| SEC-03 | applicabile | Le varianti CommonMark del testo di un criterio selezionato invalidano la review | Test per recinti, setext, blockquote, HTML, indice; mutanti |
| SLO-01 | applicabile | Invalidazione una tantum e nuova dipendenza documentate | Revisione contratto IT/EN |
| SLO-02 | applicabile | Red/green, suite, fmt, Clippy, doctor, mutanti; test 005 invariati | Log in `evidence/` |
| COST-01 | applicabile | La dipendenza è compilata nel binario: nessun runtime o rete per l'utilizzatore; test con PATH vuoto | Suite esistente |
| UX-01 | applicabile | Contratto IT/EN aggiornato | Revisione documenti |
| OBS-01 | esclusa | Nessuna diagnostica nuova: resta il gate `stale-review` | — |
| OBS-02 | esclusa | Evidenze invariate | — |
| SEC-01 | esclusa | Si legge solo `dod.md`, già letto | — |
| SEC-02 | esclusa | Nessuna scrittura nuova | — |
| SLO-03 | esclusa | Scritture e lock invariati | — |
| COST-02 | esclusa | Nessuna soglia di prestazioni | — |
| UX-02 | esclusa | Nessun comando delle skill cambia | — |

## 5. Guardrail e decisioni

- Guardrail: fingerprint legacy invariato; nessuna approvazione derivata; nel dubbio si usa l'intero `dod.md`.
- Alternative scartate dal manutentore: whitelist sul parser a mano; intero `dod.md` sempre.

## 6. Test e validazione prevista

- Test funzionali: i test di 005 restano verdi. Nuovi casi che oggi restano `approved`:
  - recinto di 4 backtick con dentro una riga di 3 backtick e un `#`;
  - titolo setext `OBS-01` sottolineato da `===` con il testo sotto;
  - titolo `> ### OBS-01` dentro un blockquote;
  - `<h3>OBS-01</h3>` in HTML;
  - riga indice descrittiva con il testo del criterio in un paragrafo che non cita l'ID.
- Verifiche manuali: laboratorio v3 (F00, F07, F07b, baseline da zero).
- Validazione in produzione: nessun rilascio.

## 7. Domande aperte e review

- Punti bloccanti da chiarire: nessuno.
- Riferimento alla review e alla decisione umana: da registrare con `dfd decide`.

## 8. Revisioni

- 2026-10-31: prima bozza.
