---
name: dfd-feature
description: Aprire o riprendere una feature DFD da un'idea, definendo dominio, titolo, ambito e tipo di intervento prima dell'assessment e della specifica.
---

# Apertura di una feature

Valutare prima il percorso rapido: per fix di contratto noto o editoriali circoscritti usare `quick`, evitando di creare una feature con documenti separati. Per gli altri casi seguire il flusso qui sotto.

Leggere `.dfd/config.json` e `.dfd/kit/templates/contratto_toolkit.md`. Identificare dominio, problema, titolo, ambito e tipo (`feature`, `refactor`, `incident`, `new-service`). Chiarire solo i dati necessari per distinguere l'intervento; KPI, scenari e dubbi possono essere raffinati nella specifica iterativa.

Usare `dfd status` per cercare un intervento già aperto. Se esiste, riprenderlo senza creare duplicati o modificare manualmente `state.json`. Se nuovo, scegliere un ID lowercase con trattini ed eseguire:

```sh
dfd feature <id> --domain <dominio> --title "..." --scope "..." --kind feature
```

La CLI crea stato e rischio iniziale senza attribuire approvazioni o classificare dati ignoti. Il dominio deve essere registrato; se manca, inizializzarlo con `dfd init`. La Fase 0 può essere completata mentre si raffina l'idea e diventa necessaria prima del design. Mostrare ID, dominio, ambito e prossimo passo `dfd-assess`. Non chiedere di riconfermare informazioni già fornite.

Valutare prima rapido/light/full. Per fix circoscritti che ripristinano un contratto noto o modifiche editoriali usare dfd quick con ID, dominio, titolo, owner e kind behavior-fix/editorial; leggere il contratto del toolkit per il solo change.json. Non creare specifica o piano separati. In caso di escalation usare dfd promote con un nuovo ID, preservando l’origine. Il singolo dev non richiede PR o secondo reviewer; context e team.json sono facoltativi.
