---
name: dfd-assess
description: Classificare il rischio di una feature, refactor, intervento post-incidente o nuovo servizio con le cinque dimensioni DFD e scegliere il percorso rapido, light o full.
---

# Rischio di una feature

Leggere `.dfd/config.json`, l'assessment e la DoD del dominio interessato. Le risorse si trovano nel repository adottante: `.dfd/kit/rischio.md`, `.dfd/kit/adozione.md` per il brownfield, `.dfd/kit/templates/contratto_toolkit.md` per i formati.

Prima di generare artefatti feature, verificare se esiste un record rapido: in quel caso usare il solo record e le regole del percorso rapido.

1. Identificare una feature con titolo e ambito espliciti; per aprirla o riprenderla usare `dfd-feature`. Se assente, creare `dfd feature <id> --domain <dominio> --title "..." --scope "..." --kind feature`. I tipi sono `feature`, `refactor`, `incident`, `new-service`. Riprendere gli artefatti se la feature esiste.
2. Valutare sicurezza, affidabilità/SLO, costo, impatto business e complessità architetturale. Registrare in `risk.json` le cinque dimensioni `security`, `reliability`, `cost`, `business`, `architecture`, ciascuna con `level` e `rationale`. Usare `low`, `medium`, `high`; mantenere `null` dove mancano informazioni e spiegare cosa serve.
3. Valutare il cambiamento, senza attribuirgli automaticamente tutte le lacune storiche del prodotto. Considerare però le lacune che influenzano concretamente il rischio dell'intervento. Per un post-incidente collegare il problema al criterio DoD che potrebbe prevenirlo.
4. Eseguire `dfd assess <id>`. Non assegnare manualmente lo stato o il percorso: la CLI applica regole riproducibili. Tutte low oppure una sola medium danno light; almeno due medium senza high danno full proporzionato; una high dà full completo. Questa precedenza risolve la sovrapposizione delle regole testuali. Per `new-service` il percorso è full completo secondo la guida di adozione, mantenendo distinto il rischio.
5. La fascia di costo per transazione dal 5% a meno del 10% non è classificata dal framework: lasciarla da chiarire se non ci sono altri elementi sufficienti o una policy del team. Non trattare un dato ignoto come low.

Riportare motivazioni, informazioni mancanti e percorso. Le escalation motivate del team possono essere annotate nella specifica; la prima versione non applica override automatici. Procedere con `dfd-specify` quando l'assessment è completo.

Se l’ID è un record rapido, leggere .dfd/changes/<id>.json e usare assess senza generare risk.json o report separati. Tutte le dimensioni low, ambito noto e non critico, contratto invariato e nessun dubbio ammettono rapido; una medium impone almeno light. Non trattare critical o dati ignoti come fix trascurabili.
