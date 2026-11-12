# Assessment del rischio DFD

Rischio: "medium"
Percorso: "full-proportional"

- security: "low" — La CLI legge in più solo file dichiarati e radici di dominio, tramite Store (niente escape, niente symlink); nessuna esecuzione e nessuna scrittura nuova.
- reliability: "medium" — Un errore nel calcolo dei fingerprint invaliderebbe review e green esistenti, oppure non invaliderebbe quelli da invalidare; i design senza acceptance devono mantenere i fingerprint attuali.
- cost: "low" — Hash dei soli file dichiarati e visita delle radici dei casi; nessuna dipendenza o servizio.
- business: "low" — Toolkit in solo sviluppo, unico utilizzatore il manutentore.
- architecture: "medium" — Nuovi campi facoltativi in design.json (criteria[].acceptance) e lifecycle.json (acceptance_roots); il fingerprint di design e quello per criterio includono file del progetto fuori da .dfd.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
