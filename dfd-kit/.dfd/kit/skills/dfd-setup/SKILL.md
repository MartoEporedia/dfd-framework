---
name: dfd-setup
description: Definire o aggiornare la Fase 0 DFD di un dominio: DoD, guardrail, template locali e convenzioni di processo, prima del design delle feature.
---

# Setup DFD di dominio

Leggere `.dfd/kit/fondamenta.md`, `.dfd/kit/processo-e2e.md` e `.dfd/kit/templates/contratto_toolkit.md`. La Fase 0 costituisce il riferimento condiviso del dominio, paragonabile alla constitution di SDD, ed evolve con i learning di produzione.

1. Se il dominio non esiste, usare `dfd-init`. Se mancano gli artefatti della Fase 0 in un dominio esistente, rieseguire `dfd init` con harness e modalità del progetto: aggiunge i file mancanti preservando il lavoro locale.
2. Leggere policy e decisioni esistenti e gli artefatti in `.dfd/domains/<dominio>/`. Definire iterativamente `dod.md` e `criteria.json`, preservando gli identificatori. Separare requisiti del nuovo lavoro e lacune baseline.
3. Completare `guardrails.md` con principi architetturali, contratti, sicurezza, SLO, osservabilità e costo. Collegare anche i guardrail comuni del progetto quando pertinenti.
4. Adattare i template in `templates/` oppure mantenerli invariati. Documentare la scelta e le differenze in `process.md`, insieme a naming, classificazione del rischio, checklist, collocazione degli artefatti e responsabilità.
5. Chiarire i dubbi con domande mirate e aggiornamenti successivi. Non inventare soglie o decisioni. Eseguire `dfd setup --domain <dominio>` per rilevare lacune strutturali. `ready` indica completezza strutturale, non approvazione umana o qualità semantica.
6. Usare lo stesso percorso per aggiornare il setup dopo cambiamenti architetturali, pattern ricorrenti o incidenti. Riconfermare le review interessate quando cambiano contratti, criteri selezionati, guardrail o policy di maturità; il testo descrittivo di template e processo non invalida le nuove review selettive. Le review legacy restano conservative.

L'assessment di adozione resta distinto dal setup: un debito legacy fuori ambito non impedisce automaticamente la Fase 0. Quando il setup è pronto, proseguire con rischio e specifica delle feature.

Definire la maturità in lifecycle.json: development-only con ci_required false oppure release-preparation con scelta CI motivata. Leggere il contratto per schema e default legacy; non confondere maturità, adozione e rischio. In solo sviluppo la suite finale locale sostituisce il requisito di introdurre CI; la pre-release resta disabilitata.

Concordare percorsi rapido/light/full e verifiche proporzionate. Team opzionale: assenza di team.json mantiene il lavoro individuale senza review indipendente o integrazione obbligatorie. Per un team definire policy, deleghe e gestione dei conflitti. Mantenere contratti normativi nel catalogo e nei guardrail: le nuove review sono selettive e template/processo descrittivi non devono introdurre vincoli nascosti.
