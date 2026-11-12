<!-- dfd-language --> [Italiano](../CONTRIBUTING.md) | [English](CONTRIBUTING.md)

<a id="sviluppare-dfd-kit"></a>

<a id="processo-di-sviluppo-obbligatorio"></a>

<a id="struttura"></a>

<a id="comandi"></a>

<a id="convenzioni-e-verifiche"></a>

<a id="mutation-test"></a>

<a id="esempio-ripetere-solo-la-verifica-sullintegrità-delle-evidenze"></a>

<a id="contributi"></a>

<a id="documenti-del-framework"></a>

<a id="sviluppo-prima-della-ci"></a>

<a id="fix-rapidi-singolo-sviluppatore-e-team"></a>

# Developing DFD Kit

CLI users need only the binary. Development requires stable Rust/Cargo, rustfmt and Clippy.

## Mandatory development process

Since 2026-10-09 CLI/skill work, including fixes/refactoring/support artifacts, follows DFD. Read [AGENTS.md](../../AGENTS.md), existing [domain process](../.dfd/domains/dfd-kit/process.md), [DoD](../.dfd/domains/dfd-kit/dod.md) and [assessment](../.dfd/domains/dfd-kit/assessment.md). Adopted root is dfd-kit/, brownfield, Codex. Installed skills live in .agents/skills; edit canonical skills/ sources.

From dfd-kit/ run dist/dfd-macos-arm64 setup --domain dfd-kit and status (or platform/current candidate binary). Complete missing Phase 0 first. Assess risk and choose rapid/light/full. Rapid uses one scoped record. Light/full require iterative specification, human design review, then derived plan and TDD. Reuse actual relevant human decisions; structural checks confer no approval. Record real red/green for behavior fixes, pertinent editorial checks and mutants for important invariants. Prepare pre-release/release review only for distribution. Keep all artifacts in the repository according to the [contract](templates/contratto_toolkit.md).

## Structure

- src/cli.rs: commands/options; src/model.rs: versioned JSON/domain types.
- src/lib.rs: install/adoption/risk/design gates; src/i18n.rs and i18n/en.json: offline localization.
- src/development.rs: TDD plans/evidence/gates and the exec command; src/quick.rs: rapid verification/promotion.
- src/collaboration.rs: optional team policy/integrated evidence; src/release.rs: operational readiness/rollout/release review.
- src/store.rs: path checks, source-root traversal, lock and atomic writes; build.rs: embedded documents/templates/skills.
- skills/, templates/, examples/: canonical resources; en/: English counterparts.
- tests/workflow.rs: temporary-project CLI tests; scripts/mutation_test.py and mutations/gates.json: curated gate mutants.

## Commands

Run from dfd-kit/:

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked --release
./target/release/dfd doctor
python3 scripts/check_translations.py
```

Add --offline when dependencies are cached. Preserve Cargo.lock. Use rustfmt/snake_case and dfd-action skill names. Maintain both Italian and English resources; see [glossary/maintenance](../../en/glossario.md). Versioned JSON is schema_version 1 and rejects unknown fields. Preserve user artifacts, idempotence, missing-data handling, feature isolation, decision invalidation and proportionate adoption. Tests run without external programs in PATH. doctor checks both embedded languages; real harness behavior requires a pilot.

The CI .github/workflows/dfd-kit.yml runs on push to main, on pull requests and manually: fmt and Clippy, mutants, and builds/tests for Linux x86_64 musl, macOS arm64/x86_64 and Windows x86_64, producing binary archives with checksums as artifacts. Local success does not certify other operating systems. The workflow runs from dfd-kit/.

## Releases

The dfd-kit domain is in release-preparation with CI required. A release comes from the feature that carries it:

1. update CHANGELOG.md and en/CHANGELOG.md, moving the [Unreleased] entries into a `## [X.Y.Z] - YYYY-MM-DD` section, and set version in Cargo.toml (with Cargo.lock) to X.Y.Z;
2. check with `python3 scripts/release_notes.py --check vX.Y.Z` and read the notes with `python3 scripts/release_notes.py vX.Y.Z`;
3. complete DFD pre-release and release review, with the real CI log of the PR recorded as kind ci;
4. after the merge, create and push the tag vX.Y.Z on the main commit.

The workflow .github/workflows/release.yml repeats the check, runs the full CI, then publishes the release with the four archives, a combined SHA256SUMS and the IT/EN notes. It never overwrites an existing release. A wrong release is fixed with a new patch version.

## Mutation testing

Python 3.9+ and Cargo suffice; no additional Python packages/cargo-mutants. Curated mutants cover risk, human review, fingerprints, evidence hashes, TDD chronology, CI, rollout, readiness, communications, schema versions, rapid/solo/selective review and integrated snapshots. They also cover status exit code and effective phase, DoD text of selected criteria read with the CommonMark parser, decision snapshots and changes, log markers, suite entrypoints and roots, per-criterion fingerprints and dfd exec.

```sh
python3 -m unittest discover -s tests -p 'test_mutation_runner.py'
python3 scripts/mutation_test.py
python3 scripts/mutation_test.py --only evidence-file-integrity
```

Default Cargo offline; --online fetches dependencies. --timeout 180 limits each Cargo invocation. The runner copies inputs under target/, mutates one fragment per isolated copy, and never edits original source/distributed binaries. On macOS/Linux timeouts terminate child processes. Build cache is target/mutation-build/. Baseline suite and selected test existence are checked first. Each source fragment must match exactly once; missing/ambiguous fragments fail. Run targeted tests first, then full suite before declaring a mutant survived.

JSON/logs default to target/mutation-results/; --output changes location. Reports include source/test/manifest hashes, results and score. killed means test failure; survived means full suite also passed. Compilation errors, missing tests and timeouts are distinct and do not count as kills. Exit 0 requires all selected mutants killed; failed baseline/incomplete runs invalidate results. Locks protect reports; remove after interruption only when no runner is active.

The score covers curated mutations only. Add id/file/original/replacement/observable behavior/at least one detecting test to mutations/gates.json. Prefer compilable mutants. Strengthen tests or justify equivalent survivors rather than removing them to improve scores. The mutations job in .github/workflows/dfd-kit.yml preserves artifacts even on failure.

## Contributions and framework resources

Describe problem, final behavior and actual validation; link issues and update templates/skills/reading paths together. Use concise imperative commits, such as feat: add brownfield adoption assessment. Existing local-history conventions are not assumed.

Root framework documents remain canonical; toolkit copies adopt revisions explicitly. Toolkit-specific template corrections do not overwrite framework originals. Domain lifecycle is development-only: traced final local suite, no simulated CI; distribution requires explicit policy transition/review reconfirmation.

For scoped noncritical fixes use quick with title/owner/domain and one record. Solo needs no PR/second reviewer. Enable optional team policy explicitly. Promote critical/uncertain changes, retaining origin. See contract for gates and integrated evidence.
