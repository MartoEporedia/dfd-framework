<!-- dfd-language --> [Italiano](../README.md) | [English](README.md)

<a id="come-leggere-questo-repo-per-ruolo"></a>

<a id="progetti-ancora-in-sviluppo"></a>

<a id="processo-proporzionato-anche-per-un-solo-dev"></a>

# DFD Framework

DoD-First Driven connects iterative specifications, tests and production validation through verifiable Definition of Done criteria and risk-proportionate reviews.

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
