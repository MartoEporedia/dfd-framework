# Framework e toolkit bilingui

- Stato: design proposto, decisione di indirizzo già espressa in chat
- Data: 2026-10-21
- Owner: manutentore del repository

## Obiettivo e decisione umana

L’utente richiede italiano e inglese per tutto. Ha approvato la proposta: pagine separate con selettore, struttura inglese speculare, italiano completo preservato, toolkit localizzato e identificatori comuni. Nessuna approvazione di release è stata espressa.

## Ambito e scenari

1. Conservare percorsi italiani; aggiungere `en/` per framework, template ed esempi, con navigazione reciproca. README principale breve bilingue e percorsi completi per ruolo separati.
2. Aggiungere `dfd-kit/en/` per guide, risorse, template e dodici skill. Glossario condiviso e controllo automatico della parità, link e identificatori normativi.
3. Selezionare it/en esplicitamente, mantenere IT per i progetti legacy. Preferenza di progetto separata dal contratto JSON esistente; override CLI per singola invocazione. Cambio persistente aggiorna risorse gestite soltanto dopo preflight e preserva documenti compilati.
4. Localizzare help, diagnostica e nuovi artefatti generati; non tradurre chiavi JSON, enum, ID, comandi, nomi file, testo utente o evidenze. Le istruzioni EN restano normative come IT.
5. Nessuna rete o runtime aggiuntivo per usare il binario. Stato, rischio, gate, fingerprint ed exit code restano identici.

## Criteri e verifiche

- OBS-01, OBS-02: diagnostica localizzata, contratti macchina stabili ed evidenze reali.
- SEC-01, SEC-02, SEC-03: confini e lock invariati, preflight conflitti e preservazione; nessuna approvazione simulata.
- SLO-01, SLO-02: compatibilità legacy, test EN/IT, cambio lingua e override, suite, fmt, Clippy, doctor e mutanti mirati.
- COST-01: risorse statiche e test con PATH vuoto.
- UX-01, UX-02: navigazione, glossario, parità delle risorse, riferimenti e skill coerenti.
- SLO-03 e COST-02 esclusi: persistenza per file invariata; nessuna nuova soglia di performance.

## Rischi e limiti

Percorso full proporzionato: architettura e affidabilità medium per la portata trasversale. Traduzioni normative devono preservare ruoli, soglie, condizioni e decisioni umane. Cambio lingua non traduce retroattivamente documenti già compilati. Il pilota reale dei quattro harness resta distinto da doctor. Solo sviluppo, nessun rilascio.
