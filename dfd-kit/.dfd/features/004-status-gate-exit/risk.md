# Assessment del rischio DFD

Rischio: "low"
Percorso: "light"

- security: "low" — Nessun nuovo accesso a file o approvazione; il gate resta calcolato dagli artefatti e nessuna decisione umana viene derivata.
- reliability: "medium" — Cambia il contratto di exit code di status (0 → 2 con gate bloccato): script e skill che chiamano status devono gestire il nuovo codice; la fase registrata non deve regredire né fuorviare.
- cost: "low" — Nessuna dipendenza, servizio o costo di esecuzione aggiuntivo.
- business: "low" — Toolkit in solo sviluppo, nessun rilascio; il cambiamento rende visibile un blocco che oggi passa inosservato.
- architecture: "low" — Modifica locale a status, verify e all'exit code in lib.rs/development.rs; nessun nuovo modulo, schema JSON o file di stato.

Una sola dimensione Medium segue light; nuovi servizi seguono full completo.
