---
name: dfd-specify
description: Scrivere o aggiornare una specifica di feature DFD dopo l'assessment del rischio, collegando obiettivi, criteri DoD, test e guardrail.
---

# Specifica guidata dalla DoD

Leggere stato e rischio della feature, `.dfd/domains/<dominio>/dod.md`, `criteria.json`, `guardrails.md`, `process.md` e `templates/` dello stesso dominio. Usare `.dfd/kit/templates/contratto_toolkit.md` per il formato strutturato. Leggere `.dfd/kit/processo-e2e.md` solo per chiarire i prerequisiti del design.

1. Verificare con `dfd setup --domain <dominio>` che la Fase 0 sia pronta; completare le lacune con `dfd-setup`. Verificare che il rischio sia completo e attuale con `dfd status <id>`. Se cambia il rischio, aggiornare `risk.json` e rieseguire `assess`.
2. Usare `dfd specify <id>` per creare o riprendere gli artefatti. La CLI non sovrascrive una specifica; restituisce i dubbi aperti se esiste già. Usare il template locale del dominio.
3. Costruire la specifica iterativamente. Registrare dubbi, assunzioni e alternative in `design.json.open_questions`; formulare domande mirate, incorporare le risposte in entrambi gli artefatti e ripetere il ciclo. Procedere intanto sulle sezioni indipendenti. Non chiedere di riconfermare risposte già espresse e non trasformare assunzioni in fatti. Rimuovere un dubbio solo quando chiarito e conservarne l'esito nella specifica. La presenza di dubbi non impedisce di lavorare alla bozza; blocca il gate di approvazione.
4. Compilare `spec.md` con contesto, obiettivo, ambito, scenari, contratti, rischi e piano di test. Per light usare il dettaglio necessario al cambiamento. Per full dettagliare dipendenze, failure mode, sicurezza, SLO, costo e validazione futura del rilascio. Restare proporzionati al rischio.
5. Compilare `design.json` in coerenza con il Markdown: owner, obiettivo, metriche di successo, riferimento all'epic, criteri applicabili/esclusi, test e guardrail. Usare solo criteri presenti nella DoD del dominio. Ogni criterio `changes` deve essere valutato; ogni esclusione deve essere motivata. Le lacune `baseline` non coinvolte dal cambiamento restano nel piano di adozione.
6. Per ogni criterio applicabile descrivere implementazione e verifica prevista. La verifica è un piano, non un test già eseguito. Conservare domande aperte come tali; non inventare approvazioni dell'epic, metriche di produzione o risultati CI.
7. Eseguire `dfd review-design <id>` per rilevare lacune strutturali. Aggiornare entrambi gli artefatti quando cambia il design.

Presentare la specifica e i punti aperti per la review. Non trasformare il completamento dei documenti in approvazione del design.
