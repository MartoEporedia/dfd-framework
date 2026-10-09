use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum ChangeKind {
    BehaviorFix,
    Editorial,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Change {
    schema_version: u32,
    id: String,
    domain: String,
    title: String,
    owner: String,
    kind: ChangeKind,
    problem: String,
    scope: String,
    known_contract: String,
    scope_bounded: bool,
    changes_contract: bool,
    critical: bool,
    open_questions: Vec<String>,
    risk: Risk,
    criteria: Vec<String>,
    files: Vec<FileEvidence>,
    checks: Vec<TestEvidence>,
    verification_notes: String,
    collaboration: Option<collaboration::Context>,
    decisions: Vec<Decision>,
}
fn path(id: &str) -> Result<String> {
    slug(id)?;
    Ok(format!(".dfd/changes/{id}.json"))
}
pub(super) fn exists(store: &Store, id: &str) -> Result<bool> {
    Ok(store.path(&path(id)?)?.exists())
}
fn read(store: &Store, id: &str) -> Result<Change> {
    let change: Change = store.json(&path(id)?)?;
    if change.id != id {
        return Err(Error::message(
            "ID del record rapido non corrispondente al percorso.",
        ));
    }
    slug(&change.domain)?;
    if change.risk.schema_version != 1
        || change
            .collaboration
            .as_ref()
            .is_some_and(|c| c.schema_version != 1 || c.owner != change.owner)
    {
        return Err(Error::message(
            "Schema o owner incoerente nel record rapido.",
        ));
    }
    if !config(store)?.domains.contains(&change.domain) {
        return Err(Error::message("Dominio rapido non registrato."));
    }
    Ok(change)
}
pub(super) fn create(
    store: &Store,
    id: &str,
    domain: &str,
    title: &str,
    owner: &str,
    kind: ChangeKind,
) -> Result<Value> {
    slug(id)?;
    slug(domain)?;
    if exists(store, id)? {
        return status(store, id);
    }
    if store.path(&format!(".dfd/features/{id}"))?.exists() {
        return Err(Error::message("ID già usato da una feature."));
    }
    let adoption = adoption(store, domain)?;
    if config(store)?.mode == Mode::Brownfield && !adoption.pilot && adoption.current_level < 2 {
        return Err(Error::message(
            "Dominio fuori dal pilota: definire l'ambito di adozione.",
        ));
    }
    let errors = setup_errors(store, domain)?;
    if !errors.is_empty() {
        return Err(Error::message(errors.join("; ")));
    }
    if !nonblank(title) || !nonblank(owner) {
        return Err(Error::message("Titolo e owner obbligatori."));
    }
    let change = Change {
        schema_version: 1,
        id: id.into(),
        domain: domain.into(),
        title: title.into(),
        owner: owner.into(),
        kind,
        problem: String::new(),
        scope: String::new(),
        known_contract: String::new(),
        scope_bounded: false,
        changes_contract: false,
        critical: false,
        open_questions: vec![],
        risk: Risk {
            schema_version: 1,
            dimensions: DIMENSIONS
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
                .collect(),
        },
        criteria: vec![],
        files: vec![],
        checks: vec![],
        verification_notes: String::new(),
        collaboration: None,
        decisions: vec![],
    };
    store.save(&path(id)?, &change, true)?;
    status(store, id)
}
fn route(change: &Change) -> Result<(String, Vec<String>)> {
    let risk = classify(&change.risk, Kind::Feature)?;
    if !risk.missing.is_empty() {
        return Ok((
            "unknown".into(),
            vec![format!("Rischio da chiarire: {}.", risk.missing.join(", "))],
        ));
    }
    let low = change
        .risk
        .dimensions
        .values()
        .all(|d| d.level == Some(Level::Low));
    let bounded = change.scope_bounded
        && !change.changes_contract
        && !change.critical
        && change.open_questions.is_empty()
        && (change.kind == ChangeKind::Editorial || nonblank(&change.known_contract));
    if low && bounded {
        return Ok(("rapid".into(), vec![]));
    }
    let selected = if change.critical {
        "full-complete".into()
    } else {
        serde_json::to_value(risk.route)?
            .as_str()
            .unwrap_or("light")
            .to_string()
    };
    Ok((selected,vec!["Intervento non rapido: chiarire i dubbi o promuovere a light/full; nessun override per ridurre il rischio.".into()]))
}
fn fingerprint(store: &Store, change: &Change) -> Result<String> {
    let mut value = serde_json::to_value(change)?;
    value.as_object_mut().unwrap().remove("decisions");
    let policy = lifecycle(store, &change.domain)?;
    let team = collaboration::policy(store, &change.domain)?;
    let catalog: Catalog = store.json(&format!(".dfd/domains/{}/criteria.json", change.domain))?;
    let criteria: Vec<_> = catalog
        .criteria
        .iter()
        .filter(|c| change.criteria.contains(&c.id))
        .collect();
    Ok(hash(&serde_json::to_vec(
        &json!({"change":value,"lifecycle":policy,"team":team,"criteria":criteria,
        "guardrails":store.hash(&format!(".dfd/domains/{}/guardrails.md",change.domain))?,"common_guardrails":store.hash(".dfd/guardrails.md")?}),
    )?))
}
fn errors(store: &Store, change: &Change) -> Result<Vec<String>> {
    let (_, mut errors) = route(change)?;
    errors.extend(setup_errors(store, &change.domain)?);
    if [
        &change.title,
        &change.owner,
        &change.problem,
        &change.scope,
        &change.verification_notes,
    ]
    .iter()
    .any(|s| !nonblank(s))
    {
        errors.push(
            "Completare problema, ambito, owner e motivazione delle verifiche pertinenti.".into(),
        );
    }
    let catalog: Catalog = store.json(&format!(".dfd/domains/{}/criteria.json", change.domain))?;
    if change.criteria.is_empty() {
        errors.push("Collegare almeno un criterio DoD pertinente.".into());
    }
    let mut seen = BTreeSet::new();
    for id in &change.criteria {
        if !seen.insert(id) || !catalog.criteria.iter().any(|c| &c.id == id) {
            errors.push(format!("Criterio rapido ignoto o duplicato: {id}."));
        }
    }
    if change.files.is_empty() {
        errors.push("Registrare i file verificati con hash.".into());
    }
    let mut files = BTreeSet::new();
    for file in &change.files {
        if !files.insert(&file.path) {
            errors.push("File verificato duplicato.".into());
        }
        errors.extend(development::file_errors(store, file)?);
    }
    let mut red = None;
    let mut green = None;
    let mut ci = None;
    for check in &change.checks {
        let failures = collaboration::check_errors(store, check)?;
        if failures.is_empty() {
            let date = DateTime::parse_from_rfc3339(&check.executed_at).unwrap();
            match check.kind {
                CheckKind::Red => red = Some(date),
                CheckKind::Green => green = Some(date),
                CheckKind::Ci => ci = Some(date),
                CheckKind::Suite => {}
            }
        }
        errors.extend(failures);
    }
    if green.is_none() {
        errors.push("Serve una verifica pertinente green reale.".into());
    }
    if change.kind == ChangeKind::BehaviorFix
        && (red.is_none() || green.is_some_and(|g| red.is_some_and(|r| r > g)))
    {
        errors.push(
            "Fix comportamentale: serve regressione red prima del green, sul bug atteso.".into(),
        );
    }
    if lifecycle(store, &change.domain)?.ci_required
        && (ci.is_none() || green.is_some_and(|g| ci.is_some_and(|c| c < g)))
    {
        errors.push("La policy del dominio richiede CI verde successiva al green.".into());
    }
    Ok(errors)
}
pub(super) fn status(store: &Store, id: &str) -> Result<Value> {
    let change = read(store, id)?;
    let (route, _) = route(&change)?;
    let errors = errors(store, &change)?;
    let fingerprint = fingerprint(store, &change)?;
    let policy = collaboration::policy(store, &change.domain)?;
    let gate = if !errors.is_empty() {
        "blocked"
    } else if let Some(latest) = change.decisions.last() {
        if latest.fingerprint != fingerprint {
            "stale-review"
        } else if latest.decision == Verdict::Approved {
            "change-verified"
        } else {
            "changes-requested"
        }
    } else if policy.independent_review {
        "awaiting-human-review"
    } else {
        "change-verified"
    };
    Ok(
        json!({"change":id,"domain":change.domain,"route":route,"gate":gate,"errors":errors,"owner":change.owner,"independent_review_required":policy.independent_review,"collaboration":change.collaboration,"decisions":change.decisions,
        "next":if route=="unknown" {"dfd-assess"} else if route!="rapid" {"dfd-feature"} else if gate=="change-verified" {"development-complete"} else if gate=="awaiting-human-review" || gate=="stale-review" {"dfd-review-design"} else {"dfd-implement"},"release":"not-approved"}),
    )
}
pub(super) fn verify(store: &Store, id: &str) -> Result<(Value, i32)> {
    let output = status(store, id)?;
    let exit = if output["gate"] == "change-verified" {
        0
    } else {
        2
    };
    Ok((output, exit))
}
pub(super) fn decide(store: &Store, id: &str, input: HumanDecision) -> Result<(Value, i32)> {
    let mut change = read(store, id)?;
    if input.verdict == Verdict::ApprovedWithConditions {
        return Err(Error::message(
            "Condizioni aperte richiedono light/full: promuovere il record rapido.",
        ));
    }
    let errors = errors(store, &change)?;
    if input.verdict != Verdict::ChangesRequested && !errors.is_empty() {
        return Err(Error::message(errors.join("; ")));
    }
    let mut decision = decision_record(input, fingerprint(store, &change)?)?;
    let files: Vec<_> = change.files.iter().map(|f| f.path.clone()).collect();
    collaboration::bind(store, change.collaboration.as_ref(), &files, &mut decision)?;
    let policy = collaboration::policy(store, &change.domain)?;
    let review_errors = collaboration::reviewer_errors(&policy, &change.owner, &decision);
    if !review_errors.is_empty() {
        return Err(Error::message(review_errors.join("; ")));
    }
    if policy.independent_review
        && change
            .collaboration
            .as_ref()
            .is_none_or(|c| !nonblank(&c.pull_request))
    {
        return Err(Error::message(
            "Policy team: collegare la PR e la revisione prima della review.",
        ));
    }
    change.decisions.push(decision);
    store.save(&path(id)?, &change, false)?;
    verify(store, id)
}
pub(super) fn set_context(
    store: &Store,
    id: &str,
    context: collaboration::Context,
) -> Result<Value> {
    let mut change = read(store, id)?;
    change.owner = context.owner.clone();
    change.collaboration = Some(context);
    store.save(&path(id)?, &change, false)?;
    status(store, id)
}
pub(super) fn promote(store: &Store, id: &str, to: &str) -> Result<Value> {
    let change = read(store, id)?;
    slug(to)?;
    if !nonblank(&change.scope) {
        return Err(Error::message("Definire l'ambito prima di promuovere."));
    }
    if exists(store, to)? {
        return Err(Error::message("ID di destinazione già usato."));
    }
    let output = new_feature(
        store,
        to,
        &change.domain,
        &change.title,
        &change.scope,
        Kind::Feature,
        true,
    )?;
    let mut risk = change.risk;
    if change.critical {
        risk.dimensions.insert(
            "reliability".into(),
            RiskDimension {
                level: Some(Level::High),
                rationale:
                    "Escalation conservativa: intervento dichiarato critico nel record rapido."
                        .into(),
            },
        );
    }
    store.save(&format!(".dfd/features/{to}/risk.json"), &risk, false)?;
    store.save(
        &format!(".dfd/features/{to}/origin.json"),
        &json!({"schema_version":1,"change":id,"snapshot_sha256":store.hash(&path(id)?)?}),
        true,
    )?;
    if let Some(context) = change.collaboration {
        store.save(
            &format!(".dfd/features/{to}/collaboration.json"),
            &context,
            true,
        )?;
    }
    Ok(json!({"feature":output,"assessment":assess(store,to)?.0,"origin":id}))
}
pub(super) fn list(store: &Store) -> Result<Vec<Value>> {
    let path = store.path(".dfd/changes")?;
    let mut ids = vec![];
    if path.exists() {
        for entry in fs::read_dir(path)? {
            let p = entry?.path();
            if p.extension().is_some_and(|ext| ext == "json") {
                ids.push(p.file_stem().unwrap().to_string_lossy().to_string());
            }
        }
    }
    ids.sort();
    ids.iter().map(|id| status(store, id)).collect()
}
