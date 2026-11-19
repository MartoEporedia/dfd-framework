# Piano – 014-quoted-placeholders

## Task `quoted-placeholders`

- Criteri: SEC-03, SLO-01, SLO-02, COST-02, UX-01.
- Implementazione: `without_quotations` in `src/lib.rs` rimuove il testo tra virgolette doppie, il codice inline e i blocchi recintati prima del controllo dei segnaposto in `validate_design`; due mutanti mirati; CHANGELOG EN/IT; versione 0.3.1.
- Verifica: red con `src/lib.rs` prima del fix (commit `1405854`) e green di `cargo test quoted_placeholders_do_not_block_review`; mutanti; suite locale; CI reale sulla PR.
