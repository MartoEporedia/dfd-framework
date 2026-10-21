<!-- dfd-language --> [Italiano](glossario.md) | [English](en/glossario.md)

# Glossario e manutenzione delle lingue

Il framework offre due percorsi completi: italiano nei file originali e inglese in `en/`. Il toolkit segue la stessa convenzione in `dfd-kit/en/`. I nomi dei file rimangono comuni per semplificare riferimenti, installazione e corrispondenze; titoli e contenuti seguono la lingua scelta.

| Italiano | English | Convenzione |
|---|---|---|
| DoD estesa | Extended Definition of Done | Conservare DoD |
| Specifica di feature | Feature specification | Il design vive nella specifica |
| Criterio | Criterion | ID invariato, es. SEC-01 |
| Osservabilità | Observability | OBS |
| Sicurezza | Security | SEC |
| Affidabilità | Reliability | SLO |
| Costo | Cost | COST |
| Percorso rapido | Rapid route | Comando quick, enum rapid |
| Percorso light | Light route | light invariato |
| Full proporzionato | Proportionate full | Enum full-proportional |
| Full completo | Complete full | Enum full-complete |
| Solo sviluppo | Development only | Enum development-only |
| Preparazione al rilascio | Release preparation | Enum release-preparation |
| Evidenza | Evidence | Una verifica pianificata non è una prova eseguita |
| Responsabile | Owner / accountable person | Distinguere i ruoli RACI |
| Review | Review | Richiede la decisione umana prevista |
| Guardrail | Guardrail | Vincolo normativo |
| Da riconfermare | Stale review | Enum stale-review |

## Regole di manutenzione

1. Aggiornare entrambe le lingue nello stesso cambiamento. Non trasformare una traduzione in una modifica del comportamento: soglie, ruoli, criteri, condizioni e distinzioni tra controlli automatici e decisioni umane devono essere equivalenti.
2. Conservare comandi, chiavi JSON, enum, ID dei criteri, nomi dei file e riferimenti macchina. Adattare il testo in inglese naturale, mantenendo tutto il contenuto operativo. Gli esempi restano esempi, i placeholder restano da compilare.
3. Preservare i percorsi italiani. Ogni pagina ha un collegamento alla corrispondente lingua; gli alias degli anchor italiani mantengono funzionanti i riferimenti nei percorsi inglesi.
4. La radice contiene il framework canonico. Le copie del toolkit adottano esplicitamente le revisioni; il template di specifica del toolkit resta distinto da quello generale. Non tradurre automaticamente artefatti già compilati sotto `.dfd/` o copie installate delle skill.
5. Eseguire `python3 dfd-kit/scripts/check_translations.py`: verifica corrispondenze, hash di sincronizzazione, link/anchor, ID dei criteri e catalogo dei messaggi. Dopo la revisione semantica di una coppia aggiornata usare `--update-manifest`; l’hash registra l’allineamento dichiarato, senza certificare la qualità della traduzione.
6. Per codice e risorse incorporate eseguire anche fmt, Clippy, test e doctor secondo CONTRIBUTING. Il binario incorpora entrambe le lingue senza servizi esterni. La navigazione fra lingue è per i sorgenti: il bundle installato contiene il percorso scelto, con riferimenti locali coerenti.

## Lingua del toolkit

```sh
dfd --lang en init --harness codex --domain checkout
dfd language it
dfd language en
dfd --lang it status
```

`init` e `install` con `--lang` salvano la preferenza; `language` aggiorna risorse gestite e preferenza usando l’adapter attuale. Negli altri comandi `--lang` vale per la sola invocazione. Senza `.dfd/language.json` la lingua è italiana. Il formato è `{"schema_version":1,"language":"en"}`. Le preferenze non valide sono errori espliciti.

I documenti e template locali già compilati rimangono nella lingua originale. I nuovi domini usano la lingua selezionata; per cambiare i template di un dominio esistente occorre rivederli esplicitamente. Il cambio lingua rifiuta conflitti prima di aggiornare file gestiti e preferenza, con atomicità per singolo file.
