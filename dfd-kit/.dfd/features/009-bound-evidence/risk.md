# Assessment del rischio DFD

Rischio: "medium"
Percorso: "full-proportional"

- security: "medium" — La CLI esegue per la prima volta un comando del progetto, solo su richiesta esplicita, senza shell e con il lock del progetto; il percorso dei log e la lettura dell'albero passano da Store (niente escape, niente symlink).
- reliability: "medium" — Con source_roots lo sviluppo si blocca quando i file divergono dall'ultima suite o un criterio cambia dopo il green: radici troppo ampie o regole errate bloccherebbero lavoro legittimo.
- cost: "low" — Hash dei file sotto le radici dichiarate; nessuna dipendenza o servizio.
- business: "low" — Toolkit in solo sviluppo, unico utilizzatore il manutentore.
- architecture: "medium" — Nuovo comando dfd exec e nuovi campi facoltativi in evidence.json e lifecycle.json; regole nuove nel gate di sviluppo.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
