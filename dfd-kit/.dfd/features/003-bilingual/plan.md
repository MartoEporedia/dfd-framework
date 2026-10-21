# Piano di sviluppo bilingue

## Ambito

Un task integrato `bilingual` copre OBS-01, OBS-02, SEC-01, SEC-02, SEC-03, SLO-01, SLO-02, COST-01, UX-01, UX-02. Implementare il supporto trasversale it/en e le risorse corrispondenti senza cambiare contratti macchina o testo utente.

## Verifiche

La regressione CLI red dimostra che `--lang` e `language` mancano nel binario precedente. Green verifica lingua persistente/temporanea, risorse inglesi, conflitti prima delle scritture, preservazione, schemi e symlink, approvazioni correnti e quattro adapter con PATH vuoto. Le traduzioni editoriali usano controlli utili di parità, link, anchor, ID, catalogo e revisione semantica; nessun red artificiale documentale. Suite finale Rust, test Python, fmt, Clippy, build/doctor e mutanti mirati.

## Maturità

Solo sviluppo, nessuna CI simulata, release o prova harness dichiarata. I documenti storici dei domini e delle feature restano preservati.
