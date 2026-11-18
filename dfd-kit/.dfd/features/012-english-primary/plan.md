# Piano di sviluppo – Inglese come lingua principale

## Ambito e test

Inglese nei percorsi originali del framework e del toolkit, italiano spostato in it/ e dfd-kit/it/; link, navigazione, manifest delle traduzioni, build del bundle, script di release e AGENTS.md adattati; default della CLI en, anche per i progetti senza .dfd/language.json

Red/green: default en senza language.json (status, help, init), it preservato con language.json o --lang, bundle en dai percorsi originali e it da it/; test esistenti aggiornati senza indebolire le asserzioni; test_release_notes.py sui nuovi percorsi; mutanti language-resource-selection, language-persistence e nuovo mutante sul default; check_translations con 0 errori; binario candidato su un progetto copiato senza language.json e su uno in it; CI reale sulla PR.

## Task TDD

### default-language – Default della CLI en senza language.json, fuori da un progetto e per init/install senza --lang

Owner: manutentore del repository
Criteri: OBS-01, SLO-01, SEC-02

Implementazione: src/i18n.rs: Language::En come default, project_language e from_args restituiscono En senza preferenza; messaggio di errore della lingua invariato nel catalogo. Le risorse già installate non vengono riscritte.

Verifica: Red/green di cargo test default_language_is_english (status/help/init senza language.json in inglese; language.json it e --lang it restano italiani; nessun file riscritto).

### bundle-layout – Inglese nei percorsi originali, italiano in it/ e dfd-kit/it/, bundle costruiti dai nuovi percorsi

Owner: manutentore del repository
Criteri: UX-02, COST-01, SEC-01

Implementazione: git mv delle 56 coppie con uno script che opera solo dentro il repository e ricalcola link e navigazione (inglese per primo); build.rs legge l'inglese dai percorsi originali e l'italiano da it/; stessi percorsi installati.

Verifica: Red/green di cargo test installed_bundles_follow_english_primary_layout (risorse installate in en uguali ai file dei percorsi originali, in it uguali a quelli di it/, senza riga di navigazione); doctor.

### docs-layout – Controllo delle traduzioni, manifest e documentazione della nuova struttura

Owner: manutentore del repository
Criteri: UX-01, OBS-02, SEC-03

Implementazione: scripts/check_translations.py con coppie inglese→it/, controllo dei file italiani non accoppiati, manifest rigenerato; AGENTS.md, glossario, CONTRIBUTING, README (prompt con en di default) e CHANGELOG con la migrazione 'dfd language it'.

Verifica: Red: check_translations.py fallisce sulla nuova struttura con lo script precedente; green: 56 coppie e 0 errori dopo l'aggiornamento.

### release-notes – Note di release dai nuovi percorsi del changelog, inglese per primo

Owner: manutentore del repository
Criteri: SLO-02, COST-02

Implementazione: scripts/release_notes.py legge CHANGELOG.md (inglese) e it/CHANGELOG.md (italiano); mutante sul default della lingua in mutations/gates.json; suite e mutanti mirati.

Verifica: Red/green di python3 -m unittest tests.test_release_notes; mutanti language-*; suite finale locale e CI reale sulla PR.

## Ciclo di sviluppo

Per ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.
