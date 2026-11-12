# Piano di sviluppo – Casi di accettazione come dipendenze del design

## Ambito e test

Campo facoltativo criteria[].acceptance in design.json: file di casi di accettazione (test come dati) inclusi nel fingerprint di design e nel fingerprint del criterio; campo di dominio acceptance_roots in lifecycle.json con avviso per i file di casi non dichiarati da alcuna feature

Red/green: caso dichiarato modificato dopo l'approvazione (stale-review con percorso in changes, review selettiva e legacy); caso modificato dopo il green con source_roots (task bloccato); validazione (criterio escluso, file mancante, percorso esterno, symlink, duplicato); acceptance_roots (avviso, dichiarato da altra feature, radice mancante); due feature nello stesso dominio; fingerprint legacy invariato. fmt, Clippy, suite, doctor, check_translations, mutanti. Manuale: status delle feature 001-009 invariato col binario eec9dffa; laboratorio v4 rieseguito.

## Task TDD

### task-1 – Soddisfare OBS-01

Owner: manutentore del repository
Criteri: OBS-01

Implementazione: Errori di validazione, changes e avvisi nominano criterio e percorso del file di casi.

Verifica: Test workflow red/green (acceptance nel design e nel criterio, validazione, acceptance_roots, due feature, fingerprint legacy), suite locale, mutanti, laboratorio v4.

### task-2 – Soddisfare OBS-02

Owner: manutentore del repository
Criteri: OBS-02

Implementazione: criteria[].acceptance lega criterio, file di casi e decisione di design.

Verifica: Test workflow red/green (acceptance nel design e nel criterio, validazione, acceptance_roots, due feature, fingerprint legacy), suite locale, mutanti, laboratorio v4.

### task-3 – Soddisfare SEC-01

Owner: manutentore del repository
Criteri: SEC-01

Implementazione: Percorsi di acceptance e acceptance_roots via Store: niente escape né symlink; visita con le regole di source_roots.

Verifica: Test workflow red/green (acceptance nel design e nel criterio, validazione, acceptance_roots, due feature, fingerprint legacy), suite locale, mutanti, laboratorio v4.

### task-4 – Soddisfare SEC-03

Owner: manutentore del repository
Criteri: SEC-03

Implementazione: Un file di casi cambiato invalida la review di design e, con source_roots, il green dei task del criterio.

Verifica: Test workflow red/green (acceptance nel design e nel criterio, validazione, acceptance_roots, due feature, fingerprint legacy), suite locale, mutanti, laboratorio v4.

### task-5 – Soddisfare SLO-01

Owner: manutentore del repository
Criteri: SLO-01

Implementazione: Campi facoltativi in schema_version 1; lista vuota non serializzata; fingerprint invariati senza acceptance.

Verifica: Test workflow red/green (acceptance nel design e nel criterio, validazione, acceptance_roots, due feature, fingerprint legacy), suite locale, mutanti, laboratorio v4.

### task-6 – Soddisfare SLO-02

Owner: manutentore del repository
Criteri: SLO-02

Implementazione: Red/green, suite, fmt, Clippy, doctor, check_translations, mutanti.

Verifica: Test workflow red/green (acceptance nel design e nel criterio, validazione, acceptance_roots, due feature, fingerprint legacy), suite locale, mutanti, laboratorio v4.

### task-7 – Soddisfare COST-01

Owner: manutentore del repository
Criteri: COST-01

Implementazione: Nessuna dipendenza; la CLI legge soltanto i file dichiarati e le radici.

Verifica: Test workflow red/green (acceptance nel design e nel criterio, validazione, acceptance_roots, due feature, fingerprint legacy), suite locale, mutanti, laboratorio v4.

### task-8 – Soddisfare UX-01

Owner: manutentore del repository
Criteri: UX-01

Implementazione: Contratto, guide e skill IT/EN (dfd-setup, dfd-specify, dfd-review-design, dfd-implement).

Verifica: Test workflow red/green (acceptance nel design e nel criterio, validazione, acceptance_roots, due feature, fingerprint legacy), suite locale, mutanti, laboratorio v4.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
