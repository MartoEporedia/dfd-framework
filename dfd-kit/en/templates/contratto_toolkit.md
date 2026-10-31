<!-- dfd-language --> [Italiano](../../templates/contratto_toolkit.md) | [English](contratto_toolkit.md)

<a id="contratto-degli-artefatti--dfd-kit-01"></a>

<a id="comandi-locali"></a>

<a id="dominio-e-adozione"></a>

<a id="fase-0--setup-dfd-di-dominio"></a>

<a id="feature-e-rischio"></a>

<a id="design-e-decisioni"></a>

<a id="sviluppo-tdd-dfd"></a>

<a id="evidenze-di-esecuzione"></a>

<a id="gate-di-sviluppo"></a>

<a id="rollout-e-preparazione-operativa"></a>

<a id="review-di-release-e-decisioni"></a>

<a id="scritture-concorrenti"></a>

<a id="compilare-il-solo-dfdchangesfix-idjson-e-collegare-logfile-reali"></a>

<a id="se-la-policy-team-richiede-review-o-si-vuole-registrarne-una-realmente-espressa"></a>

<a id="se-emergono-rischi-o-quando-si-prepara-una-distribuzione"></a>

# Artifact contract – DFD Kit 0.1

## Local commands

Use the platform's dfd binary without additional runtime/packages. Run at product root or use `dfd --project <repo> <command>`; --project also works after the subcommand. Use the binary path if it is outside PATH. No command invokes an LLM. Toolkit development requires Rust/Cargo.

| Command | Purpose |
|---|---|
| `init --harness codex --domain checkout --mode brownfield` | Install and scaffold pilot domain |
| `setup --domain checkout` | Structurally check domain Phase 0 |
| `install --harness claude-code` | Switch adapter preserving state/team content |
| `feature 001-paypal --domain checkout --title "PayPal" --scope "New payment method"` | Create feature |
| `assess 001-paypal` | Classify risk.json |
| `specify 001-paypal` | Create/resume iterative specification without overwrite |
| `review-design 001-paypal` | Structural report/current design gate |
| `plan 001-paypal [--refresh]` | TDD plan from current approved design |
| `verify 001-paypal` | Check development evidence |
| `pre-release 001-paypal [--refresh]` | Prepare/resume rollout after verified development |
| `review-release 001-paypal` | Structural release checks/activations/gate |
| `decide 001-paypal --stage release ... --human-confirmed` | Separate human release decision |
| `doctor` | Check embedded skills/resources |
| `status [001-paypal]` | Resume work and display gaps |
| `decide 001-paypal --decision approved --reviewer "TL" --role tech-lead --note "Review completed" --human-confirmed` | Declare an expressed human decision |

Output is JSON. Exit 0 means command success; 1 means input/read error/conflict; 2 means incomplete assessment/setup or failed structural checks. Argument-parser errors also use 2. status returns 2 when a displayed gate is blocked (setup or design blocked, development blocked-plan or blocked-evidence, release blocked, quick record blocked) and lists it in blocking; it returns 0 when no gate is blocked, including while awaiting human review (awaiting-human-review, stale-review, changes-requested). With an ID blocking contains setup, design, development, release or change; without an ID it contains setup:<domain>, feature:<id>/<gate> and change:<id>. Scripts treat 2 as a blocked gate and 1 as a command error. review-design may succeed while awaiting human review: gate, not exit code alone, determines workflow state.

One active adapter avoids duplicate skills. install preserves features. `.dfd/kit/` is managed: updates reject locally modified resources rather than overwriting them. Existing AGENTS.md, CLAUDE.md and Copilot instructions are preserved.

## Domain and adoption

config.json records mode (brownfield/greenfield) and domains. adoption.json records pilot, current_level/target_level (0–3), six areas (process, observability, security, reliability, cost, culture), baseline_gaps and next_steps. Each area has status (unknown/gap/observed), evidence references and notes. assessment.md explains priorities/rationale/pilot work. Every JSON uses schema_version 1; unknown fields/unsupported versions are rejected.

Keep readable dod.md and structured criteria.json consistent:

```json
{"schema_version":1,"criteria":[
  {"id":"OBS-01","statement":"Error metrics on changed flow","scope":"changes"},
  {"id":"SLO-01","statement":"Define legacy flow SLOs","scope":"baseline"}
]}
```

Evaluate every changes criterion as applicable or excluded with justification. Link baseline criteria when relevant; unrelated historical debt does not automatically block changes. Design needs at least one applicable criterion.

## Phase 0 – Domain DFD setup

init prepares but does not complete setup. In `.dfd/domains/<domain>/`, maintain dod.md/criteria.json, guardrails.md (architecture/contracts/security/SLOs/observability/cost), process.md (naming/risk/checklists/artifacts/responsibilities/local differences) and eight local framework templates. Optional rapid/collaboration templates also exist.

setup returns phase domain-setup, gate blocked/ready, errors and next. It checks existence, nonempty content, unresolved setup placeholders and valid unique criteria consistent across catalog/DoD. Reusable-template placeholders are intentional. It neither judges semantic policy quality nor grants approval/adoption. status without ID keeps domains as names and adds setups; with ID it includes setup and directs incomplete domains to dfd-setup.

Phase 0 is shared across features, comparable to an SDD constitution. feature/assess can collect ideas/risk before setup; specify requires ready setup and uses local specifica_feature.md. Design rechecks setup. Update on architecture changes, recurring patterns and learning. Relevant normative guardrails/criteria affect fingerprints. New selective reviews exclude descriptive conventions/templates; legacy reviews keep conservative dependencies/history. Rerun init with the same mode/domain to add missing files without overwriting existing work. Common `.dfd/guardrails.md` remains available.

## Feature and risk

Features live in `.dfd/features/<id>/`. CLI-managed state.json records domain, kind, scope, phase, risk, route and decision history; never edit it to bypass gates.

risk.json requires security, reliability, cost, business and architecture, each with level low/medium/high/null and rationale, including Low. assess generates risk.md and fingerprints input; rerun after changes. Ordered rules: any High → high/full-complete; at least two Medium without High → medium/full-proportional; otherwise low/light. Missing facts stay unresolved. new-service always follows full-complete while retaining distinct risk classification. No automatic override.

## Design and decisions

specify iterates: when both spec.md/design.json exist it returns resumed true and open_questions without changing content/phase/decisions. A partial pair causes a preserving conflict. Update both artifacts after focused questions; questions block approval but permit drafting.

```json
{
  "schema_version":1,
  "owner":"team-checkout",
  "objective":"Enable the new payment method",
  "business_metrics":["Checkout completion"],
  "epic":{"reference":"ticket-123","approved_by":"PM","approved_at":"2026-01-01T09:00:00Z"},
  "criteria":[{"id":"OBS-01","applicability":"applicable","implementation":"Error counter","verification":"Metric emission test"}],
  "test_plan":"Unit and integration tests",
  "guardrails":"Existing payment policies",
  "open_questions":[]
}
```

Exclusions use applicability excluded and justification. Epic dates use nonfuture ISO 8601 with timezone. Structural checks authenticate neither approvals nor test execution/semantic quality.

Design reports are regenerable; semantic notes belong in review-notes.md. decide records only expressed human decisions; --human-confirmed is a declaration, not authentication. Roles: tech-lead, architect, delegated reviewer; solo responsibilities may coincide. For approved-with-conditions repeat `--condition "text|owner|stage"`. Conditions due during development require an updated human decision after resolution. pre-release/release/rollout/post-release conditions stay visible for their due phase; unknown deadlines are conservatively due during development. Resolve/reconfirm design conditions due by release before release approval.

New fingerprints cover specification, structured design, risk, selected criteria, guardrails, lifecycle, adoption mode, review notes and feature context. Legacy fingerprints additionally cover DoD, full catalog, detailed adoption/configuration and local process/templates. Changes cause stale-review, or blocked when new structural gaps appear. History remains. Locally editable artifacts provide no tamper-proof audit or automatic merge/deploy enforcement.

## Development (DFD TDD)

dfd-feature/plan/implement/verify guide opening, planning, harness execution and checks. The CLI invokes neither LLMs nor project commands; all four adapters share skills.

plan requires current approved/approved-with-conditions design, creates plan.md/plan.json/evidence.json and sets phase development. Initial tasks derive from applicable criteria; refine iteratively:

```json
{"schema_version":1,"design_fingerprint":"<current design fingerprint>","tasks":[
  {"id":"task-1","owner":"team-checkout","description":"Count PayPal errors","criteria":["OBS-01"],"implementation":"Emit error counter","verification":"Integration test"}
],"open_questions":[]}
```

Cover all applicable criteria; task IDs are unique, owner/description/implementation/verification required; excluded/unknown criteria rejected. Open questions prevent readiness but allow iteration. CLI checks references, not semantic Markdown/JSON equivalence. Repeated plan preserves files; partial artifacts cause preserving errors. After changed design reconfirm human review first, then plan --refresh: update design reference/phase while preserving tasks/Markdown/evidence. Reassess tasks and rerun affected checks; historical evidence is not current evidence.

### Execution evidence

The harness executes tests and records real logs. All paths are project-relative. evidence.json contains schema_version, current design_fingerprint/plan_hash, files with path/sha256, checks and refactor_notes. Example structure (replace hashes with actual values):

```json
{"schema_version":1,"design_fingerprint":"<current>","plan_hash":"<current>",
 "files":[{"path":"src/checkout.rs","sha256":"<SHA-256>"}],
 "checks":[
  {"task":"task-1","kind":"red","command":"cargo test checkout_metrics","executed_at":"2026-01-01T10:00:00Z","exit_code":1,"log":{"path":"proof/red.log","sha256":"<SHA-256>"}},
  {"task":"task-1","kind":"green","command":"cargo test checkout_metrics","executed_at":"2026-01-01T10:01:00Z","exit_code":0,"log":{"path":"proof/green.log","sha256":"<SHA-256>"}},
  {"task":null,"kind":"suite","command":"cargo test --locked","executed_at":"2026-01-01T10:02:00Z","exit_code":0,"log":{"path":"proof/suite.log","sha256":"<SHA-256>"}}
 ],"refactor_notes":"Refactoring and instrumentation completed; suite rerun"}
```

Record relevant code/test/instrumentation files, exact current hashes, nonempty logs, commands and nonfuture timezone dates. Each task requires a real expected-requirement red failure followed by passing green; final local/CI suite follows all greens. Required CI uses kind ci and actual CI logs, never relabeled local execution. Changes to verified files/logs/design/plan invalidate evidence.

verify regenerates development-review.md; complete/current evidence sets phase development-verified without moving back a pre-release or release-reviewed phase; failed reverification returns phase development from development-verified or any later phase. Recorded phase is the last step taken, not the gate. status <id> adds effective_phase, derived from current gates and never saved: idea (risk not classified), assessed (no specification), design (design not approved or to reconfirm), design-reviewed (development not started), development (development blocked), development-verified (development complete, pre-release not started), pre-release (release being prepared or to reconfirm), release-reviewed (release approved, also with conditions). Exit 0 means development complete or pre-release readiness by policy; gaps return 2. Hashes establish local integrity, not authenticity, actual execution, quality or semantic coverage. development-complete stops; ready-for-pre-release allows pre-release/review-release only.

## Pre-release

Requires current verified development, not merely a recorded phase. Creates local-template rollout.md and rollout.json, setting phase pre-release. Repeated invocation preserves content; a partial pair conflicts. --refresh updates development reference/phase while preserving the plan. Iterate open_questions and both documents. No command contacts stakeholders, configures external resources or deploys.

### Rollout and operational preparation

- Required: owner, artifact, revision, environment, strategy_rationale, on_call, rollback_trigger, rollback_procedure and rollback_verification. Semantic review verifies declared revision against final evidence; CLI does not query registries/remotes.
- Strategies: direct/canary/segmented/combined. Final-environment steps require unique id, audience, percentage 1–100, positive duration_minutes, promotion and stop. Percentages never decrease and end at 100% of intended audience. Direct uses one stage; canary/combined require at least two and partial initial exposure. Staging is separate evidence.
- Each applicable design criterion needs id/signal/threshold/verification and Markdown reference. Exclusions must not be reintroduced. Baselines/thresholds come from domain/review; structural checks do not certify adequacy.
- alarms/dashboards/feature_flag entries have name, owner, status, reference, activation_plan, justification and evidence. planned requires owner/reference/activation plan; ready requires owner/reference and current local path/sha256 evidence; excluded requires justification and no evidence. Alarms/dashboards must be declared and cannot be excluded for full. A flag may be excluded with a justified alternative exposure mechanism.
- Evaluate exactly load/resilience/security in non_functional_tests. applicable needs execution; excluded needs justification and execution null. Execution has command, nonfuture timezone executed_at, exit_code 0 and nonempty current hashed log. Review evaluates exclusions against risk/policy.
- communications contains at least one actual communication with audience/owner/informed_at/reference. CLI sends nothing and authenticates no reference. Missing communication remains work for authorized team tools.

Illustrative partial shape (complete every required field above for a real release):

```json
{"schema_version":1,"development_fingerprint":"<generated>","owner":"team-checkout",
 "artifact":"registry/checkout:revision-123","revision":"revision-123","environment":"production",
 "strategy":"direct","strategy_rationale":"Scoped change within guardrails",
 "steps":[{"id":"production","audience":"Intended tenants","percentage":100,"duration_minutes":30,"promotion":"OBS-01 within threshold","stop":"Stop on error increase"}],
 "criteria":[{"id":"OBS-01","signal":"Error count","threshold":"Within baseline for 30 minutes","verification":"Dashboard and metric test"}],
 "open_questions":[]}
```

## Release review and decisions

review-release regenerates release-review.md; semantic notes belong in release-notes.md. status.release is separate from design gate and development. release.checklist selects domain light/full; pending_activation shows planned resources.

release.gate: not-started, blocked, awaiting-human-review, stale-review, changes-requested, approved, approved-with-conditions. Structural gaps/missing preparation return 2; structurally ready returns 0 even while awaiting decision. Gate/conditions remain authoritative.

Default decide stage is design. --stage release records separate release-decisions.json with schema_version 1 and decisions, declared reviewer/role/note/conditions/human confirmation. RACI assigns Tech Lead/Architect or policy-delegated reviewer; other findings belong in notes. A checklist never automatically approves.

Approvals need structurally valid preparation. Planned resources prevent unconditional approved: use assigned activation conditions. Resolve design conditions due by pre-release/release and reconfirm first. changes-requested can be recorded on incomplete drafts while retaining blocked status/history.

Release decisions set phase release-reviewed without changing design history. Next actions include dfd-pre-release, dfd-review-release, resolve-release-conditions, rollout-with-conditions and rollout. Pending activation/conditions due before rollout require resolution; rollout/post-release conditions remain visible. Record a new expressed human decision after resolution; CLI never automatically closes conditions or executes rollout.

Development references include design fingerprint/latest decision, plan and evidence.json. Release fingerprints add rollout.md/json and semantic notes; local evidence hashes are rechecked. Changes may block/invalidate review. pre-release --refresh preserves history without transferring approval. Regenerable reports/recorded phase are excluded from fingerprints to avoid invalidation by checks alone. Rollout execution and post-release learning remain subsequent phases.

## Concurrent writes

Mutating commands acquire `.dfd/lock`; each file is atomically replaced. After forced interruption remove a stale lock only after confirming no active DFD command. Multi-file installation is not a transaction. Preflight errors preserve managed files.

<a id="policy-di-maturità-del-dominio"></a>

## Domain maturity policy

Optional `.dfd/domains/<domain>/lifecycle.json`:

```json
{"schema_version":1,"mode":"development-only","ci_required":false,"rationale":"Local prototype: CI premature; traced final local suite required"}
```

All fields required; unknown fields rejected. Modes: development-only/release-preparation. Nonblank rationale required; development-only requires ci_required false. In release preparation justify CI against risk/target: true requires ci evidence; false accepts local suite as a justified human-reviewed alternative. Missing policy retains legacy release-preparation with required CI. Malformed policy errors never silently fall back. Maturity is separate from adoption mode/level and risk route.

Adding/removing/changing lifecycle invalidates relevant review/plan/evidence; absent-file fingerprints remain compatible. Reassess policy/specification, record actual human decision, refresh plan and rerun affected checks. suite means actual final local execution, ci means actual CI; both require exit 0, valid nonfuture timestamp and nonempty current hashed log. Any invalid declared check blocks even if optional. Final suite and required CI follow all greens.

In development-only verify/status return development-complete; release.gate is not-applicable; pre-release rejects even with passing CI and creates no release artifacts. Release-preparation success gives ready-for-pre-release. Updated suite/lifecycle formats require the updated CLI.

<a id="percorso-rapido"></a>

## Rapid route

```sh
dfd quick fix-id --domain checkout --title "Restore expected behavior" --owner "Dev" --kind behavior-fix
# Complete only .dfd/changes/fix-id.json and reference real logs/files
dfd assess fix-id
dfd verify fix-id
# Only for an actually expressed review:
dfd decide fix-id --decision approved --reviewer "Reviewer" --role reviewer --note "PR reviewed" --human-confirmed
# Escalation or distribution:
dfd promote fix-id --to feature-fix-id
```

quick resumes without overwrite; IDs cannot collide with features. Kinds: behavior-fix/editorial. Record fields: schema_version 1, id/domain/title/owner/kind/problem/scope/known_contract/scope_bounded/changes_contract/critical/open_questions/risk/criteria/files/checks/verification_notes/collaboration/decisions. risk uses schema 1 and the same five dimensions; files/checks follow evidence formats. Scaffolding invents no risk/tests/review.

Rapid requires all Low, bounded scope, unchanged contract, noncritical behavior, no questions and known contract for fixes. Unknown gives route unknown; one Medium requires at least light; critical gives full-complete. Escalation cannot receive rapid approval. assess/review-design support quick records without extra reports. verify checks real files/logs/green and, for behavior fixes, relevant earlier failed red. Editorial has no artificial red. Required CI must be real and after green; no extra full suite when policy does not require it.

Solo valid evidence gives change-verified without a new human approval or synthetic decision. independent_review requires PR/current revision and an actual reviewer different from owner; until then awaiting-human-review. Recorded reviews bind record/criteria/policy/file fingerprints; changes yield stale-review or blocked evidence. Conditional approvals require light/full promotion. Recording any decision always requires human-confirmed.

status lists changes alongside features; a blocked quick record makes status blocking (blocking ["change"], exit 2). Quick gates: blocked, awaiting-human-review, stale-review, changes-requested, change-verified. verify returns 0 only for change-verified, else 2; invalid input remains 1. Quick completion never approves release. promote creates a distinct feature and origin.json with source/hash, preserving record and copying risk/context. critical conservatively escalates reliability to High for reassessment. Quick decisions never transfer as design/release approval.

<a id="contesto-individuale-e-team"></a>

## Solo and team context

No team.json required for solo work. Optional domain policy has schema_version 1, independent_review and integrated_checks_required booleans. Missing means false/false; malformed values error.

```sh
dfd context feature-id --owner "Alice" --branch "fix/alice-123" --revision "integrated-commit" --issue "issue-123" --pull-request "pr-123"
dfd decide feature-id --decision approved --reviewer "Bob" --role reviewer --note "Current PR reviewed" --reference "pr-123" --file src/flow.rs --file tests/flow.rs --human-confirmed
```

context writes feature collaboration.json or quick collaboration. Fields: schema_version/owner/branch/revision/issue/pull_request/integration (null or object). Revision updates leave specification unchanged but reviews/evidence must match. decide optionally records reference and repeated file snapshots with revision. One applicable PR decision avoids duplicate approval; identities remain declared.

For light/full team policy is checked before pre-release/release review. Independence requires different reviewer, PR/current revision and snapshot covering every evidence file. Integrated checks require matching integration.revision, current files covering verified files, and valid suite/ci checks/logs/dates; ci when lifecycle requires CI.

```json
{"revision":"integrated-commit","files":[{"path":"src/flow.rs","sha256":"<hash>"}],"checks":[{"kind":"suite","task":null,"command":"test integrated version","executed_at":"<ISO 8601 date>","exit_code":0,"log":{"path":"proof/integration.log","sha256":"<hash>"}}]}
```

CLI does not query Git/GitHub, authenticate revisions or certify interaction coverage; semantic review evaluates origin/adequacy. A local lock coordinates one directory; Git handles clones/conflicts without a shared per-change central registry.

<a id="fingerprint-selettivi-e-compatibilità"></a>

## Selective fingerprints and compatibility

New features use selective_review true. Specification/design/risk, selected criteria (catalog statement and the dod.md text citing them), domain/common guardrails, lifecycle and greenfield/brownfield context remain dependencies. For each selected criterion the dod.md blocks citing its ID count: list item with continuation lines and sub-items, table row, heading or paragraph; a line starting with an ID opens a new block; a heading citing a criterion also binds its whole section up to the next heading of the same or higher level; whitespace is normalized. If a selected criterion appears only as a bare ID, without descriptive text in its block, the whole dod.md becomes a dependency. A block citing several criteria, such as an index, binds them all. Changing a selected criterion's DoD text requires reconfirming the review; unselected criteria and whitespace-only changes do not. Selective reviews approved with an earlier CLI become stale-review once and must be reconfirmed. Descriptive process/templates, detailed adoption, unreferenced baseline criteria and other registered domains do not automatically invalidate design. Normative requirements belong in the fingerprinted catalog, criterion text in dod.md or guardrails/policy. Adding an unassessed changes criterion still blocks.

Legacy features lacking selective_review default false with prior fingerprints. Use --legacy-review for new conservative features; never manually migrate decisions. Reviews/code/integration remain snapshot-bound: selectivity does not permit stale evidence. New commands/formats require updated CLI.

<a id="lingua-del-progetto"></a>

## Project language

`dfd --lang en init ...` selects and persists English on initialization; it/en are supported. `dfd --lang en install --harness codex` persists the selected language while updating managed resources. `dfd language it` or `dfd language en` changes the preference and managed resources with the current adapter. File `.dfd/language.json` contains `{"schema_version":1,"language":"en"}`; absence means Italian. Invalid preferences fail explicitly.

For other commands --lang is an invocation-only override. Help, CLI-owned messages and newly generated resources use that language. Commands/keys/enums/IDs/exit codes and user text remain stable. Switching language preserves existing domain/feature documents and local templates, including their original language. To change an existing domain's template, explicitly review/edit it; new domains use the newly selected language. Conflicts are preflighted before resources/preference change. Atomicity remains per file.
