<!-- dfd-language --> [Italiano](../harness-proposta.md) | [English](harness-proposta.md)

<a id="dfd-kit--integrazione-negli-harness-di-coding"></a>

# DFD Kit – Coding harness integration

> **Status**: MVP 0.1 implemented; real pilot validation pending  
> **Date**: 2026-10-08  
> **References**: [Foundations](fondamenta.md), [Process](processo-e2e.md), [Risk](rischio.md), [RACI](raci.md).

<a id="1-obiettivo-e-primo-utilizzatore"></a>

## 1. Objective and first user

Help a team define domain DoD, classify/specify features, derive tests/checks, prepare release and record outcomes in its repository. Initial user: a Tech Lead/developer adopting DFD in an existing product, including solo developers combining clearly assigned RACI roles. Harnesses supply models/tools/permissions; DFD supplies shared skills/templates/deterministic checks, without a separate runtime or LLM service.

<a id="2-ipotesi-di-adozione"></a>

## 2. Adoption hypothesis

Preliminary adoption risk is Medium: actual use is unproven and reviews require new habits. Hypothesis: guided work reduces coordination more than it adds documentation. Validate one real feature, measuring time, manual intervention, detected gaps and artifacts actually used in review. Defer dashboards, ticket integrations, multi-agent orchestration and automated deployment until the core workflow proves useful.

<a id="3-esperienza-proposta"></a>

## 3. Proposed experience

CLI covers init/install/setup/feature/assess/specify/review-design/decide/plan/verify/pre-release/review-release/status/doctor, plus quick/promote/context and language selection. dfd-implement guides harness TDD; rollout execution and learn remain future work. Stable skill names use harness-specific invocation. Preserve business goals/KPIs/epic approval; repeated commands update/resume without losing decisions. See the [command contract](templates/contratto_toolkit.md).

<a id="4-nucleo-comune-e-adapter"></a>

## 4. Shared core and adapters

One canonical SKILL.md workflow per phase feeds four adapters. Load only phase-relevant resources. Installer tracks managed files, rejects local conflicts and avoids duplicate compatibility-directory skills. One active adapter per repository; changing adapter preserves state. Discovery/invocation/resource access require real version/surface pilots, distinct from installation tests. Hooks/specialized packaging are future work.

| Harness | Project skill directory | Reference |
|---|---|---|
| Claude Code | .claude/skills/name/SKILL.md | [Skills](https://code.claude.com/docs/en/skills) |
| Codex | .agents/skills/name/SKILL.md | [Skills](https://learn.chatgpt.com/docs/build-skills) |
| OpenCode | .opencode/skills/name/SKILL.md, compatible .agents/.claude | [Skills](https://docs.opencode.ai/docs/skills/) |
| GitHub Copilot | .github/skills/name/SKILL.md, compatible .agents/.claude | [Skills](https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/customize-cloud-agent/add-skills) |

The Rust binary embeds skills/templates/documents. cli.rs defines adapters; model.rs defines versioned schemas; store.rs owns persistence. Preserve existing AGENTS.md/CLAUDE.md/Copilot policy. Agents propose/interpret, validators check structure/references/prerequisites, humans make RACI decisions. Agent reports do not approve work.

The adopting .dfd/ contains config, common guardrails, domains with DoD/catalog/guardrails/process/templates/adoption/assessment, and separate feature folders with state, risk, specification, design review, plan, development evidence/review, rollout, release review/decisions/notes and eventual learning. Markdown carries readable context; versioned JSON carries verifiable references. See [contract](templates/contratto_toolkit.md).

<a id="5-tracciabilità-e-gate"></a>

## 5. Traceability and gates

DoD criterion → specification → task/test → evidence → release validation. Applicable criteria need checks or reviewed justified exclusions. Evidence records origin, command/procedure, result, date and revision; distinguish automatic/manual/missing. Links alone do not prove execution; checkmarks do not prove requirements. Changed specifications/revisions invalidate affected evidence/review. Conditions retain owner/due phase. MVP gates guide work; merge/deploy enforcement requires later CI integration.

<a id="6-mvp-e-criteri-di-accettazione"></a>

## 6. MVP acceptance

- Initialization preserves existing files/instructions and is repeatable.
- Features have isolated artifacts/state and resume across harnesses from .dfd, without chat history.
- Installer avoids duplicates and rejects modified managed files.
- Risk classification is reproducible and leaves unknown data unresolved.
- Specifications reference existing domain criteria.
- Missing prerequisites prevent passing gates.
- Human decisions remain distinguishable from agent proposals.
- Pilot completes an actual review and records manual work.

The initial milestone was design review; planning/TDD verification and pre-release/release review are implemented. Real harness pilot and portability remain to validate; rollout execution/learning follow later.

<a id="7-chiarimenti-del-framework-prima-dellautomazione"></a>

## 7. Clarifications before automation

Risk prose overlaps at one Medium. Ordered precedence is any High → full complete, at least two Medium → full proportionate, otherwise light. Team escalations are justified decisions, not automatic overrides. Transaction-cost growth 5% to below 10% needs clarification/policy before automatic classification. DoD links use templates/dod-estesa.md. Feature template and Checkout DoD are distinct; PayPal is illustrative, not test/production evidence.

<a id="8-riferimento-esterno"></a>

## 8. External reference

[Spec Kit](https://github.com/github/spec-kit) illustrates toolkit distribution through processes/templates/agent integrations. DFD develops its own DoD/risk/evidence/post-release chain; direct reuse/extension remains an option to evaluate against requirements and adapter maintenance.
