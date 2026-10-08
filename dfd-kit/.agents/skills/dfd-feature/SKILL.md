---
name: dfd-feature
description: Aprire o riprendere una feature DFD da un'idea, definendo dominio, titolo, ambito e tipo di intervento prima dell'assessment e della specifica.
---

# Apertura di una feature

Leggere `.dfd/config.json` e `.dfd/kit/templates/contratto_toolkit.md`. Identificare dominio, problema, titolo, ambito e tipo (`feature`, `refactor`, `incident`, `new-service`). Chiarire solo i dati necessari per distinguere l'intervento; KPI, scenari e dubbi possono essere raffinati nella specifica iterativa.

Usare `dfd status` per cercare un intervento già aperto. Se esiste, riprenderlo senza creare duplicati o modificare manualmente `state.json`. Se nuovo, scegliere un ID lowercase con trattini ed eseguire:

```sh
dfd feature <id> --domain <dominio> --title "..." --scope "..." --kind feature
```

La CLI crea stato e rischio iniziale senza attribuire approvazioni o classificare dati ignoti. Il dominio deve essere registrato; se manca, inizializzarlo con `dfd init`. La Fase 0 può essere completata mentre si raffina l'idea e diventa necessaria prima del design. Mostrare ID, dominio, ambito e prossimo passo `dfd-assess`. Non chiedere di riconfermare informazioni già fornite.
