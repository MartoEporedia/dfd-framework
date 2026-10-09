# Configurazione Rollout – [Nome Feature]

> **Stato**: draft / in review / attiva / completata  
> **Data creazione**: YYYY-MM-DD  
> **Proprietari**: [@team / @persona]  
> **Servizio / Dominio**: [es. Checkout]  
> **Riferimenti**:
> - [Specifica di Feature – Nome Feature]
> - [DoD Estesa – Servizio X]
> - [Architecture Guardrails]

---

## 1. Panoramica

**Feature**: [breve descrizione, es. “Nuovo metodo di pagamento PayPal”]  
**Ambiente**: [staging / prod]  
**Tipo di rollout**:  
- [ ] Canary tecnico (frazione di traffico)  
- [x] Rollout per tenant / segmento (feature flag)  
- [ ] Combinato (canary + rollout per tenant)

---

## 2. Feature flag / configurazione

**Sistema**: [es. AWS AppConfig, LaunchDarkly, flag interno]

**Flag**:

```yaml
flag:
  name: checkout.paypal.enabled
  type: boolean
  scope: tenant
  default: false
```

**Configurazione per tenant** (esempio):

```yaml
tenants:
  - id: internal
    enabled: true
    percentage: 100
  - id: market-de
    enabled: true
    percentage: 100
  - id: market-it
    enabled: true
    percentage: 100
  - id: market-fr
    enabled: false
    percentage: 0
```

---

## 3. Fasi di rollout

| Fase | Descrizione                       | Tenant / Segmento       | % Traffico | Durata minima | Criteri di promozione                         |
|------|-----------------------------------|--------------------------|------------|---------------|-----------------------------------------------|
| 1    | Staging                           | tenant finti             | 100%       | 24h           | Nessun errore critico, test E2E verdi         |
| 2    | Prod – tenant interni             | `internal`               | 5%         | 48h           | Vedi sezione 4                                |
| 3    | Prod – mercato DE                 | `market-de`              | 20%        | 72h           | Vedi sezione 4                                |
| 4    | Prod – mercato IT                 | `market-it`              | 50%        | 72h           | Vedi sezione 4                                |
| 5    | Prod – tutti i tenant abilitati   | tutti con `enabled=true` | 100%       | –             | Vedi sezione 4                                |

---

## 4. Criteri di promozione / stop / rollback

### 4.1 Criteri tecnici (canary)

Monitorati su CloudWatch (o sistema equivalente):

- **Error rate 5xx** su `POST /checkout`:
  - Soglia: < **1%** per 5 min
- **Latenza p95** per `paymentMethod=PAYPAL`:
  - Soglia: < **3 s** per 10 min
- **CheckoutFailed** per `errorCode` PayPal:
  - Soglia: < **10%** (fallimenti attesi inclusi)

**Azioni**:

- Se una soglia viene superata:
  - [ ] Ferma il rollout (blocca promozione alla fase successiva)
  - [ ] Se in fase avanzata, valuta rollback parziale (disabilita flag per tenant interessati)

### 4.2 Criteri business / UX

- **Tasso di completamento checkout** (globale e per `paymentMethod`):
  - Baseline: ultimi 14 giorni senza PayPal
  - Soglia: nessuna riduzione > **5%** rispetto alla baseline
- **Ticket / segnalazioni supporto**:
  - Soglia: nessun aumento significativo (> X% rispetto alla media) legato a PayPal

**Azioni**:

- Se il tasso di completamento peggiora > 5%:
  - [ ] Ferma il rollout
  - [ ] Analizza per `tenant` e `paymentMethod`
  - [ ] Se necessario, rollback (disabilita flag per tenant interessati)

### 4.3 Criteri di costo

- **Costo mensile del servizio Checkout**:
  - Soglia: variazione < **15%** su finestra 7 giorni (DoD COST-03)
- **Costo per transazione stimato**:
  - Soglia: variazione < **20%**

**Azioni**:

- Se le soglie vengono superate:
  - [ ] Ferma il rollout
  - [ ] Apri review architetturale / economica

---

## 5. Allarmi e dashboard

**Dashboard**:

- [ ] `Checkout – Servizio` (request, error, latency)
- [ ] `Checkout – Flusso business` (CheckoutCompleted, CheckoutFailed per paymentMethod)
- [ ] `Checkout – Costo` (costo mensile, costo per transazione)

**Allarmi** (CloudWatch Alarms o equivalente):

- [ ] `Checkout-5xx-ErrorRate` – soglia 1% per 5 min
- [ ] `Checkout-PayPal-Latency-p95` – soglia 3 s per 10 min
- [ ] `Checkout-CompletionRate-Drop` – riduzione > 5% vs baseline
- [ ] `Checkout-Cost-Monthly-Increase` – aumento > 15% vs media 30 giorni

---

## 6. Piano di rollback

**Trigger di rollback**:

- Error rate 5xx > 1% per 5 min
- Peggioramento tasso di completamento > 5%
- Aumento costo oltre soglie DoD (COST-03)

**Azioni**:

1. Disabilita flag `checkout.paypal.enabled` per i tenant interessati (o tutti).
2. Verifica che:
   - error rate torni nella norma
   - tasso di completamento si stabilizzi
3. Documenta l’incidente (post-mortem / nota interna).
4. Ridefinisci piano di rollout (se necessario).

---

## 7. Responsabilità e comunicazione

- **Owner rollout**: [@persona]
- **On-call durante rollout**: [@persona / @team]
- **Comunicazione**:
  - [ ] Avviso a Supporto / Customer Success prima di ogni fase
  - [ ] Canale Slack / Teams dedicato: `#checkout-paypal-rollout`
  - [ ] Aggiornamento stato su [dashboard / pagina di stato]

---

## 8. Storico esecuzioni

| Data       | Fase | Esito      | Note                                    |
|------------|------|------------|-----------------------------------------|
| YYYY-MM-DD | 1    | completata | Nessun errore critico                   |
| YYYY-MM-DD | 2    | completata | Leggero aumento latenza, entro soglie   |
| YYYY-MM-DD | 3    | in corso   | Monitoraggio in corso                   |

## Maturità e policy delle verifiche

- Modalità del dominio: solo sviluppo / preparazione al rilascio.
- CI richiesta: sì / no; motivazione rispetto a rischio e destinazione.
- Suite finale: comando, ambiente, data, esito e riferimento al log reale.
- Dubbi o requisiti rinviati alla pre-release: owner e condizione di attivazione.

In solo sviluppo, completare design e verifiche proporzionate senza richiedere CI, rollout, on-call o comunicazioni di rilascio anticipatamente. Le checklist di release si applicano quando si prepara una distribuzione. Per passare al rilascio rivalutare rischio e policy, riconfermare review e aggiornare evidenze; non segnare come eseguite attività rinviate.

## Proporzionalità e collaborazione

Valutare prima il percorso rapido per fix circoscritti e non critici: in quel caso usare un solo [record rapido](cambiamento_rapido.md), senza compilare questo template integralmente. Per light/full mantenere il dettaglio utile al rischio. Owner e verifiche sono necessari; PR, reviewer indipendente e prove integrate dipendono dalle [convenzioni facoltative del team](collaborazione.md). Il singolo dev non deve simulare ruoli o comunicazioni. Riutilizzare review già pertinenti; testare gli effetti del fallimento e le regressioni importanti.
