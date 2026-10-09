---
name: dfd-implement
description: Implementare una feature DFD con TDD dopo la review di design, eseguendo test, codice, refactor e strumentazione e raccogliendo evidenze verificabili.
---

# Sviluppo TDD DFD

Leggere `.dfd/kit/processo-e2e.md` (Sviluppo), `.dfd/kit/templates/contratto_toolkit.md`, specifica, piano e guardrail del dominio. Rispettare le istruzioni e gli strumenti del repository adottante. La CLI non implementa codice e non lancia test: è l'agente dell'harness a svolgere queste attività con i propri strumenti.

Prima di seguire il flusso feature, consultare `status`: un record rapido individuale usa direttamente le verifiche pertinenti e `verify`, senza piano separato né seconda approvazione. Applicare la policy di review indipendente solo se configurata.

1. Eseguire `dfd status <id>`. Procedere se design e piano sono attuali. Se manca il piano usare `dfd plan <id>`; se il design è modificato, completare la review pertinente. Le condizioni di approvazione restano visibili: risolvere quelle previste entro lo sviluppo prima di dichiararlo concluso.
2. Per ogni task scrivere il test del comportamento o vincolo DoD atteso; eseguirlo e conservare il fallimento pertinente (red), poi implementare il minimo necessario e ottenere il verde (green). Un errore di ambiente o sintassi non dimostra il comportamento atteso. Implementare anche log, metriche e trace previsti. Applicare refactor e rieseguire i test. Se emerge un dubbio di design, registrarlo e chiarirlo iterativamente prima del lavoro che ne dipende.
3. Salvare i log reali in `.dfd/features/<id>/evidence/` o collegare log CI scaricati nel repository. Non inventare risultati, date o hash. Compilare `evidence.json` secondo il contratto: riferimenti al design e piano attuali, file di codice/test/strumentazione e hash SHA-256, verifiche red/green per task e suite finale verde: kind suite per esecuzione locale, kind ci per CI reale secondo lifecycle.json. Registrare comando, data con timezone, exit code e log con hash. Calcolare gli hash con gli strumenti disponibili nell'harness; `dfd status` espone `design_fingerprint` e `plan_hash` nello stato di sviluppo.
4. Documentare refactor e osservabilità in `refactor_notes`, anche quando non servono modifiche ulteriori. Se cambia il piano o un file verificato, rieseguire le verifiche pertinenti e aggiornare le evidenze. In development-only raccogliere la suite finale locale senza introdurre CI. In release-preparation, se ci_required true e la CI manca, il gate resta aperto; un’alternativa locale richiede aggiornamento motivato della policy e riconferma della review.
5. Eseguire `dfd verify <id>` e correggere i problemi segnalati. Non modificare lo stato per superare i gate. Per condizioni dovute entro lo sviluppo, ottenere e registrare la nuova decisione del responsabile umano dopo la loro risoluzione; le condizioni per pre-release o release restano aperte nella fase prevista.

Completare il lavoro autorizzato e riportare criteri implementati, test realmente eseguiti e lacune residue. Il gate `development-complete` conclude lo sviluppo senza aprire la pre-release. Il gate `ready-for-pre-release` indica prontezza strutturale per preparare il rilascio, senza approvarlo o effettuarlo.

Per rapido lavorare nel solo record, senza plan separato: riprodurre il bug con una regressione red pertinente, correggere e ottenere green; editoriale richiede solo una verifica utile. Concentrarsi su effetti del fallimento, perdita di dati, autorizzazioni, retry e compatibilità. Mutanti e suite completa solo quando il rischio li rende utili. Se emergono dubbi/contratti nuovi o rischio critico promuovere a light/full.
