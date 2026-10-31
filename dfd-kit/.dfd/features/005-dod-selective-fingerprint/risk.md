# Assessment del rischio DFD

Rischio: "medium"
Percorso: "full-proportional"

- security: "medium" — Tocca l'invalidazione delle approvazioni umane: chiude una via di falsa approvazione (DoD cambiata dopo la review senza riconferma); un errore nell'estrazione lascerebbe aperto il varco o lo sposterebbe.
- reliability: "medium" — Cambia la formula del fingerprint selettivo: le review selettive già approvate diventano stale-review una volta con il nuovo binario; un'estrazione dei paragrafi troppo larga produrrebbe invalidazioni superflue.
- cost: "low" — Lettura e hash di dod.md già presenti nel calcolo; nessuna dipendenza.
- business: "low" — Toolkit in solo sviluppo, unico utilizzatore il manutentore; nessun rilascio.
- architecture: "low" — Estensione locale di design_fingerprint in lib.rs; nessun nuovo modulo, comando o schema JSON.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
