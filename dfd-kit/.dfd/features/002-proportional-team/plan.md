# Piano di sviluppo – percorsi proporzionati

I task iniziali per criterio sono stati consolidati in tre responsabilità implementative, mantenendo tutti i criteri applicabili. I red dei tre gruppi sono stati eseguiti prima della prima implementazione e sono conservati senza alterazioni. Il chiarimento successivo del manutentore elimina il gate umano aggiuntivo dal rapido individuale.

## task-rapid – Record rapido e autonomia individuale

Owner: Marco Martorana. Criteri: OBS-01, OBS-02, SEC-01, SEC-02, SEC-03, SLO-01, SLO-02, COST-01, UX-01, UX-02.

Implementare quick, prove pertinenti, escalation e promozione conservativa; aggiornare documenti e skill.

Verifiche: Red iniziale rapid_; green sui test rapid_ e suite finale, mutanti su criticità/regressione/gate individuale.

## task-team – Collaborazione facoltativa e integrazione corrente

Owner: Marco Martorana. Criteri: OBS-01, OBS-02, SEC-03, SLO-01, SLO-02, UX-01.

Implementare context e team.json opzionali, review indipendente e prove integrate sulla revisione corrente.

Verifiche: Red iniziale team_requires_; green con owner/reviewer, revisione, log, snapshot e copertura integrata; mutanti mirati.

## task-selective – Review selettive e compatibilità

Owner: Marco Martorana. Criteri: SEC-03, SLO-01, SLO-02, UX-01, UX-02.

Fingerprint selettivo per nuove feature, conservativo per legacy; criteri e guardrail normativi, template descrittivi.

Verifiche: Red iniziale selective_review_; green selettività più suite legacy e mutation sulla selezione dei criteri.

Suite finale locale obbligatoria per questa feature full; nessuna CI richiesta nella policy development-only. Doctor e controlli statici delle skill non sostituiscono il pilota degli altri harness. Nessuna release autorizzata.
