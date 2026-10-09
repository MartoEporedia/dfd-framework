---
name: dfd-verify
description: Verificare le evidenze dello sviluppo DFD, la copertura dei task e la validità dei log e dei file prima di passare alla pre-release.
---

# Verifica dello sviluppo

Leggere `.dfd/kit/templates/contratto_toolkit.md`, specifica, piano ed `evidence.json`. Eseguire `dfd verify <id>`: produce `development-review.md` e aggiorna la fase solo se gli artefatti verificati sono completi e attuali.

Valutare anche il contenuto: i test red devono fallire per il requisito atteso, i green devono verificare i criteri associati; il log della suite finale locale/CI secondo lifecycle.json deve riferirsi alla versione finale con refactor e strumentazione. Gli hash verificano l'integrità locale dei riferimenti, non autenticità dei log, copertura semantica o qualità del codice.

Se mancano test o strumentazione proseguire con `dfd-implement`; se il piano è incompleto usare `dfd-plan`; se cambia il design, riconfermare la review pertinente. Conservare i risultati già validi, rieseguendo quelli invalidati. Non dichiarare verde una CI non eseguita.

Riportare `blocked-plan`, `blocked-evidence`, `development-complete` o `ready-for-pre-release` con lacune e condizioni. Con development-complete fermarsi allo sviluppo verificato; con ready-for-pre-release il prossimo passo è `dfd-pre-release`, seguita da `dfd-review-release`; l’esecuzione del rollout resta una fase successiva.

verify supporta anche i record rapidi: con policy individuale le evidenze pertinenti valide danno change-verified senza nuovo gate umano. Con review indipendente serve la decisione PR reale e attuale. Controllare che il red riproduca il bug, non problemi di ambiente; non imporre red artificiale alle modifiche editoriali. Il completamento rapido non approva distribuzioni.
