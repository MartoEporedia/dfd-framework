# Criteri di Rischio Feature – DoD-First Driven (DFD)

> **Stato**: draft / stabile  
> **Ultimo aggiornamento**: YYYY-MM-DD  
> **Proprietari**: [@team / @persona]  
> **Riferimenti**:
> - [Processo End-to-End DFD]
> - [Checklist di Review DFD – Design]
> - [Checklist di Review DFD – Release]
> - [Checklist di Review DFD – Design (Light)]
> - [Checklist di Review DFD – Release (Light)]

Questo documento definisce **criteri pratici** per decidere se una feature deve seguire il percorso **DFD full** (checklist complete) o **DFD light** (checklist leggere).

---

## 1. Obiettivo

Evitare di applicare un processo “pesante” a feature piccole / a basso rischio, mantenendo al contempo rigore su:

- sicurezza;
- affidabilità / SLO;
- costo;
- impatto su utenti e business.

La classificazione del rischio guida la scelta tra:

- **Checklist full** (design + release);
- **Checklist light** (design + release).

---

## 2. Dimensioni di rischio

Valuta la feature lungo queste **5 dimensioni**:

1. **Sicurezza**  
2. **Affidabilità / SLO**  
3. **Costo**  
4. **Impatto su utenti / business**  
5. **Complessità architetturale**

Per ogni dimensione, assegna un livello: **Basso**, **Medio**, **Alto**.

---

### 2.1 Sicurezza

**Basso**:

- Nessun cambiamento su:
  - controlli di accesso / autorizzazioni;
  - gestione dati sensibili (PII, dati di pagamento, ecc.);
  - audit / logging di sicurezza.
- Nessun nuovo endpoint esposto verso l’esterno.
- Nessun cambiamento su crittografia, gestione segreti, policy IAM.

**Medio**:

- Piccoli cambiamenti su:
  - logging di sicurezza (nuovi eventi di audit);
  - regole di autorizzazione esistenti (aggiunta di casi, senza cambiare modello).
- Nuovo endpoint interno (non esposto direttamente a utenti esterni).

**Alto**:

- Nuovi controlli di accesso / autorizzazioni.
- Nuovi dati sensibili trattati o loggati.
- Cambiamenti su:
  - crittografia;
  - gestione segreti;
  - policy IAM critiche.
- Nuovo endpoint pubblico che tratta dati sensibili.

---

### 2.2 Affidabilità / SLO

**Basso**:

- Nessun impatto atteso su:
  - latenza;
  - error rate;
  - disponibilità.
- Nessuna nuova dipendenza esterna critica.
- Cambiamenti confinati a:
  - UI;
  - testi;
  - logica interna non critica.

**Medio**:

- Possibile impatto lieve su latenza / error rate, ma:
  - entro margini ampi rispetto agli SLO;
  - con fallback / degradazione accettabile.
- Nuova dipendenza esterna non critica (es. servizio interno già stabile).

**Alto**:

- Impatto significativo su:
  - latenza (avvicina o supera SLO);
  - error rate;
  - disponibilità.
- Nuova dipendenza esterna critica (es. provider di pagamento, servizio core).
- Cambiamenti su:
  - retry, timeout, circuit breaker;
  - gestione errori di dipendenze critiche.

---

### 2.3 Costo

**Basso**:

- Nessun impatto significativo sul costo del servizio.
- Nessun nuovo uso intensivo di risorse (CPU, memoria, I/O, storage).

**Medio**:

- Aumento stimato del costo:
  - < 5% mensile per il servizio;
  - o < 5% per transazione.
- Nuovo uso moderato di risorse (es. nuove metriche custom, log aggiuntivi).

**Alto**:

- Aumento stimato del costo:
  - ≥ 5% mensile per il servizio;
  - o ≥ 10% per transazione.
- Nuovo uso intensivo di risorse (es. batch pesanti, nuove pipeline dati, nuovi storage significativi).

---

### 2.4 Impatto su utenti / business

**Basso**:

- Cambiamenti non visibili agli utenti finali (es. refactor, log, metriche interne).
- Piccoli cambiamenti UI / testi, senza impatto su flussi critici.
- Nessun impatto su KPI business principali (conversione, churn, revenue).

**Medio**:

- Cambiamenti visibili agli utenti, ma:
  - confinati a parti non critiche del prodotto;
  - con impatto limitato su KPI business.
- Nuova feature opzionale, non abilitata di default per tutti.

**Alto**:

- Cambiamenti su flussi critici (es. checkout, login, pagamento, onboarding).
- Feature abilitata di default per tutti gli utenti / tenant.
- Possibile impatto significativo su KPI business (positivo o negativo).

---

### 2.5 Complessità architetturale

**Basso**:

- Cambiamenti locali a un singolo servizio / modulo.
- Nessuna modifica a:
  - contratti API pubblici;
  - eventi di dominio condivisi;
  - schemi dati chiave.
- Nessun nuovo pattern architetturale introdotto.

**Medio**:

- Cambiamenti che coinvolgono 2–3 servizi / moduli.
- Piccole modifiche a:
  - contratti API (campi aggiuntivi, backward compatible);
  - eventi di dominio (nuovi eventi, senza rompere consumatori esistenti).

**Alto**:

- Cambiamenti che coinvolgono molti servizi / moduli.
- Modifiche a:
  - contratti API pubblici (breaking changes o rischi di regressione);
  - eventi di dominio critici;
  - schemi dati chiave.
- Introduzione di nuovi pattern architetturali significativi (es. nuovo stile di comunicazione, nuovo boundary di dominio).

---

## 3. Regole di classificazione

### 3.1 Feature “Low Risk” → DFD Light

Una feature è **Low Risk** (adatta a **DFD light**) se:

- **Tutte** le dimensioni sono valutate **Basso**, **oppure**
- Al massimo **una** dimensione è **Medio**, e nessuna è **Alto**.

Esempi tipici:

- Refactor interno senza cambiamenti esterni.
- Piccoli cambiamenti UI / testi.
- Nuove metriche / log interni, senza impatto su performance o costo significativo.
- Bug fix non critici, con test locali e impatto limitato.

Per queste feature:

- si usa la **Checklist di Review DFD – Design (Light)**;
- si usa la **Checklist di Review DFD – Release (Light)**.

---

### 3.2 Feature “Medium Risk” → DFD Full (semplificato)

Una feature è **Medium Risk** se:

- Almeno **una** dimensione è **Medio**;
- Nessuna dimensione è **Alto**.

Per queste feature:

- si usa il **processo DFD full**, ma:
  - le checklist possono essere applicate in modo “proporzionato” (focus sulle dimensioni Medio/Alto);
  - la review può essere più leggera (meno partecipanti, tempo più breve).

Esempi tipici:

- Nuova feature opzionale per un sottoinsieme di tenant.
- Modifiche a flussi non critici (es. report interni, dashboard).
- Nuove integrazioni con servizi interni già stabili.

---

### 3.3 Feature “High Risk” → DFD Full (completo)

Una feature è **High Risk** se:

- Almeno **una** dimensione è **Alto**.

Per queste feature:

- si usa il **processo DFD full** senza semplificazioni;
- le checklist vanno compilate in modo **completo e dettagliato**;
- la review deve coinvolgere tutti i ruoli rilevanti (Tech Lead, Security, Ops, Product, ecc.).

Esempi tipici:

- Nuovi metodi di pagamento / cambiamenti su flussi di pagamento.
- Cambiamenti su sicurezza (accesso, autorizzazioni, dati sensibili).
- Modifiche architetturali significative (nuovi boundary, nuovi servizi core).
- Feature abilitata di default per tutti gli utenti su flussi critici.

---

## 4. Processo di classificazione

### 4.1 Chi classifica

- La classificazione iniziale è fatta da:
  - **Tech Lead** + **Developer** principale;
  - con contributo di **Product** per la dimensione “Impatto su utenti / business”.

- In caso di dubbi (specialmente su sicurezza / SLO / costo):
  - coinvolgere **Security**, **Ops / SRE**, o **Architect** per una valutazione congiunta.

### 4.2 Quando classificare

- La classificazione va fatta **prima di iniziare il design** (idealmente quando si crea l’epic o si inizia a raffinarla).
- Può essere aggiornata durante il design, se emergono nuovi rischi.

### 4.3 Dove documentare

- La classificazione va annotata:
  - nell’**epic / ticket** (es. campo “Rischio DFD: Low / Medium / High”);
  - e/o nella **Specifica di Feature** (sezione “Classificazione del rischio”).

Esempio:

```markdown
## Classificazione del rischio DFD

- Sicurezza: Basso  
- Affidabilità / SLO: Medio  
- Costo: Basso  
- Impatto su utenti / business: Medio  
- Complessità architetturale: Basso  

**Classificazione complessiva**: Medium Risk → DFD Full (semplificato)
```

---

## 5. Esempi pratici

### 5.1 Esempio 1 – Refactor interno

- Sicurezza: Basso  
- Affidabilità / SLO: Basso  
- Costo: Basso  
- Impatto su utenti / business: Basso  
- Complessità architetturale: Basso  

**Classificazione**: Low Risk → **DFD Light**

---

### 5.2 Esempio 2 – Nuova dashboard interna

- Sicurezza: Medio (nuovi eventi di audit)  
- Affidabilità / SLO: Basso  
- Costo: Medio (nuove query, possibile aumento costo DB)  
- Impatto su utenti / business: Basso (solo utenti interni)  
- Complessità architetturale: Medio (coinvolge 2 servizi)

**Classificazione**: Medium Risk → **DFD Full (semplificato)**

---

### 5.3 Esempio 3 – Nuovo metodo di pagamento (PayPal)

- Sicurezza: Alto (nuovi dati, nuovi flussi di pagamento)  
- Affidabilità / SLO: Alto (nuova dipendenza critica, impatto su latenza e error rate)  
- Costo: Medio (aumento costo per transazione)  
- Impatto su utenti / business: Alto (flusso checkout, KPI conversione)  
- Complessità architetturale: Alto (nuovo provider, nuovi flussi end-to-end)

**Classificazione**: High Risk → **DFD Full (completo)**

---

## 6. Eccezioni

In casi particolari, il **Tech Lead** o l’**Architect** possono decidere di:

- trattare una feature **Medium Risk** come **High Risk** (es. per prudenza, se il team è poco esperto);
- trattare una feature **High Risk** con un percorso **full rafforzato** (es. più review, più test, rollout più graduale).

Le eccezioni vanno annotate nella specifica di feature e, se rilevanti, condivise con il team.
