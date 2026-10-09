# DFD Framework

Framework DoD-First Driven: dalla specifica iterativa al rilascio, con criteri verificabili e review proporzionate al rischio.

## Come leggere questo repo (per ruolo)

### Product Manager / Product Owner

1. **[Fondamenta](fondamenta.md)** – per capire l’idea centrale e il “perché” di DFD.  
2. **[Processo End-to-End](processo-e2e.md)** – per vedere come si inserisce il prodotto nelle varie fasi (soprattutto Idea/Epic e Post-release).  
3. **[Criteri di Rischio](rischio.md)** – per capire quando una feature richiede un percorso full o light.  
4. **[Template Specifica di Feature](templates/specifica_feature.md)** – per vedere come vengono strutturati obiettivi, KPI e impatto su utenti/business.

Focus: obiettivi business, KPI, priorità, impatto su UX e metriche di prodotto.

---

### Tech Lead / Architect

1. **[Fondamenta](fondamenta.md)** – visione d’insieme del framework.  
2. **[Processo End-to-End](processo-e2e.md)** – per governare le fasi e i gate di review.  
3. **[Modello RACI](raci.md)** – per chiarire ruoli e responsabilità nel team.  
4. **[Criteri di Rischio](rischio.md)** – per decidere il livello di rigore (full vs light).  
5. **[Template DoD Estesa](templates/dod-estesa.md)** + **[Esempio Checkout](examples/checkout_dod.md)** – per definire o affinare la DoD del dominio.  
6. **[Template Specifica di Feature](templates/specifica_feature.md)** + **[Esempio PayPal](examples/checkout_paypal_spec.md)** – per guidare la progettazione delle feature.

Focus: coerenza architetturale, guardrail, sicurezza, SLO, evoluzione della DoD.

---

### Developer

1. **[Fondamenta](fondamenta.md)** – per capire la logica di fondo (DoD come contratto).  
2. **[Processo End-to-End](processo-e2e.md)** – per sapere cosa ci si aspetta in ogni fase.  
3. **[Template Specifica di Feature](templates/specifica_feature.md)** – per leggere/scrivere specifiche allineate a DFD.  
4. **[Checklist di Review DFD – Design](templates/review_design.md)** (e Light) – per prepararsi alle review.  
5. **[Template DoD Estesa](templates/dod-estesa.md)** – per capire quali criteri di osservabilità, sicurezza, SLO, costo devi soddisfare.

Focus: scrivere codice e test in ottica TDD DFD, strumentare log/metriche/trace, rispettare la DoD.

---

### QA / SDET

1. **[Fondamenta](fondamenta.md)** – per inquadrare il ruolo dei test in DFD.  
2. **[Processo End-to-End](processo-e2e.md)** – per vedere dove intervengono i test (sviluppo, pre-release, post-release).  
3. **[Checklist di Review DFD – Design](templates/review_design.md)** e **[Release](templates/review_release.md)** – per partecipare attivamente alle review.  
4. **[Template DoD Estesa](templates/dod-estesa.md)** – per derivare test da OBS-xx, SEC-xx, SLO-xx, COST-xx.  
5. **[Esempio PayPal](examples/checkout_paypal_spec.md)** – per vedere un caso concreto di test e validazione.

Focus: test funzionali e non funzionali, resilienza, sicurezza, validazione in produzione.

---

### Ops / SRE

1. **[Fondamenta](fondamenta.md)** – per capire il legame tra DoD, allarmi e rilascio.  
2. **[Processo End-to-End](processo-e2e.md)** – per vedere il tuo ruolo in pre-release e rollout.  
3. **[Template Configurazione Rollout](templates/configurazione_rollout.md)** – per definire fasi, soglie, rollback.  
4. **[Template DoD Estesa](templates/dod-estesa.md)** – per allineare allarmi e dashboard ai criteri DoD.  
5. **[Esempio Checkout](examples/checkout_dod.md)** – per vedere un caso concreto di osservabilità e SLO.

Focus: allarmi, dashboard, canary, rollout, gestione incidenti legati a SLO e osservabilità.

---

### Security Engineer

1. **[Fondamenta](fondamenta.md)** – per inquadrare il ruolo della sicurezza in DFD.  
2. **[Criteri di Rischio](rischio.md)** – per identificare le feature high risk su sicurezza.  
3. **[Template DoD Estesa](templates/dod-estesa.md)** – sezione Sicurezza (SEC-xx).  
4. **[Checklist di Review DFD – Design](templates/review_design.md)** e **[Release](templates/review_release.md)** – per partecipare alle review con focus security.  
5. **[Esempio PayPal](examples/checkout_paypal_spec.md)** – per vedere un caso concreto di criteri SEC-xx.

Focus: controlli di accesso, audit, protezione dati, conformità, review di sicurezza.

---

### Support / Customer Success

1. **[Fondamenta](fondamenta.md)** – per capire la logica di validazione in produzione.  
2. **[Processo End-to-End](processo-e2e.md)** – per vedere quando sei coinvolto (pre-release, post-release).  
3. **[Criteri di Rischio](rischio.md)** – per capire quali feature possono avere più impatto sugli utenti.  
4. **[Esempio PayPal](examples/checkout_paypal_spec.md)** – per vedere come una feature può impattare UX e KPI.

Focus: feedback utenti, segnalazioni post-release, impatto su UX e business KPI.

## Progetti ancora in sviluppo

DFD supporta la modalità **solo sviluppo**, con specifiche iterative, review e verifiche locali tracciate anche quando la CI è prematura. La **preparazione al rilascio** si attiva esplicitamente quando si decide di distribuire, rivalutando rischio, policy CI e requisiti operativi. Vedere [fondamenta](fondamenta.md#maturità-del-progetto-solo-sviluppo-e-preparazione-al-rilascio) e [processo](processo-e2e.md#maturità-del-progetto-solo-sviluppo-e-preparazione-al-rilascio).

## Processo proporzionato, anche per un solo dev

Scegliere **rapido** per fix circoscritti entro un contratto noto, **light** per cambiamenti contenuti da specificare e **full** per rischi maggiori. Il rapido usa un solo [record](templates/cambiamento_rapido.md), con regressione red/green per bug e verifiche pertinenti per modifiche editoriali. DFD funziona per singoli sviluppatori e team; PR, review indipendente e verifiche integrate dipendono dalle [convenzioni di collaborazione](templates/collaborazione.md). Vedere [rischio](rischio.md#percorsi-proporzionati-rapido-light-e-full) e [CLI](dfd-kit/README.md).
