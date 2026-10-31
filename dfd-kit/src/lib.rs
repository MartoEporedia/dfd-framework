pub mod cli;
mod collaboration;
mod development;
pub mod i18n;
pub mod model;
mod quick;
mod release;
mod store;

use chrono::{DateTime, Utc};
use cli::{Cli, Command, Harness};
use model::*;
use regex::Regex;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    sync::OnceLock,
};
use store::{hash, Store};

mod bundle {
    include!(concat!(env!("OUT_DIR"), "/bundle.rs"));
}
const SKILLS: [&str; 12] = [
    "dfd-init",
    "dfd-setup",
    "dfd-feature",
    "dfd-assess",
    "dfd-specify",
    "dfd-review-design",
    "dfd-plan",
    "dfd-implement",
    "dfd-verify",
    "dfd-pre-release",
    "dfd-review-release",
    "dfd-status",
];
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Error(String);
impl Error {
    pub fn message(value: impl Into<String>) -> Self {
        Self(i18n::text(&value.into()))
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self(error.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }
}
pub type Result<T> = std::result::Result<T, Error>;

fn slug(value: &str) -> Result<()> {
    static RE: OnceLock<Regex> = OnceLock::new();
    if value.len() > 64
        || !RE
            .get_or_init(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap())
            .is_match(value)
    {
        return Err(Error::message(
            "Identificatore non valido: lowercase e trattini, massimo 64 caratteri.",
        ));
    }
    Ok(())
}
fn nonblank(value: &str) -> bool {
    !value.trim().is_empty()
}
fn resource(name: &str) -> Result<&'static str> {
    i18n::files()
        .iter()
        .find(|(path, _)| *path == name)
        .map(|(_, body)| *body)
        .ok_or_else(|| {
            Error::message(crate::localized_format!(
                "Risorsa incorporata assente: {name}",
                "Missing embedded resource: {name}",
                name = name
            ))
        })
}
fn config(store: &Store) -> Result<Config> {
    let config: Config = store.json(".dfd/config.json")?;
    let mut seen = BTreeSet::new();
    for domain in &config.domains {
        slug(domain)?;
        if !seen.insert(domain) {
            return Err(Error::message("Domini duplicati in config.json."));
        }
    }
    Ok(config)
}
fn adoption(store: &Store, domain: &str) -> Result<Adoption> {
    slug(domain)?;
    let adoption: Adoption = store.json(&format!(".dfd/domains/{domain}/adoption.json"))?;
    if adoption.current_level > 3
        || adoption.target_level > 3
        || adoption
            .areas
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>()
            != AREAS.into_iter().collect()
    {
        return Err(Error::message(
            "Assessment di adozione non valido: livelli 0–3 e sei aree richieste.",
        ));
    }
    for (name, area) in &adoption.areas {
        if matches!(area.status, AdoptionStatus::Observed)
            && (area.evidence.is_empty() || area.evidence.iter().any(|source| !nonblank(source)))
        {
            return Err(Error::message(crate::localized_format!(
                "Area osservata senza evidenze: {name}",
                "Observed area without evidence: {name}",
                name = name
            )));
        }
    }
    Ok(adoption)
}
fn state(store: &Store, id: &str) -> Result<State> {
    slug(id)?;
    let state: State = store.json(&format!(".dfd/features/{id}/state.json"))?;
    slug(&state.domain)?;
    if state.id != id || !nonblank(&state.title) || !nonblank(&state.scope) {
        return Err(Error::message(
            "Stato della feature incoerente o senza titolo/ambito.",
        ));
    }
    if !config(store)?.domains.contains(&state.domain) {
        return Err(Error::message(
            "Dominio della feature non registrato nel progetto.",
        ));
    }
    Ok(state)
}

pub fn classify(risk: &Risk, kind: Kind) -> Result<Classification> {
    if risk
        .dimensions
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>()
        != DIMENSIONS.into_iter().collect()
    {
        return Err(Error::message(
            "Sono richieste esattamente le cinque dimensioni DFD.",
        ));
    }
    let missing: Vec<String> = DIMENSIONS
        .iter()
        .filter(|name| {
            let dimension = &risk.dimensions[**name];
            dimension.level.is_none() || !nonblank(&dimension.rationale)
        })
        .map(|name| name.to_string())
        .collect();
    if !missing.is_empty() {
        return Ok(Classification {
            risk: None,
            route: None,
            missing,
        });
    }
    let levels: Vec<Level> = risk
        .dimensions
        .values()
        .filter_map(|item| item.level)
        .collect();
    let level = if levels.contains(&Level::High) {
        Level::High
    } else if levels
        .iter()
        .filter(|level| **level == Level::Medium)
        .count()
        >= 2
    {
        Level::Medium
    } else {
        Level::Low
    };
    let route = if kind == Kind::NewService || level == Level::High {
        Route::FullComplete
    } else if level == Level::Medium {
        Route::FullProportional
    } else {
        Route::Light
    };
    Ok(Classification {
        risk: Some(level),
        route: Some(route),
        missing,
    })
}
fn assessment_fingerprint(store: &Store, state: &State) -> Result<String> {
    Ok(hash(&serde_json::to_vec(&json!({
        "risk": store.hash(&format!(".dfd/features/{}/risk.json", state.id))?, "kind": state.kind,
    }))?))
}
fn current_risk(store: &Store, state: &State) -> Result<Classification> {
    let risk: Risk = store.json(&format!(".dfd/features/{}/risk.json", state.id))?;
    let result = classify(&risk, state.kind)?;
    if !result.missing.is_empty()
        || state.assessment_hash.as_deref() != Some(&assessment_fingerprint(store, state)?)
    {
        return Err(Error::message(
            "Assessment assente o modificato: completare risk.json ed eseguire assess.",
        ));
    }
    if state.risk != result.risk || state.route != result.route {
        return Err(Error::message(
            "Stato incoerente con il rischio: rieseguire assess.",
        ));
    }
    Ok(result)
}

fn install(store: &Store, harness: Harness) -> Result<Value> {
    doctor()?;
    let old = if store.path(".dfd/install.json")?.exists() {
        store.json::<Manifest>(".dfd/install.json")?
    } else {
        Manifest {
            schema_version: 1,
            version: VERSION.into(),
            harness: harness.name().into(),
            files: BTreeMap::new(),
        }
    };
    let mut payload = BTreeMap::new();
    for (relative, body) in i18n::files() {
        payload.insert(format!(".dfd/kit/{relative}"), *body);
        if let Some(skill) = relative.strip_prefix("skills/") {
            payload.insert(format!("{}/{skill}", harness.directory()), *body);
        }
    }
    // Validate all ownership and conflicts before changing any managed file.
    for (relative, expected) in &old.files {
        let is_skill = Harness::ALL.iter().any(|adapter| {
            relative.starts_with(&format!("{}/dfd-", adapter.directory()))
                && relative.ends_with("/SKILL.md")
        });
        if !relative.starts_with(".dfd/kit/") && !is_skill {
            return Err(Error::message(crate::localized_format!(
                "Voce non gestibile nel manifest: {relative}",
                "Unmanaged manifest entry: {relative}",
                relative = relative
            )));
        }
        let path = store.path(relative)?;
        if path.exists() && (!path.is_file() || store.hash(relative)? != *expected) {
            return Err(Error::message(crate::localized_format!(
                "Installazione annullata; file modificato preservato: {relative}",
                "Installation cancelled; modified file preserved: {relative}",
                relative = relative
            )));
        }
    }
    for relative in payload.keys() {
        if store.path(relative)?.exists() && !old.files.contains_key(relative) {
            return Err(Error::message(crate::localized_format!(
                "Installazione annullata; file non gestito preservato: {relative}",
                "Installation cancelled; unmanaged file preserved: {relative}",
                relative = relative
            )));
        }
    }
    for adapter in Harness::ALL {
        let directory = store.path(adapter.directory())?;
        if !directory.exists() {
            continue;
        }
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.starts_with("dfd-") {
                continue;
            }
            let relative = format!("{}/{name}/SKILL.md", adapter.directory());
            let path = store.path(&relative)?;
            if path.exists() && !old.files.contains_key(&relative) {
                return Err(Error::message(crate::localized_format!(
                    "Skill DFD non gestita nel percorso di compatibilità: {relative}",
                    "Unmanaged DFD skill in compatibility path: {relative}",
                    relative = relative
                )));
            }
        }
    }
    for (relative, content) in &payload {
        store.write(relative, content.as_bytes(), false)?;
    }
    for relative in old
        .files
        .keys()
        .filter(|relative| !payload.contains_key(*relative))
    {
        let path = store.path(relative)?;
        if path.exists() {
            fs::remove_file(&path)?;
        }
        // Do not remove supporting files belonging to the team.
        if relative.ends_with("/SKILL.md") {
            let _ = fs::remove_dir(path.parent().unwrap());
        }
    }
    let files = payload
        .iter()
        .map(|(path, body)| (path.clone(), hash(body.as_bytes())))
        .collect();
    store.save(
        ".dfd/install.json",
        &Manifest {
            schema_version: 1,
            version: VERSION.into(),
            harness: harness.name().into(),
            files,
        },
        false,
    )?;
    Ok(json!({"harness": harness.name(), "version": VERSION, "skills": SKILLS.len()}))
}

const DOMAIN_TEMPLATES: [&str; 8] = [
    "dod-estesa.md",
    "specifica_feature.md",
    "review_design.md",
    "review_design_light.md",
    "review_release.md",
    "review_release_light.md",
    "configurazione_rollout.md",
    "review_indice.md",
];

fn setup_files() -> Vec<String> {
    let mut files = vec!["guardrails.md".into(), "process.md".into()];
    files.extend(
        DOMAIN_TEMPLATES
            .iter()
            .map(|name| format!("templates/{name}")),
    );
    files
}

fn lifecycle(store: &Store, domain: &str) -> Result<Lifecycle> {
    let relative = format!(".dfd/domains/{domain}/lifecycle.json");
    if !store.path(&relative)?.exists() {
        return Ok(Lifecycle {
            schema_version: 1,
            mode: LifecycleMode::ReleasePreparation,
            ci_required: true,
            rationale: crate::i18n::text(
                "Compatibilità legacy: CI richiesta prima della pre-release.",
            ),
        });
    }
    let policy: Lifecycle = store.json(&relative)?;
    if !nonblank(&policy.rationale)
        || (policy.mode == LifecycleMode::DevelopmentOnly && policy.ci_required)
    {
        return Err(Error::message("Policy lifecycle: motivare la modalità e la scelta CI; development-only richiede ci_required false."));
    }
    Ok(policy)
}

fn setup_errors(store: &Store, domain: &str) -> Result<Vec<String>> {
    slug(domain)?;
    if !config(store)?.domains.contains(&domain.to_string()) {
        return Err(Error::message("Dominio non inizializzato: eseguire init."));
    }
    let root = format!(".dfd/domains/{domain}");
    let mut errors = vec![];
    lifecycle(store, domain)?;
    collaboration::policy(store, domain)?;
    for file in ["dod.md", "guardrails.md", "process.md"] {
        let relative = format!("{root}/{file}");
        if !store.path(&relative)?.is_file() {
            errors.push(crate::localized_format!(
                "Fase 0: manca {relative}; eseguire init e completare il setup.",
                "Phase 0: missing {relative}; run init and complete setup.",
                relative = relative
            ));
            continue;
        }
        let body = store.read(&relative)?;
        if !nonblank(&body)
            || [
                "TODO",
                "YYYY-MM-DD",
                "[Prodotto / Dominio / Servizio]",
                "[Product / Domain / Service]",
            ]
            .iter()
            .any(|marker| body.contains(marker))
        {
            errors.push(crate::localized_format!(
                "Fase 0: completare {relative}.",
                "Phase 0: complete {relative}.",
                relative = relative
            ));
        }
    }
    for name in DOMAIN_TEMPLATES {
        let relative = format!("{root}/templates/{name}");
        if !store.path(&relative)?.is_file() || !nonblank(&store.read(&relative)?) {
            errors.push(crate::localized_format!(
                "Fase 0: template locale assente o vuoto: {relative}.",
                "Phase 0: local template missing or empty: {relative}.",
                relative = relative
            ));
        }
    }
    let relative = format!("{root}/criteria.json");
    if !store.path(&relative)?.is_file() {
        errors.push(crate::localized_format!(
            "Fase 0: manca {relative}.",
            "Phase 0: missing {relative}.",
            relative = relative
        ));
        return Ok(errors);
    }
    let catalog: Catalog = store.json(&relative)?;
    if catalog.criteria.is_empty() {
        errors.push(crate::i18n::text("Fase 0: catalogo DoD vuoto."));
    }
    let dod = format!("{root}/dod.md");
    let dod_ids = if store.path(&dod)?.is_file() {
        criterion_ids(&store.read(&dod)?)
    } else {
        BTreeSet::new()
    };
    let mut ids = BTreeSet::new();
    for item in &catalog.criteria {
        if !valid_criterion(&item.id)
            || !nonblank(&item.statement)
            || item.statement.contains("TODO")
        {
            errors.push(crate::localized_format!(
                "Fase 0: criterio non valido: {}.",
                "Phase 0: invalid criterion: {}.",
                item.id
            ));
        }
        if !ids.insert(item.id.clone()) {
            errors.push(crate::localized_format!(
                "Fase 0: criterio duplicato: {}.",
                "Phase 0: duplicate criterion: {}.",
                item.id
            ));
        }
        if !dod_ids.contains(&item.id) {
            errors.push(crate::localized_format!(
                "Fase 0: criterio {} assente in dod.md.",
                "Phase 0: criterion {} missing in dod.md.",
                item.id
            ));
        }
    }
    for id in dod_ids.difference(&ids) {
        errors.push(crate::localized_format!(
            "Fase 0: criterio {id} assente nel catalogo.",
            "Phase 0: criterion {id} missing from catalog.",
            id = id
        ));
    }
    Ok(errors)
}

fn setup_status(store: &Store, domain: &str) -> Result<Value> {
    let errors = setup_errors(store, domain)?;
    Ok(
        json!({"domain":domain,"phase":"domain-setup","gate":if errors.is_empty() {"ready"} else {"blocked"},"errors":errors,
        "lifecycle":lifecycle(store, domain)?,
        "next":if errors.is_empty() {"dfd-assess"} else {"dfd-setup"}}),
    )
}

fn setup(store: &Store, domain: &str) -> Result<(Value, i32)> {
    let output = setup_status(store, domain)?;
    let exit = if output["gate"] == "ready" { 0 } else { 2 };
    Ok((output, exit))
}

fn init(store: &Store, harness: Harness, domain: &str, mode: Mode) -> Result<Value> {
    slug(domain)?;
    let mut config = if store.path(".dfd/config.json")?.exists() {
        config(store)?
    } else {
        Config {
            schema_version: 1,
            mode,
            domains: vec![],
            created_at: Utc::now(),
        }
    };
    if config.mode != mode {
        return Err(Error::message(
            "Progetto già inizializzato in un'altra modalità.",
        ));
    }
    let domain_root = format!(".dfd/domains/{domain}");
    let present = config.domains.iter().any(|name| name == domain);
    if store.path(&domain_root)?.exists() && !present {
        return Err(Error::message(
            "Dominio non registrato già presente: preservato.",
        ));
    }
    if present {
        adoption(store, domain)?;
    }
    // Preflight user artifact paths before installation.
    for file in ["dod.md", "criteria.json", "adoption.json", "assessment.md"] {
        store.path(&format!("{domain_root}/{file}"))?;
    }
    for file in setup_files() {
        store.path(&format!("{domain_root}/{file}"))?;
    }
    store.path(".dfd/guardrails.md")?;
    install(store, harness)?;
    if !present {
        store.write(
            &format!("{domain_root}/dod.md"),
            resource("templates/dod-estesa.md")?.as_bytes(),
            true,
        )?;
        store.save(
            &format!("{domain_root}/criteria.json"),
            &Catalog {
                schema_version: 1,
                criteria: vec![],
            },
            true,
        )?;
        let areas = AREAS
            .iter()
            .map(|name| {
                (
                    name.to_string(),
                    Area {
                        status: AdoptionStatus::Unknown,
                        evidence: vec![],
                        notes: String::new(),
                    },
                )
            })
            .collect();
        store.save(
            &format!("{domain_root}/adoption.json"),
            &Adoption {
                schema_version: 1,
                pilot: true,
                current_level: 0,
                target_level: if mode == Mode::Brownfield { 1 } else { 2 },
                areas,
                baseline_gaps: vec![],
                next_steps: vec![],
            },
            true,
        )?;
        store.write(
            &format!("{domain_root}/assessment.md"),
            resource("templates/assessment_adozione.md")?.as_bytes(),
            true,
        )?;
        config.domains.push(domain.to_string());
    }
    if !store.path(".dfd/guardrails.md")?.exists() {
        store.write(".dfd/guardrails.md", crate::i18n::text("# Guardrail del progetto\n\nTODO: collegare policy, architettura e vincoli esistenti.\n").as_bytes(), true)?;
    }
    for (file, source) in [
        ("guardrails.md", "guardrail_dominio.md"),
        ("process.md", "processo_dominio.md"),
    ] {
        let relative = format!("{domain_root}/{file}");
        if !store.path(&relative)?.exists() {
            store.write(
                &relative,
                resource(&format!("templates/{source}"))?.as_bytes(),
                true,
            )?;
        }
    }
    for name in DOMAIN_TEMPLATES
        .into_iter()
        .chain(["cambiamento_rapido.md", "collaborazione.md"])
    {
        let relative = format!("{domain_root}/templates/{name}");
        if !store.path(&relative)?.exists() {
            store.write(
                &relative,
                resource(&format!("templates/{name}"))?.as_bytes(),
                true,
            )?;
        }
    }
    store.save(".dfd/config.json", &config, false)?;
    Ok(
        json!({"domain": domain, "mode": mode, "harness": harness.name(), "next": crate::i18n::text("Completare la Fase 0 e verificare con dfd setup --domain <dominio>.")}),
    )
}

fn new_feature(
    store: &Store,
    id: &str,
    domain: &str,
    title: &str,
    scope: &str,
    kind: Kind,
    selective_review: bool,
) -> Result<Value> {
    slug(id)?;
    slug(domain)?;
    let config = config(store)?;
    if !config.domains.contains(&domain.to_string()) {
        return Err(Error::message("Dominio non inizializzato."));
    }
    let adoption = adoption(store, domain)?;
    if config.mode == Mode::Brownfield && !adoption.pilot && adoption.current_level < 2 {
        return Err(Error::message(
            "Dominio fuori dal pilota: definire esplicitamente l'ambito di adozione.",
        ));
    }
    if !nonblank(title) || !nonblank(scope) {
        return Err(Error::message("Titolo e ambito sono obbligatori."));
    }
    if quick::exists(store, id)? {
        return Err(Error::message("ID già usato da un record rapido."));
    }
    if store.path(&format!(".dfd/features/{id}"))?.exists() {
        return Err(Error::message(
            "Feature già presente: usare status per riprenderla.",
        ));
    }
    let state = State {
        schema_version: 1,
        id: id.into(),
        domain: domain.into(),
        title: title.into(),
        scope: scope.into(),
        kind,
        phase: "idea".into(),
        created_at: Utc::now(),
        risk: None,
        route: None,
        assessment_hash: None,
        decisions: vec![],
        selective_review,
    };
    let dimensions = DIMENSIONS
        .iter()
        .map(|name| {
            (
                name.to_string(),
                RiskDimension {
                    level: None,
                    rationale: String::new(),
                },
            )
        })
        .collect();
    store.save(&format!(".dfd/features/{id}/state.json"), &state, true)?;
    store.save(
        &format!(".dfd/features/{id}/risk.json"),
        &Risk {
            schema_version: 1,
            dimensions,
        },
        true,
    )?;
    Ok(
        json!({"feature": id, "next": crate::i18n::text("Compilare risk.json, poi eseguire assess.")}),
    )
}

fn assess(store: &Store, id: &str) -> Result<(Value, i32)> {
    let mut state = state(store, id)?;
    let risk: Risk = store.json(&format!(".dfd/features/{id}/risk.json"))?;
    let result = classify(&risk, state.kind)?;
    state.risk = result.risk;
    state.route = result.route;
    state.phase = if result.missing.is_empty() {
        "assessed"
    } else {
        "idea"
    }
    .into();
    state.assessment_hash = if result.missing.is_empty() {
        Some(assessment_fingerprint(store, &state)?)
    } else {
        None
    };
    let mut report = crate::localized_format!(
        "# Assessment del rischio DFD\n\nRischio: {}\nPercorso: {}\n\n",
        "# DFD risk assessment\n\nRisk: {}\nRoute: {}\n\n",
        serde_json::to_string(&result.risk)?,
        serde_json::to_string(&result.route)?
    );
    for name in DIMENSIONS {
        let dimension = &risk.dimensions[name];
        report.push_str(&format!(
            "- {name}: {} — {}\n",
            serde_json::to_string(&dimension.level)?,
            dimension.rationale
        ));
    }
    report.push_str(&crate::i18n::text(
        "\nUna sola dimensione Medium segue light; nuovi servizi seguono full completo.\n",
    ));
    store.save(&format!(".dfd/features/{id}/state.json"), &state, false)?;
    store.write(
        &format!(".dfd/features/{id}/risk.md"),
        report.as_bytes(),
        false,
    )?;
    let exit = if result.missing.is_empty() { 0 } else { 2 };
    Ok((serde_json::to_value(result)?, exit))
}

fn specify(store: &Store, id: &str) -> Result<Value> {
    let mut state = state(store, id)?;
    let errors = setup_errors(store, &state.domain)?;
    if !errors.is_empty() {
        return Err(Error::message(crate::localized_format!(
            "Setup del dominio incompleto: {}",
            "Incomplete domain setup: {}",
            errors.join("; ")
        )));
    }
    current_risk(store, &state)?;
    let spec_exists = store.path(&format!(".dfd/features/{id}/spec.md"))?.exists();
    let design_exists = store
        .path(&format!(".dfd/features/{id}/design.json"))?
        .exists();
    if spec_exists && design_exists {
        let design: Design = store.json(&format!(".dfd/features/{id}/design.json"))?;
        return Ok(
            json!({"feature":id,"resumed":true,"open_questions":design.open_questions,
            "next":crate::i18n::text("Riprendere spec.md e design.json: chiarire i dubbi e aggiornare entrambi iterativamente, poi review-design.")}),
        );
    }
    if spec_exists || design_exists {
        return Err(Error::message("Specifica parziale: preservata; ripristinare il file mancante prima di riprendere specify."));
    }
    let design = Design {
        schema_version: 1,
        owner: String::new(),
        objective: String::new(),
        business_metrics: vec![],
        epic: Epic {
            reference: String::new(),
            approved_by: String::new(),
            approved_at: String::new(),
        },
        criteria: vec![],
        test_plan: String::new(),
        guardrails: String::new(),
        open_questions: vec![],
    };
    store.save(&format!(".dfd/features/{id}/design.json"), &design, true)?;
    store.write(
        &format!(".dfd/features/{id}/spec.md"),
        store
            .read(&format!(
                ".dfd/domains/{}/templates/specifica_feature.md",
                state.domain
            ))?
            .as_bytes(),
        true,
    )?;
    state.phase = "design".into();
    store.save(&format!(".dfd/features/{id}/state.json"), &state, false)?;
    Ok(
        json!({"feature": id, "next": crate::i18n::text("Completare spec.md e design.json, poi review-design.")}),
    )
}

fn criterion_ids(markdown: &str) -> BTreeSet<String> {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\b(?:OBS|SEC|SLO|COST|UX)-[0-9]{2,}\b").unwrap())
        .find_iter(markdown)
        .map(|item| item.as_str().to_string())
        .collect()
}
/// Markdown units of dod.md: a list item with its continuation lines and deeper
/// sub-items, a table row, a heading or a paragraph. A line whose content starts with
/// a criterion ID always opens a new unit. Whitespace is normalized.
fn dod_blocks(markdown: &str) -> Vec<String> {
    static ITEM: OnceLock<Regex> = OnceLock::new();
    static LEAD: OnceLock<Regex> = OnceLock::new();
    let item = ITEM.get_or_init(|| Regex::new(r"^(\s*)(?:[-*+]|[0-9]+[.)])\s+").unwrap());
    let lead = LEAD
        .get_or_init(|| Regex::new(r"^[\s>*_`\[]*(?:OBS|SEC|SLO|COST|UX)-[0-9]{2,}\b").unwrap());
    let mut blocks = vec![];
    // Current unit and, for a list item, the indentation of its marker.
    let mut current: Option<(String, Option<usize>)> = None;
    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            blocks.extend(current.take().map(|(text, _)| text));
            continue;
        }
        let marker = item
            .captures(line)
            .map(|c| (c[1].len(), c.get(0).unwrap().end()));
        let content = marker.map_or(line, |(_, end)| &line[end..]);
        let nested = matches!(
            (&current, marker),
            (Some((_, Some(parent))), Some((indent, _))) if indent > *parent
        );
        let single = trimmed.starts_with('#') || trimmed.starts_with('|');
        if single || lead.is_match(content) || (marker.is_some() && !nested) || current.is_none() {
            blocks.extend(current.take().map(|(text, _)| text));
            current = Some((String::new(), marker.map(|(indent, _)| indent)));
        }
        let (text, _) = current.as_mut().unwrap();
        text.push(' ');
        text.push_str(trimmed);
        if single {
            blocks.extend(current.take().map(|(text, _)| text));
        }
    }
    blocks.extend(current.map(|(text, _)| text));
    // A heading citing a criterion also binds its whole section, down to the next
    // heading of the same or a higher level (DoD written as "### OBS-01" + text).
    let lines: Vec<_> = markdown.lines().collect();
    for (start, line) in lines.iter().enumerate() {
        let level = line.chars().take_while(|c| *c == '#').count();
        if level == 0 || criterion_ids(line).is_empty() {
            continue;
        }
        let mut section = String::new();
        for (index, next) in lines.iter().enumerate().skip(start) {
            let next_level = next.chars().take_while(|c| *c == '#').count();
            if index > start && next_level > 0 && next_level <= level {
                break;
            }
            section.push(' ');
            section.push_str(next.trim());
        }
        blocks.push(section);
    }
    blocks
        .iter()
        .map(|block| block.split_whitespace().collect::<Vec<_>>().join(" "))
        .collect()
}
/// True when a block holds only criterion IDs, numbers and markup.
fn bare_reference(block: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    !RE.get_or_init(|| Regex::new(r"\b(?:OBS|SEC|SLO|COST|UX)-[0-9]{2,}\b").unwrap())
        .replace_all(block, "")
        .chars()
        .any(char::is_alphabetic)
}
fn valid_criterion(value: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^(?:OBS|SEC|SLO|COST|UX)-[0-9]{2,}$").unwrap())
        .is_match(value)
}
fn validate_design(store: &Store, state: &State) -> Result<Vec<String>> {
    let mut errors = setup_errors(store, &state.domain)?;
    if let Err(error) = current_risk(store, state) {
        errors.push(error.to_string());
    }
    let root = format!(".dfd/features/{}", state.id);
    if !store.path(&format!("{root}/spec.md"))?.is_file()
        || !store.path(&format!("{root}/design.json"))?.is_file()
    {
        errors.push(crate::i18n::text("Specifica assente: eseguire specify."));
        return Ok(errors);
    }
    let design: Design = store.json(&format!("{root}/design.json"))?;
    let spec = store.read(&format!("{root}/spec.md"))?;
    for placeholder in ["TODO", "[Nome Feature]", "[Feature Name]", "YYYY-MM-DD"] {
        if spec.contains(placeholder) {
            errors.push(crate::i18n::text(
                "Specifica con placeholder da completare.",
            ));
            break;
        }
    }
    for (name, value) in [
        ("owner", &design.owner),
        ("objective", &design.objective),
        ("test_plan", &design.test_plan),
        ("guardrails", &design.guardrails),
    ] {
        if !nonblank(value) {
            errors.push(crate::localized_format!(
                "design.json: manca {name}.",
                "design.json: missing {name}.",
                name = name
            ));
        }
    }
    if design.business_metrics.is_empty()
        || design.business_metrics.iter().any(|value| !nonblank(value))
    {
        errors.push(crate::i18n::text(
            "Definire almeno una metrica di successo.",
        ));
    }
    if !nonblank(&design.epic.reference) || !nonblank(&design.epic.approved_by) {
        errors.push(crate::i18n::text(
            "Riferimento e approvazione dell'epic mancanti.",
        ));
    }
    match DateTime::parse_from_rfc3339(&design.epic.approved_at) {
        Ok(date) if date <= Utc::now() => {}
        _ => errors.push(crate::i18n::text(
            "Data approvazione epic non valida: usare ISO 8601 con timezone, non futura.",
        )),
    }
    if !design.open_questions.is_empty() {
        errors.push(crate::i18n::text("Domande di design ancora aperte."));
    }
    let domain_root = format!(".dfd/domains/{}", state.domain);
    adoption(store, &state.domain)?;
    let catalog: Catalog = store.json(&format!("{domain_root}/criteria.json"))?;
    if catalog.criteria.is_empty() {
        errors.push(crate::i18n::text(
            "Catalogo DoD vuoto: adattare la DoD del dominio.",
        ));
        return Ok(errors);
    }
    let dod_ids = criterion_ids(&store.read(&format!("{domain_root}/dod.md"))?);
    let spec_ids = criterion_ids(&spec);
    let mut ids = BTreeSet::new();
    for item in &catalog.criteria {
        if !valid_criterion(&item.id) || !nonblank(&item.statement) {
            errors.push(crate::localized_format!(
                "Criterio DoD non valido: {}.",
                "Invalid DoD criterion: {}.",
                item.id
            ));
        }
        if !ids.insert(item.id.clone()) {
            errors.push(crate::localized_format!(
                "Criterio DoD duplicato: {}.",
                "Duplicate DoD criterion: {}.",
                item.id
            ));
        }
        if !dod_ids.contains(&item.id) {
            errors.push(crate::localized_format!(
                "Criterio {} non presente in dod.md.",
                "Criterion {} not present in dod.md.",
                item.id
            ));
        }
    }
    let mut covered = BTreeSet::new();
    let mut applicable = 0;
    for item in &design.criteria {
        if !ids.contains(&item.id) {
            errors.push(crate::localized_format!(
                "Criterio sconosciuto: {}.",
                "Unknown criterion: {}.",
                item.id
            ));
        }
        if !covered.insert(item.id.clone()) {
            errors.push(crate::localized_format!(
                "Criterio duplicato nella specifica: {}.",
                "Duplicate criterion in specification: {}.",
                item.id
            ));
        }
        match item.applicability {
            Applicability::Applicable => {
                applicable += 1;
                if !nonblank(&item.implementation) || !nonblank(&item.verification) {
                    errors.push(crate::localized_format!(
                        "Criterio {}: implementazione/verifica mancanti.",
                        "Criterion {}: missing implementation/verification.",
                        item.id
                    ));
                }
                if !spec_ids.contains(&item.id) {
                    errors.push(crate::localized_format!(
                        "Criterio {} non citato in spec.md.",
                        "Criterion {} not referenced in spec.md.",
                        item.id
                    ));
                }
            }
            Applicability::Excluded if !nonblank(&item.justification) => {
                errors.push(crate::localized_format!(
                    "Esclusione {} senza motivazione.",
                    "Exclusion {} has no justification.",
                    item.id
                ))
            }
            Applicability::Excluded => {}
        }
    }
    for item in catalog
        .criteria
        .iter()
        .filter(|item| item.scope == CriterionScope::Changes && !covered.contains(&item.id))
    {
        errors.push(crate::localized_format!(
            "Criterio per il nuovo lavoro non valutato: {}.",
            "Criterion for new work has not been assessed: {}.",
            item.id
        ));
    }
    if applicable == 0 {
        errors.push(crate::i18n::text(
            "Collegare almeno un criterio DoD applicabile.",
        ));
    }
    for id in spec_ids.difference(&ids) {
        errors.push(crate::localized_format!(
            "Riferimento DoD sconosciuto in spec.md: {id}.",
            "Unknown DoD reference in spec.md: {id}.",
            id = id
        ));
    }
    Ok(errors)
}

fn design_fingerprint(store: &Store, state: &State) -> Result<String> {
    let mut hashes = BTreeMap::new();
    for file in ["spec.md", "design.json", "risk.json"] {
        let relative = format!(".dfd/features/{}/{file}", state.id);
        hashes.insert(relative.clone(), store.hash(&relative)?);
    }
    if state.selective_review {
        let design: Design = store.json(&format!(".dfd/features/{}/design.json", state.id))?;
        let catalog: Catalog =
            store.json(&format!(".dfd/domains/{}/criteria.json", state.domain))?;
        let ids: BTreeSet<_> = design.criteria.iter().map(|c| c.id.as_str()).collect();
        let mut criteria: Vec<_> = catalog
            .criteria
            .iter()
            .filter(|c| ids.contains(c.id.as_str()))
            .collect();
        criteria.sort_by(|a, b| a.id.cmp(&b.id));
        hashes.insert(
            "selected-criteria".into(),
            hash(&serde_json::to_vec(&criteria)?),
        );
        // The DoD text people read is a dependency too (issue #5): for each selected
        // criterion, the dod.md blocks citing it, with whitespace normalized.
        let dod = store.read(&format!(".dfd/domains/{}/dod.md", state.domain))?;
        let blocks = dod_blocks(&dod);
        let selected: BTreeMap<_, Vec<_>> = ids
            .iter()
            .map(|id| {
                let citing: Vec<_> = blocks
                    .iter()
                    .filter(|block| criterion_ids(block).contains(*id))
                    .cloned()
                    .collect();
                // Cited only as a bare ID: the descriptive text is elsewhere, so the
                // whole dod.md stays a dependency.
                if citing.iter().all(|block| bare_reference(block)) {
                    (*id, vec![dod.clone()])
                } else {
                    (*id, citing)
                }
            })
            .collect();
        hashes.insert("selected-dod".into(), hash(&serde_json::to_vec(&selected)?));
        let guardrails = format!(".dfd/domains/{}/guardrails.md", state.domain);
        hashes.insert(guardrails.clone(), store.hash(&guardrails)?);
    } else {
        for file in ["dod.md", "criteria.json", "adoption.json"] {
            let relative = format!(".dfd/domains/{}/{file}", state.domain);
            hashes.insert(relative.clone(), store.hash(&relative)?);
        }
        for file in setup_files() {
            let relative = format!(".dfd/domains/{}/{file}", state.domain);
            hashes.insert(
                relative.clone(),
                if store.path(&relative)?.is_file() {
                    store.hash(&relative)?
                } else {
                    "absent".into()
                },
            );
        }
    }
    // Absent policy preserves legacy fingerprints; creating or removing it invalidates reviews.
    let policy = format!(".dfd/domains/{}/lifecycle.json", state.domain);
    if store.path(&policy)?.exists() {
        hashes.insert(policy.clone(), store.hash(&policy)?);
    }
    hashes.insert(
        ".dfd/guardrails.md".into(),
        store.hash(".dfd/guardrails.md")?,
    );
    if state.selective_review {
        hashes.insert(
            "project-context".into(),
            hash(&serde_json::to_vec(
                &json!({"mode":config(store)?.mode,"domain":state.domain}),
            )?),
        );
    } else {
        hashes.insert(".dfd/config.json".into(), store.hash(".dfd/config.json")?);
    }
    let notes = format!(".dfd/features/{}/review-notes.md", state.id);
    hashes.insert(
        notes.clone(),
        if store.path(&notes)?.exists() {
            store.hash(&notes)?
        } else {
            "absent".into()
        },
    );
    Ok(hash(&serde_json::to_vec(
        &json!({"files":hashes,"context":{
            "id":state.id,"domain":state.domain,"title":state.title,"scope":state.scope,"kind":state.kind,"risk":state.risk,"route":state.route,
        }}),
    )?))
}
fn gate(store: &Store, state: &State, errors: &[String]) -> Result<&'static str> {
    if !errors.is_empty() {
        return Ok("blocked");
    }
    let Some(latest) = state.decisions.last() else {
        return Ok("awaiting-human-review");
    };
    if latest.fingerprint != design_fingerprint(store, state)? {
        return Ok("stale-review");
    }
    Ok(match latest.decision {
        Verdict::Approved => "approved",
        Verdict::ApprovedWithConditions => "approved-with-conditions",
        Verdict::ChangesRequested => "changes-requested",
    })
}
fn review_design(store: &Store, id: &str) -> Result<(Value, i32)> {
    let state = state(store, id)?;
    let errors = validate_design(store, &state)?;
    let gate = gate(store, &state, &errors)?;
    let mut report = crate::localized_format!(
        "# Review di design DFD\n\nGate: {gate}\n\n## Controlli strutturali\n\n",
        "# DFD design review\n\nGate: {gate}\n\n## Structural checks\n\n",
        gate = gate
    );
    if errors.is_empty() {
        report.push_str(&crate::i18n::text(
            "Nessuna lacuna strutturale rilevata. Serve la review semantica del responsabile.\n",
        ));
    }
    for error in &errors {
        report.push_str(&format!("- {error}\n"));
    }
    report.push_str(&crate::i18n::text(
        "\nI controlli non provano la qualità del design e non attribuiscono approvazioni umane.\n",
    ));
    store.write(
        &format!(".dfd/features/{id}/design-review.md"),
        report.as_bytes(),
        false,
    )?;
    let exit = if errors.is_empty() { 0 } else { 2 };
    Ok((
        json!({"feature":id,"gate":gate,"errors":errors,"route":state.route,"decisions":state.decisions}),
        exit,
    ))
}

struct HumanDecision {
    verdict: Verdict,
    reviewer: String,
    role: Role,
    note: String,
    conditions: Vec<String>,
    confirmed: bool,
    reference: Option<String>,
    files: Vec<String>,
}
fn decision_record(input: HumanDecision, fingerprint: String) -> Result<Decision> {
    if !input.confirmed {
        return Err(Error::message(
            "Registrare solo decisioni umane esplicite con --human-confirmed.",
        ));
    }
    if !nonblank(&input.reviewer) || !nonblank(&input.note) {
        return Err(Error::message("Reviewer e motivazione obbligatori."));
    }
    if (input.verdict == Verdict::ApprovedWithConditions) != !input.conditions.is_empty() {
        return Err(Error::message(
            "Usare approved-with-conditions e specificare almeno una condizione.",
        ));
    }
    let mut conditions = vec![];
    for condition in input.conditions {
        let parts: Vec<_> = condition.split('|').collect();
        if parts.len() != 3 || parts.iter().any(|value| !nonblank(value)) {
            return Err(Error::message(
                "Formato condizione: testo|responsabile|fase.",
            ));
        }
        conditions.push(Condition {
            text: parts[0].into(),
            owner: parts[1].into(),
            due_phase: parts[2].into(),
        });
    }
    Ok(Decision {
        decision: input.verdict,
        reviewer: input.reviewer,
        role: input.role,
        note: input.note,
        conditions,
        recorded_at: Utc::now(),
        fingerprint,
        reference: input.reference,
        revision: None,
        files: vec![],
    })
}
fn decide(store: &Store, id: &str, input: HumanDecision) -> Result<(Value, i32)> {
    let mut state = state(store, id)?;
    let errors = validate_design(store, &state)?;
    if input.verdict != Verdict::ChangesRequested && !errors.is_empty() {
        return Err(Error::message(crate::localized_format!(
            "Gate non approvabile: {}",
            "Gate cannot be approved: {}",
            errors.join("; ")
        )));
    }
    let fingerprint = design_fingerprint(store, &state)?;
    let files = input.files.clone();
    let mut decision = decision_record(input, fingerprint)?;
    let context = collaboration::context(store, id)?;
    collaboration::bind(store, context.as_ref(), &files, &mut decision)?;
    state.decisions.push(decision);
    state.phase = "design-reviewed".into();
    store.save(&format!(".dfd/features/{id}/state.json"), &state, false)?;
    review_design(store, id)
}

fn status_feature(store: &Store, id: &str) -> Result<Value> {
    let state = state(store, id)?;
    let errors = validate_design(store, &state)?;
    let gate = gate(store, &state, &errors)?;
    let mut output = serde_json::to_value(&state)?;
    output["gate"] = json!(gate);
    output["errors"] = json!(errors);
    output["collaboration"] = serde_json::to_value(collaboration::context(store, id)?)?;
    output["adoption"] = serde_json::to_value(adoption(store, &state.domain)?)?;
    let setup = setup_status(store, &state.domain)?;
    output["setup"] = setup.clone();
    let development = development::status(store, &state)?;
    output["development"] = development.clone();
    let release = release::status(store, &state)?;
    output["release"] = release.clone();
    output["next"] = json!(if setup["gate"] != "ready" {
        "dfd-setup"
    } else if state.risk.is_none() {
        "dfd-assess"
    } else if !store.path(&format!(".dfd/features/{id}/spec.md"))?.exists() {
        "dfd-specify"
    } else if matches!(gate, "approved" | "approved-with-conditions") {
        if development["gate"] == "development-complete" {
            "development-complete"
        } else if development["gate"] == "ready-for-pre-release" {
            release["next"].as_str().unwrap()
        } else if development["gate"] == "not-started" || development["gate"] == "blocked-plan" {
            "dfd-plan"
        } else {
            "dfd-implement"
        }
    } else {
        "dfd-review-design"
    });
    output["blocking"] = json!(blocking_gates(&output));
    output["effective_phase"] = json!(effective_phase(
        store,
        &state,
        gate,
        &development,
        &release
    )?);
    Ok(output)
}

/// Gates shown by `status` that are structurally blocked (issue #7). Waiting for a
/// human (`awaiting-human-review`, `stale-review`, `changes-requested`) is not blocked.
fn blocking_gates(output: &Value) -> Vec<&'static str> {
    let mut blocking = vec![];
    if output["setup"]["gate"] == "blocked" {
        blocking.push("setup");
    }
    if output["gate"] == "blocked" {
        blocking.push("design");
    }
    if matches!(
        output["development"]["gate"].as_str(),
        Some("blocked-plan" | "blocked-evidence")
    ) {
        blocking.push("development");
    }
    if output["release"]["gate"] == "blocked" {
        blocking.push("release");
    }
    blocking
}

/// Phase derived from the current gates; `state.phase` keeps the last recorded step.
fn effective_phase(
    store: &Store,
    state: &State,
    gate: &str,
    development: &Value,
    release: &Value,
) -> Result<&'static str> {
    Ok(if state.risk.is_none() {
        "idea"
    } else if !store
        .path(&format!(".dfd/features/{}/spec.md", state.id))?
        .exists()
    {
        "assessed"
    } else if !matches!(gate, "approved" | "approved-with-conditions") {
        "design"
    } else {
        match development["gate"].as_str() {
            Some("not-started") => "design-reviewed",
            Some("development-complete") => "development-verified",
            Some("ready-for-pre-release") => match release["gate"].as_str() {
                Some("not-started") => "development-verified",
                Some("approved" | "approved-with-conditions") => "release-reviewed",
                _ => "pre-release",
            },
            _ => "development",
        }
    })
}

fn status(store: &Store, id: Option<&str>) -> Result<Value> {
    let config = config(store)?;
    if let Some(id) = id {
        if quick::exists(store, id)? {
            return quick::status(store, id);
        }
        return status_feature(store, id);
    }
    let directory = store.path(".dfd/features")?;
    let mut ids = vec![];
    if directory.exists() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let id = entry.file_name().to_string_lossy().into_owned();
            if store.path(&format!(".dfd/features/{id}"))?.is_dir() {
                ids.push(id);
            }
        }
    }
    ids.sort();
    let features: Vec<_> = ids
        .iter()
        .map(|id| status_feature(store, id))
        .collect::<Result<_>>()?;
    let setups = config
        .domains
        .iter()
        .map(|domain| setup_status(store, domain))
        .collect::<Result<Vec<_>>>()?;
    let changes = quick::list(store)?;
    let mut blocking = vec![];
    for setup in &setups {
        if setup["gate"] == "blocked" {
            blocking.push(format!(
                "setup:{}",
                setup["domain"].as_str().unwrap_or_default()
            ));
        }
    }
    for feature in &features {
        for gate in feature["blocking"].as_array().into_iter().flatten() {
            if gate != "setup" {
                blocking.push(format!(
                    "feature:{}/{}",
                    feature["id"].as_str().unwrap_or_default(),
                    gate.as_str().unwrap_or_default()
                ));
            }
        }
    }
    for change in &changes {
        if change["gate"] == "blocked" {
            blocking.push(format!(
                "change:{}",
                change["change"].as_str().unwrap_or_default()
            ));
        }
    }
    Ok(
        json!({"version":VERSION,"mode":config.mode,"domains":config.domains,"setups":setups,"features":features,"changes":changes,"blocking":blocking}),
    )
}

fn doctor_language() -> Result<Value> {
    let mut names = vec![];
    let reference = Regex::new(r"`\.dfd/kit/([^`<> ]+\.md)`").unwrap();
    for (path, body) in i18n::files()
        .iter()
        .filter(|(path, _)| path.starts_with("skills/"))
    {
        let name = path.split('/').nth(1).unwrap();
        slug(name)?;
        let lines: Vec<_> = body.lines().take(4).collect();
        if lines.len() != 4
            || lines[0] != "---"
            || lines[1] != format!("name: {name}")
            || lines[3] != "---"
            || !lines[2].starts_with("description: ")
            || lines[2].len() > 1037
            || lines[2] == "description: "
        {
            return Err(Error::message(crate::localized_format!(
                "Frontmatter skill non valido: {path}",
                "Invalid skill frontmatter: {path}",
                path = path
            )));
        }
        for capture in reference.captures_iter(body) {
            resource(&capture[1])?;
        }
        names.push(name);
    }
    if names.iter().copied().collect::<BTreeSet<_>>() != SKILLS.into_iter().collect() {
        return Err(Error::message("Skill del workflow mancanti o inattese."));
    }
    Ok(
        json!({"version":VERSION,"skills":names,"resources":i18n::files().len(),"runtime_dependencies":[]}),
    )
}

fn doctor() -> Result<Value> {
    let selected = i18n::current();
    for language in [i18n::Language::It, i18n::Language::En] {
        let _scope = i18n::Scope::enter(language);
        doctor_language()?;
    }
    let _scope = i18n::Scope::enter(selected);
    doctor_language()
}

pub fn execute(cli: Cli) -> Result<(Value, i32)> {
    let _explicit_scope = i18n::Scope::enter(cli.lang.unwrap_or_default());
    if matches!(cli.command, Command::Doctor) {
        return Ok((doctor()?, 0));
    }
    execute_inner(cli)
}

fn execute_inner(cli: Cli) -> Result<(Value, i32)> {
    // Doctor does not depend on a working repository or external resources.
    if matches!(cli.command, Command::Doctor) {
        return Ok((doctor()?, 0));
    }
    let store = Store::new(&cli.project)?;
    let _lock = if matches!(cli.command, Command::Status { .. }) {
        None
    } else {
        Some(store.lock()?)
    };
    let language = cli.lang.unwrap_or(i18n::project_language(&store)?);
    let _scope = i18n::Scope::enter(language);
    match cli.command {
        Command::Language { language } => {
            config(&store)?;
            store.path(".dfd/language.json")?;
            let manifest: Manifest = store.json(".dfd/install.json")?;
            let harness = Harness::ALL
                .into_iter()
                .find(|h| h.name() == manifest.harness)
                .ok_or_else(|| Error::message("Harness installato sconosciuto."))?;
            let _scope = i18n::Scope::enter(language);
            install(&store, harness)?;
            i18n::save(&store, language)?;
            Ok((
                json!({"language": language, "existing_artifacts": "preserved"}),
                0,
            ))
        }
        Command::Install { harness } => {
            store.path(".dfd/language.json")?;
            let output = install(&store, harness)?;
            if cli.lang.is_some() {
                i18n::save(&store, i18n::current())?;
            }
            Ok((output, 0))
        }
        Command::Init {
            harness,
            domain,
            mode,
        } => {
            store.path(".dfd/language.json")?;
            let output = init(&store, harness, &domain, mode)?;
            if cli.lang.is_some() {
                i18n::save(&store, i18n::current())?;
            }
            Ok((output, 0))
        }
        Command::Feature {
            id,
            domain,
            title,
            scope,
            kind,
            legacy_review,
        } => Ok((
            new_feature(&store, &id, &domain, &title, &scope, kind, !legacy_review)?,
            0,
        )),
        Command::Quick {
            id,
            domain,
            title,
            owner,
            kind,
        } => Ok((
            quick::create(&store, &id, &domain, &title, &owner, kind)?,
            0,
        )),
        Command::Promote { id, to } => Ok((quick::promote(&store, &id, &to)?, 0)),
        Command::Context {
            id,
            owner,
            branch,
            revision,
            issue,
            pull_request,
        } => Ok((
            collaboration::set(
                &store,
                &id,
                collaboration::Context {
                    schema_version: 1,
                    owner,
                    branch,
                    revision,
                    issue,
                    pull_request,
                    integration: None,
                },
            )?,
            0,
        )),
        Command::Setup { domain } => setup(&store, &domain),
        Command::Assess { id } => {
            if quick::exists(&store, &id)? {
                let output = quick::status(&store, &id)?;
                let exit = if output["route"] == "unknown" { 2 } else { 0 };
                Ok((output, exit))
            } else {
                assess(&store, &id)
            }
        }
        Command::Specify { id } => Ok((specify(&store, &id)?, 0)),
        Command::ReviewDesign { id } => {
            if quick::exists(&store, &id)? {
                let output = quick::status(&store, &id)?;
                let exit = if output["errors"].as_array().unwrap().is_empty() {
                    0
                } else {
                    2
                };
                Ok((output, exit))
            } else {
                review_design(&store, &id)
            }
        }
        Command::Plan { id, refresh } => development::plan(&store, &id, refresh),
        Command::Verify { id } => {
            if quick::exists(&store, &id)? {
                quick::verify(&store, &id)
            } else {
                development::verify(&store, &id)
            }
        }
        Command::PreRelease { id, refresh } => release::prepare(&store, &id, refresh),
        Command::ReviewRelease { id } => release::review(&store, &id),
        Command::Decide {
            id,
            stage,
            decision,
            reviewer,
            role,
            note,
            reference,
            file,
            condition,
            human_confirmed,
        } => (match stage {
            ReviewStage::Design => {
                if quick::exists(&store, &id)? {
                    quick::decide
                } else {
                    decide
                }
            }
            ReviewStage::Release => release::decide,
        })(
            &store,
            &id,
            HumanDecision {
                verdict: decision,
                reviewer,
                role,
                note,
                conditions: condition,
                confirmed: human_confirmed,
                reference,
                files: file,
            },
        ),
        Command::Status { id } => {
            let output = status(&store, id.as_deref())?;
            let exit = if output["blocking"].as_array().is_some_and(|b| !b.is_empty()) {
                2
            } else {
                0
            };
            Ok((output, exit))
        }
        Command::Doctor => unreachable!(),
    }
}

#[cfg(test)]
mod dod_block_tests {
    use super::dod_blocks;

    #[test]
    fn dod_blocks_split_grouped_criteria_and_keep_details() {
        let blocks = dod_blocks(
            "# DoD\n\n- **Osservabilità**: OBS-01, OBS-02\n- Osservabilità\n  - OBS-01: contare\n    gli errori\n    - inclusi i timeout\n  - OBS-02: tracciare\n\nSLO-01: soglia\nSLO-02: altra soglia\n",
        );
        assert_eq!(
            blocks,
            [
                "# DoD",
                "- **Osservabilità**: OBS-01, OBS-02",
                "- Osservabilità",
                "- OBS-01: contare gli errori - inclusi i timeout",
                "- OBS-02: tracciare",
                "SLO-01: soglia",
                "SLO-02: altra soglia",
            ]
        );
    }
}
