use super::*;

/// Default markers for passing logs: unittest, pytest and cargo failures or skips.
/// Generated directories skipped under source_roots unless the domain sets source_excludes.
const DEFAULT_SOURCE_EXCLUDES: [&str; 3] = ["__pycache__", "node_modules", "target"];

fn source_excludes(policy: &Lifecycle) -> Vec<String> {
    policy.source_excludes.clone().unwrap_or_else(|| {
        DEFAULT_SOURCE_EXCLUDES
            .iter()
            .map(|name| name.to_string())
            .collect()
    })
}

const DEFAULT_LOG_MARKERS: [&str; 6] = [
    "FAILED",
    "skipped",
    "Traceback",
    "xfail",
    "XFAIL",
    "panicked",
];

fn entry_errors(store: &Store, state: &State) -> Result<Vec<String>> {
    let mut errors = validate_design(store, state)?;
    let gate = gate(store, state, &errors)?;
    if !matches!(gate, "approved" | "approved-with-conditions") {
        errors.push(crate::localized_format!(
            "Sviluppo: serve una review di design approvata e attuale (gate: {gate}).",
            "Development requires a current approved design review (gate: {gate}).",
            gate = gate
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
            description: crate::localized_format!("Soddisfare {}", "Satisfy {}", criterion.id),
            criteria: vec![criterion.id.clone()],
            implementation: criterion.implementation.clone(),
            verification: criterion.verification.clone(),
        })
        .collect();
    let mut markdown = crate::localized_format!(
        "# Piano di sviluppo – {}\n\n## Ambito e test\n\n{}\n\n{}\n\n## Task TDD\n\n",
        "# Development plan – {}\n\n## Scope and tests\n\n{}\n\n{}\n\n## TDD tasks\n\n",
        state.title,
        state.scope,
        design.test_plan
    );
    for task in &tasks {
        markdown.push_str(&crate::localized_format!(
            "### {} – {}\n\nOwner: {}\nCriteri: {}\n\nImplementazione: {}\n\nVerifica: {}\n\n",
            "### {} – {}\n\nOwner: {}\nCriteria: {}\n\nImplementation: {}\n\nVerification: {}\n\n",
            task.id,
            task.description,
            task.owner,
            task.criteria.join(", "),
            task.implementation,
            task.verification
        ));
    }
    markdown.push_str(&crate::i18n::text("## Ciclo di sviluppo\n\nPer ogni task: test rosso sul comportamento atteso, implementazione minima, test verde, refactor e riesecuzione dei test. Conservare le evidenze e verificare la suite finale secondo lifecycle.json: locale (suite) o CI quando richiesta.\n"));
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
        errors.push(crate::i18n::text("Piano derivato da un design precedente: riconfermare la review e usare plan --refresh."));
    }
    let markdown = store.read(&format!(".dfd/features/{}/plan.md", state.id))?;
    if !nonblank(&markdown) || markdown.contains("TODO") || !plan.open_questions.is_empty() {
        errors.push(crate::i18n::text(
            "Piano incompleto o dubbi aperti: chiarire prima dello sviluppo.",
        ));
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
        errors.push(crate::i18n::text("Piano senza task."));
    }
    for task in &plan.tasks {
        slug(&task.id)?;
        if !ids.insert(&task.id) {
            errors.push(crate::localized_format!(
                "Task duplicato: {}.",
                "Duplicate task: {}.",
                task.id
            ));
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
            errors.push(crate::localized_format!(
                "Task incompleto: {}.",
                "Incomplete task: {}.",
                task.id
            ));
        }
        let mut task_ids = BTreeSet::new();
        for criterion in &task.criteria {
            if !task_ids.insert(criterion) || !applicable.contains(criterion.as_str()) {
                errors.push(crate::localized_format!(
                    "Task {}: criterio sconosciuto, escluso o duplicato: {criterion}.",
                    "Task {}: unknown, excluded or duplicate criterion: {criterion}.",
                    task.id,
                    criterion = criterion
                ));
            }
            covered.insert(criterion.as_str());
            if !criterion_ids(&markdown).contains(criterion) {
                errors.push(crate::localized_format!(
                    "Piano Markdown: manca {criterion}.",
                    "Markdown plan: missing {criterion}.",
                    criterion = criterion
                ));
            }
        }
    }
    for id in applicable.difference(&covered) {
        errors.push(crate::localized_format!(
            "Criterio applicabile senza task: {id}.",
            "Applicable criterion has no task: {id}.",
            id = id
        ));
    }
    Ok(errors)
}

pub(super) fn file_errors(store: &Store, file: &FileEvidence) -> Result<Vec<String>> {
    let path = store.path(&file.path)?;
    if !path.is_file() {
        return Ok(vec![crate::localized_format!(
            "Evidenza assente: {}.",
            "Missing evidence: {}.",
            file.path
        )]);
    }
    if file.sha256 != store.hash(&file.path)? {
        return Ok(vec![crate::localized_format!(
            "Evidenza modificata: {}; rieseguire le verifiche.",
            "Modified evidence: {}; rerun verification.",
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
        errors.push(crate::i18n::text(
            "Artefatti di sviluppo parziali: ripristinare plan.json, plan.md ed evidence.json.",
        ));
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
        errors.push(crate::i18n::text(
            "Evidenze riferite a un design o piano precedente: rieseguire le verifiche.",
        ));
    }
    if evidence.files.is_empty() {
        errors.push(crate::i18n::text(
            "Registrare i file di codice, test e strumentazione verificati.",
        ));
    }
    let mut seen = BTreeSet::new();
    for file in &evidence.files {
        if !seen.insert(&file.path) {
            errors.push(crate::localized_format!(
                "File verificato duplicato: {}.",
                "Duplicate verified file: {}.",
                file.path
            ));
        }
        errors.extend(file_errors(store, file)?);
    }
    if !nonblank(&evidence.refactor_notes) {
        errors.push(crate::i18n::text("Documentare refactor e strumentazione osservabilità, oppure motivare perché non necessari."));
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
            errors.push(crate::i18n::text("Comando di test mancante."));
            valid = false;
        }
        let executed_at = DateTime::parse_from_rfc3339(&check.executed_at).ok();
        match executed_at {
            Some(date) if date <= Utc::now() => {}
            _ => {
                errors.push(crate::i18n::text(
                    "Data esecuzione test non valida o futura.",
                ));
                valid = false;
            }
        }
        let log_errors = file_errors(store, &check.log)?;
        if !log_errors.is_empty() {
            valid = false;
        }
        errors.extend(log_errors);
        if store.path(&check.log.path)?.is_file() && !nonblank(&store.read(&check.log.path)?) {
            errors.push(crate::i18n::text("Log di test vuoto."));
            valid = false;
        }
        if let Some(task) = &check.task {
            if !task_ids.contains(task.as_str()) {
                errors.push(crate::localized_format!(
                    "Evidenza per task sconosciuto: {task}.",
                    "Evidence for unknown task: {task}.",
                    task = task
                ));
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
                        errors.push(crate::localized_format!(
                            "Esito TDD non valido per {task}.",
                            "Invalid TDD result for {task}.",
                            task = task
                        ));
                    }
                } else {
                    errors.push(crate::i18n::text("Test red/green senza task."));
                }
            }
            CheckKind::Suite | CheckKind::Ci => errors.push(crate::i18n::text(
                "Suite finale locale/CI non verde o non valida.",
            )),
        }
    }
    for task in &task_ids {
        if !red.contains_key(task) || !green.contains_key(task) {
            errors.push(crate::localized_format!(
                "Task {task}: servono evidenze red e green.",
                "Task {task}: red and green evidence required.",
                task = task
            ));
        } else if red[task] > green[task]
            || suite.is_some_and(|date| date < green[task])
            || (policy.ci_required && ci.is_some_and(|date| date < green[task]))
        {
            errors.push(crate::localized_format!("Task {task}: rispettare l'ordine red → green → suite finale (CI quando richiesta).", "Task {task}: follow the order red → green → final suite (CI when required).", task = task));
        }
    }
    if suite.is_none() {
        errors.push(crate::i18n::text(
            "Serve evidenza della suite finale verde, locale o CI secondo policy.",
        ));
    }
    if policy.ci_required && ci.is_none() {
        errors.push(crate::i18n::text(
            "Serve evidenza della suite completa verde in CI.",
        ));
    }
    if let Some(decision) = state.decisions.last() {
        for condition in &decision.conditions {
            if !matches!(
                condition.due_phase.as_str(),
                "pre-release" | "release" | "rollout" | "post-release"
            ) {
                errors.push(crate::localized_format!("Condizione da risolvere prima della pre-release: {} ({}). Registrare la nuova decisione umana con decide.", "Condition to resolve before pre-release: {} ({}). Record the new human decision with decide.", condition.text, condition.owner));
            }
        }
    }
    // Suspicious markers in passing logs: warnings only, the gate is unchanged (issue #4).
    let markers: Vec<String> = policy.log_markers.clone().unwrap_or_else(|| {
        DEFAULT_LOG_MARKERS
            .iter()
            .map(|marker| marker.to_string())
            .collect()
    });
    let mut warnings = vec![];
    for check in &evidence.checks {
        // Only passing green/suite/ci logs are suspicious: a red log is expected to fail.
        let passing = check.exit_code == 0
            && matches!(
                check.kind,
                CheckKind::Green | CheckKind::Suite | CheckKind::Ci
            );
        if !passing {
            continue;
        }
        let Some(body) = store
            .path(&check.log.path)
            .ok()
            .filter(|file| file.is_file())
            .and_then(|_| store.read(&check.log.path).ok())
        else {
            continue;
        };
        let kind = serde_json::to_value(check.kind)?;
        for marker in markers.iter().filter(|marker| !marker.is_empty()) {
            if body.contains(marker.as_str()) {
                warnings.push(crate::localized_format!(
                    "Log {} ({}, exit 0) contiene il marcatore «{}».",
                    "Log {} ({}, exit 0) contains the marker “{}”.",
                    check.log.path,
                    kind.as_str().unwrap_or_default(),
                    marker
                ));
            }
        }
    }
    // Acceptance files nobody declares are not protected by any review: warn.
    if !policy.acceptance_roots.is_empty() {
        let mut files = BTreeMap::new();
        for root in &policy.acceptance_roots {
            store.tree(root, &source_excludes(&policy), &mut files)?;
        }
        let declared = declared_acceptance(store, &state.domain)?;
        for path in files.keys().filter(|path| !declared.contains(*path)) {
            warnings.push(crate::localized_format!(
                "File di casi non dichiarato da alcun criterio: {}.",
                "Acceptance file not declared by any criterion: {}.",
                path
            ));
        }
    }
    // Evidence bound to the files and the design it ran on (issue #3): only when the
    // domain declares source_roots.
    let current_criteria = criteria_fingerprints(store, state)?;
    if !policy.source_roots.is_empty() {
        let mut current = BTreeMap::new();
        for root in &policy.source_roots {
            store.tree(root, &source_excludes(&policy), &mut current)?;
        }
        let latest_suite = evidence
            .checks
            .iter()
            .filter(|check| {
                matches!(check.kind, CheckKind::Suite | CheckKind::Ci) && check.exit_code == 0
            })
            .filter_map(|check| {
                DateTime::parse_from_rfc3339(&check.executed_at)
                    .ok()
                    .map(|date| (date, check))
            })
            .max_by_key(|(date, _)| *date)
            .map(|(_, check)| check);
        if let Some(suite) = latest_suite {
            match &suite.snapshot {
                None => errors.push(crate::i18n::text(
                    "Suite finale senza snapshot dei file: rieseguirla con dfd exec o registrarne lo snapshot.",
                )),
                Some(snapshot) => {
                    let mut differing: Vec<_> = current
                        .iter()
                        .filter(|(path, sha)| snapshot.get(*path) != Some(*sha))
                        .map(|(path, _)| path.clone())
                        .collect();
                    differing.extend(
                        snapshot
                            .keys()
                            .filter(|path| !current.contains_key(*path))
                            .cloned(),
                    );
                    if !differing.is_empty() {
                        errors.push(crate::localized_format!(
                            "Suite eseguita su una versione diversa dei file: {}; rieseguire la suite.",
                            "Suite run on a different version of the files: {}; rerun the suite.",
                            differing.join(", ")
                        ));
                    }
                }
            }
        }
        let declared: BTreeSet<_> = evidence.files.iter().map(|file| &file.path).collect();
        for path in current.keys().filter(|path| !declared.contains(path)) {
            warnings.push(crate::localized_format!(
                "File sotto source_roots non dichiarato in evidence.files: {}.",
                "File under source_roots not declared in evidence.files: {}.",
                path
            ));
        }
        for task in &plan.tasks {
            let latest_green = evidence
                .checks
                .iter()
                .filter(|check| {
                    check.kind == CheckKind::Green
                        && check.exit_code == 0
                        && check.task.as_deref() == Some(task.id.as_str())
                })
                .filter_map(|check| {
                    DateTime::parse_from_rfc3339(&check.executed_at)
                        .ok()
                        .map(|date| (date, check))
                })
                .max_by_key(|(date, _)| *date)
                .map(|(_, check)| check);
            let Some(green) = latest_green else { continue };
            for criterion in &task.criteria {
                let recorded = green
                    .criteria_fingerprints
                    .as_ref()
                    .and_then(|map| map.get(criterion));
                if recorded != current_criteria.get(criterion) {
                    errors.push(crate::localized_format!(
                        "Task {}: criterio {} cambiato dopo il green: servono nuovi red/green.",
                        "Task {}: criterion {} changed after the green: new red/green required.",
                        task.id,
                        criterion
                    ));
                }
            }
        }
    }
    let ready = errors.is_empty();
    Ok(
        json!({"gate":if !ready {"blocked-evidence"} else if policy.mode == LifecycleMode::DevelopmentOnly {"development-complete"} else {"ready-for-pre-release"}, "errors":errors,"lifecycle":policy,
        "design_fingerprint":fingerprint,"plan_hash":plan_hash(store, &state.id)?,"warnings":warnings,"criteria_fingerprints":current_criteria,"next":if !ready {"dfd-implement"} else if policy.mode == LifecycleMode::DevelopmentOnly {"development-complete"} else {"dfd-pre-release"}}),
    )
}

pub(super) fn verify(store: &Store, id: &str) -> Result<(Value, i32)> {
    let mut state = state(store, id)?;
    let output = status(store, &state)?;
    let ready = matches!(
        output["gate"].as_str(),
        Some("ready-for-pre-release" | "development-complete")
    );
    let mut report = crate::localized_format!(
        "# Verifica sviluppo DFD\n\nGate: {}\n\n",
        "# DFD development verification\n\nGate: {}\n\n",
        output["gate"]
    );
    for error in output["errors"].as_array().unwrap() {
        report.push_str(&format!("- {}\n", error.as_str().unwrap()));
    }
    let warnings = output["warnings"].as_array().cloned().unwrap_or_default();
    if !warnings.is_empty() {
        report.push_str(&crate::localized_format!(
            "\n## Avvisi\n\n",
            "\n## Warnings\n\n"
        ));
        for warning in &warnings {
            report.push_str(&format!("- {}\n", warning.as_str().unwrap_or_default()));
        }
    }
    report.push_str(&crate::i18n::text("\nControlli strutturali su evidenze dichiarate e hash locali; non certificano autenticità dei log, qualità del codice o approvazione del rilascio.\n"));
    store.write(
        &format!(".dfd/features/{id}/development-review.md"),
        report.as_bytes(),
        false,
    )?;
    // Later phases are not moved back by a successful verify; a failed verify returns
    // any verified or later phase to development (issue #7).
    let later = matches!(state.phase.as_str(), "pre-release" | "release-reviewed");
    let phase = if ready && !later {
        Some("development-verified")
    } else if !ready && (later || state.phase == "development-verified") {
        Some("development")
    } else {
        None
    };
    if let Some(phase) = phase {
        state.phase = phase.into();
        store.save(&format!(".dfd/features/{id}/state.json"), &state, false)?;
    }
    Ok((
        json!({"feature":id,"development":output,"phase":state.phase}),
        if ready { 0 } else { 2 },
    ))
}

/// Run a project command on explicit request and record the check (issue #3): no shell,
/// project lock held by the caller, numbered log in evidence/, snapshot of source_roots
/// (or of evidence.files), criteria fingerprints for red/green, recorded_by "dfd".
pub(super) fn exec(
    store: &Store,
    id: &str,
    kind: ExecKind,
    task: Option<String>,
    command: &[String],
) -> Result<(Value, i32)> {
    let state = state(store, id)?;
    let root = format!(".dfd/features/{id}");
    let evidence_path = format!("{root}/evidence.json");
    if !store.path(&evidence_path)?.is_file() {
        return Err(Error::message("Evidenze assenti: eseguire prima dfd plan."));
    }
    let plan: Plan = store.json(&format!("{root}/plan.json"))?;
    let kind = match kind {
        ExecKind::Red => CheckKind::Red,
        ExecKind::Green => CheckKind::Green,
        ExecKind::Suite => CheckKind::Suite,
    };
    let criteria = match (kind, &task) {
        (CheckKind::Suite, None) => None,
        (CheckKind::Suite, Some(_)) => {
            return Err(Error::message(
                "La suite non appartiene a un task: omettere --task.",
            ))
        }
        (_, None) => return Err(Error::message("Red e green richiedono --task.")),
        (_, Some(task)) => {
            let Some(planned) = plan.tasks.iter().find(|t| &t.id == task) else {
                return Err(Error::message(crate::localized_format!(
                    "Task sconosciuto nel piano: {task}.",
                    "Unknown task in the plan: {task}.",
                    task = task
                )));
            };
            let current = criteria_fingerprints(store, &state)?;
            Some(
                planned
                    .criteria
                    .iter()
                    .filter_map(|c| current.get(c).map(|fp| (c.clone(), fp.clone())))
                    .collect::<BTreeMap<_, _>>(),
            )
        }
    };
    let Some((program, args)) = command.split_first() else {
        return Err(Error::message("Indicare il comando da eseguire dopo --."));
    };
    let output = std::process::Command::new(program)
        .args(args)
        .current_dir(&store.root)
        .output()
        .map_err(|error| {
            Error::message(crate::localized_format!(
                "Comando non avviabile: {program}: {error}",
                "Command could not start: {program}: {error}",
                program = program,
                error = error
            ))
        })?;
    let executed_at = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
    let exit_code = output.status.code().unwrap_or(-1);
    let mut evidence: Evidence = store.json(&evidence_path)?;
    let policy = lifecycle(store, &state.domain)?;
    let mut snapshot = BTreeMap::new();
    if policy.source_roots.is_empty() {
        for file in &evidence.files {
            if store.path(&file.path)?.is_file() {
                snapshot.insert(file.path.clone(), store.hash(&file.path)?);
            }
        }
    } else {
        for source in &policy.source_roots {
            store.tree(source, &source_excludes(&policy), &mut snapshot)?;
        }
    }
    let label = serde_json::to_value(kind)?;
    let label = label.as_str().unwrap_or_default();
    let suffix = task.as_ref().map(|t| format!("-{t}")).unwrap_or_default();
    let mut number = evidence.checks.len() + 1;
    let log = loop {
        let candidate = format!("{root}/evidence/{number:02}-exec-{label}{suffix}.log");
        if !store.path(&candidate)?.exists() {
            break candidate;
        }
        number += 1;
    };
    let line = command.join(" ");
    let mut body = format!("$ {line}\n# executed_at: {executed_at}\n").into_bytes();
    body.extend_from_slice(&output.stdout);
    body.extend_from_slice(&output.stderr);
    body.extend_from_slice(format!("# exit_code: {exit_code}\n").as_bytes());
    store.write(&log, &body, true)?;
    let check = TestEvidence {
        task,
        kind,
        command: line,
        executed_at,
        exit_code,
        log: FileEvidence {
            path: log.clone(),
            sha256: store.hash(&log)?,
        },
        snapshot: Some(snapshot),
        criteria_fingerprints: criteria,
        recorded_by: Some("dfd".into()),
    };
    let recorded = serde_json::to_value(&check)?;
    evidence.checks.push(check);
    store.save(&evidence_path, &evidence, false)?;
    Ok((json!({"feature":id,"check":recorded,"log":log}), 0))
}
