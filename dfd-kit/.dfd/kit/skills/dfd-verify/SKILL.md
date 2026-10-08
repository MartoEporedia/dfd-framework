---
name: dfd-verify
description: Verificare le evidenze dello sviluppo DFD, la copertura dei task e la validità dei log e dei file prima di passare alla pre-release.
---

# Verifica dello sviluppo

Leggere `.dfd/kit/templates/contratto_toolkit.md`, specifica, piano ed `evidence.json`. Eseguire `dfd verify <id>`: produce `development-review.md` e aggiorna la fase solo se gli artefatti verificati sono completi e attuali.

Valutare anche il contenuto: i test red devono fallire per il requisito atteso, i green devono verificare i criteri associati; il log CI deve riferirsi alla versione finale con refactor e strumentazione. Gli hash verificano l'integrità locale dei riferimenti, non autenticità dei log, copertura semantica o qualità del codice.

Se mancano test o strumentazione proseguire con `dfd-implement`; se il piano è incompleto usare `dfd-plan`; se cambia il design, riconfermare la review pertinente. Conservare i risultati già validi, rieseguendo quelli invalidati. Non dichiarare verde una CI non eseguita.

Riportare `blocked-plan`, `blocked-evidence` o `ready-for-pre-release` con lacune e condizioni. Il prossimo passo dopo la verifica positiva è `dfd-pre-release`, seguita da `dfd-review-release`; l’esecuzione del rollout resta una fase successiva.
