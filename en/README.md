<!-- dfd-language -->
[Italiano](../README.md) | [English](README.md)

<a id="come-leggere-questo-repo-per-ruolo"></a>

<a id="progetti-ancora-in-sviluppo"></a>

<a id="processo-proporzionato-anche-per-un-solo-dev"></a>

# DFD Framework

DoD-First Driven connects iterative specifications, tests and production validation through verifiable Definition of Done criteria and risk-proportionate reviews.

<a id="installare-dfd-con-un-agente-qualsiasi-harness"></a>

## Installing DFD with an agent (any harness)

Paste the following prompt into your coding agent (Claude Code, Codex, OpenCode, GitHub Copilot, Cursor, Gemini CLI, Windsurf, Cline or others), opened in the repository adopting DFD. The agent obtains the `dfd` CLI, installs the skills in the harness format and stops before Phase 0, which is completed with `dfd-init`.

```text
Install the DFD (DoD-First Driven) framework in this repository.
Source: https://github.com/MartoEporedia/dfd-framework (toolkit in dfd-kit/).

Rules: do not overwrite existing files (AGENTS.md, CLAUDE.md, Copilot instructions,
harness rules); do not modify managed files in .dfd/ or in the installed skill
directories; ask me before installing system-wide software; do not invent DoD,
SLOs or thresholds.

1. Context. Identify the harness you are running in, the operating system and
   architecture. Ask me only what you cannot infer:
   - pilot domain (e.g. checkout);
   - mode: brownfield if code or processes already exist, otherwise greenfield;
   - language: en or it (Italian is the CLI default).

2. dfd CLI. Use `dfd` if it is already on PATH. Otherwise clone the repository
   into a temporary directory, outside this project:
   - macOS arm64: use dfd-kit/dist/dfd-macos-arm64, verify its checksum against
     dfd-kit/dist/SHA256SUMS and make it executable (chmod +x);
   - other systems: build with `cargo build --locked --release` in dfd-kit/
     (requires Rust; the binary is target/release/dfd, dfd.exe on Windows).
   Copy the binary to a PATH directory as `dfd`, or note its absolute path and
   use it in the following commands. Run `dfd doctor`.

3. Adapter. Choose the native adapter if the harness is supported:
   claude-code (.claude/skills/), opencode (.opencode/skills/),
   copilot (.github/skills/), codex (.agents/skills/).
   For any other harness use codex: it installs the skills in .agents/skills/,
   the most portable format.

4. Init. Run:
   dfd --lang <language> --project <repo root> init --harness <adapter> --domain <domain> --mode <mode>
   If a managed file conflicts, stop and show me the conflict.

5. Non-native harnesses. If you used codex as a fallback, make the twelve skills
   (dfd-init, dfd-setup, dfd-feature, dfd-assess, dfd-specify, dfd-review-design,
   dfd-plan, dfd-implement, dfd-verify, dfd-pre-release, dfd-review-release,
   dfd-status) discoverable through your harness's native mechanism (rules,
   commands, workflows or instruction files): prefer references to
   .agents/skills/<skill>/SKILL.md over copies; if the harness requires copies,
   state that .agents/skills/ is the source and that they must be regenerated
   after `dfd install`. Append instructions to existing files without rewriting
   them.

6. Verify and report. Check that .dfd/ and the skills exist and that your harness
   sees them. Report: binary and version used, adapter, files created or
   modified, any limitations. Tell me how to invoke the first skill in your
   harness (e.g. /dfd-init, $dfd-init) to complete Phase 0: assessment, DoD,
   guardrails and domain conventions, verified with `dfd setup --domain <domain>`.
   Do not start Phase 0 without my confirmation.
```

See [DFD Kit](../dfd-kit/en/README.md) for adapters, language and switching harness (`dfd install --harness <adapter>`).

## Reading paths by role

### Product Manager / Product Owner

1. [Foundations](fondamenta.md): central idea and rationale.
2. [End-to-end process](processo-e2e.md): product participation, especially Idea/Epic and Post-release.
3. [Risk criteria](rischio.md): choose full/light.
4. [Feature specification](templates/specifica_feature.md): objectives, KPIs and user/business impact.

Focus: business goals, KPIs, priorities, UX and product metrics.

### Tech Lead / Architect

1. [Foundations](fondamenta.md).
2. [Process](processo-e2e.md): phases/review gates.
3. [RACI](raci.md): responsibilities.
4. [Risk](rischio.md): proportionate rigor.
5. [Extended DoD](templates/dod-estesa.md) and [Checkout example](examples/checkout_dod.md).
6. [Feature specification](templates/specifica_feature.md) and [PayPal example](examples/checkout_paypal_spec.md).

Focus: architecture, guardrails, security, SLOs and DoD evolution.

### Developer

1. [Foundations](fondamenta.md): DoD as contract.
2. [Process](processo-e2e.md): phase expectations.
3. [Specification](templates/specifica_feature.md).
4. [Design review](templates/review_design.md) and [light version](templates/review_design_light.md).
5. [Extended DoD](templates/dod-estesa.md).

Focus: DFD TDD, logs/metrics/traces and DoD compliance.

### QA / SDET

1. [Foundations](fondamenta.md).
2. [Process](processo-e2e.md): development, pre-release and post-release tests.
3. [Design](templates/review_design.md) and [release](templates/review_release.md) review.
4. [Extended DoD](templates/dod-estesa.md): derive tests from OBS-xx, SEC-xx, SLO-xx and COST-xx.
5. [PayPal example](examples/checkout_paypal_spec.md).

Focus: functional/nonfunctional tests, resilience, security and production validation.

### Ops / SRE

1. [Foundations](fondamenta.md).
2. [Process](processo-e2e.md): pre-release/rollout responsibilities.
3. [Rollout configuration](templates/configurazione_rollout.md).
4. [Extended DoD](templates/dod-estesa.md): align alarms/dashboards.
5. [Checkout example](examples/checkout_dod.md).

Focus: alarms, dashboards, canaries, rollout and SLO/observability incidents.

### Security Engineer

1. [Foundations](fondamenta.md).
2. [Risk](rischio.md): high-security-risk features.
3. [Extended DoD](templates/dod-estesa.md): SEC-xx.
4. [Design](templates/review_design.md) and [release](templates/review_release.md) review.
5. [PayPal example](examples/checkout_paypal_spec.md).

Focus: access, audit, data protection, compliance and security review.

### Support / Customer Success

1. [Foundations](fondamenta.md).
2. [Process](processo-e2e.md): pre-release/post-release participation.
3. [Risk](rischio.md): user impact.
4. [PayPal example](examples/checkout_paypal_spec.md).

Focus: feedback, post-release reports, UX and business KPIs.

## Projects still in development

Development-only supports iterative specifications, reviews and traced local checks while CI is premature. Switch explicitly to release preparation when distributing, reassessing risk, CI and operational requirements. See [foundations](fondamenta.md#maturità-del-progetto-solo-sviluppo-e-preparazione-al-rilascio) and [process](processo-e2e.md#maturità-del-progetto-solo-sviluppo-e-preparazione-al-rilascio).

## Proportionate process, including solo developers

Choose rapid for scoped known-contract fixes, light for contained work needing specification, full for higher risk. Rapid uses [one record](templates/cambiamento_rapido.md), red/green for bugs and relevant editorial checks. Team PR/independence/integration requirements come from [optional policy](templates/collaborazione.md). See [risk](rischio.md#percorsi-proporzionati-rapido-light-e-full) and [DFD Kit](../dfd-kit/en/README.md).

## Language and contribution

Read the complete framework in either language; filenames and criterion IDs are shared. See the [glossary and maintenance rules](glossario.md). Toolkit documentation, skills and templates also have both versions.
