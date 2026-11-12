<!-- dfd-language --> [Italiano](../CHANGELOG.md) | [English](CHANGELOG.md)

# Changelog

All notable changes to DFD Kit. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and versions follow [Semantic Versioning](https://semver.org/). Each version is published as a [GitHub release](https://github.com/MartoEporedia/dfd-framework/releases) with binaries and checksums.

## [Unreleased]

## [0.2.0] - 2026-11-12

### Added

- `criteria[].acceptance` in `design.json`: each applicable criterion declares its acceptance case files (tests as data). The files enter the design fingerprint, so a changed case makes only the declaring feature `stale-review` and `changes` names it, and the criterion fingerprint, so with `source_roots` new red/green are required (#15).
- `acceptance_roots` in `lifecycle.json`: warning for case files that no criterion of the domain declares (#15).

### Changed

- The contract and the `dfd-setup`, `dfd-specify`, `dfd-review-design`, `dfd-implement` skills describe acceptance cases (#15).

## [0.1.0] - 2026-11-12

First published release.

### Added

- DFD framework in Italian and English and the `dfd` CLI with twelve skills for the Claude Code, OpenCode, Copilot and Codex adapters: domain Phase 0, risk and rapid/light/full route, iterative specification, design review, TDD plan, evidence, pre-release, rollout and release review (#1).
- Optional team collaboration with an explicit policy (#1).
- Installation prompt for an agent in any harness (#2).
- `status` exposes `effective_phase` and `blocking` (#8).
- With `stale-review`, `changes` lists what changed since the last decision: every decision stores the snapshot of the fingerprint inputs (#11).
- `log_markers` in `lifecycle.json`: warnings for markers such as `FAILED` or `skipped` in the logs of passing checks (#12).
- `suite_entrypoints` in `lifecycle.json`: suite scripts are part of the design fingerprint (#12).
- `source_roots` and `source_excludes` in `lifecycle.json`: the final suite stays bound to the files it ran on (#13).
- Per-criterion design fingerprints: a criterion changed after the green requires new red/green (#13).
- `dfd exec` command, which runs a project command without a shell and records the check with log and snapshot (#13).
- Active GitHub Actions CI and a release workflow on push of a `v*` tag: binaries for Linux x86_64 (musl), macOS arm64 and x86_64, Windows x86_64, `SHA256SUMS` and notes taken from this changelog.
- Curated mutation tests for the gates.

### Changed

- `status` exits with code 2 when a shown gate is blocked (#8).
- Selective reviews include the `dod.md` text of the selected criteria (#9), read with a CommonMark parser (#10).
- The `dfd-kit` domain moves from `development-only` to `release-preparation` with CI required.
- The installation prompt downloads the binary from the release instead of looking for it in `dfd-kit/dist/`, which is not in the repository.
- README and CONTRIBUTING describe `status` signals, `dfd exec` and mutants (#14).

### Fixed

- `verify` no longer moves the `pre-release` and `release-reviewed` phases back when it succeeds (#8).
- Changes to the DoD text of selected criteria no longer go unnoticed in selective reviews (#9, #10).
