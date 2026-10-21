---
name: dfd-review-release
description: Review DFD release preparation with light/full checklists and record the expressed human decision on rollout, evidence, thresholds and conditions.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-review-release/SKILL.md) | [English](SKILL.md)

<a id="review-di-release"></a>

# Release review

Read `.dfd/kit/templates/contratto_toolkit.md`, `.dfd/kit/raci.md` and feature artifacts. Use domain review_release_light.md for light, review_release.md for full.

Run review-release; release-review.md is regenerable and shows structural checks/gate/planned activations. Semantically review revision, CI, nonfunctional tests, DoD, thresholds, policy, rollback, on-call and communications. Examine actual evidence content: hashes do not authenticate origin. Save findings in release-notes.md.

Present proposed outcome/conditions. Record only an expressed human decision, reusing relevant existing conversation authorization:

```sh
dfd decide <id> --stage release --decision approved --reviewer "..." --role tech-lead --note "..." --human-confirmed
```

Use approved-with-conditions and repeated `--condition "text|owner|stage"` for conditions. Planned resources require conditional approval with assigned activation; ready resources need evidence. Resolve design conditions due by release and record the updated human decision first. Release history is separate from design.

Rerun status. Changed rollout/notes/design/design decision/evidence invalidate review; structural gaps block it. Conditions do not close automatically: obtain the updated human decision after resolution. Report outcome, conditions and next action. The CLI records review, never executes rollout.

Assess lifecycle CI policy against risk/target, including justified local alternatives when ci_required false. development-only cannot approve release. Evaluate origin/completeness of required integrated evidence, current snapshots and PR review. The CLI authenticates neither identities nor Git revisions. Serious escaped bugs need effective regression tests and systemic DoD updates; raw test counts do not prove release safety.
