<!-- dfd-language --> [Italiano](../../templates/collaborazione.md) | [English](collaborazione.md)

<a id="convenzioni-facoltative-per-la-collaborazione"></a>

# Optional collaboration conventions

- Shared-rule owner:
- Change naming and issue/branch/PR references:
- Delegated roles and independent review policy:
- Conflict handling across clones:
- Integrated revision verification before distribution:
- Setup-change impact on open features:

Team policy is optional. Solo developers use the same routes without mandatory references or independent reviews. Adapt this team example:

```json
{"schema_version":1,"independent_review":true,"integrated_checks_required":true}
```

Save as `.dfd/domains/<domain>/team.json`; absence means both false. Integrated evidence must cover verified change files and declared revision. Git coordinates clones/merges; a local lock does not replace Git.
