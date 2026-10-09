# Assessment di adozione — DFD Kit

- Data: 2026-10-09
- Owner: manutentore del repository
- Modalità: brownfield; pilota attivo
- Livello osservato: 0; obiettivo: 1

## 1. Stato osservato

Il mandato DFD è attivo e la Fase 0 definisce il dominio. Non è ancora dimostrato un ciclo completo di feature con review e rollout: il setup non incrementa automaticamente il livello. L’[assessment macchina](adoption.json) distingue le sei aree e collega evidenze disponibili.

La [baseline locale conservata](evidence/mutation-baseline/report.json) registra suite iniziale passata e 18 mutanti curati eliminati. La [provenienza](evidence/mutation-baseline/provenance.json) conserva hash degli input e dei log; gli hash dei sorgenti e del manifest sono stati confrontati alla cattura. Questi risultati precedono l’adozione e non costituiscono CI, audit di sicurezza, copertura generale delle mutazioni o approvazione umana.

## 2. Lacune baseline

- CI disponibile come template, rinviata in solo sviluppo; prove multipiattaforma da raccogliere prima di dichiarare il supporto al rilascio.
- Pilota reale delle skill negli harness ancora da eseguire.
- Implementazione preesistente senza cicli feature DFD tracciati; nessuna approvazione retroattiva.
- Baseline quantitativa dei tempi e delle risorse da raccogliere.

## 3. Pilota e prossimi interventi

1. Prima feature pilota: modalità solo sviluppo con suite locale tracciata; valutare CI alla transizione al rilascio.
2. Seconda proposta: esecuzione del rollout e relativo tracciamento, da specificare e classificare.
3. Terza proposta: learning post-release e aggiornamento della DoD, da specificare e classificare.
4. Verificare il pilota nell’harness Codex e poi gli altri harness; retrospettiva dopo tre cicli o prima in caso di blocchi.

Questi interventi sono proposte di sequenza, non feature approvate o implementate dal setup. Il manutentore verifica l’adozione dopo i primi tre cicli, osservando dubbi chiariti, collegamenti criterio/test, attriti dei gate, regressioni e tempi. Aggiornare assessment, DoD e convenzioni con risultati reali; l’obiettivo di livello 1 rimane aperto.
