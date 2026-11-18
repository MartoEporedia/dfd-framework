<!-- dfd-language --> [English](../../templates/cambiamento_rapido.md) | [Italiano](cambiamento_rapido.md)

# Record di cambiamento rapido

- ID / owner:
- Problema osservato:
- Parole del richiedente, alla lettera:
- Ambito circoscritto e no-go:
- Tipo: fix comportamentale / editoriale.
- Contratto noto ripristinato (riferimento al design/DoD approvato):
- Rischio: cinque dimensioni con livello e motivazione; tutte low per il rapido.
- Contratti o guardrail modificati: no; comportamento critico: no; dubbi aperti: nessuno.
- Criteri DoD pertinenti:
- Stati non previsti toccati dal fix: reazione esplicita e osservabile, oppure "nessuno".
- Verifiche scelte e perimetro motivato:
- Fix: regressione red pertinente prima del green, con comandi, date, esiti e log.
- Editoriale: verifica pertinente green; nessun red artificiale.
- File verificati e snapshot:
- Contesto di collaborazione: issue / branch / PR / revisione, se utilizzato.
- Review indipendente, solo quando richiesta: reviewer, riferimento, revisione e file esaminati.

Il singolo sviluppatore non deve aprire una PR o ottenere un secondo reviewer per usare il rapido. La policy del dominio può richiedere CI o review indipendente. Non dichiarare test o review non eseguiti. Rischio maggiore o dubbi richiedono light/full; una distribuzione richiede il percorso di release pertinente.
