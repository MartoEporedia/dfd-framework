# Configurazione Rollout – Release GitHub v0.3.0 di dfd-kit

> **Stato**: in review  
> **Data creazione**: 2026-11-18  
> **Proprietari**: Marco Martorana (manutentore)  
> **Servizio / Dominio**: dfd-kit  
> **Riferimenti**: [spec.md](spec.md), [rollout.json](rollout.json), PR #20, CI run 44313937009

## 1. Panoramica

La release porta l'inglese come lingua principale (feature 012, cambiamento incompatibile) e gli stati non previsti e la qualità della specifica nel kit (feature 013). Contiene i quattro archivi (Linux x86_64 musl, macOS arm64, macOS x86_64, Windows x86_64), un `SHA256SUMS` complessivo e le note EN/IT della sezione `0.3.0` del changelog.

- **Ambiente**: GitHub Releases del repository pubblico.
- **Revisione**: commit di merge della PR #20 su `main`. Il contenuto è verificato al commit `40cd5bb`, e il workflow di release riesegue tutta la CI sul commit del tag.
- **Dove la versione deve essere in uso**: il Mac del manutentore (`~/.local/bin/dfd`), aggiornato dal binario `dfd-macos-arm64` della release.

## 2. Strategia

Diretta, in una sola fase al 100%, come per le versioni precedenti: l'esposizione la controlla il tag `v0.3.0`.

- **Promozione**: workflow verde; quattro archivi e `SHA256SUMS`; `dfd-macos-arm64` scaricato, checksum verificato, `dfd --version` uguale a 0.3.0 e `dfd doctor` riuscito; note EN e IT.
- **Stop**: workflow fallito, archivio mancante, checksum non corrispondente o `doctor` fallito.

## 3. Segnali e soglie per criterio

| Criterio | Segnale | Soglia |
|---|---|---|
| SEC-02 | template dei domini esistenti dopo l'aggiornamento | test di preservazione verdi; `language` e `install` non riscrivono i template |
| SEC-03 | istruzioni delle skill sui criteri aggiunti | test delle skill verde in en e it |
| SLO-01 | comandi e contratti sui progetti esistenti | suite verde; passaggio all'inglese documentato, migrazione `dfd language it` |
| SLO-02 | job quality, mutations, binaries | tutti success nella PR #20 e nel workflow Release |
| COST-02 | durata delle verifiche | nessun mutante nuovo; CI nei tempi abituali |
| UX-01 | note e documentazione | sezioni EN e IT della 0.3.0; `check_translations` senza errori |
| UX-02 | skill installate per adapter | `doctor` riesce sul binario scaricato |

## 4. Risorse

- **Allarme** (planned): notifica GitHub di workflow fallito, come per le versioni precedenti.
- **Dashboard** (ready): pagine Actions e Releases; evidenza nel log CI della PR #20.
- **Controllo dell'esposizione**: il tag.

## 5. Test non funzionali

- **load**: escluso, perché non c'è un servizio.
- **resilience**: `kit_resources_adopt_unforeseen_states_revision`, risorse installate in en e it.
- **security**: `skills_guide_unforeseen_states_and_added_criteria`, skill che non approvano criteri aggiunti in modo implicito.

## 6. Rollback

- **Trigger**: workflow fallito dopo la creazione della release, archivio mancante, checksum errato o `doctor` fallito.
- **Procedura**: `gh release delete v0.3.0 --cleanup-tag --yes`, correzione, release `v0.3.1`. La v0.2.0 resta disponibile.
- **Verifica**: `gh release view v0.3.0` fallisce; la release successiva supera la promozione.
- **Reperibile**: Marco Martorana.

## 7. Comunicazioni

PR #20 pubblica con il piano della v0.3.0; il manutentore è informato in chat.
