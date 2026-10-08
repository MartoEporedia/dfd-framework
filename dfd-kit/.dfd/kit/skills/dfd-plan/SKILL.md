---
name: dfd-plan
description: Derivare un piano di sviluppo TDD dalla specifica DFD approvata, mappando task, criteri DoD, test e strumentazione prima dell'implementazione.
---

# Piano di sviluppo DFD

Leggere `.dfd/kit/templates/contratto_toolkit.md`, il setup del dominio e gli artefatti della feature. Eseguire `dfd status <id>`: serve una review di design approvata o approvata con condizioni, ancora attuale. Chiarire eventuali condizioni che impediscono l'avvio; non autoapprovare il design.

Usare `dfd plan <id>`. Crea `plan.md`, `plan.json` ed `evidence.json` oppure riprende il lavoro esistente senza sovrascriverlo. I task iniziali provengono dai criteri applicabili del design: raffinarli in passi eseguibili, con owner, implementazione e verifica. Tutti i criteri applicabili devono essere coperti; non reintrodurre criteri esclusi. Allineare JSON e Markdown, includendo test funzionali, non funzionali e strumentazione pertinenti al rischio.

Registrare dubbi in `plan.json.open_questions`, chiarirli iterativamente e conservarne l'esito nel piano. Se cambiano ambito, criteri o decisioni del design, aggiornare prima la specifica e riconfermare la review pertinente. Dopo una nuova approvazione usare `dfd plan <id> --refresh`: aggiorna il riferimento al design preservando i task; rivederli e rigenerare le evidenze coinvolte.

Presentare task e condizioni aperte, poi proseguire con `dfd-implement`. Il piano è un artefatto di lavoro, non prova dell'esecuzione dei test.
