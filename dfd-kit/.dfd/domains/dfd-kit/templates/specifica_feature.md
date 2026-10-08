# Specifica di Feature – [Nome Feature]

> **Stato**: draft  
> **Data creazione**: YYYY-MM-DD  
> **Proprietari**: TODO  
> **Servizio / Dominio**: TODO  
> **Epic / Ticket**: TODO

## 1. Contesto, obiettivo e ambito

- Problema e valore atteso: TODO
- Metriche di successo: TODO
- Componenti modificati: TODO
- Fuori scope: TODO
- Nel brownfield: processi esistenti da riusare e lacune baseline pertinenti: TODO

## 2. Classificazione del rischio

- Riferimento all'assessment: TODO
- Rischio e percorso light/full: TODO
- Motivazioni e dati ancora mancanti: TODO

## 3. Comportamenti e contratti

- Scenari principali e criteri di accettazione: TODO
- Errori, failure mode e comportamenti degradati: TODO
- API, eventi, dati e dipendenze coinvolti: TODO
- Compatibilità e migrazioni, se necessarie: TODO

## 4. Allineamento alla DoD del dominio

Usare gli identificatori reali della DoD. Valutare ogni criterio richiesto per i cambiamenti; motivare le esclusioni. Collegare le lacune baseline quando influenzano l'intervento.

| Criterio DoD | Applicabilità | Implementazione o motivo di esclusione | Verifica prevista |
|---|---|---|---|
| TODO | TODO | TODO | TODO |

## 5. Guardrail e decisioni

- Guardrail architetturali, di sicurezza, osservabilità e costo: TODO
- Eccezioni motivate, responsabili e decisioni: TODO
- Collegamenti a policy e ADR esistenti: TODO

## 6. Test e validazione prevista

- Test funzionali derivati dai criteri: TODO
- Test non funzionali proporzionati al rischio: TODO
- Verifiche manuali e loro evidenze attese: TODO
- Piano di validazione in produzione, soglie e rollback: TODO

## 7. Domande aperte e review

- Punti bloccanti da chiarire: TODO
- Condizioni e responsabili: TODO
- Riferimento alla review e alla decisione umana: TODO

## 8. Revisioni

- YYYY-MM-DD: prima bozza


## Convenzioni locali — DFD Kit

Usare la [DoD di dominio](../dod.md), il [catalogo](../criteria.json) e le [convenzioni di processo](../process.md). Sostituire gli esempi del servizio web con comportamenti della CLI e delle skill: integrità dei file, correttezza dei gate, compatibilità, esiti dei comandi e attriti nell’harness. Non riusare valori di esempio come evidenze.

Collegare verifiche realmente eseguite e motivare le esclusioni. Per rollout e review di release, identificare binario candidato/checksum, progetti di prova, pilota dell’harness, risorse e rollback della distribuzione; risorse non provate restano planned. Registrare dubbi nella specifica e riprendere l’iterazione senza sovrascritture. Placeholder e checklist vuote sono intenzionali in questo template.
