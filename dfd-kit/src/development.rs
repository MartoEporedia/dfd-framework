use super::*;

fn entry_errors(store: &Store, state: &State) -> Result<Vec<String>> {
    let mut errors = validate_design(store, state)?;
    let gate = gate(store, state, &errors)?;
    if !matches!(gate, "approved" | "approved-with-conditions") {
        errors.push(format!(
            "Sviluppo: serve una review di design approvata e attuale (gate: {gate})."
        ));
    }
    Ok(errors)
}

fn plan_hash(store: &Store, id: &str) -> Result<String> {
    let root = format!(".dfd/features/{id}");
    Ok(hash(&serde_json::to_vec(&json!({
        "json":store.hash(&format!("{root}/plan.json"))?,
        "markdown":store.hash(&format!("{root}/plan.md"))?,
    }))?))
}

pub(super) fn plan(store: &Store, id: &str, refresh: bool) -> Result<(Value, i32)> {
    let mut state = state(store, id)?;
    let errors = entry_errors(store, &state)?;
    if !errors.is_empty() {
        return Err(Error::message(errors.join("; ")));
    }
    let root = format!(".dfd/features/{id}");
    let files = ["plan.json", "plan.md", "evidence.json"];
    let present = files
        .iter()
        .map(|file| {
            store
                .path(&format!("{root}/{file}"))
                .map(|path| path.exists())
        })
        .collect::<Result<Vec<_>>>()?;
    if present.iter().any(|exists| *exists) {
        if !present.iter().all(|exists| *exists) {
            return Err(Error::message(
                "Artefatti di sviluppo parziali: preservati; ripristinare i file mancanti.",
            ));
        }
        if refresh {
            let mut plan: Plan = store.json(&format!("{root}/plan.json"))?;
            plan.design_fingerprint = design_fingerprint(store, &state)?;
            store.save(&format!("{root}/plan.json"), &plan, false)?;
            state.phase = "development".into();
            store.save(&format!("{root}/state.json"), &state, false)?;
        }
        let output = status(store, &state)?;
        let exit = if output["gate"] == "blocked-plan" {
            2
        } else {
            0
        };
        return Ok((
            json!({"feature":id,"resumed":true,"development":output}),
            exit,
        ));
    }
    let design: Design = store.json(&format!("{root}/design.json"))?;
    let fingerprint = design_fingerprint(store, &state)?;
    let tasks: Vec<Task> = design
        .criteria
        .iter()
        .filter(|criterion| criterion.applicability == Applicability::Applicable)
        .enumerate()
        .map(|(index, criterion)| Task {
            id: format!("task-{}", index + 1),
            owner: design.owner.clone(),
            description: format!("Soddisfare {}", criterion.id),
            criteria: vec![criterion.id.clone()],
            implementation: criterion.implementation.clone(),
            verification: criterion.verification.clone(),
        })
        .collect();
    let mut markdown = format!(
        "# Piano di sviluppo – {}\n\n## Ambito e test\n\n{}\n\n{}\n\n## Task TDD\n\n",
        state.title, state.scope, design.test_plan
    );
    for task in &tasks {
        markdown.push_str(&format!(
            "### {} – {}\n\nOwner: {}\nCriteri: {}\n\nImplementazione: {}\n\nVerifica: {}\n\n",
            task.id,
            task.description,
            task.owner,
            task.criteria.join(", "),
            task.implementation,
            task.verification
        ));
    }
    markdown.push_str("## Ciclo di sviluppo\n\nPer ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.\n");
    store.save(
        &format!("{root}/plan.json"),
        &Plan {
            schema_version: 1,
            design_fingerprint: fingerprint.clone(),
            tasks,
            open_questions: vec![],
        },
        true,
    )?;
    store.write(&format!("{root}/plan.md"), markdown.as_bytes(), true)?;
    store.save(
        &format!("{root}/evidence.json"),
        &Evidence {
            schema_version: 1,
            design_fingerprint: fingerprint,
            plan_hash: plan_hash(store, id)?,
            files: vec![],
            checks: vec![],
            refactor_notes: String::new(),
        },
        true,
    )?;
    state.phase = "development".into();
    store.save(&format!("{root}/state.json"), &state, false)?;
    Ok((
        json!({"feature":id,"resumed":false,"phase":"development","next":"dfd-implement"}),
        0,
    ))
}

fn plan_errors(store: &Store, state: &State, plan: &Plan) -> Result<Vec<String>> {
    let mut errors = vec![];
    if plan.design_fingerprint != design_fingerprint(store, state)? {
        errors.push("Piano derivato da un design precedente: riconfermare la review e usare plan --refresh.".into());
    }
    let markdown = store.read(&format!(".dfd/features/{}/plan.md", state.id))?;
    if !nonblank(&markdown) || markdown.contains("TODO") || !plan.open_questions.is_empty() {
        errors.push("Piano incompleto o dubbi aperti: chiarire prima dello sviluppo.".into());
    }
    let design: Design = store.json(&format!(".dfd/features/{}/design.json", state.id))?;
    let applicable: BTreeSet<_> = design
        .criteria
        .iter()
        .filter(|criterion| criterion.applicability == Applicability::Applicable)
        .map(|criterion| criterion.id.as_str())
        .collect();
    let mut covered = BTreeSet::new();
    let mut ids = BTreeSet::new();
    if plan.tasks.is_empty() {
        errors.push("Piano senza task.".into());
    }
    for task in &plan.tasks {
        slug(&task.id)?;
        if !ids.insert(&task.id) {
            errors.push(format!("Task duplicato: {}.", task.id));
        }
        if [
            &task.owner,
            &task.description,
            &task.implementation,
            &task.verification,
        ]
        .iter()
        .any(|value| !nonblank(value) || value.contains("TODO"))
            || task.criteria.is_empty()
        {
            errors.push(format!("Task incompleto: {}.", task.id));
        }
        let mut task_ids = BTreeSet::new();
        for criterion in &task.criteria {
            if !task_ids.insert(criterion) || !applicable.contains(criterion.as_str()) {
                errors.push(format!(
                    "Task {}: criterio sconosciuto, escluso o duplicato: {criterion}.",
                    task.id
                ));
            }
            covered.insert(criterion.as_str());
            if !criterion_ids(&markdown).contains(criterion) {
                errors.push(format!("Piano Markdown: manca {criterion}."));
            }
        }
    }
    for id in applicable.difference(&covered) {
        errors.push(format!("Criterio applicabile senza task: {id}."));
    }
    Ok(errors)
}

pub(super) fn file_errors(store: &Store, file: &FileEvidence) -> Result<Vec<String>> {
    let path = store.path(&file.path)?;
    if !path.is_file() {
        return Ok(vec![format!("Evidenza assente: {}.", file.path)]);
    }
    if file.sha256 != store.hash(&file.path)? {
        return Ok(vec![format!(
            "Evidenza modificata: {}; rieseguire le verifiche.",
            file.path
        )]);
    }
    Ok(vec![])
}

pub(super) fn status(store: &Store, state: &State) -> Result<Value> {
    let root = format!(".dfd/features/{}", state.id);
    let present = ["plan.json", "plan.md", "evidence.json"]
        .iter()
        .map(|name| {
            store
                .path(&format!("{root}/{name}"))
                .map(|path| path.is_file())
        })
        .collect::<Result<Vec<_>>>()?;
    if !present.iter().any(|exists| *exists) {
        return Ok(json!({"gate":"not-started","errors":[],"next":"dfd-plan"}));
    }
    let mut errors = entry_errors(store, state)?;
    if !present.iter().all(|exists| *exists) {
        errors.push(
            "Artefatti di sviluppo parziali: ripristinare plan.json, plan.md ed evidence.json."
                .into(),
        );
    }
    if !errors.is_empty() {
        return Ok(json!({"gate":"blocked-plan","errors":errors,"next":"dfd-plan"}));
    }
    let plan: Plan = store.json(&format!("{root}/plan.json"))?;
    errors.extend(plan_errors(store, state, &plan)?);
    if !errors.is_empty() {
        return Ok(json!({"gate":"blocked-plan","errors":errors,"next":"dfd-plan"}));
    }
    let evidence: Evidence = store.json(&format!("{root}/evidence.json"))?;
    let fingerprint = design_fingerprint(store, state)?;
    if evidence.design_fingerprint != fingerprint
        || evidence.plan_hash != plan_hash(store, &state.id)?
    {
        errors.push(
            "Evidenze riferite a un design o piano precedente: rieseguire le verifiche.".into(),
        );
    }
    if evidence.files.is_empty() {
        errors.push("Registrare i file di codice, test e strumentazione verificati.".into());
    }
    let mut seen = BTreeSet::new();
    for file in &evidence.files {
        if !seen.insert(&file.path) {
            errors.push(format!("File verificato duplicato: {}.", file.path));
        }
        errors.extend(file_errors(store, file)?);
    }
    if !nonblank(&evidence.refactor_notes) {
        errors.push("Documentare refactor e strumentazione osservabilità, oppure motivare perché non necessari.".into());
    }
    let task_ids: BTreeSet<_> = plan.tasks.iter().map(|task| task.id.as_str()).collect();
    let mut red = BTreeMap::new();
    let mut green = BTreeMap::new();
    let policy = lifecycle(store, &state.domain)?;
    let mut suite = None;
    let mut ci = None;
    for check in &evidence.checks {
        let mut valid = true;
        if !nonblank(&check.command) {
            errors.push("Comando di test mancante.".into());
            valid = false;
        }
        let executed_at = DateTime::parse_from_rfc3339(&check.executed_at).ok();
        match executed_at {
            Some(date) if date <= Utc::now() => {}
            _ => {
                errors.push("Data esecuzione test non valida o futura.".into());
                valid = false;
            }
        }
        let log_errors = file_errors(store, &check.log)?;
        if !log_errors.is_empty() {
            valid = false;
        }
        errors.extend(log_errors);
        if store.path(&check.log.path)?.is_file() && !nonblank(&store.read(&check.log.path)?) {
            errors.push("Log di test vuoto.".into());
            valid = false;
        }
        if let Some(task) = &check.task {
            if !task_ids.contains(task.as_str()) {
                errors.push(format!("Evidenza per task sconosciuto: {task}."));
                valid = false;
            }
        }
        match check.kind {
            CheckKind::Suite | CheckKind::Ci if check.exit_code == 0 && valid => {
                let date = executed_at.unwrap();
                suite = Some(suite.map_or(date, |latest: DateTime<chrono::FixedOffset>| {
                    latest.max(date)
                }));
                if check.kind == CheckKind::Ci {
                    ci = Some(ci.map_or(date, |latest: DateTime<chrono::FixedOffset>| {
                        latest.max(date)
                    }));
                }
            }
            CheckKind::Red | CheckKind::Green => {
                if let Some(task) = &check.task {
                    if valid && check.kind == CheckKind::Red && check.exit_code > 0 {
                        red.insert(task.as_str(), executed_at.unwrap());
                    } else if valid && check.kind == CheckKind::Green && check.exit_code == 0 {
                        green.insert(task.as_str(), executed_at.unwrap());
                    } else {
                        errors.push(format!("Esito TDD non valido per {task}."));
                    }
                } else {
                    errors.push("Test red/green senza task.".into());
                }
            }
            CheckKind::Suite | CheckKind::Ci => {
                errors.push("Suite finale locale/CI non verde o non valida.".into())
            }
        }
    }
    for task in &task_ids {
        if !red.contains_key(task) || !green.contains_key(task) {
            errors.push(format!("Task {task}: servono evidenze red e green."));
        } else if red[task] > green[task]
            || suite.is_some_and(|date| date < green[task])
            || (policy.ci_required && ci.is_some_and(|date| date < green[task]))
        {
            errors.push(format!(
                "Task {task}: rispettare l'ordine red → green → suite finale (CI quando richiesta)."
            ));
        }
    }
    if suite.is_none() {
        errors.push("Serve evidenza della suite finale verde, locale o CI secondo policy.".into());
    }
    if policy.ci_required && ci.is_none() {
        errors.push("Serve evidenza della suite completa verde in CI.".into());
    }
    if let Some(decision) = state.decisions.last() {
        for condition in &decision.conditions {
            if !matches!(
                condition.due_phase.as_str(),
                "pre-release" | "release" | "rollout" | "post-release"
            ) {
                errors.push(format!("Condizione da risolvere prima della pre-release: {} ({}). Registrare la nuova decisione umana con decide.", condition.text, condition.owner));
            }
        }
    }
    let ready = errors.is_empty();
    Ok(
        json!({"gate":if !ready {"blocked-evidence"} else if policy.mode == LifecycleMode::DevelopmentOnly {"development-complete"} else {"ready-for-pre-release"}, "errors":errors,"lifecycle":policy,
        "design_fingerprint":fingerprint,"plan_hash":plan_hash(store, &state.id)?,"next":if !ready {"dfd-implement"} else if policy.mode == LifecycleMode::DevelopmentOnly {"development-complete"} else {"dfd-pre-release"}}),
    )
}

pub(super) fn verify(store: &Store, id: &str) -> Result<(Value, i32)> {
    let mut state = state(store, id)?;
    let output = status(store, &state)?;
    let ready = matches!(
        output["gate"].as_str(),
        Some("ready-for-pre-release" | "development-complete")
    );
    let mut report = format!("# Verifica sviluppo DFD\n\nGate: {}\n\n", output["gate"]);
    for error in output["errors"].as_array().unwrap() {
        report.push_str(&format!("- {}\n", error.as_str().unwrap()));
    }
    report.push_str("\nControlli strutturali su evidenze dichiarate e hash locali; non certificano autenticità dei log, qualità del codice o approvazione del rilascio.\n");
    store.write(
        &format!(".dfd/features/{id}/development-review.md"),
        report.as_bytes(),
        false,
    )?;
    if ready || state.phase == "development-verified" {
        state.phase = if ready {
            "development-verified"
        } else {
            "development"
        }
        .into();
        store.save(&format!(".dfd/features/{id}/state.json"), &state, false)?;
    }
    Ok((
        json!({"feature":id,"development":output,"phase":state.phase}),
        if ready { 0 } else { 2 },
    ))
}
