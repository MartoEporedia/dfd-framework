# Repository Guidelines

## Project Structure & Module Organization

This repository documents the DoD-First Driven (DFD) framework, primarily in Italian.

- `README.md` provides reading paths by contributor role.
- `fondamenta.md`, `processo-e2e.md`, `rischio.md`, and `raci.md` define principles, lifecycle, risk classification, and responsibilities.
- `templates/` contains reusable feature specifications, extended Definition of Done documents, design/release checklists, and rollout plans. Full and light reviews are separate files.
- `examples/` contains checkout documentation illustrating the framework.

The repository root contains framework documentation. `dfd-kit/` contains the Rust CLI, harness skills, automated tests and tooling; follow [DFD Kit contribution guidelines](dfd-kit/CONTRIBUTING.md) for its development. Keep reusable framework forms in `templates/` and concrete worked examples in `examples/`.

## Development & Validation Commands

Framework documentation requires no build step or local server. Edit Markdown directly and preview it in your editor or repository viewer. For the CLI and skills, use the validation commands in `dfd-kit/CONTRIBUTING.md`.

- `rg --files -g '*.md'`: list documentation files.
- `rg -n 'OBS-|SEC-|SLO-|COST-|UX-' templates examples`: inspect criterion references across artifacts.
- `git diff --check`: check patch whitespace when working in a Git checkout; preserve intentional Markdown hard breaks.

## Documentation Style & Naming Conventions

Write framework content in Italian and preserve established DFD terminology. Use one `#` title, hierarchical `##`/`###` headings, and numbered sections where neighboring documents use them. Indent nested bullets by two spaces; use `- [ ]` for checklist items and fenced blocks with language identifiers for examples.

Follow existing lowercase filenames, such as `review_design_light.md` and `processo-e2e.md`. Preserve metadata blocks for status, dates (`YYYY-MM-DD`), owners, and references. Keep criterion identifiers stable, such as `OBS-01` and `SEC-02`. No formatter or linter is configured.

## Testing Guidelines

The root documentation has no automated test framework or coverage threshold; DFD Kit has Rust workflow tests and mutation tests. Before submitting documentation, preview changed documents, verify relative links and heading anchors, and check consistency between principles, templates, and examples. Resolve links against actual filenames: the extended DoD template is `templates/dod-estesa.md`. Clearly distinguish template placeholders from completed example values.

## Commit & Pull Request Guidelines

Local Git history is unavailable, so an established commit convention cannot be verified. Use concise, imperative messages, for example `docs: clarify light review criteria`.

Describe the problem, affected documents, and validation performed in each PR. Link related issues when available. Update dependent templates, examples, and README reading paths together when changing framework behavior; explain any changes to risk rules or responsibilities.

## DFD obbligatorio per CLI e skill

Dal 2026-10-09, tutti gli sviluppi della CLI e delle skill in `dfd-kit/` devono seguire DFD, inclusi bugfix, refactor e modifiche agli artefatti di supporto necessari al cambiamento.

- Riprendere gli artefatti DFD esistenti e verificare lo stato con la CLI. Se manca il setup del dominio, completare prima la Fase 0: DoD, guardrail, template locali e convenzioni di processo.
- Aprire o riprendere una feature con ambito esplicito, classificare il rischio e scegliere il percorso light/full previsto dal framework.
- Definire la specifica iterativamente, chiarendo i dubbi e collegando criteri DoD, test e vincoli; completare la review di design prima dello sviluppo.
- Derivare il piano dal design approvato, sviluppare con TDD e raccogliere evidenze reali. Eseguire le verifiche e i mutation test pertinenti al cambiamento.
- Preparare la pre-release e la review di release quando il cambiamento è destinato al rilascio. Aggiornare gli artefatti e riconfermare i gate invalidati dalle modifiche.
- Registrare le decisioni umane solo quando espresse; riusare quelle già pertinenti nella conversazione. Un controllo automatico positivo non sostituisce la review umana.

Il percorso operativo e i comandi sono descritti in [CONTRIBUTING.md](dfd-kit/CONTRIBUTING.md) e nel [contratto del toolkit](dfd-kit/templates/contratto_toolkit.md). Il setup e gli artefatti delle feature devono vivere nel repository, senza dipendere dalla sola cronologia della chat.
