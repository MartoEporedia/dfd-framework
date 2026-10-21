<!-- dfd-language --> [Italiano](../../templates/processo_dominio.md) | [English](processo_dominio.md)

<a id="convenzioni-dfd-di-dominio"></a>

<a id="responsabilità-e-aggiornamenti"></a>

<a id="dod-e-criteri"></a>

<a id="classificazione-e-review"></a>

<a id="template-locali"></a>

<a id="collocazione-e-uso-degli-artefatti"></a>

# Domain DFD conventions

## Responsibilities and updates

TODO: owner and review date. Update setup after domain changes, recurring patterns, incidents and production learning.

## DoD and criteria

TODO: relevant concerns and stable OBS-01, SEC-01, SLO-01, COST-01, UX-01 IDs; align dod.md/criteria.json.

## Classification and review

TODO: application of central risk rules and light/full checklists.

## Local templates

TODO: unchanged/adapted templates and differences from the central framework. Template placeholders are intentional.

## Artifact placement and use

TODO: domain/feature artifact locations, review procedures and iterative question clarification.

<a id="maturità-del-progetto-solo-sviluppo-e-preparazione-al-rilascio"></a>

## Project maturity: development only and release preparation

Maturity is independent of DFD adoption level and feature risk. Phase 0 declares mode, CI policy and justification.

| Mode | Development checks | Outcome |
|---|---|---|
| `development-only` | Light/full: design review, TDD and final local suite; rapid: relevant checks; optional CI | Development complete; no pre-release or distribution approval |
| `release-preparation` | Same checks; CI required by default; a local alternative needs risk/target justification and human review | Ready for pre-release, followed by a separate release review |

Missing CI does not block development-only. Applicable checks remain mandatory with command, date, result and real log; local tests are never labeled CI. The final suite, when required, follows green tests. Declared failed CI still blocks even when optional.

Before distribution explicitly switch to release preparation; reassess risk, criteria, CI policy and target-environment checks. Reconfirm invalidated reviews and update plans/evidence. Development-only does not weaken security/risk controls, and local completion does not replace human release approval. Local experiments can produce learning before production data exists.

Declare lifecycle.json according to the toolkit contract; justify CI policy and the release transition.
