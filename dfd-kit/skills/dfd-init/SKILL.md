---
name: dfd-init
description: Introdurre DFD in un repository nuovo o esistente, scegliere domini pilota e costruire assessment di adozione, DoD e guardrail. Usare per onboarding DFD, non per classificare il rischio di una singola feature.
---

# Adozione di DFD

Le risorse del toolkit sono in `.dfd/kit/` nel repository adottante. Prima dell'installazione usare i corrispondenti file alla radice del toolkit. Leggere `.dfd/kit/adozione.md` solo se il prodotto esiste già, e `.dfd/kit/fondamenta.md` per definire la DoD.

## Percorso

1. Rispettare le istruzioni del repository. Identificare harness, dominio e ambito; usare `brownfield` quando esistono già codice o processi. In un repository esistente riusare policy, test, CI, monitoring e review, indicando le fonti realmente lette.
2. Eseguire `dfd init --harness <adapter> --domain <dominio> --mode brownfield`. Per un altro repository usare `dfd --project <repo> init ...`. Se il binario non è nel PATH, usare il percorso fornito dall’utente. Gli adapter sono `claude-code`, `opencode`, `copilot`, `codex`.
3. Compilare `.dfd/domains/<dominio>/adoption.json`: processo, osservabilità, sicurezza, SLO, costo, cultura. `status` è `observed`, `gap` o `unknown`; `evidence` contiene riferimenti alle fonti, non supposizioni. Motivare livello attuale e target nel documento `assessment.md`.
4. Nel brownfield proporre 1–2 domini pilota e 3–5 interventi, senza migrare tutto il legacy. Registrare lacune in `baseline_gaps` e passi incrementali in `next_steps`. Non dichiarare il livello target raggiunto solo perché sono stati creati i file.
5. Adattare `dod.md` e `criteria.json` alla realtà del dominio. Ogni criterio ha `id`, `statement`, `scope`: `changes` per requisiti del nuovo lavoro, `baseline` per obiettivi di miglioramento dell'esistente. Usare identificatori `OBS-01`, `SEC-01`, `SLO-01`, `COST-01`, `UX-01`; conservare quelli già presenti. Non inventare SLO o soglie: marcare i punti da chiarire.
6. Completare la Fase 0 con `dfd-setup`: collegare `.dfd/domains/<dominio>/guardrails.md` alle policy esistenti e agli eventuali guardrail comuni `.dfd/guardrails.md`; documentare convenzioni in `process.md` e adattamenti nei template locali. Separare fatti osservati, proposte e informazioni mancanti. Chiedere input mirati solo sui punti che impediscono di definire il pilota.

Riferimento per formati e comandi: `.dfd/kit/templates/contratto_toolkit.md`. L'inizializzazione è ripetibile e preserva gli artefatti esistenti. Le review DFD si aggiungono alle review del team. Al termine indicare dominio pilota, lacune note e stato della Fase 0 verificato con `dfd setup --domain <dominio>` e prima feature da trattare con `dfd-assess`.
