---
name: dfd-pre-release
description: Preparare il rilascio di una feature DFD verificata, definendo rollout, soglie, rollback, allarmi, dashboard, test non funzionali e comunicazioni prima della review di release.
---

# Preparazione del rilascio

Leggere `.dfd/kit/processo-e2e.md` (Pre-release e DoR Rilascio), `.dfd/kit/templates/contratto_toolkit.md`, gli artefatti della feature e il setup del dominio. Eseguire `dfd status <id>`: lo sviluppo deve essere verificato e attuale.

Usare `dfd pre-release <id>` per creare o riprendere `rollout.md` e `rollout.json` senza sovrascriverli. Il Markdown proviene dal template locale del dominio; sostituire esempi e placeholder con dati reali. Affinare il piano iterativamente, registrando e chiarendo i dubbi in `open_questions`.

Definire artefatto e revisione da rilasciare, ambiente, owner e on-call; motivare rollout diretto, canary o segmentato. Le fasi strutturate descrivono l'esposizione nell'ambiente finale, fino al 100% dell'audience prevista; le prove staging restano evidenze separate. Collegare ogni criterio DoD applicabile a segnale, soglia e verifica in produzione. Non inventare soglie o baseline: riprendere quelle del dominio oppure chiarirle.

Definire trigger, procedura e verifica del rollback; predisporre allarmi, dashboard e controllo dell'esposizione. Per risorse pronte registrare evidenze locali con hash; per risorse pianificate indicare owner, riferimento e piano di attivazione. Nel light motivare eventuali esclusioni; il full richiede allarmi e dashboard. Un flag può essere escluso motivando il meccanismo alternativo. Dichiarare eseguiti oppure esclusi con motivazione test di carico, resilienza e sicurezza.

Registrare solo comunicazioni realmente effettuate con audience, owner, data e riferimento. Non inviare messaggi o configurare servizi esterni senza l'autorizzazione pertinente; predisporre il contenuto e riportare l'attività ancora necessaria. L'installazione della skill non costituisce una dichiarazione che gli stakeholder siano informati.

Se cambiano design, piano o evidenze, completare le verifiche precedenti e usare `dfd pre-release <id> --refresh`, rivalutando artefatto, soglie e preparazione. Il comando aggiorna il riferimento allo sviluppo preservando i contenuti. Eseguire `dfd review-release <id>`, correggere le lacune e proseguire con `dfd-review-release`. Non dichiarare il rilascio effettuato.

Prima di avviare la preparazione verificare che il dominio sia in release-preparation (default legacy in assenza di lifecycle.json). In development-only fermarsi allo sviluppo verificato; per distribuire aggiornare esplicitamente la policy, rivalutare rischio/CI e riconfermare gli artefatti invalidati.

Quando team.json richiede verifiche integrate, collegare collaboration.json alla revisione integrata e registrare snapshot/files/checks reali. I test dei branch isolati non bastano. La review PR deve corrispondere alla revisione/file esaminati. Questi obblighi non vengono imposti automaticamente al singolo dev; un record rapido va prima promosso a feature per la distribuzione.
