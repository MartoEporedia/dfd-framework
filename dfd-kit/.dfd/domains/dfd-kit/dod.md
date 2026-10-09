# DoD di dominio — DFD Kit

- Stato: setup operativo; adozione in pilota
- Data: 2026-10-09
- Owner: manutentore del repository
- Ambito: CLI Rust, skill condivise, adapter, template incorporati, test, runner e documentazione necessari al toolkit.

## 1. Criteri per i cambiamenti

Ogni feature seleziona i criteri applicabili e motiva le esclusioni nella DoD estesa. Una modifica solo editoriale non richiede verifiche di codice prive di relazione con il cambiamento. Le decisioni umane restano distinte dai controlli automatici.

- **OBS-01**: Diagnostica JSON, exit code e stato descrivono correttamente errori, prerequisiti e prossimo passo; nessun gate dichiara evidenze o approvazioni inesistenti.
- **OBS-02**: Ogni criterio applicabile è collegato a verifiche reali, risultati e decisioni; file e log delle evidenze mantengono hash, provenienza e ordine temporale verificabili.
- **SEC-01**: Percorsi e symlink non consentono letture o scritture oltre i confini autorizzati del progetto; input non validi vengono rifiutati preservando lo stato.
- **SEC-02**: Installazione, inizializzazione e iterazioni preservano file dell’utente e artefatti esistenti; conflitti e aggiornamenti sono espliciti e ripetibili.
- **SEC-03**: Approvazioni umane sono registrate solo se espresse e pertinenti; modifiche invalidano i fingerprint previsti. Skill e CLI non inventano test, comunicazioni o decisioni.
- **SLO-01**: Comandi, schemi JSON versionati, adapter e contratti restano compatibili oppure documentano migrazione e impatto; versioni non supportate sono rifiutate.
- **SLO-02**: Le verifiche pertinenti passano: test comportamentali, fmt, Clippy e doctor per codice o risorse incorporate; regressioni dei gate hanno test e mutanti mirati, con evidenze red/green per lo sviluppo.
- **SLO-03**: Scritture atomiche per file e lock preservano il contratto documentato anche in caso di errore; non si promette atomicità di transazioni su più file.
- **COST-01**: Il binario per l’utilizzatore funziona senza toolchain, interpreti, rete o servizi AI; dipendenze e costi di sviluppo sono distinti da quelli di esecuzione.
- **COST-02**: Le verifiche sono proporzionate al cambiamento; tempi e risorse sono misurati quando rilevanti, con soglie motivate da dati e senza budget inventati.
- **UX-01**: Help, documentazione e skill condividono comandi e prerequisiti; la specifica può essere ripresa senza sovrascritture, espone i dubbi e li risolve iterativamente.
- **UX-02**: Skill e adapter sono coerenti e reperibili; controlli statici e prove reali nell’harness sono distinti, con istruzioni operative verificabili.

## 2. Baseline da migliorare

Queste lacune non sono soddisfatte dal setup e non autorizzano evidenze simulate. Diventano bloccanti quando un cambiamento dipende dalla capacità mancante.

- **SLO-04**: Valutare e attivare CI quando necessaria per rischio e destinazione del rilascio, raccogliendo esiti reali; in solo sviluppo mantenere una suite finale locale tracciata.
- **UX-03**: Eseguire un pilota reale delle skill nell’harness Codex e verificare gli altri harness prima di dichiararne la validazione operativa.

## 3. Evidenze e revisione

La [baseline](assessment.md) descrive ciò che è osservato oggi. Il [catalogo](criteria.json) è il riferimento macchina; gli identificatori restano stabili. Non sono pertinenti SLO di disponibilità di un servizio web: il prodotto è un toolkit locale. Tempi di esecuzione e consumo di risorse richiedono misure prima di fissare soglie.

Aggiornare DoD e catalogo insieme dopo cambiamenti di contratto, regressioni o learning; riconfermare le review invalidate. Le evidenze locali non certificano CI o sistemi operativi non provati.
