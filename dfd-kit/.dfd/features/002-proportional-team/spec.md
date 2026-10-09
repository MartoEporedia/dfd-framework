# Percorsi proporzionati, singolo sviluppatore e collaborazione facoltativa

Owner: Marco Martorana. Data: 2026-10-09.

## Decisione

Il manutentore ha approvato nella conversazione rapido/light/full, test mirati ai rischi gravi e uso in team, chiedendone implementazione in documenti e CLI. Non si autorizza alcuna distribuzione.

## Contratto

Rapido: un solo change.json per intervento, con problema, ambito, owner, rischio, vincoli dichiarati, file e verifiche, eventuali riferimenti issue/branch/PR/revisione e decisioni umane. Nessuna specifica o piano separati. Tutte le dimensioni devono essere low, ambito noto, nessun contratto/guardrail cambiato, nessun comportamento critico o dubbio aperto; un fix comportamentale deve ripristinare un contratto noto e avere regressione red prima di green. Editoriale richiede una verifica pertinente, senza red artificiale. Rischio ignoto blocca; dubbio o impatto maggiore indica light/full. La review della PR può costituire l’unica decisione sul cambiamento rapido, registrata solo quando realmente espressa.

Light e full mantengono i gate esistenti. Per le nuove feature il fingerprint usa i criteri coinvolti e i guardrail operativi: aggiornamenti editoriali ai template/processo o al piano di adozione non invalidano automaticamente il design. Il catalogo strutturato è il contratto normativo dei criteri; modificarlo oppure modificare guardrail/lifecycle invalida i design interessati. Compatibilità conservativa per le feature legacy.

Team: policy di dominio opzionale, owner e riferimenti per intervento, niente stato centrale da aggiornare per ogni fix. Review e prove riferite alla revisione dichiarata e a snapshot dei file; file/revisione cambiati invalidano la review pertinente. Un clone non è coordinato da lock remoti: usare Git, ID distinti e gestione esplicita dei conflitti, senza last-writer-wins automatico. Prima della release, quando la policy team lo richiede, serve una verifica dell’integrazione corrente che copra i file della feature; esiti locali isolati non bastano. La CLI verifica dichiarazioni e hash, senza interrogare GitHub o autenticare identità/revisioni. Rapido non bypassa la review di release: una distribuzione riprende una feature light/full con riferimento al record rapido.

## Verifiche

- OBS-01: Test rapid_escalates e individual_defaults: rischio ignoto/critico e stato tecnico individuale corretto.
- OBS-02: Test rapid_invalid_logs, rapid_reviews e team_requires: hash, date, revisione e copertura dei file.
- SEC-01: Suite esistente su percorsi/symlink e preservazione; nuovi record rifiutano collisioni e schema incoerente.
- SEC-02: Test rapid_preserves e suite install/init: record originale conservato e collisioni rifiutate.
- SEC-03: Test solo/team, review stale e selettività; mutanti contro reviewer coincidente, revisione obsoleta e extra gate individuale.
- SLO-01: Suite completa incluse fixture legacy e test default selective_review; contratto documentato.
- SLO-02: Log reali red/green, suite finale locale e report mutation con baseline verde; nessuna CI dichiarata.
- SLO-03: Nessuna modifica al meccanismo di persistenza o ai budget; verifiche esistenti preservate.
- COST-01: Test CLI con PATH vuoto e build offline; distinzione esplicita fra strumenti di sviluppo e utilizzo.
- COST-02: Nessuna modifica al meccanismo di persistenza o ai budget; verifiche esistenti preservate.
- UX-01: Test ripresa, escalation e autonomia individuale; verifica link/anchor e copie canoniche dei documenti.
- UX-02: doctor, integrità manifest e parità delle risorse installate; pilota harness resta baseline esplicita.

## Chiarimento sul singolo sviluppatore

Il manutentore ha precisato che il lavoro in team non è obbligatorio. La policy team è facoltativa: il singolo dev può completare un fix rapido con record ed evidenze pertinenti, senza PR, secondo reviewer o ulteriore gate di design. Il gate change-verified è tecnico e non registra approvazioni umane inesistenti. Se si registra volontariamente una review, questa rimane legata allo snapshot esaminato. Le policy team possono richiedere review indipendente e verifiche integrate prima della distribuzione.
