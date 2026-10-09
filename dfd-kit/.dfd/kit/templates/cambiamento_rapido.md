# Record di cambiamento rapido

- ID / owner:
- Problema osservato:
- Ambito circoscritto:
- Tipo: fix comportamentale / editoriale.
- Contratto noto ripristinato (riferimento al design/DoD approvato):
- Rischio: cinque dimensioni con livello e motivazione; tutte low per il rapido.
- Contratti o guardrail modificati: no; comportamento critico: no; dubbi aperti: nessuno.
- Criteri DoD pertinenti:
- Verifiche scelte e perimetro motivato:
- Fix: regressione red pertinente prima del green, con comandi, date, esiti e log.
- Editoriale: verifica pertinente green; nessun red artificiale.
- File verificati e snapshot:
- Contesto di collaborazione: issue / branch / PR / revisione, se utilizzato.
- Review indipendente, solo quando richiesta: reviewer, riferimento, revisione e file esaminati.

Il singolo sviluppatore non deve aprire una PR o ottenere un secondo reviewer per usare il rapido. La policy del dominio può richiedere CI o review indipendente. Non dichiarare test o review non eseguiti. Rischio maggiore o dubbi richiedono light/full; una distribuzione richiede il percorso di release pertinente.
