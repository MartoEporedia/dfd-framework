---
name: dfd-status
description: Resume DFD work across harnesses or sessions, inspecting phase, risk, decisions, blockers and the next action.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-status/SKILL.md) | [English](SKILL.md)

<a id="riprendere-il-workflow-dfd"></a>

# Resuming the DFD workflow

Run `dfd status [<id>]` at the adopting root. Without ID it lists domains/setups/features/quick changes; with ID it checks design and related gates. Use `.dfd/kit/templates/contratto_toolkit.md` to interpret fields.

- State lives in .dfd, not chat history. Read relevant domain/feature artifacts only.
- Incomplete Phase 0 → dfd-setup, displaying concrete gaps.
- Missing risk → dfd-assess; missing specification → dfd-specify; review-ready design → dfd-review-design.
- Current approved design → dfd-plan, then dfd-implement/dfd-verify by development gate.
- development-complete ends development-only; never propose pre-release. Distribution requires explicit policy transition/reassessment/reconfirmation; release.gate not-applicable reflects that separation.
- ready-for-pre-release → dfd-pre-release, then dfd-review-release.
- release shows gate/history/checklist/planned activations/conditions. Distinguish rollout, rollout-with-conditions and resolve-release-conditions without claiming rollout execution.
- Distinguish blocked, awaiting-human-review, stale-review, changes-requested, approved and approved-with-conditions. Show conditions/owners/due phase.
- Structural success is not approval; old decisions do not approve changed artifacts. Never edit fingerprints or invent evidence to advance.
- For brownfield report current/target adoption and relevant baseline gaps. Invoking a skill does not increase adoption level.

Report phase, concrete blocker and next step. development distinguishes plans/checks, never completed release. Route/gate determine quick versus feature work. Solo work needs no collaboration.json/team.json. Verified rapid ends development; promote before distribution and reassess release needs. Never automatically resolve Git conflicts in evidence/decisions.
