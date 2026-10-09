# Checklist di Review DFD – Release

> **Tipo di review**: release  
> **Data**: YYYY-MM-DD  
> **Partecipanti**: [@team / @persone]  
> **Riferimenti**:
> - [Specifica di Feature]
> - [DoD Estesa – Servizio X]
> - [Configurazione Rollout]
> - [Processo End-to-End DFD – Definition of Ready]

---

## 0. Verifica Definition of Ready (DoR) – Release

Prima di procedere con la review di release, verificare che la feature soddisfi i seguenti criteri:

- [ ] **Tutti i test applicabili verdi** secondo la policy locale/CI del dominio (unitari, integrazione, E2E).  
- [ ] **Test non funzionali** eseguiti (se rilevanti: carico, resilienza, sicurezza).  
- [ ] **Configurazione Rollout** definita:
  - fasi di rollout;
  - criteri di promozione / stop / rollback;
  - allarmi e dashboard.
- [ ] **Allarmi e dashboard** configurati (o piano chiaro per attivarli).  
- [ ] **Supporto / stakeholder** informati (almeno i team chiave: Support, Ops, Security se rilevante).  
- [ ] **Review DFD (design)** almeno “approvata con condizioni” (nessun blocco architetturale / di sicurezza aperto).

Se questi criteri non sono soddisfatti, la review di release non dovrebbe procedere; si completa prima la pre-release.

---

## 1. Allineamento alla DoD Estesa (verifica implementazione)

### 1.1 Osservabilità

- [ ] I criteri **OBS-xx** citati nella specifica sono effettivamente implementati.
- [ ] Le metriche obbligatorie (request, error, latency) sono presenti per ogni nuovo endpoint.
- [ ] Le custom metric business (se previste) sono definite e visibili.
- [ ] I log includono: `tenantId`, `userId`, `operation`, `outcome`, `errorCode`, `traceId`.
- [ ] Il trace ID è presente su log e metriche.
- [ ] Le dashboard sono aggiornate / create e mostrano le metriche rilevanti.

### 1.2 Sicurezza

- [ ] I criteri **SEC-xx** citati nella specifica sono effettivamente implementati.
- [ ] I controlli di accesso (ruoli, policy) sono configurati.
- [ ] La protezione dati è verificata (nessun dato sensibile in chiaro nei log).
- [ ] Gli eventi di audit sono loggati come previsto.

### 1.3 Affidabilità / SLO

- [ ] I criteri **SLO-xx** citati nella specifica sono rispettati.
- [ ] L’impatto su latenza, error rate, disponibilità è entro le stime.
- [ ] Le mitigazioni per dipendenze esterne (timeout, retry, fallback) sono implementate.

### 1.4 Costo

- [ ] I criteri **COST-xx** citati nella specifica sono rispettati.
- [ ] La stima di impatto sul costo è coerente con l’implementazione.
- [ ] Le soglie di allarme costo sono configurate (se rilevanti).

### 1.5 Esperienza utente / business KPI

- [ ] I KPI business impattati sono monitorabili (dashboard, metriche).
- [ ] Le soglie di peggioramento accettabile (UX-02) sono chiare e misurabili.
- [ ] Il piano di validazione in produzione è pronto per essere eseguito.

---

## 2. Guardrail globali (verifica finale)

- [ ] La feature non viola i guardrail architetturali.
- [ ] La feature non viola i guardrail di sicurezza.
- [ ] La feature non viola i guardrail di osservabilità.
- [ ] La feature non viola i guardrail di costo.
- [ ] Eventuali eccezioni sono esplicite, motivate e approvate.

---

## 3. Test e validazione (stato attuale)

### 3.1 Test funzionali

- [ ] Tutti i test unitari verdi.
- [ ] Tutti i test di integrazione verdi.
- [ ] Tutti i test end-to-end verdi.

### 3.2 Test non funzionali

- [ ] Test di carico eseguiti (se rilevante per SLO) – esito: [ok / problemi].
- [ ] Test di resilienza eseguiti – esito: [ok / problemi].
- [ ] Test di sicurezza eseguiti – esito: [ok / problemi].

### 3.3 Validazione in produzione (shift-right)

- [ ] Piano di canary tecnico definito (metriche, soglie, rollback).
- [ ] Piano di rollout per tenant / segmento definito.
- [ ] Criteri di promozione / stop / rollback chiari e configurati.

---

## 4. Operatività e supporto

- [ ] Il supporto / customer success è informato della feature.
- [ ] Gli alert operativi sono configurati (chi viene notificato, su quali soglie).
- [ ] La procedura di rollback è chiara e testata (almeno a livello di “walkthrough”).

---

## 5. Decisioni e aperture

- [ ] Decisioni prese in review: [elenco sintetico]
- [ ] Aperture residue (da decidere / investigare): [elenco]

---

## 6. Esito della review di release

- [ ] **OK a rilasciare** (può procedere il rollout in produzione)
- [ ] **OK con condizioni**: [condizioni da soddisfare prima o durante il rollout]
- [ ] **Da rivedere**: [motivi, cosa manca]

**Firme / approvazioni**:

- [@persona1] – [ruolo]
- [@persona2] – [ruolo]

## Maturità e policy delle verifiche

- Modalità del dominio: solo sviluppo / preparazione al rilascio.
- CI richiesta: sì / no; motivazione rispetto a rischio e destinazione.
- Suite finale: comando, ambiente, data, esito e riferimento al log reale.
- Dubbi o requisiti rinviati alla pre-release: owner e condizione di attivazione.

In solo sviluppo, completare design e verifiche proporzionate senza richiedere CI, rollout, on-call o comunicazioni di rilascio anticipatamente. Le checklist di release si applicano quando si prepara una distribuzione. Per passare al rilascio rivalutare rischio e policy, riconfermare review e aggiornare evidenze; non segnare come eseguite attività rinviate.

## Proporzionalità e collaborazione

Valutare prima il percorso rapido per fix circoscritti e non critici: in quel caso usare un solo [record rapido](cambiamento_rapido.md), senza compilare questo template integralmente. Per light/full mantenere il dettaglio utile al rischio. Owner e verifiche sono necessari; PR, reviewer indipendente e prove integrate dipendono dalle [convenzioni facoltative del team](collaborazione.md). Il singolo dev non deve simulare ruoli o comunicazioni. Riutilizzare review già pertinenti; testare gli effetti del fallimento e le regressioni importanti.
