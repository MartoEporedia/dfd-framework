# Assessment del rischio DFD

Rischio: "medium"
Percorso: "full-proportional"

- security: "medium" — Chiude divergenze tra parser e CommonMark che permettevano di cambiare la DoD di un criterio selezionato senza riconferma (recinti, titoli setext/blockquote/HTML, indice descrittivo).
- reliability: "medium" — Cambia di nuovo la formula del fingerprint selettivo: le review selettive approvate diventano stale-review una volta; un'estrazione errata invaliderebbe troppo o troppo poco.
- cost: "low" — Dipendenza compilata nel binario: nessun runtime, rete o servizio per l'utilizzatore.
- business: "low" — Toolkit in solo sviluppo, unico utilizzatore il manutentore.
- architecture: "medium" — Nuova dipendenza pulldown-cmark (0.13, senza feature di default: bitflags, memchr, unicase) al posto del parser scritto a mano in lib.rs.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
