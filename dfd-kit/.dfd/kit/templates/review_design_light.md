# Checklist di Review DFD – Design (Light)

> **Tipo di review**: design (feature piccola / basso rischio)  
> **Data**: YYYY-MM-DD  
> **Partecipanti**: [@team / @persone]  
> **Riferimenti**:
> - [Specifica di Feature (anche leggera)]
> - [DoD Estesa – Servizio X]

> **Quando usare questa versione**:  
> Feature che **non toccano** pagamento, sicurezza, SLO critici o architettura core.  
> Esempi: piccoli cambiamenti UI, testi, metriche interne, refactor non funzionali.

---

## 0. DoR – Design (Light)

- [ ] **Obiettivo chiaro** (1 frase: cosa cambia e perché).  
- [ ] **Servizio / dominio** identificato.  
- [ ] **Nessun impatto atteso** su sicurezza, SLO critici o costo significativo.  
- [ ] **OK informale** del Tech Lead per procedere con design leggero.

---

## 1. DoD Estesa (punti essenziali)

### 1.1 Osservabilità

- [ ] Nessun nuovo endpoint critico introdotto (o, se introdotto, metriche base già disponibili).  
- [ ] Log coerenti con lo standard del servizio (se ci sono nuovi log).  
- [ ] Nessun impatto negativo su dashboard esistenti.

### 1.2 Sicurezza

- [ ] Nessun cambiamento su controlli di accesso / autorizzazioni.  
- [ ] Nessun nuovo dato sensibile trattato o loggato.

### 1.3 Affidabilità / SLO

- [ ] Nessun impatto atteso su latenza, error rate o disponibilità.  
- [ ] Nessuna nuova dipendenza esterna critica.

### 1.4 Costo

- [ ] Nessun impatto significativo sul costo del servizio.

---

## 2. Guardrail globali

- [ ] La feature non viola i guardrail architetturali.  
- [ ] La feature non viola i guardrail di sicurezza.  
- [ ] La feature non viola i guardrail di osservabilità.  
- [ ] La feature non viola i guardrail di costo.

---

## 3. Test e validazione

- [ ] Test unitari / di integrazione previsti (almeno per la logica nuova / modificata).  
- [ ] Nessun test di carico / resilienza richiesto (o già coperto da test esistenti).  
- [ ] Validazione in produzione:  
  - [ ] rollout diretto o canary leggero (se il team lo prevede).

---

## 4. Esito della review di design (Light)

- [ ] **Approvata** (può procedere allo sviluppo)  
- [ ] **Approvata con condizioni**: [condizioni da soddisfare]  
- [ ] **Da rivedere**: [motivi, cosa manca]

**Firme / approvazioni**:

- [@persona] – [ruolo]

## Maturità e policy delle verifiche

- Modalità del dominio: solo sviluppo / preparazione al rilascio.
- CI richiesta: sì / no; motivazione rispetto a rischio e destinazione.
- Suite finale: comando, ambiente, data, esito e riferimento al log reale.
- Dubbi o requisiti rinviati alla pre-release: owner e condizione di attivazione.

In solo sviluppo, completare design e verifiche proporzionate senza richiedere CI, rollout, on-call o comunicazioni di rilascio anticipatamente. Le checklist di release si applicano quando si prepara una distribuzione. Per passare al rilascio rivalutare rischio e policy, riconfermare review e aggiornare evidenze; non segnare come eseguite attività rinviate.

## Proporzionalità e collaborazione

Valutare prima il percorso rapido per fix circoscritti e non critici: in quel caso usare un solo [record rapido](cambiamento_rapido.md), senza compilare questo template integralmente. Per light/full mantenere il dettaglio utile al rischio. Owner e verifiche sono necessari; PR, reviewer indipendente e prove integrate dipendono dalle [convenzioni facoltative del team](collaborazione.md). Il singolo dev non deve simulare ruoli o comunicazioni. Riutilizzare review già pertinenti; testare gli effetti del fallimento e le regressioni importanti.
