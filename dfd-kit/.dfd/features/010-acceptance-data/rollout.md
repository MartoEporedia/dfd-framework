# Configurazione Rollout – Release GitHub v0.2.0 di dfd-kit

> **Stato**: in review  
> **Data creazione**: 2026-11-12  
> **Proprietari**: Marco Martorana (manutentore)  
> **Servizio / Dominio**: dfd-kit  
> **Riferimenti**: [spec.md](spec.md), [rollout.json](rollout.json), PR #15, CI run 43218912063

## 1. Panoramica

La release porta i casi di accettazione come dipendenze del design (`criteria[].acceptance`) e l'avviso sui casi non dichiarati (`acceptance_roots`). Contiene i quattro archivi (Linux x86_64 musl, macOS arm64, macOS x86_64, Windows x86_64), un `SHA256SUMS` complessivo e le note IT/EN della sezione `0.2.0` del changelog.

- **Ambiente**: GitHub Releases del repository pubblico.
- **Revisione**: commit di merge della PR #15 su `main`. Il contenuto è verificato al commit `7f81362`, e il workflow di release riesegue tutta la CI sul commit del tag.

## 2. Strategia

Diretta, in una sola fase al 100%, come per la v0.1.0: l'esposizione la controlla il tag `v0.2.0`.

- **Promozione**: workflow verde; quattro archivi e `SHA256SUMS`; `dfd-macos-arm64` scaricato, checksum verificato, `dfd --version` uguale a 0.2.0 e `dfd doctor` riuscito; note IT ed EN.
- **Stop**: workflow fallito, archivio mancante, checksum non corrispondente o `doctor` fallito.

## 3. Segnali e soglie per criterio

| Criterio | Segnale | Soglia |
|---|---|---|
| OBS-01 | errori, `changes` e avvisi nei test del job `ci` | test `acceptance_*` verdi; criterio e percorso nominati |
| OBS-02 | legame tra criterio, caso e decisione | test verde; tag sul merge di #15, `ci` e `publish` verdi |
| SEC-01 | validazione dei percorsi | percorsi esterni e symlink rifiutati |
| SEC-03 | invalidazione dopo un caso cambiato | `stale-review` e nuovo green richiesti; mutanti `acceptance-*` intercettati |
| SLO-01 | `status` sui progetti esistenti | gate e fingerprint invariati; suite verde |
| SLO-02 | job quality, mutations, binaries | tutti success; mutanti 71/71 |
| COST-01 | binario scaricato | `doctor` senza toolchain né rete |
| UX-01 | note e documentazione | sezioni IT ed EN della 0.2.0; `check_translations` senza errori |

## 4. Risorse

- **Allarme** (planned): notifica GitHub di workflow fallito. È la stessa risorsa della v0.1.0, con la stessa condizione ancora da verificare.
- **Dashboard** (ready): pagine Actions e Releases; evidenza nel log CI della PR #15.
- **Controllo dell'esposizione**: il tag.

## 5. Test non funzionali

- **load**: escluso, perché non c'è un servizio.
- **resilience**: `undeclared_acceptance_files_are_warned`. Un caso orfano è segnalato e le radici non valide bloccano la Fase 0.
- **security**: `acceptance_files_bind_the_criterion_to_the_design`. Percorsi esterni, symlink, duplicati e criteri esclusi vengono rifiutati.

## 6. Rollback

- **Trigger**: workflow fallito dopo la creazione della release, archivio mancante, checksum errato o `doctor` fallito.
- **Procedura**: `gh release delete v0.2.0 --cleanup-tag --yes`, correzione, release `v0.2.1`. La v0.1.0 resta disponibile.
- **Verifica**: `gh release view v0.2.0` fallisce; la release successiva supera la promozione.
- **Reperibile**: Marco Martorana.

## 7. Comunicazioni

PR #15 pubblica, aggiornata il 2026-11-12 con il piano della v0.2.0; il manutentore è informato in chat.
