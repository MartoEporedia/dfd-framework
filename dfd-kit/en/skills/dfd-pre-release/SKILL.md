---
name: dfd-pre-release
description: Prepare a verified DFD feature for release: rollout, thresholds, rollback, alarms, dashboards, nonfunctional tests and real communications.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-pre-release/SKILL.md) | [English](SKILL.md)

<a id="preparazione-del-rilascio"></a>

# Release preparation

Read `.dfd/kit/processo-e2e.md` (Pre-release/Release DoR), `.dfd/kit/templates/contratto_toolkit.md`, feature/domain artifacts and status. Development must be verified/current: development absent from blocking (exit 2 means a blocked gate, not a command error). Domain must be release-preparation (legacy default without lifecycle.json). In development-only stop; explicitly update policy/reassess risk/CI and reconfirm invalidated artifacts before distribution.

Run pre-release to create/resume rollout.md/json without overwriting. Markdown uses the domain template; replace placeholders/examples with real values. Iterate with open_questions.

Define artifact/revision, environment, owner/on-call and justified direct/canary/segmented strategy. Stages describe final-environment exposure to 100% of intended audience; staging tests are separate evidence. Map every applicable criterion to production signal/threshold/verification from domain/review, never invented baselines.

Define rollback trigger/procedure/verification and alarms/dashboards/exposure control. Ready resources need local hashed evidence; planned resources need owner/reference/activation plan. Light exclusions require justification; full requires alarms/dashboards. A flag may be excluded with an alternative exposure mechanism. Evaluate load/resilience/security as executed or justified exclusions.

Record only communications actually made: audience, owner, date, reference. Prepare content and report pending work; never send messages/configure external services without relevant authorization. Installing a skill does not inform stakeholders.

After design/plan/evidence changes, reverify then use pre-release --refresh, reassessing artifact/thresholds/preparation. Run review-release, fix gaps and use dfd-review-release. Do not claim release execution. If team.json requires integrated checks, link collaboration.json to current revision and real snapshots/files/checks; isolated branches are insufficient and PR review must match examined files/revision. Do not impose team obligations on solo developers. Promote a quick record before distribution.
