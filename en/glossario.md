<!-- dfd-language --> [Italiano](../glossario.md) | [English](glossario.md)

<a id="glossario-e-manutenzione-delle-lingue"></a>

<a id="regole-di-manutenzione"></a>

<a id="lingua-del-toolkit"></a>

# Glossary and language maintenance

The framework has complete Italian and English reading paths. Original Italian files stay in place; English counterparts live in en/. Toolkit resources follow the same convention under dfd-kit/en/. Shared filenames simplify references, installation and pairing; titles/content use the selected language.

| Italiano | English | Convention |
|---|---|---|
| DoD estesa | Extended Definition of Done | Keep DoD |
| Specifica di feature | Feature specification | Design lives in the specification |
| Criterio | Criterion | Stable ID, e.g. SEC-01 |
| Osservabilità | Observability | OBS |
| Sicurezza | Security | SEC |
| Affidabilità | Reliability | SLO |
| Costo | Cost | COST |
| Percorso rapido | Rapid route | quick command, rapid enum |
| Percorso light | Light route | Stable light identifier |
| Full proporzionato | Proportionate full | full-proportional enum |
| Full completo | Complete full | full-complete enum |
| Solo sviluppo | Development only | development-only enum |
| Preparazione al rilascio | Release preparation | release-preparation enum |
| Evidenza | Evidence | Planned checks are not executed evidence |
| Responsabile | Owner / accountable person | Keep RACI distinctions |
| Review | Review | Required human decision remains necessary |
| Guardrail | Guardrail | Normative constraint |
| Da riconfermare | Stale review | stale-review enum |

## Maintenance rules

1. Update both languages in the same change. Translation must preserve thresholds, roles, criteria, conditions and the distinction between structural checks and human decisions.
2. Keep commands, JSON keys, enums, criterion IDs, filenames and machine references stable. Use natural English while retaining operational content. Examples remain illustrative; placeholders still need completion.
3. Preserve Italian paths. Each page links its counterpart; Italian anchor aliases preserve references in English paths.
4. Framework-root documents remain canonical. Toolkit copies explicitly adopt revisions; its feature template remains distinct from the general one. Never automatically translate completed .dfd artifacts or installed skill copies.
5. Run `python3 dfd-kit/scripts/check_translations.py` to check pairing, synchronization hashes, links/anchors, criterion IDs and message catalog. After semantic review of an updated pair, use --update-manifest. Hashes record declared alignment; they do not certify translation quality.
6. For embedded resources/code also run fmt, Clippy, tests and doctor per CONTRIBUTING. Both languages are embedded offline. Source language navigation is removed from installed resources, whose selected-language paths stay coherent.

## Toolkit language

```sh
dfd --lang en init --harness codex --domain checkout
dfd language it
dfd language en
dfd --lang it status
```

Explicit --lang on init/install persists; language updates managed resources/preference with the current adapter. Other commands use invocation-only overrides. Missing .dfd/language.json means Italian. Format: {"schema_version":1,"language":"en"}. Invalid preferences fail explicitly.

Existing documents and local templates retain their original content/language. New domains use the selected language; explicitly review an existing domain's templates to change its working language. Language switches preflight conflicts before updating managed resources/preference; writes remain atomic per file.
