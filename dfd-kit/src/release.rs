use super::*;

fn history(store: &Store, id: &str) -> Result<ReleaseHistory> {
    let file = format!(".dfd/features/{id}/release-decisions.json");
    if store.path(&file)?.exists() {
        store.json(&file)
    } else {
        Ok(ReleaseHistory {
            schema_version: 1,
            decisions: vec![],
        })
    }
}

fn development_fingerprint(store: &Store, state: &State) -> Result<String> {
    let mut files = BTreeMap::new();
    for name in ["plan.md", "plan.json", "evidence.json"] {
        let path = format!(".dfd/features/{}/{name}", state.id);
        files.insert(path.clone(), store.hash(&path)?);
    }
    Ok(hash(&serde_json::to_vec(
        &json!({"design":design_fingerprint(store, state)?, "decision":state.decisions.last(), "files":files}),
    )?))
}

fn entry_errors(store: &Store, state: &State) -> Result<Vec<String>> {
    let development = development::status(store, state)?;
    if development["gate"] == "ready-for-pre-release" {
        return Ok(vec![]);
    }
    let mut errors = vec!["Pre-release: completare e verificare lo sviluppo attuale.".into()];
    errors.extend(
        development["errors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|error| error.as_str().unwrap().to_string()),
    );
    Ok(errors)
}

fn resource(name: &str) -> PreparedResource {
    PreparedResource {
        name: name.into(),
        owner: String::new(),
        status: Preparation::Planned,
        reference: String::new(),
        activation_plan: String::new(),
        justification: String::new(),
        evidence: vec![],
    }
}

pub(super) fn prepare(store: &Store, id: &str, refresh: bool) -> Result<(Value, i32)> {
    let mut state = state(store, id)?;
    let errors = entry_errors(store, &state)?;
    if !errors.is_empty() {
        return Err(Error::message(errors.join("; ")));
    }
    let root = format!(".dfd/features/{id}");
    let present = ["rollout.md", "rollout.json"]
        .iter()
        .map(|name| {
            store
                .path(&format!("{root}/{name}"))
                .map(|path| path.exists())
        })
        .collect::<Result<Vec<_>>>()?;
    if present.iter().any(|exists| *exists) {
        if !present.iter().all(|exists| *exists) {
            return Err(Error::message(
                "Pre-release parziale: preservata; ripristinare rollout.md e rollout.json.",
            ));
        }
        if refresh {
            let mut rollout: Rollout = store.json(&format!("{root}/rollout.json"))?;
            rollout.development_fingerprint = development_fingerprint(store, &state)?;
            store.save(&format!("{root}/rollout.json"), &rollout, false)?;
            state.phase = "pre-release".into();
            store.save(&format!("{root}/state.json"), &state, false)?;
        }
        return Ok((
            json!({"feature":id,"resumed":true,"release":status(store, &state)?}),
            0,
        ));
    }
    let design: Design = store.json(&format!("{root}/design.json"))?;
    let rollout = Rollout {
        schema_version: 1,
        development_fingerprint: development_fingerprint(store, &state)?,
        owner: design.owner,
        artifact: String::new(),
        revision: String::new(),
        environment: "production".into(),
        strategy: if state.route == Some(Route::Light) {
            RolloutStrategy::Direct
        } else {
            RolloutStrategy::Canary
        },
        strategy_rationale: String::new(),
        steps: vec![],
        criteria: design
            .criteria
            .iter()
            .filter(|item| item.applicability == Applicability::Applicable)
            .map(|item| ReleaseCriterion {
                id: item.id.clone(),
                signal: String::new(),
                threshold: String::new(),
                verification: item.verification.clone(),
            })
            .collect(),
        alarms: vec![resource("Allarmi del rilascio")],
        dashboards: vec![resource("Dashboard del rilascio")],
        feature_flag: resource("Feature flag / controllo dell'esposizione"),
        rollback_trigger: String::new(),
        rollback_procedure: String::new(),
        rollback_verification: String::new(),
        on_call: String::new(),
        communications: vec![],
        non_functional_tests: ["load", "resilience", "security"]
            .iter()
            .map(|area| NonFunctionalTest {
                area: area.to_string(),
                applicability: Applicability::Applicable,
                justification: String::new(),
                execution: None,
            })
            .collect(),
        open_questions: vec![],
    };
    let template = store.read(&format!(
        ".dfd/domains/{}/templates/configurazione_rollout.md",
        state.domain
    ))?;
    store.save(&format!("{root}/rollout.json"), &rollout, true)?;
    store.write(&format!("{root}/rollout.md"), template.as_bytes(), true)?;
    state.phase = "pre-release".into();
    store.save(&format!("{root}/state.json"), &state, false)?;
    Ok((
        json!({"feature":id,"resumed":false,"phase":"pre-release","next":"dfd-pre-release"}),
        0,
    ))
}

fn complete(value: &str) -> bool {
    nonblank(value) && !value.contains("TODO") && !value.contains("YYYY-MM-DD")
}
fn date_valid(value: &str) -> bool {
    DateTime::parse_from_rfc3339(value).is_ok_and(|date| date <= Utc::now())
}
fn resource_errors(
    store: &Store,
    resource: &PreparedResource,
    allow_exclusion: bool,
) -> Result<Vec<String>> {
    let mut errors = vec![];
    if !complete(&resource.name) {
        errors.push("Risorsa operativa senza nome.".into());
    }
    match resource.status {
        Preparation::Excluded => {
            if !allow_exclusion
                || !complete(&resource.justification)
                || !resource.evidence.is_empty()
            {
                errors.push(format!(
                    "Esclusione non ammessa o non motivata: {}.",
                    resource.name
                ));
            }
        }
        Preparation::Planned | Preparation::Ready => {
            if !complete(&resource.owner) || !complete(&resource.reference) {
                errors.push(format!("Owner e riferimento richiesti: {}.", resource.name));
            }
            if resource.status == Preparation::Planned && !complete(&resource.activation_plan) {
                errors.push(format!("Piano di attivazione mancante: {}.", resource.name));
            }
            if resource.status == Preparation::Ready && resource.evidence.is_empty() {
                errors.push(format!(
                    "Risorsa dichiarata pronta senza evidenze: {}.",
                    resource.name
                ));
            }
        }
    }
    for evidence in &resource.evidence {
        errors.extend(development::file_errors(store, evidence)?);
    }
    Ok(errors)
}

fn validate(store: &Store, state: &State) -> Result<Vec<String>> {
    let mut errors = entry_errors(store, state)?;
    let root = format!(".dfd/features/{}", state.id);
    for name in ["rollout.md", "rollout.json"] {
        if !store.path(&format!("{root}/{name}"))?.is_file() {
            errors.push(format!("Pre-release: manca {name}; eseguire pre-release."));
        }
    }
    if !errors.is_empty() {
        return Ok(errors);
    }
    let rollout: Rollout = store.json(&format!("{root}/rollout.json"))?;
    if rollout.development_fingerprint != development_fingerprint(store, state)? {
        errors.push("Rollout riferito a uno sviluppo precedente: verificare le evidenze e usare pre-release --refresh.".into());
    }
    let markdown = store.read(&format!("{root}/rollout.md"))?;
    if !complete(&markdown) || markdown.contains("[Nome Feature]") {
        errors.push("Completare rollout.md e rimuovere i placeholder.".into());
    }
    for (name, value) in [
        ("owner", &rollout.owner),
        ("artifact", &rollout.artifact),
        ("revision", &rollout.revision),
        ("environment", &rollout.environment),
        ("strategy_rationale", &rollout.strategy_rationale),
        ("rollback_trigger", &rollout.rollback_trigger),
        ("rollback_procedure", &rollout.rollback_procedure),
        ("rollback_verification", &rollout.rollback_verification),
        ("on_call", &rollout.on_call),
    ] {
        if !complete(value) {
            errors.push(format!("rollout.json: completare {name}."));
        }
    }
    if !rollout.open_questions.is_empty() {
        errors.push("Dubbi di pre-release ancora aperti.".into());
    }
    if rollout.steps.is_empty() {
        errors.push("Definire almeno una fase di rollout.".into());
    }
    let mut ids = BTreeSet::new();
    let mut previous = 0;
    for step in &rollout.steps {
        slug(&step.id)?;
        if !ids.insert(&step.id)
            || !complete(&step.audience)
            || !complete(&step.promotion)
            || !complete(&step.stop)
            || step.percentage == 0
            || step.percentage > 100
            || step.percentage < previous
            || step.duration_minutes == 0
        {
            errors.push(format!("Fase di rollout non valida: {}.", step.id));
        }
        previous = step.percentage;
    }
    if previous != 100 {
        errors.push("Il rollout deve terminare al 100% dell'audience prevista.".into());
    }
    match rollout.strategy {
        RolloutStrategy::Direct if rollout.steps.len() != 1 => {
            errors.push("Rollout diretto: usare una sola fase al 100%.".into())
        }
        RolloutStrategy::Canary | RolloutStrategy::Combined
            if rollout.steps.len() < 2
                || rollout
                    .steps
                    .first()
                    .is_none_or(|step| step.percentage >= 100) =>
        {
            errors.push(
                "Canary: prevedere esposizione iniziale parziale e successiva promozione.".into(),
            )
        }
        _ => {}
    }
    let design: Design = store.json(&format!("{root}/design.json"))?;
    let applicable: BTreeSet<_> = design
        .criteria
        .iter()
        .filter(|item| item.applicability == Applicability::Applicable)
        .map(|item| item.id.as_str())
        .collect();
    let mut covered = BTreeSet::new();
    let markdown_ids = criterion_ids(&markdown);
    for criterion in &rollout.criteria {
        if !applicable.contains(criterion.id.as_str())
            || !covered.insert(criterion.id.as_str())
            || [
                &criterion.signal,
                &criterion.threshold,
                &criterion.verification,
            ]
            .iter()
            .any(|value| !complete(value))
        {
            errors.push(format!(
                "Criterio di rilascio sconosciuto, duplicato o incompleto: {}.",
                criterion.id
            ));
        }
        if !markdown_ids.contains(&criterion.id) {
            errors.push(format!("rollout.md: manca {}.", criterion.id));
        }
    }
    for id in applicable.difference(&covered) {
        errors.push(format!(
            "Criterio applicabile senza validazione in produzione: {id}."
        ));
    }
    for id in markdown_ids.difference(&applicable.iter().map(|id| id.to_string()).collect()) {
        errors.push(format!(
            "rollout.md: criterio non applicabile o sconosciuto: {id}."
        ));
    }
    if rollout.alarms.is_empty() || rollout.dashboards.is_empty() {
        errors.push(
            "Dichiarare allarmi e dashboard, anche come esclusioni motivate nel percorso light."
                .into(),
        );
    }
    let light = state.route == Some(Route::Light);
    for resource in rollout.alarms.iter().chain(&rollout.dashboards) {
        errors.extend(resource_errors(store, resource, light)?);
    }
    errors.extend(resource_errors(store, &rollout.feature_flag, true)?);
    let mut areas = BTreeSet::new();
    for test in &rollout.non_functional_tests {
        if !["load", "resilience", "security"].contains(&test.area.as_str())
            || !areas.insert(test.area.as_str())
        {
            errors.push(format!(
                "Area di test non funzionale sconosciuta o duplicata: {}.",
                test.area
            ));
        }
        match test.applicability {
            Applicability::Excluded => {
                if !complete(&test.justification) || test.execution.is_some() {
                    errors.push(format!(
                        "Esclusione test non motivata o incoerente: {}.",
                        test.area
                    ));
                }
            }
            Applicability::Applicable => match &test.execution {
                None => errors.push(format!("Test non funzionale da eseguire: {}.", test.area)),
                Some(execution) => {
                    if !complete(&execution.command)
                        || !date_valid(&execution.executed_at)
                        || execution.exit_code != 0
                    {
                        errors.push(format!("Esecuzione test non valida: {}.", test.area));
                    }
                    errors.extend(development::file_errors(store, &execution.log)?);
                    if store.path(&execution.log.path)?.is_file()
                        && !nonblank(&store.read(&execution.log.path)?)
                    {
                        errors.push(format!("Log test vuoto: {}.", test.area));
                    }
                }
            },
        }
    }
    if areas != ["load", "resilience", "security"].into_iter().collect() {
        errors.push(
            "Valutare carico, resilienza e sicurezza, eseguiti oppure esclusi con motivazione."
                .into(),
        );
    }
    if rollout.communications.is_empty() {
        errors.push("Registrare la comunicazione a Supporto / stakeholder.".into());
    }
    for communication in &rollout.communications {
        if [
            &communication.audience,
            &communication.owner,
            &communication.reference,
        ]
        .iter()
        .any(|value| !complete(value))
            || !date_valid(&communication.informed_at)
        {
            errors.push("Comunicazione incompleta o con data non valida.".into());
        }
    }
    if let Some(decision) = state.decisions.last() {
        for condition in &decision.conditions {
            if !matches!(condition.due_phase.as_str(), "rollout" | "post-release") {
                errors.push(format!("Condizione di design da risolvere prima della release: {}. Registrare la nuova decisione umana.", condition.text));
            }
        }
    }
    Ok(errors)
}

fn fingerprint(store: &Store, state: &State) -> Result<String> {
    let mut files = BTreeMap::new();
    for name in ["rollout.md", "rollout.json", "release-notes.md"] {
        let path = format!(".dfd/features/{}/{name}", state.id);
        files.insert(
            path.clone(),
            if store.path(&path)?.is_file() {
                store.hash(&path)?
            } else {
                "absent".into()
            },
        );
    }
    Ok(hash(&serde_json::to_vec(
        &json!({"development":development_fingerprint(store, state)?,"files":files}),
    )?))
}

fn planned(rollout: &Rollout) -> Vec<String> {
    rollout
        .alarms
        .iter()
        .chain(&rollout.dashboards)
        .chain(std::iter::once(&rollout.feature_flag))
        .filter(|resource| resource.status == Preparation::Planned)
        .map(|resource| resource.name.clone())
        .collect()
}

pub(super) fn status(store: &Store, state: &State) -> Result<Value> {
    let root = format!(".dfd/features/{}", state.id);
    let history = history(store, &state.id)?;
    let any = store.path(&format!("{root}/rollout.json"))?.exists()
        || store.path(&format!("{root}/rollout.md"))?.exists();
    if !any && history.decisions.is_empty() {
        return Ok(
            json!({"gate":"not-started","errors":[],"decisions":[],"next":"dfd-pre-release"}),
        );
    }
    let errors = validate(store, state)?;
    let mut gate = "blocked";
    let mut next = "dfd-pre-release";
    let mut pending = vec![];
    if errors.is_empty() {
        let rollout: Rollout = store.json(&format!("{root}/rollout.json"))?;
        pending = planned(&rollout);
        gate = if let Some(decision) = history.decisions.last() {
            if decision.fingerprint != fingerprint(store, state)? {
                "stale-review"
            } else {
                match decision.decision {
                    Verdict::Approved => "approved",
                    Verdict::ApprovedWithConditions => "approved-with-conditions",
                    Verdict::ChangesRequested => "changes-requested",
                }
            }
        } else {
            "awaiting-human-review"
        };
        next = match gate {
            "approved" => "rollout",
            "approved-with-conditions" => {
                let decision = history.decisions.last().unwrap();
                if pending.is_empty()
                    && decision.conditions.iter().all(|condition| {
                        matches!(condition.due_phase.as_str(), "rollout" | "post-release")
                    })
                {
                    "rollout-with-conditions"
                } else {
                    "resolve-release-conditions"
                }
            }
            _ => "dfd-review-release",
        };
    }
    Ok(
        json!({"gate":gate,"errors":errors,"next":next,"decisions":history.decisions,"pending_activation":pending,
        "checklist":if state.route == Some(Route::Light) {"review_release_light.md"} else {"review_release.md"}}),
    )
}

pub(super) fn review(store: &Store, id: &str) -> Result<(Value, i32)> {
    let state = state(store, id)?;
    let output = status(store, &state)?;
    let mut report = format!("# Review di release DFD\n\nGate: {}\n\n", output["gate"]);
    for error in output["errors"].as_array().unwrap() {
        report.push_str(&format!("- {}\n", error.as_str().unwrap()));
    }
    for pending in output["pending_activation"]
        .as_array()
        .into_iter()
        .flatten()
    {
        report.push_str(&format!(
            "- Attivazione pianificata: {}\n",
            pending.as_str().unwrap()
        ));
    }
    report.push_str("\nControlli strutturali: serve la review semantica del responsabile. La CLI non comunica agli stakeholder, configura risorse o esegue il rollout.\n");
    store.write(
        &format!(".dfd/features/{id}/release-review.md"),
        report.as_bytes(),
        false,
    )?;
    let exit = if matches!(output["gate"].as_str(), Some("blocked" | "not-started")) {
        2
    } else {
        0
    };
    Ok((json!({"feature":id,"release":output}), exit))
}

pub(super) fn decide(store: &Store, id: &str, input: HumanDecision) -> Result<(Value, i32)> {
    let mut state = state(store, id)?;
    let errors = validate(store, &state)?;
    if input.verdict != Verdict::ChangesRequested && !errors.is_empty() {
        return Err(Error::message(format!(
            "Release non approvabile: {}",
            errors.join("; ")
        )));
    }
    if input.verdict == Verdict::Approved {
        let rollout: Rollout = store.json(&format!(".dfd/features/{id}/rollout.json"))?;
        if !planned(&rollout).is_empty() {
            return Err(Error::message("Risorse ancora da attivare: registrare approved-with-conditions con owner e fase, oppure completare l'attivazione."));
        }
    }
    let decision = decision_record(input, fingerprint(store, &state)?)?;
    let mut history = history(store, id)?;
    history.decisions.push(decision);
    store.path(&format!(".dfd/features/{id}/release-review.md"))?;
    store.save(
        &format!(".dfd/features/{id}/release-decisions.json"),
        &history,
        false,
    )?;
    state.phase = "release-reviewed".into();
    store.save(&format!(".dfd/features/{id}/state.json"), &state, false)?;
    review(store, id)
}
