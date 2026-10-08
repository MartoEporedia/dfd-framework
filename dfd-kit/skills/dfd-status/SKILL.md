---
name: dfd-status
description: Riprendere il lavoro DFD da un altro harness o sessione, leggere fase, rischio, decisioni e blocchi e individuare la prossima azione.
---

# Riprendere il workflow DFD

Eseguire `dfd status [<id>]` nella radice del repository adottante. Senza identificatore elenca setup dei domini e feature; con identificatore verifica anche il gate di design. Usare `.dfd/kit/templates/contratto_toolkit.md` per interpretare i campi.

- Lo stato vive in `.dfd/`, non nella cronologia della conversazione. Leggere solo gli artefatti del dominio e della feature pertinenti.
- Se la Fase 0 è incompleta, proporre `dfd-setup` e mostrare le lacune del dominio.
- Se mancano dati di rischio, proporre `dfd-assess`; se manca la specifica, `dfd-specify`; se il design è pronto, `dfd-review-design`.
- Dopo la review approvata, proporre `dfd-plan`, poi `dfd-implement` o `dfd-verify` secondo il gate di sviluppo. `ready-for-pre-release` propone `dfd-pre-release`; con preparazione completa usare `dfd-review-release`.
- Il campo `release` mostra gate, storico delle decisioni, checklist light/full, attivazioni pianificate e condizioni. Distinguere `rollout`, `rollout-with-conditions` e `resolve-release-conditions` senza dichiarare eseguito il rilascio.
- Distinguere `blocked`, `awaiting-human-review`, `stale-review`, `changes-requested`, `approved` e `approved-with-conditions`. Mostrare condizioni, responsabili e fase prevista.
- Un report strutturale positivo non è un'approvazione. Una decisione precedente non vale per documenti modificati. Non correggere manualmente i fingerprint o inventare evidenze per avanzare il gate.
- Nel brownfield richiamare livello corrente e target del dominio e lacune baseline pertinenti. Il solo uso di una skill non aumenta il livello di adozione.

Presentare fase, blocco concreto e prossimo passo. Il campo `development` distingue piano e verifiche di sviluppo; non dichiara il rilascio completato.
