---
name: dfd-review-release
description: Revisionare la preparazione di una release DFD con checklist light o full e registrare la decisione umana su rollout, evidenze, soglie e condizioni.
---

# Review di release

Leggere `.dfd/kit/templates/contratto_toolkit.md`, `.dfd/kit/raci.md` e gli artefatti della feature. Usare la checklist locale `.dfd/domains/<dominio>/templates/review_release_light.md` per light oppure `review_release.md` per full.

Eseguire `dfd review-release <id>`. Il report `release-review.md` è rigenerabile e contiene controlli strutturali, gate e attivazioni pianificate. Verificare semanticamente coerenza tra revisione da rilasciare, CI, test non funzionali, DoD, soglie, policy, rollback, on-call e comunicazioni. Controllare il contenuto delle evidenze: gli hash non ne autenticano l'origine. Salvare note e rilievi in `release-notes.md`, separatamente dal report automatico.

Presentare al responsabile umano esito proposto e condizioni. Registrare una decisione solo quando espressa dal responsabile, senza chiedere di ripeterla se già presente nella conversazione:

```sh
dfd decide <id> --stage release --decision approved --reviewer "..." --role tech-lead --note "..." --human-confirmed
```

Per condizioni usare `approved-with-conditions` e ripetere `--condition "testo|responsabile|fase"`. Le risorse pianificate richiedono un'approvazione condizionata con attivazione assegnata; quelle dichiarate pronte richiedono evidenze. Risolvere le condizioni di design dovute entro la release e far registrare la decisione aggiornata prima dell'approvazione di release. Le decisioni di release hanno uno storico separato da quelle di design.

Rieseguire `status`. Le modifiche a rollout, note, design, decisione di design o evidenze rendono la review da riconfermare; le lacune strutturali bloccano il gate. Le condizioni non sono chiuse automaticamente: ottenere una nuova decisione umana dopo la loro risoluzione. Riportare esito, condizioni e prossima azione. La CLI registra l'esito della review e non esegue il rollout.
