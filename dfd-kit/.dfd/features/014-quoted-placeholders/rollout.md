# Configurazione Rollout – Release GitHub v0.3.1 di dfd-kit

> **Stato**: in review  
> **Data creazione**: 2026-11-19  
> **Proprietari**: Marco Martorana (manutentore)  
> **Servizio / Dominio**: dfd-kit  
> **Riferimenti**: [spec.md](spec.md), [rollout.json](rollout.json), PR #22 e #23, CI run 44431507465

## 1. Panoramica

Patch release: `review-design` non blocca più i segnaposto dentro citazioni e codice della specifica, così le parole del richiedente si citano alla lettera. Contiene i quattro archivi (Linux x86_64 musl, macOS arm64, macOS x86_64, Windows x86_64), un `SHA256SUMS` complessivo e le note EN/IT della sezione `0.3.1`.

- **Ambiente**: GitHub Releases del repository pubblico.
- **Revisione**: commit di merge della PR #23 su `main`, contenuto verificato al commit `c04612e`; il workflow di release riesegue tutta la CI sul commit del tag.
- **Dove la versione deve essere in uso**: il Mac del manutentore (`~/.local/bin/dfd`), dal binario `dfd-macos-arm64` della release.

## 2. Strategia

Diretta, in una fase al 100%: l'esposizione la controlla il tag `v0.3.1`.

- **Promozione**: workflow verde; quattro archivi e `SHA256SUMS`; `dfd-macos-arm64` scaricato, checksum verificato, `dfd --version` uguale a 0.3.1 e `dfd doctor` riuscito; note EN e IT.
- **Stop**: workflow fallito, archivio mancante, checksum non corrispondente o `doctor` fallito.

## 3. Segnali e soglie per criterio

| Criterio | Segnale | Soglia |
|---|---|---|
| SEC-03 | controllo dei segnaposto | citazioni e codice non bloccano; testo libero blocca; mutanti intercettati |
| SLO-01 | comandi e contratti | suite verde; gate delle specifiche approvate invariati |
| SLO-02 | job quality, mutations, binaries | tutti success nella PR #23 e nel workflow Release |
| COST-02 | costo delle verifiche | due mutanti nuovi; CI nei tempi abituali |
| UX-01 | note della release | sezioni EN e IT della 0.3.1; `check_translations` senza errori |

## 4. Risorse

- **Allarme** (ready): notifica GitHub di workflow fallito, attiva su GitHub ed email (verificata il 2026-11-18, feature 013).
- **Dashboard** (ready): pagine Actions e Releases; evidenza nel log CI della PR #23.
- **Controllo dell'esposizione**: il tag.

## 5. Test non funzionali

- **load**: escluso, nessun servizio.
- **resilience**: suite completa (fmt, Clippy, 95 test, doctor, test Python, traduzioni).
- **security**: `quoted_placeholders_do_not_block_review`, il segnaposto nel testo libero continua a bloccare.

## 6. Rollback

- **Trigger**: workflow fallito dopo la creazione della release, archivio mancante, checksum errato o `doctor` fallito.
- **Procedura**: `gh release delete v0.3.1 --cleanup-tag --yes`, correzione, release `v0.3.2`. La v0.3.0 resta disponibile.
- **Verifica**: `gh release view v0.3.1` fallisce; la release successiva supera la promozione.
- **Reperibile**: Marco Martorana.

## 7. Comunicazioni

PR #23 pubblica con il piano della v0.3.1; il manutentore è informato in chat.
