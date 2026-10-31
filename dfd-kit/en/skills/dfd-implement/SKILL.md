---
name: dfd-implement
description: Implement an approved DFD feature through TDD, refactoring and instrumentation, collecting real verifiable execution evidence.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-implement/SKILL.md) | [English](SKILL.md)

<a id="sviluppo-tdd-dfd"></a>

# DFD TDD development

Read `.dfd/kit/processo-e2e.md` (Development), `.dfd/kit/templates/contratto_toolkit.md`, specification, plan and domain guardrails. Follow adopting-repository instructions/tools. The harness agent implements and executes tests; the CLI does neither.

1. Check status. During development exit 2 with blocking ["development"] is expected until evidence is complete; design or setup in blocking must be resolved first. Design/plan must be current; create a missing plan or reconfirm changed design. Resolve conditions due during development before declaring completion.
2. For each task write/run the expected behavior or DoD constraint test and keep the relevant red failure. Implement minimally, obtain green, add required logs/metrics/traces, refactor and rerun. Syntax/environment failures do not prove the behavior. Record/clarify design questions before dependent work.
3. Save real logs under `.dfd/features/<id>/evidence/` or repository-local downloaded CI logs. Never invent results/dates/hashes. evidence.json references current design/plan, code/test/instrumentation SHA-256 hashes, red/green checks per task and final suite (suite for local, ci for real CI). Record command, timezone date, exit code, log/hash. Status exposes development design_fingerprint/plan_hash.
4. Document refactor/instrumentation in refactor_notes, explaining when unnecessary. Changed plans/files require reruns and refreshed evidence. In development-only use a final local suite; if release-preparation requires CI and none exists, keep the gate open. A justified local alternative needs explicit policy change and review reconfirmation.
5. Run verify and fix gaps without editing state to bypass gates. Resolved conditions due by development need the responsible human's updated decision; later conditions stay open for their phase.

Report implemented criteria, actual tests and residual gaps. development-complete ends local development; ready-for-pre-release permits preparation only, not release. Rapid uses one record: relevant red/green for behavior fixes, useful green checks for editorial changes without artificial red. Test failure effects, data loss, authorization, retry and compatibility. Suites/mutants are proportionate; new contracts/questions or critical risk require light/full promotion. Solo rapid needs no separate plan/second approval; apply independence only if configured.
