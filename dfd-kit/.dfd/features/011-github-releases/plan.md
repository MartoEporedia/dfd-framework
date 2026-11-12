# Piano di sviluppo – Release GitHub con changelog

## Ambito e test

CI GitHub Actions attiva, workflow di release al push di un tag v* con i binari dei quattro sistemi e SHA256SUMS, CHANGELOG.md IT/EN come corpo della release, dominio dfd-kit in release-preparation con CI richiesta

Red/green di tests/test_release_notes.py (sezioni IT/EN, tag non conforme, versione diversa da Cargo.toml, sezione assente o vuota, ultima sezione, Unreleased); suite locale (fmt, Clippy, test, doctor, check_translations, test Python); CI reale sulla PR con log scaricati come kind ci; revisione dei workflow; release v0.1.0 verificata scaricando archivi e checksum.

## Task TDD

### task-1 – Soddisfare OBS-01

Owner: manutentore del repository
Criteri: OBS-01

Implementazione: release_notes.py --check nomina tag, versione o sezione errati; job falliti fermano la release.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-2 – Soddisfare OBS-02

Owner: manutentore del repository
Criteri: OBS-02

Implementazione: Log CI reali registrati come kind ci; release legata a tag e commit.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-3 – Soddisfare SEC-01

Owner: manutentore del repository
Criteri: SEC-01

Implementazione: contents: read per la CI, contents: write solo nel job di pubblicazione; azioni a versione fissata.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-4 – Soddisfare SEC-02

Owner: manutentore del repository
Criteri: SEC-02

Implementazione: Release esistente non sovrascritta; dist locale invariato.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-5 – Soddisfare SEC-03

Owner: manutentore del repository
Criteri: SEC-03

Implementazione: Release solo dopo pre-release e review di release umana; tag dal manutentore o su sua richiesta esplicita.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-6 – Soddisfare SLO-01

Owner: manutentore del repository
Criteri: SLO-01

Implementazione: CLI e contratti invariati.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-7 – Soddisfare SLO-02

Owner: manutentore del repository
Criteri: SLO-02

Implementazione: Test dello script, suite locale, CI reale.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-8 – Soddisfare SLO-04

Owner: manutentore del repository
Criteri: SLO-04

Implementazione: CI attivata e richiesta per la release (ci_required true).

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-9 – Soddisfare COST-01

Owner: manutentore del repository
Criteri: COST-01

Implementazione: Nessuna dipendenza nuova nel binario; script con la sola libreria standard.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-10 – Soddisfare COST-02

Owner: manutentore del repository
Criteri: COST-02

Implementazione: Timeout dei mutanti commisurato alla durata reale misurata.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

### task-11 – Soddisfare UX-01

Owner: manutentore del repository
Criteri: UX-01

Implementazione: README, CONTRIBUTING, prompt di installazione e changelog IT/EN coerenti.

Verifica: tests/test_release_notes.py red/green, suite locale, CI reale sulla PR, revisione dei workflow, prima release v0.1.0.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
