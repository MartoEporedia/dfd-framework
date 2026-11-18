# Assessment del rischio DFD

Rischio: "medium"
Percorso: "full-proportional"

- security: "low" — Nessun nuovo permesso, percorso scritto o input esterno. Il cambio di default non riscrive file dell'utente: le risorse già installate cambiano solo con 'dfd language', che preserva gli artefatti compilati (SEC-02 invariato).
- reliability: "medium" — Il default della lingua tocca ogni comando: messaggi, help, risorse installate e test che oggi presuppongono l'italiano. Un errore nello scambio dei bundle (FILES/FILES_EN) installerebbe la lingua sbagliata; serve copertura di test e mutanti sulla selezione e sul nuovo default.
- cost: "low" — Nessuna dipendenza o servizio nuovo; il binario resta offline e della stessa dimensione (stesse due lingue incorporate).
- business: "medium" — Cambiamento incompatibile visibile: i progetti senza .dfd/language.json passano da messaggi e risorse italiane a inglesi, e i link esterni ai percorsi attuali mostrano l'inglese. Scelta espressa dal manutentore il 2026-11-17 ('Passano all'inglese', 'Inversione completa dei percorsi'); utilizzatori noti pochi, impatto da documentare nel CHANGELOG e nella guida di migrazione.
- architecture: "medium" — Cambia la struttura del repository: circa 70 coppie di documenti si scambiano di percorso (inglese nei percorsi originali, italiano in it/ e dfd-kit/it/), con link, navigazione e alias degli anchor da ricalcolare; build.rs, check_translations.py, release_notes.py e AGENTS.md dipendono da quella struttura. Il bundle installato non cambia forma: i percorsi relativi in .dfd/kit/ restano gli stessi.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
