# Convenzioni facoltative per la collaborazione

- Owner delle regole comuni:
- Naming degli interventi e riferimenti a issue/branch/PR:
- Ruoli delegati per intervento e policy di review indipendente:
- Gestione dei conflitti fra cloni:
- Verifica della revisione integrata prima della distribuzione:
- Impatto delle modifiche al setup sulle feature aperte:

La policy team è facoltativa. Il singolo sviluppatore usa gli stessi percorsi senza riferimenti o review indipendente obbligatori. Esempio per un team, da adattare:

```json
{"schema_version":1,"independent_review":true,"integrated_checks_required":true}
```

Con la CLI salvarla in `.dfd/domains/<dominio>/team.json`; assenza equivale a entrambe false. Le prove integrate devono coprire i file verificati dell’intervento e la revisione dichiarata. Git coordina cloni e merge; il lock locale non sostituisce Git.
