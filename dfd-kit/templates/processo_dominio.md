# Convenzioni DFD di dominio

## Responsabilità e aggiornamenti

TODO: indicare owner e data di revisione. Aggiornare il setup dopo cambiamenti del dominio, pattern ricorrenti, incidenti e learning di produzione.

## DoD e criteri

TODO: documentare gli assi rilevanti e il naming stabile OBS-01, SEC-01, SLO-01, COST-01, UX-01; mantenere dod.md e criteria.json coerenti.

## Classificazione e review

TODO: documentare l'applicazione delle regole di rischio centrali e la scelta delle checklist light / full.

## Template locali

TODO: dichiarare se i template in templates/ sono usati senza modifiche oppure adattati; documentare le differenze rispetto al framework centrale. I placeholder nei template sono intenzionali.

## Collocazione e uso degli artefatti

TODO: indicare dove vivono gli artefatti del dominio e delle feature, come si svolgono le review e come si aggiornano le specifiche in modo iterativo per chiarire i dubbi.

## Maturità del progetto: solo sviluppo e preparazione al rilascio

La maturità del progetto è distinta dal livello di adozione DFD e dal rischio della feature. La Fase 0 dichiara una modalità, la scelta sulla CI e la relativa motivazione:

| Modalità | Verifiche di sviluppo | Uscita |
|---|---|---|
| Solo sviluppo (`development-only`) | Review di design, TDD ed evidenze reali; suite finale locale o CI disponibile, senza obbligo di introdurre CI | Sviluppo completato; nessuna pre-release o autorizzazione a distribuire |
| Preparazione al rilascio (`release-preparation`) | Stessi controlli, con CI richiesta per default; un’alternativa locale richiede motivazione rispetto a rischio e destinazione e review umana | Prontezza alla pre-release, poi review di release distinta |

L’assenza della CI non è una lacuna bloccante in solo sviluppo. Restano obbligatorie le verifiche applicabili e la tracciabilità di comando, data, risultato e log: un test locale non viene dichiarato CI. La suite finale deve seguire i test green. Una CI dichiarata fallita continua a bloccare anche quando facoltativa.

Prima di distribuire, passare esplicitamente alla preparazione al rilascio: rivalutare rischio, criteri, policy CI e verifiche nell’ambiente di destinazione. Riconfermare review e aggiornare piano ed evidenze invalidati. La modalità solo sviluppo non attenua controlli di sicurezza o rischio; il completamento locale non sostituisce l’approvazione umana del rilascio. Il learning può provenire anche dagli esperimenti locali, prima di disporre di dati di produzione.

Dichiarare la policy in `lifecycle.json` secondo il contratto del toolkit; motivare la scelta CI e la transizione al rilascio.
