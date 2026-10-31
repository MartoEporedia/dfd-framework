# Assessment del rischio DFD

Rischio: "medium"
Percorso: "full-proportional"

- security: "low" — Gli avvisi non concedono né tolgono approvazioni; i percorsi degli entrypoint passano dalla validazione di Store (niente escape dalla radice).
- reliability: "medium" — Le euristiche sui log possono dare falsi positivi (per questo solo avvisi); dichiarare gli entrypoint cambia il fingerprint di design dei domini che li adottano.
- cost: "low" — Lettura di log già hashati; nessuna dipendenza.
- business: "low" — Toolkit in solo sviluppo, unico utilizzatore il manutentore.
- architecture: "medium" — Due campi facoltativi nuovi nel contratto condiviso lifecycle.json (log_markers, suite_entrypoints) e un campo warnings nell'output di sviluppo e release.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
