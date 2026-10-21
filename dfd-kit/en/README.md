<!-- dfd-language --> [Italiano](../README.md) | [English](README.md)

<a id="sviluppare-questo-toolkit-con-dfd"></a>

<a id="provare-il-toolkit"></a>

<a id="completare-dod-guardrail-e-convenzioni-nella-cartella-del-dominio"></a>

<a id="apertura-e-sviluppo-delle-feature"></a>

<a id="raffinare-planmd-e-planjson-usare-dfd-implement-nellharness"></a>

<a id="eseguire-il-ciclo-tdd-salvare-log-e-compilare-evidencejson"></a>

<a id="pre-release-e-review-di-release"></a>

<a id="completare-rolloutmd-e-rolloutjson-con-la-skill-dfd-pre-release"></a>

<a id="dopo-la-decisione-del-responsabile-umano"></a>

<a id="organizzazione"></a>

<a id="sviluppo-prima-della-ci"></a>

<a id="fix-rapidi-singolo-sviluppatore-e-team"></a>

<a id="scegliere-la-lingua"></a>

# DFD Kit

DFD treats the Definition of Done as the continuous contract between specification, tests and production validation. DFD Kit brings that workflow to Claude Code, OpenCode, GitHub Copilot and Codex through shared skills and repository artifacts.

## Develop this toolkit with DFD

The dfd-kit domain is adopted in brownfield/development-only mode. Resume [domain process](../.dfd/domains/dfd-kit/process.md), [DoD](../.dfd/domains/dfd-kit/dod.md) and [assessment](../.dfd/domains/dfd-kit/assessment.md) following [CONTRIBUTING](CONTRIBUTING.md). Setup is structurally ready; CI is deferred until release evaluation and real harness pilots remain unverified.

## Try the toolkit

The Rust executable embeds skills/templates; users need no Python, Node, jq or other runtime. Version 0.1 covers adoption, domain setup, risk, specification, design review, TDD development, pre-release and release review. Rollout execution/post-release learning are later phases.

The initial local macOS arm64 build is under dist/dfd-macos-arm64. ci/github-actions.yml prepares other builds; activate by copying to repository-root .github/workflows/. Builds are not published releases. Put the correct binary in PATH as dfd or invoke its path; macOS/Linux may need chmod +x, Windows uses dfd.exe.

```sh
dfd doctor
dfd --lang en --project /path/product init --harness codex --domain checkout --mode brownfield
# Complete domain DoD, guardrails and conventions
dfd --project /path/product setup --domain checkout
dfd --project /path/product feature 001-paypal --domain checkout --title "PayPal" --scope "New payment method"
```

init installs twelve skills: dfd-init, dfd-setup, dfd-feature, dfd-assess, dfd-specify, dfd-review-design, dfd-plan, dfd-implement, dfd-verify, dfd-pre-release, dfd-review-release, dfd-status. Phase 0 is the shared reference: DoD, guardrails, local templates and conventions. Complete adoption/setup under .dfd/domains/<domain>/ and run setup. Idea/risk may precede setup; specify requires readiness. Unrelated baseline debt stays in adoption planning. Skills guide the harness agent; CLI checks deterministically.

| Harness | Option | Installation directory |
|---|---|---|
| Claude Code | --harness claude-code | .claude/skills/ |
| OpenCode | --harness opencode | .opencode/skills/ |
| GitHub Copilot | --harness copilot | .github/skills/ |
| Codex | --harness codex | .agents/skills/ |

Use each harness's skill syntax, e.g. /dfd-init or $dfd-init. Installation tests are distinct from real discovery/invocation pilots. One adapter is active per repository; install switches without losing features and rejects modified managed files. Existing instructions remain preserved. Brownfield begins with 1–2 domains/3–5 changes; see [adoption](adozione.md). Approval is always separate from structural checks and becomes stale after relevant changes.

## Choose the language

Use --lang en on init for a new English project; Italian remains the legacy default. `dfd language it` / `dfd language en` persist a switch, reinstalling managed resources with the active adapter. On install, --lang also persists; on other commands it overrides one invocation. Preference is .dfd/language.json. Existing documents/local templates retain their content and language; new domains use the selected language. Explicitly adapt an existing domain's templates when changing its working language. Commands, keys, enum values, IDs, exit codes and user text remain unchanged.

## Open and develop features

Use dfd-feature to establish title/domain/scope/kind, resuming existing work. specify creates/resumes spec.md/design.json without overwriting and exposes open_questions. Clarify/update both iteratively; questions block approval, not drafting. Relevant normative setup changes require reconfirmation; descriptive template/process updates do not invalidate new selective reviews. Legacy reviews stay conservative. Rerun init with the same mode/domain to add missing setup files without replacement.

After current human design approval:

```sh
dfd plan 001-paypal
# Refine plan.md/json, use dfd-implement, save real red/green logs and evidence.json
dfd verify 001-paypal
dfd status 001-paypal
```

The CLI checks criteria/task coverage, design/plan references, outcomes and file/log hashes; it neither runs project commands nor authenticates claimed results. development-only closes with a traced final local suite; CI is required only by policy. Design/code/plan/log changes require relevant reverification; after renewed approval plan --refresh preserves tasks while updating references. ready-for-pre-release is structural readiness, not approval. Use the updated binary/install command for updated skills.

## Pre-release and release review

With current verified development, dfd-pre-release prepares artifact/revision, exposure stages, promotion/stop, rollback, on-call, alarms and dashboards through the local template. Clarify questions iteratively. Record relevant nonfunctional tests and communications actually made.

```sh
dfd pre-release 001-paypal
# Complete rollout.md/json
dfd review-release 001-paypal
# Only after the responsible human expresses approval:
dfd decide 001-paypal --stage release --decision approved --reviewer "TL" --role tech-lead --note "Release reviewed" --human-confirmed
dfd status 001-paypal
```

Use domain light/full review checklist. Planned resources require explicit activation plans and approved-with-conditions. release.gate/history are separate from design/development. After changes reverify and pre-release --refresh, preserving plans; reconfirm invalidated reviews. The CLI prepares/checks artifacts; external resource configuration, communications and rollout require appropriate authorization.

## Organization and maturity

Repository-root framework documents remain canonical. Toolkit copies adapt them for embedded workflow. Templates/examples/skills are embedded. Development-only concludes with development-complete and no pre-release. Explicit release-preparation transition reassesses risk/CI and invalidated review/plan/evidence. See the [contract](templates/contratto_toolkit.md).

## Quick fixes, solo developers and teams

For bounded fixes use quick with ID/domain/title/owner, one record, justified risk and relevant regressions. Editorial changes need useful checks without artificial red. Solo work requires no PR/independent reviewer; [team policy](templates/collaborazione.md) is optional. Critical changes/questions require light/full; promote preserves origin. See [contract](templates/contratto_toolkit.md#percorso-rapido).
