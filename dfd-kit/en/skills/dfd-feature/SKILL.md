---
name: dfd-feature
description: Open or resume a DFD feature from an idea, establishing domain, title, scope and change kind before risk assessment and specification.
---

<!-- dfd-language --> [Italiano](../../../skills/dfd-feature/SKILL.md) | [English](SKILL.md)

<a id="apertura-di-una-feature"></a>

# Opening a feature

First consider rapid for scoped known-contract fixes or editorial changes: use quick and one record instead of separate feature documents.

Read `.dfd/config.json` and `.dfd/kit/templates/contratto_toolkit.md`. Identify domain, problem, title, scope and kind (feature, refactor, incident, new-service). Clarify only what distinguishes the change; refine KPIs/scenarios/questions in the iterative specification.

Use `dfd status` to find existing work. Resume without duplicates or manual state.json changes. For new work choose a lowercase hyphenated ID:

```sh
dfd feature <id> --domain <domain> --title "..." --scope "..." --kind feature
```

The CLI creates initial state/risk without approvals or classifying unknown data. Register the domain with init if missing; Phase 0 may evolve during idea refinement but is required before design. Report ID/domain/scope and dfd-assess next. Do not reconfirm supplied information.

For rapid use `dfd quick` with ID/domain/title/owner and behavior-fix/editorial. Follow the contract for `.dfd/changes/<id>.json`, without separate specification/plan. Escalate via `dfd promote` to a new ID, preserving origin. Solo work needs no PR/second reviewer; context/team.json are optional.
