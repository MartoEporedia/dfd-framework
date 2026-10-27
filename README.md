<!-- dfd-language -->
[Italiano](README.md) | [English](en/README.md)

# DFD Framework

**Italiano:** framework, template ed esempi nei percorsi qui sotto. **English:** [complete reading paths](en/README.md).

DFD Kit: [Italiano](dfd-kit/README.md) | [English](dfd-kit/en/README.md). [Glossario e manutenzione](glossario.md) / [Glossary and maintenance](en/glossario.md).

Framework DoD-First Driven: dalla specifica iterativa al rilascio, con criteri verificabili e review proporzionate al rischio.

## Installare DFD con un agente (qualsiasi harness)

Copiare il prompt seguente nell'agente di coding (Claude Code, Codex, OpenCode, GitHub Copilot, Cursor, Gemini CLI, Windsurf, Cline o altro), aperto nel repository in cui adottare DFD. L'agente ottiene la CLI `dfd`, installa le skill nel formato dell'harness e si ferma prima della Fase 0, che va completata con `dfd-init`.

```text
Installa il framework DFD (DoD-First Driven) in questo repository.
Sorgente: https://github.com/MartoEporedia/dfd-framework (toolkit in dfd-kit/).

Regole: non sovrascrivere file esistenti (AGENTS.md, CLAUDE.md, istruzioni Copilot,
regole dell'harness); non modificare i file gestiti in .dfd/ o nelle directory delle
skill installate; chiedimi conferma prima di installare software a livello di sistema;
non inventare DoD, SLO o soglie.

1. Contesto. Identifica l'harness in cui stai girando, il sistema operativo e
   l'architettura. Chiedimi solo ciò che non puoi dedurre:
   - dominio pilota (es. checkout);
   - modalità: brownfield se esistono già codice o processi, altrimenti greenfield;
   - lingua: it (default) o en.

2. CLI dfd. Usa `dfd` se è già nel PATH. Altrimenti clona il repository in una
   directory temporanea, fuori da questo progetto:
   - macOS arm64: usa dfd-kit/dist/dfd-macos-arm64, verifica il checksum con
     dfd-kit/dist/SHA256SUMS e rendilo eseguibile (chmod +x);
   - altri sistemi: compila con `cargo build --locked --release` in dfd-kit/
     (richiede Rust; il binario è target/release/dfd, dfd.exe su Windows).
   Copia il binario in una directory del PATH con nome `dfd`, oppure annota il
   percorso assoluto e usalo nei comandi successivi. Esegui `dfd doctor`.

3. Adapter. Scegli l'adapter nativo se l'harness è supportato:
   claude-code (.claude/skills/), opencode (.opencode/skills/),
   copilot (.github/skills/), codex (.agents/skills/).
   Per qualunque altro harness usa codex: installa le skill in .agents/skills/,
   il formato più portabile.

4. Init. Esegui:
   dfd --lang <lingua> --project <root del repo> init --harness <adapter> --domain <dominio> --mode <modalità>
   Se un file gestito risulta in conflitto, fermati e mostrami il conflitto.

5. Harness non nativi. Se hai usato codex come fallback, rendi le dodici skill
   (dfd-init, dfd-setup, dfd-feature, dfd-assess, dfd-specify, dfd-review-design,
   dfd-plan, dfd-implement, dfd-verify, dfd-pre-release, dfd-review-release,
   dfd-status) rilevabili dal tuo harness nel suo meccanismo nativo (regole,
   comandi, workflow o file di istruzioni): preferisci riferimenti ai file
   .agents/skills/<skill>/SKILL.md invece di copie; se l'harness richiede copie,
   indica che la fonte è .agents/skills/ e che vanno rigenerate dopo
   `dfd install`. Aggiungi le istruzioni in coda ai file esistenti, senza
   riscriverli.

6. Verifica e resoconto. Controlla che .dfd/ e le skill esistano e che il tuo
   harness le veda. Riportami: binario e versione usati, adapter, file creati o
   modificati, eventuali limiti. Indica come invocare la prima skill nel tuo
   harness (es. /dfd-init, $dfd-init) per completare la Fase 0: assessment, DoD,
   guardrail e convenzioni del dominio, verificati con `dfd setup --domain <dominio>`.
   Non avviare la Fase 0 senza la mia conferma.
```

Per i dettagli su adapter, lingua e cambio di harness (`dfd install --harness <adapter>`) vedere il [DFD Kit](dfd-kit/README.md).

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
