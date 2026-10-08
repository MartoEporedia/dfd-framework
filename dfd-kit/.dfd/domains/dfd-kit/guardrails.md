# Guardrail del dominio DFD Kit

- Data: 2026-10-09
- Owner: manutentore del repository
- Riferimenti: [guardrail comuni](../../guardrails.md), [contributi](../../../CONTRIBUTING.md), [contratto](../../../templates/contratto_toolkit.md).

## 1. Architettura e contratti

Il binario locale incorpora risorse e skill; gli adapter condividono workflow canonici. Nessun servizio remoto o modello AI è richiesto dalla CLI. Tenere validazione, persistenza, sviluppo e release coerenti con i modelli versionati. Preservare identificatori dei criteri e isolamento delle feature. Documentare esplicitamente migrazioni e incompatibilità.

## 2. Sicurezza e integrità

Validare percorsi, symlink e input; non aggirare lock e controlli tramite modifiche manuali dello stato. Scrivere atomicamente per file senza promettere transazioni multi-file. Preservare documenti e decisioni precedenti durante le iterazioni. Non conservare segreti nei log o negli artefatti; usare fixture sintetiche nei test. Hash e fingerprint attestano integrità, non approvazione o autenticità umana.

## 3. Affidabilità

Testare effetti osservabili su progetti temporanei, inclusi errori e preservazione dello stato. Per regressioni dei gate aggiungere test mirati e verificare i mutanti pertinenti; non estendere test solo per imitare l’implementazione. La compatibilità multipiattaforma richiede prove sulle piattaforme dichiarate.

## 4. Osservabilità e costo

JSON, exit code, status e report devono rendere visibili blocchi e prossimo passo. Conservare evidenze verificabili senza dichiarare eseguiti controlli solo pianificati. Misurare tempi quando il cambiamento li influenza; nessuna soglia numerica o SLA è fissata senza baseline. Limitare verifiche ridondanti e distinguere costi della toolchain da quelli del binario.

## 5. Decisioni ed eccezioni

Il manutentore approva design e release; l’agente prepara artefatti, implementa e verifica. Riutilizzare decisioni già espresse solo se pertinenti e attuali. Esclusioni motivate vivono nella feature, con owner e impatto; non possono disabilitare confini del filesystem, integrità delle evidenze o gate umani. Nuovi dubbi riaprono la specifica e le review invalidate.
