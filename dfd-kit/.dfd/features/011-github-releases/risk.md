# Assessment del rischio DFD

Rischio: "medium"
Percorso: "full-proportional"

- security: "medium" — Binari pubblicati per utenti terzi: il workflow di release ha permesso contents: write limitato al job di pubblicazione, checksum SHA-256 per ogni archivio, azioni di terze parti a versione fissata.
- reliability: "low" — Una release sbagliata si corregge con una nuova release o eliminando quella pubblicata; i binari esistenti non cambiano.
- cost: "low" — Runner GitHub Actions gratuiti per repository pubblici; nessuna dipendenza nuova nel binario.
- business: "low" — Toolkit pubblico ma con un solo utilizzatore noto, il manutentore; la prima release rende scaricabile il binario a chi segue il prompt di installazione.
- architecture: "medium" — Nuovo processo di rilascio: CI GitHub Actions attiva, workflow di release al push di un tag, changelog versionato; il dominio passa da development-only a release-preparation.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
