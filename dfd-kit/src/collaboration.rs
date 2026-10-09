use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Policy {
    pub schema_version: u32,
    pub independent_review: bool,
    pub integrated_checks_required: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Context {
    pub schema_version: u32,
    pub owner: String,
    pub branch: String,
    pub revision: String,
    pub issue: String,
    pub pull_request: String,
    pub integration: Option<Integration>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Integration {
    pub revision: String,
    pub files: Vec<FileEvidence>,
    pub checks: Vec<TestEvidence>,
}
pub(super) fn policy(store: &Store, domain: &str) -> Result<Policy> {
    let path = format!(".dfd/domains/{domain}/team.json");
    if store.path(&path)?.exists() {
        store.json(&path)
    } else {
        Ok(Policy {
            schema_version: 1,
            ..Policy::default()
        })
    }
}
pub(super) fn context(store: &Store, id: &str) -> Result<Option<Context>> {
    slug(id)?;
    let path = format!(".dfd/features/{id}/collaboration.json");
    if store.path(&path)?.exists() {
        Ok(Some(store.json(&path)?))
    } else {
        Ok(None)
    }
}
pub(super) fn set(store: &Store, id: &str, context: Context) -> Result<Value> {
    slug(id)?;
    if [&context.owner, &context.branch, &context.revision]
        .iter()
        .any(|s| !nonblank(s))
    {
        return Err(Error::message(
            "Owner, branch e revisione sono obbligatori nel contesto di collaborazione.",
        ));
    }
    if quick::exists(store, id)? {
        return quick::set_context(store, id, context);
    }
    state(store, id)?;
    let path = format!(".dfd/features/{id}/collaboration.json");
    let integration = self::context(store, id)?.and_then(|c| c.integration);
    let context = Context {
        integration,
        ..context
    };
    store.save(&path, &context, false)?;
    Ok(json!({"feature":id,"collaboration":context}))
}
pub(super) fn check_errors(store: &Store, check: &TestEvidence) -> Result<Vec<String>> {
    let mut errors = development::file_errors(store, &check.log)?;
    if !nonblank(&check.command)
        || !store.path(&check.log.path)?.is_file()
        || !nonblank(&store.read(&check.log.path)?)
    {
        errors.push("Comando e log reale non vuoto richiesti.".into());
    }
    if !DateTime::parse_from_rfc3339(&check.executed_at).is_ok_and(|date| date <= Utc::now()) {
        errors.push("Data esecuzione non valida o futura.".into());
    }
    if check.kind == CheckKind::Red {
        if check.exit_code <= 0 {
            errors.push("Il red deve fallire per il requisito atteso.".into());
        }
    } else if check.exit_code != 0 {
        errors.push("Verifica dichiarata non verde.".into());
    }
    Ok(errors)
}
pub(super) fn bind(
    store: &Store,
    context: Option<&Context>,
    files: &[String],
    decision: &mut Decision,
) -> Result<()> {
    if let Some(c) = context {
        if [&c.owner, &c.branch, &c.revision]
            .iter()
            .any(|s| !nonblank(s))
        {
            return Err(Error::message("Contesto di collaborazione incompleto."));
        }
        decision.revision = Some(c.revision.clone());
        if decision.reference.is_none() && nonblank(&c.pull_request) {
            decision.reference = Some(c.pull_request.clone());
        }
    }
    let mut seen = BTreeSet::new();
    for path in files {
        if !seen.insert(path) {
            return Err(Error::message("File di review duplicato."));
        }
        decision.files.push(FileEvidence {
            path: path.clone(),
            sha256: store.hash(path)?,
        });
    }
    Ok(())
}
pub(super) fn reviewer_errors(policy: &Policy, owner: &str, decision: &Decision) -> Vec<String> {
    if policy.independent_review && owner.trim().eq_ignore_ascii_case(decision.reviewer.trim()) {
        vec!["La policy team richiede un reviewer diverso dall'owner.".into()]
    } else {
        vec![]
    }
}
pub(super) fn release_errors(store: &Store, state: &State) -> Result<Vec<String>> {
    let policy = policy(store, &state.domain)?;
    if !policy.independent_review && !policy.integrated_checks_required {
        return Ok(vec![]);
    }
    let Some(context) = context(store, &state.id)? else {
        return Ok(vec![
            "Policy team: registrare owner, branch, PR e revisione con context.".into(),
        ]);
    };
    let mut errors = vec![];
    if [
        &context.owner,
        &context.branch,
        &context.revision,
        &context.pull_request,
    ]
    .iter()
    .any(|s| !nonblank(s))
    {
        errors.push("Contesto team incompleto: owner, branch, PR e revisione richiesti.".into());
    }
    let evidence: Evidence = store.json(&format!(".dfd/features/{}/evidence.json", state.id))?;
    if policy.independent_review {
        if let Some(decision) = state.decisions.last() {
            errors.extend(reviewer_errors(&policy, &context.owner, decision));
            if decision.revision.as_deref() != Some(context.revision.as_str())
                || decision.reference.as_ref().is_none_or(|r| !nonblank(r))
                || decision.files.is_empty()
            {
                errors.push("Review PR non riferita alla revisione e ai file correnti: registrare decide con --file e riferimento alla PR.".into());
            }
            for file in &decision.files {
                errors.extend(development::file_errors(store, file)?);
            }
            for file in &evidence.files {
                if !decision
                    .files
                    .iter()
                    .any(|reviewed| reviewed.path == file.path && reviewed.sha256 == file.sha256)
                {
                    errors.push(format!(
                        "File verificato non incluso nella review corrente: {}.",
                        file.path
                    ));
                }
            }
        } else {
            errors.push("Serve una review umana corrente.".into());
        }
    }
    if policy.integrated_checks_required {
        if let Some(integration) = &context.integration {
            if integration.revision != context.revision || integration.files.is_empty() {
                errors.push(
                    "Prove integrate riferite a una revisione diversa o senza snapshot.".into(),
                );
            }
            for file in &integration.files {
                errors.extend(development::file_errors(store, file)?);
            }
            for file in &evidence.files {
                if !integration.files.iter().any(|integrated| {
                    integrated.path == file.path && integrated.sha256 == file.sha256
                }) {
                    errors.push(format!(
                        "File non incluso nella verifica integrata: {}.",
                        file.path
                    ));
                }
            }
            let ci_required = lifecycle(store, &state.domain)?.ci_required;
            let mut green = false;
            for check in &integration.checks {
                errors.extend(check_errors(store, check)?);
                if check.kind == CheckKind::Suite || check.kind == CheckKind::Ci {
                    if !ci_required || check.kind == CheckKind::Ci {
                        green = true;
                    }
                } else {
                    errors.push("La verifica integrata richiede una suite finale, non un singolo red/green.".into());
                }
            }
            if !green {
                errors.push("Manca la suite integrata finale prevista dalla policy CI.".into());
            }
        } else {
            errors.push(
                "Mancano prove della versione integrata: i test del branch isolato non bastano."
                    .into(),
            );
        }
    }
    Ok(errors)
}
