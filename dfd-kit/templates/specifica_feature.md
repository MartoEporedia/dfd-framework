<!-- dfd-language --> [Italiano](specifica_feature.md) | [English](../en/templates/specifica_feature.md)

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

## Maturità e policy delle verifiche

- Modalità del dominio: solo sviluppo / preparazione al rilascio.
- CI richiesta: sì / no; motivazione rispetto a rischio e destinazione.
- Suite finale: comando, ambiente, data, esito e riferimento al log reale.
- Dubbi o requisiti rinviati alla pre-release: owner e condizione di attivazione.

In solo sviluppo, completare design e verifiche proporzionate senza richiedere CI, rollout, on-call o comunicazioni di rilascio anticipatamente. Le checklist di release si applicano quando si prepara una distribuzione. Per passare al rilascio rivalutare rischio e policy, riconfermare review e aggiornare evidenze; non segnare come eseguite attività rinviate.

## Proporzionalità e collaborazione

Valutare prima il percorso rapido per fix circoscritti e non critici: in quel caso usare un solo [record rapido](cambiamento_rapido.md), senza compilare questo template integralmente. Per light/full mantenere il dettaglio utile al rischio. Owner e verifiche sono necessari; PR, reviewer indipendente e prove integrate dipendono dalle [convenzioni facoltative del team](collaborazione.md). Il singolo dev non deve simulare ruoli o comunicazioni. Riutilizzare review già pertinenti; testare gli effetti del fallimento e le regressioni importanti.
