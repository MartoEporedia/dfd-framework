# Note di review – 012-english-primary

Rilievi semantici dell'agente, da affiancare al report strutturale `design-review.md`. Non sono una decisione.

- Bloccante per il gate: le quattro domande aperte in `design.json` (§7 della specifica) richiedono la risposta del manutentore.
- Coerenza: obiettivo, no-go e metriche riflettono le due scelte espresse il 2026-11-17. Il cambio di default è incompatibile per scelta; la migrazione (`dfd language it`) è l'unica azione richiesta agli utilizzatori che vogliono restare in italiano.
- Rischio residuo principale: i test che oggi presuppongono l'italiano di default. Vanno adattati passando la lingua esplicitamente dove l'asserzione riguarda testo italiano, senza eliminarli.
- Effetto collaterale noto: lo spostamento modifica file che sono evidenze di feature e record precedenti (README, CHANGELOG, template). Quelle feature diventano o restano `stale`; sono storiche e non vengono riaperte.
- Dipendenza: la PR #17 (ondata 1) va unita prima; questo branch ne contiene il commit.
