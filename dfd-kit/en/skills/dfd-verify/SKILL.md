---
name: dfd-verify
description: Verify DFD development evidence, task coverage and current file/log integrity before pre-release.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-verify/SKILL.md) | [English](SKILL.md)

<a id="verifica-dello-sviluppo"></a>

# Development verification

Read `.dfd/kit/templates/contratto_toolkit.md`, specification, plan and evidence.json. Run `dfd verify <id>`: development-review.md is generated; phase advances only for complete/current artifacts.

Review semantics too: red must fail for the expected requirement; green tests the associated criteria; final local/CI suite per lifecycle.json must cover the final refactored/instrumented version. Hashes check local integrity, not log authenticity, semantic coverage or code quality.

Missing tests/instrumentation → dfd-implement; incomplete plan → dfd-plan; changed design → reconfirm review. Preserve valid results and rerun invalidated checks. Never declare unexecuted CI passing. Report blocked-plan, blocked-evidence, development-complete or ready-for-pre-release and relevant conditions. Stop at verified development for development-complete. Otherwise prepare pre-release then review-release; rollout execution comes later.

Quick records with valid relevant evidence yield change-verified without a new human gate under solo policy; independence requires the real current PR decision. Verify behavioral red reproduces the bug, not environment problems. Editorial changes need no artificial red. Quick completion never approves distribution.
