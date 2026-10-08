# Specifica di Feature – PayPal sul Checkout

> **Stato**: esempio illustrativo, non approvato  
> **Ultimo aggiornamento**: 2026-10-08  
> **Dominio**: Checkout  
> **Riferimenti**: [DoD Checkout](checkout_dod.md), [Rischio](../rischio.md), [Adozione](../adozione.md)

Questo esempio mostra una feature pilota in un prodotto esistente. Le verifiche sono pianificate: non sono evidenze di test eseguiti né di risultati di produzione.

## 1. Obiettivo e ambito

Aggiungere PayPal come opzione di pagamento, conservando gli altri metodi e il flusso Checkout esistente. Monitorare completamento del checkout, fallimenti e abbandoni nello step di pagamento.

L'intervento comprende avvio del pagamento, gestione della conferma, errori e timeout del nuovo provider. Non comprende una revisione completa dei flussi legacy o di tutti gli SLO del prodotto. La review DFD si aggiunge alla review tecnica del team.

## 2. Classificazione del rischio

| Dimensione | Livello | Motivazione |
|---|---|---|
| Sicurezza | Alto | Nuovi flussi di pagamento e callback da autorizzare |
| Affidabilità / SLO | Alto | Nuova dipendenza esterna nel checkout |
| Costo | Medio | Richiede stima delle commissioni e del costo operativo |
| Utenti / business | Alto | Cambiamento di un flusso critico e dei KPI di conversione |
| Architettura | Alto | Integrazione con un provider e gestione della conferma |

**Percorso**: DFD full completo. La stima dei costi deve essere completata nel pilota; non è una misura osservata.

## 3. Scenari e contratti

- Avvio: l'utente seleziona PayPal; il sistema associa la sessione di pagamento all'ordine e presenta l'azione successiva.
- Conferma: il sistema verifica autenticità e correlazione della notifica prima di aggiornare lo stato dell'ordine.
- Notifiche duplicate: la stessa conferma non deve produrre ordini o addebiti duplicati.
- Fallimento o timeout: mostrare un errore comprensibile, conservare uno stato coerente e permettere una nuova scelta di pagamento.

Definire nella specifica reale contratti, stati, idempotenza, timeout e policy di retry del provider scelto. I retry non devono duplicare effetti finanziari.

## 4. Criteri DoD e verifiche previste

| Criterio | Applicazione al cambiamento | Verifica prevista |
|---|---|---|
| OBS-01 | Contare richieste, errori e latenza del nuovo flusso | Test dell'emissione delle metriche |
| OBS-02 | Correlare ordine, operazione, outcome e trace nei log | Test di integrazione dei campi e redazione dei dati |
| SEC-01 | Limitare l'accesso alle operazioni di pagamento | Test di autorizzazione e callback non valide |
| SEC-02 | Evitare dati sensibili nei log | Controlli su log di successo ed errore |
| SLO-02 | Valutare la latenza rispetto al criterio del dominio | Test di carico e dipendenza lenta |
| SLO-03 | Monitorare regressioni del tasso di errori | Test di failure e piano di canary |
| COST-02 | Stimare impatto mensile e per transazione | Stima documentata con volumi e ipotesi |
| UX-02 | Confrontare completamento con la baseline | Analisi del rollout per segmento |

La specifica reale deve valutare anche gli altri criteri richiesti dalla DoD, motivando le esclusioni. Le lacune legacy non coinvolte restano nel piano di adozione.

## 5. Review e rilascio previsti

Richiedere contributi di QA, Security, Ops e Product secondo il RACI. Il Tech Lead registra la decisione di design dopo aver chiarito contratti, rischi e test. Pianificare feature flag, rollout graduale, allarmi e rollback coerenti con la DoD del dominio; il rilascio richiede una review distinta.

**Punti aperti**: provider e contratti definitivi, costo stimato, soglie di allarme, responsabilità operative e approvazione dell'epic. Questo esempio non supera automaticamente un gate della CLI.
