# Assessment del rischio DFD

Rischio: "low"
Percorso: "light"

- security: "low" — Lo snapshot è solo informativo e non decide alcun gate; nessun nuovo percorso letto oltre ai file già fingerprintati e alle evidenze dichiarate.
- reliability: "low" — Additivo: il fingerprint e i gate restano calcolati come oggi; decisioni senza snapshot restano valide e leggibili.
- cost: "low" — Hash di file già letti dai fingerprint; nessuna dipendenza.
- business: "low" — Toolkit in solo sviluppo, unico utilizzatore il manutentore; il cambiamento aggiunge informazione al revisore.
- architecture: "medium" — Nuovo campo facoltativo nelle decisioni (state.json, release-decisions.json, record rapidi) e calcolo dello snapshot accanto ai tre fingerprint esistenti in lib.rs, release.rs e quick.rs.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
