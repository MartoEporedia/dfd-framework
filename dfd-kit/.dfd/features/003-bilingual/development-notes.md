# Verifica dello sviluppo bilingue

Il cambiamento mantiene equivalenza normativa, identificatori macchina e protezioni dei gate. Le stringhe si localizzano prima di interpolare valori utente; nessuna sostituzione regex sul contenuto dell’utente. La lingua corrente è limitata all’invocazione e ripristinata al termine. La preferenza di progetto si legge sotto il lock dei comandi che modificano lo stato.

Il cambio lingua aggiorna solo risorse gestite dopo il preflight e preserva documenti/template locali. Le scritture rimangono atomiche per file. Il bootstrap in dist resta distinto dal binario candidato in target/release. Nessun rilascio, CI remota o pilota reale degli harness è dichiarato.

I mutanti mirati coprono scelta del bundle, lettura della preferenza e rifiuto dei file modificati. Le modifiche di localizzazione non alterano gli algoritmi di rischio o fingerprint; i mutanti preesistenti restano nel manifest.
