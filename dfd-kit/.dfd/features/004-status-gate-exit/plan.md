# Piano di sviluppo – Exit code di status e fase effettiva

## Ambito e test

dfd status esce 2 con un gate bloccato; status espone effective_phase derivata dai gate; verify non regredisce fasi successive (issue #7)

Red/green su F06 riprodotto (evidenze rimosse dopo release approvata: status exit 2, blocking development, effective_phase development) e su verify che non regredisce phase dopo la release. Test per record rapido e status senza ID; test esistenti aggiornati all'exit 2. fmt, Clippy, suite, doctor, check_translations, mutanti.

## Task TDD

### status-exit – status esce 2 con un gate bloccante e lo elenca in blocking (feature, record rapido, status globale)

Owner: manutentore del repository
Criteri: OBS-01, SLO-01, SLO-02, UX-01, UX-02

Implementazione: Calcolare in status i gate bloccanti (setup blocked, design blocked, sviluppo blocked-plan/blocked-evidence, release blocked; record rapido blocked), esporli in blocking e restituire exit 2; aggiornare contratto, guide e skill IT/EN che chiamano status.

Verifica: Regressione cargo test --locked --offline --test workflow status_exit: red su F06 riprodotto (evidenze rimosse dopo release approvata, status exit 0), green con exit 2 e blocking; test esistenti aggiornati; mutante sul calcolo dei gate bloccanti; check_translations e doctor.

### effective-phase – status espone effective_phase derivata dai gate; verify non regredisce le fasi successive

Owner: manutentore del repository
Criteri: OBS-01, SEC-03, SLO-01, SLO-02, UX-01

Implementazione: Derivare effective_phase dai gate correnti senza scrivere state.json; in verify non riportare pre-release/release-reviewed a development-verified e riportare a development da ogni fase successiva quando fallisce; documentare nel contratto IT/EN.

Verifica: Regressione cargo test --locked --offline --test workflow effective_phase: red su verify dopo release approvata (phase regredisce) ed effective_phase assente, green dopo la correzione; mutante sulla transizione di fase.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
