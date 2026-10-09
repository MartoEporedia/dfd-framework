---
name: dfd-review-design
description: Revisionare una specifica DFD con la checklist light o full, controllare i prerequisiti e preparare la decisione del responsabile umano, anche sviluppatore singolo o reviewer delegato.
---

# Review di design

Leggere gli artefatti della feature e il setup del dominio: DoD, guardrail, convenzioni e template locali. In base a `state.json.route` usare `.dfd/domains/<dominio>/templates/review_design_light.md` oppure `review_design.md`. Per le responsabilità leggere `.dfd/kit/raci.md`; per i formati `.dfd/kit/templates/contratto_toolkit.md`.

Per un record rapido individuale completo usare `verify` senza seconda approvazione. Se la policy richiede review indipendente, registrarla sullo snapshot corrente del record.

1. Eseguire `dfd review-design <id>`: il report `design-review.md` contiene solo controlli strutturali e il gate corrente. Un esito automatico positivo richiede comunque la valutazione semantica e la decisione del responsabile.
2. Verificare che obiettivi, metriche, ambito, rischi, criteri DoD e test siano coerenti. Distinguere problemi bloccanti, condizioni e suggerimenti. Nel brownfield valutare il cambiamento in scope e le lacune legacy pertinenti, mantenendo l'overlay sulle review del team.
3. Aggiungere i rilievi semantici in `review-notes.md` nella cartella della feature. Non inserire il proprio report tra le decisioni umane e non dichiarare superato il gate sulla sola base di una checklist compilata. Il report automatico può essere rigenerato: conservarne separatamente i commenti.
4. Presentare al responsabile umano una proposta di esito. Registrare `approved`, `approved-with-conditions` o `changes-requested` solo dopo che il responsabile umano ha espresso quella decisione. Se la decisione è già stata espressa nella conversazione, procedere senza richiederla di nuovo.
5. Usare `dfd decide <id> --decision approved --reviewer "..." --role tech-lead --note "..." --human-confirmed`. Per condizioni usare `--condition "testo|responsabile|fase"` e `approved-with-conditions`. Reviewer e ruolo sono dichiarazioni registrate, non identità autenticate. Non usare questo comando per autoapprovare il proprio lavoro.
6. Rieseguire `status` dopo la decisione. Modifiche alla specifica, al rischio, ai criteri selezionati o ai guardrail invalidano la review precedente. Le nuove feature usano review selettive; quelle legacy mantengono il fingerprint conservativo. Conservare lo storico e riconfermare solo la decisione pertinente.

Con design approvato e attuale proseguire con `dfd-plan` e sviluppo TDD. L'approvazione del design non autorizza automaticamente un deploy. Riportare esito, condizioni e prossima azione.

Per un record rapido non aprire un nuovo design se ripristina il contratto indicato. Verificare problema, ambito e test pertinenti nel solo record. Senza independent_review il singolo dev non deve esprimere una seconda approvazione; con policy team registrare la review PR effettiva una sola volta usando decide. Non inventare decisioni. Per light/full un reviewer delegato può registrare la review pertinente già espressa nella PR.
