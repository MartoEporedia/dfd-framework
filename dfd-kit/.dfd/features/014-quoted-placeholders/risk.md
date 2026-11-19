# DFD risk assessment

Risk: "low"
Route: "light"

- security: "low" — Allentamento limitato: un segnaposto nascosto tra virgolette passerebbe, ma resta la review umana del design (SEC-03).
- reliability: "low" — Il controllo resta attivo sul testo libero: un TODO del template non citato blocca come prima; coperto da test e mutante.
- cost: "low" — Nessuna dipendenza nuova; regex già usata nel crate.
- business: "low" — Sblocca citazioni letterali richieste dalle skill; nessun effetto su progetti senza citazioni.
- architecture: "low" — Una funzione di filtro del testo prima di un controllo esistente; nessun contratto o schema cambia.

A single Medium dimension follows light; new services follow full complete.
