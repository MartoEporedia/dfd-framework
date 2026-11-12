<!-- dfd-language --> [Italiano](CHANGELOG.md) | [English](en/CHANGELOG.md)

# Changelog

Tutte le modifiche rilevanti di DFD Kit. Il formato segue [Keep a Changelog](https://keepachangelog.com/it/1.1.0/) e le versioni seguono il [versionamento semantico](https://semver.org/lang/it/). Ogni versione è pubblicata come [release GitHub](https://github.com/MartoEporedia/dfd-framework/releases) con i binari e i checksum.

## [Unreleased]

## [0.1.0] - 2026-11-12

Prima release pubblicata.

### Aggiunto

- Framework DFD in italiano e inglese e CLI `dfd` con dodici skill per gli adapter Claude Code, OpenCode, Copilot e Codex: Fase 0 di dominio, rischio e percorso rapido/light/full, specifica iterativa, review di design, piano TDD, evidenze, pre-release, rollout e review di release (#1).
- Collaborazione di team facoltativa con policy esplicita (#1).
- Prompt di installazione con un agente in qualunque harness (#2).
- `status` espone `effective_phase` e `blocking` (#8).
- Con `stale-review`, `changes` elenca ciò che è cambiato dopo l'ultima decisione: ogni decisione salva lo snapshot degli input del fingerprint (#11).
- `log_markers` in `lifecycle.json`: avvisi per marcatori come `FAILED` o `skipped` nei log di check verdi (#12).
- `suite_entrypoints` in `lifecycle.json`: gli script della suite entrano nel fingerprint di design (#12).
- `source_roots` e `source_excludes` in `lifecycle.json`: la suite finale resta legata ai file su cui è stata eseguita (#13).
- Fingerprint di design per criterio: un criterio cambiato dopo il green richiede nuovi red/green (#13).
- Comando `dfd exec`, che esegue un comando del progetto senza shell e registra il check con log e snapshot (#13).
- CI GitHub Actions attiva e workflow di release al push di un tag `v*`: binari per Linux x86_64 (musl), macOS arm64 e x86_64, Windows x86_64, `SHA256SUMS` e note prese da questo changelog.
- Mutation test curati per i gate.

### Modificato

- `status` esce con codice 2 quando un gate mostrato è bloccato (#8).
- Le review selettive includono il testo di `dod.md` dei criteri selezionati (#9), letto con un parser CommonMark (#10).
- Il dominio `dfd-kit` passa da `development-only` a `release-preparation` con CI richiesta.
- Il prompt di installazione scarica il binario dalla release invece di cercarlo in `dfd-kit/dist/`, che non è nel repository.
- README e CONTRIBUTING descrivono segnali di `status`, `dfd exec` e mutanti (#14).

### Corretto

- `verify` non riporta più indietro le fasi `pre-release` e `release-reviewed` quando riesce (#8).
- Modifiche al testo DoD dei criteri selezionati non passavano inosservate nelle review selettive (#9, #10).
