# Configurazione Rollout – Release GitHub v0.1.0 di dfd-kit

> **Stato**: in review  
> **Data creazione**: 2026-11-12  
> **Proprietari**: Marco Martorana (manutentore)  
> **Servizio / Dominio**: dfd-kit  
> **Riferimenti**: [spec.md](spec.md), [rollout.json](rollout.json), PR #16, CI run 43209565384

## 1. Panoramica

Prima release pubblicata di dfd-kit: quattro archivi (Linux x86_64 musl, macOS arm64, macOS x86_64, Windows x86_64), un `SHA256SUMS` complessivo e le note IT/EN prese da `CHANGELOG.md` ed `en/CHANGELOG.md`.

- **Ambiente**: GitHub Releases del repository pubblico.
- **Revisione**: commit di merge della PR #16 su `main`. Il contenuto è verificato al commit `210c08c`, e il workflow di release riesegue tutta la CI sul commit del tag.

## 2. Strategia

Diretta, in una sola fase al 100%: un binario scaricabile non ha traffico da frazionare. L'esposizione la controlla il tag: senza `v0.1.0` non esiste release. Il workflow pubblica solo se il controllo di tag e changelog e la CI completa sono verdi nello stesso run.

- **Promozione**: workflow verde; quattro archivi e `SHA256SUMS` presenti; `dfd-macos-arm64` scaricato, checksum verificato e `dfd doctor` riuscito; note con le sezioni IT ed EN.
- **Stop**: workflow fallito, archivio mancante, checksum non corrispondente o `doctor` fallito.

## 3. Segnali e soglie per criterio

| Criterio | Segnale | Soglia |
|---|---|---|
| OBS-01 | `release_notes.py --check` nel job `check` | exit 0 per v0.1.0; un tag incoerente non crea release |
| OBS-02 | legame tra tag, commit e run | tag sul merge di #16; `ci` e `publish` verdi nello stesso run |
| SEC-01 | permessi dei job | solo `publish` con `contents: write` |
| SEC-02 | unicità della release | nessuna v0.1.0 preesistente; nessuna sovrascrittura |
| SEC-03 | decisione di release prima del tag | decisione registrata; tag su richiesta esplicita del manutentore |
| SLO-01 | `cargo test` nel job `ci` | tutti verdi, 0 ignorati |
| SLO-02 | job quality, mutations, binaries | tutti success; mutanti al 100% |
| SLO-04 | CI su PR e tag | success in entrambi |
| COST-01 | binario scaricato | `doctor` riesce senza toolchain né rete |
| COST-02 | durata del job mutations | sotto 20 minuti (4 min 44 s misurati sulla PR) |
| UX-01 | note e prompt di installazione | sezioni IT ed EN presenti; checksum verificato con il prompt |

## 4. Risorse

- **Allarme** (planned): notifica GitHub di workflow fallito. Va verificata dopo il primo run di release: il contratto richiede una decisione di release con condizione.
- **Dashboard** (ready): pagine Actions e Releases del repository; evidenza nel log della CI della PR.
- **Controllo dell'esposizione**: il tag (flag escluso, con motivazione in `rollout.json`).

## 5. Test non funzionali

- **load**: escluso, perché GitHub serve i file e non c'è un servizio di dfd-kit.
- **resilience**: `tests/test_release_notes.py`. Tag, versione o changelog incoerenti non producono una release.
- **security**: `tests/test_workflows.py`. Permessi minimi, un solo job con scrittura, azioni a versione fissata, nessuna sovrascrittura.

## 6. Rollback

- **Trigger**: workflow fallito dopo la creazione della release, archivio mancante, checksum errato o `doctor` fallito.
- **Procedura**: `gh release delete v0.1.0 --cleanup-tag --yes`, correzione sul ramo, release `v0.1.1` con la sua sezione nel changelog.
- **Verifica**: `gh release view v0.1.0` fallisce; la release successiva supera la promozione.
- **Reperibile**: Marco Martorana.

## 7. Comunicazioni

PR #16 pubblica (2026-11-12) per chi segue il repository; il manutentore è informato in chat.
