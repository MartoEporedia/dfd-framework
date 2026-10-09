# Assessment del rischio DFD

Rischio: "medium"
Percorso: "full-proportional"

- security: "low" — Nessuna nuova esecuzione esterna o riduzione dei controlli sui file.
- reliability: "medium" — Cambia la semantica del gate di sviluppo; mantenere CI stretta per default e isolamento dalla release.
- cost: "low" — Riuso delle verifiche locali; nessun servizio o costo aggiuntivo.
- business: "low" — Toolkit in sviluppo; nessuna distribuzione o modifica di un servizio cliente.
- architecture: "medium" — Policy condivisa da setup, fingerprint, sviluppo, release e skill.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
