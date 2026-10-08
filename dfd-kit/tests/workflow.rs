use chrono::Utc;
use serde_json::{json, Value};
use std::{fs, process::Command};
use tempfile::TempDir;

struct Project(TempDir);

impl Project {
    fn new() -> Self {
        Self(tempfile::tempdir().unwrap())
    }
    fn path(&self, relative: &str) -> std::path::PathBuf {
        self.0.path().join(relative)
    }
    fn write(&self, relative: &str, value: &str) {
        let path = self.path(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, value).unwrap();
    }
    fn save(&self, relative: &str, value: &Value) {
        self.write(relative, &serde_json::to_string_pretty(value).unwrap());
    }
    fn read(&self, relative: &str) -> Value {
        serde_json::from_str(&fs::read_to_string(self.path(relative)).unwrap()).unwrap()
    }
    fn run(&self, args: &[&str], expected: i32) -> Value {
        // No interpreters, jq, harness, or source tree at runtime.
        let output = Command::new(env!("CARGO_BIN_EXE_dfd"))
            .env_clear()
            .env("PATH", "/dfd-no-external-tools")
            .arg("--project")
            .arg(self.0.path())
            .args(args)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(expected),
            "args: {args:?}\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if output.stdout.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&output.stdout).unwrap()
        }
    }
    fn init(&self) {
        self.run(&["init", "--harness", "codex", "--domain", "checkout"], 0);
    }
    fn feature(&self, id: &str) {
        self.run(
            &[
                "feature",
                id,
                "--domain",
                "checkout",
                "--title",
                "PayPal",
                "--scope",
                "Nuovo pagamento",
            ],
            0,
        );
    }
    fn risk(&self, id: &str) {
        let dimensions = [
            "security",
            "reliability",
            "cost",
            "business",
            "architecture",
        ]
        .into_iter()
        .map(|key| {
            (
                key.to_owned(),
                json!({"level":"low","rationale":"Nessuna modifica in questa dimensione"}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
        self.save(
            &format!(".dfd/features/{id}/risk.json"),
            &json!({"schema_version":1,"dimensions":dimensions}),
        );
        self.run(&["assess", id], 0);
    }
    fn ready(&self) {
        self.init();
        self.write(
            ".dfd/domains/checkout/guardrails.md",
            "# Guardrail\nUsare il provider pagamenti esistente.\n",
        );
        self.write(".dfd/domains/checkout/dod.md", "# DoD Checkout\nOBS-01: contare gli errori del nuovo flusso.\nSLO-01: baseline legacy da migliorare.\n");
        self.save(
            ".dfd/domains/checkout/criteria.json",
            &json!({"schema_version":1,"criteria":[
                {"id":"OBS-01","statement":"Contare errori","scope":"changes"},
                {"id":"SLO-01","statement":"Definire SLO legacy","scope":"baseline"}
            ]}),
        );
        self.write(".dfd/domains/checkout/process.md", "# Processo\nRegole di rischio centrali; checklist light/full; template invariati; artefatti in .dfd; owner Checkout.\n");
        self.run(&["setup", "--domain", "checkout"], 0);
        self.feature("001-paypal");
        self.risk("001-paypal");
        self.run(&["specify", "001-paypal"], 0);
        self.write(".dfd/features/001-paypal/spec.md", "# PayPal\nObiettivo: pagare con PayPal.\nOBS-01: contatore errori verificato con test di integrazione.\n");
        self.save(".dfd/features/001-paypal/design.json", &json!({
            "schema_version":1,"owner":"checkout-team","objective":"Pagare con PayPal",
            "business_metrics":["Tasso di completamento checkout"],
            "epic":{"reference":"ticket-123","approved_by":"PM","approved_at":Utc::now().to_rfc3339()},
            "criteria":[{"id":"OBS-01","applicability":"applicable","implementation":"Contatore errori","verification":"Test di integrazione"}],
            "test_plan":"Test del nuovo flusso", "guardrails":"Usare il provider esistente","open_questions":[]
        }));
    }
    fn approve(&self) {
        self.run(
            &[
                "decide",
                "001-paypal",
                "--decision",
                "approved",
                "--reviewer",
                "TL",
                "--role",
                "tech-lead",
                "--note",
                "Review semantica effettuata",
                "--human-confirmed",
            ],
            0,
        );
    }
}

#[test]
fn doctor_works_without_project_or_external_tools() {
    let output = Command::new(env!("CARGO_BIN_EXE_dfd"))
        .env_clear()
        .args(["--project", "/nonexistent", "doctor"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["runtime_dependencies"], json!([]));
    assert_eq!(value["skills"].as_array().unwrap().len(), 12);
}

#[test]
fn all_adapters_install_embedded_skills_and_preserve_instructions() {
    for (adapter, path) in [
        ("codex", ".agents/skills"),
        ("claude-code", ".claude/skills"),
        ("copilot", ".github/skills"),
        ("opencode", ".opencode/skills"),
    ] {
        let p = Project::new();
        for file in ["AGENTS.md", "CLAUDE.md", ".github/copilot-instructions.md"] {
            p.write(file, "Istruzioni del team\n");
        }
        p.run(&["init", "--domain", "checkout", "--harness", adapter], 0);
        for skill in [
            "dfd-init",
            "dfd-feature",
            "dfd-plan",
            "dfd-implement",
            "dfd-verify",
            "dfd-pre-release",
            "dfd-review-release",
        ] {
            assert!(p.path(&format!("{path}/{skill}/SKILL.md")).is_file());
        }
        assert_eq!(
            fs::read_to_string(p.path(".dfd/kit/adozione.md")).unwrap(),
            include_str!("../adozione.md")
        );
        for file in ["AGENTS.md", "CLAUDE.md", ".github/copilot-instructions.md"] {
            assert_eq!(
                fs::read_to_string(p.path(file)).unwrap(),
                "Istruzioni del team\n"
            );
        }
    }
}

#[test]
fn repeated_init_preserves_domain_work_and_is_fingerprint_stable() {
    let p = Project::new();
    p.ready();
    p.approve();
    let before = p.read(".dfd/domains/checkout/adoption.json");
    p.init();
    assert_eq!(p.read(".dfd/domains/checkout/adoption.json"), before);
    assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "approved");
}

#[test]
fn switch_adapter_keeps_feature_and_review_without_duplicate_entrypoints() {
    let p = Project::new();
    p.ready();
    p.approve();
    let before = p.read(".dfd/features/001-paypal/state.json");
    p.run(&["install", "--harness", "claude-code"], 0);
    assert!(!p.path(".agents/skills/dfd-init/SKILL.md").exists());
    assert!(p.path(".claude/skills/dfd-init/SKILL.md").exists());
    assert_eq!(p.read(".dfd/features/001-paypal/state.json"), before);
    assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "approved");
}

#[test]
fn modified_managed_file_stops_installation_before_any_changes() {
    let p = Project::new();
    p.init();
    p.write(".agents/skills/dfd-specify/SKILL.md", "Modifica del team");
    let manifest = p.read(".dfd/install.json");
    p.run(&["install", "--harness", "claude-code"], 1);
    assert_eq!(p.read(".dfd/install.json"), manifest);
    assert!(!p.path(".claude/skills/dfd-init/SKILL.md").exists());
    assert_eq!(
        fs::read_to_string(p.path(".agents/skills/dfd-specify/SKILL.md")).unwrap(),
        "Modifica del team"
    );
}

#[test]
fn unmanaged_compatible_skill_is_preserved_and_blocks_installation() {
    let p = Project::new();
    p.write(".claude/skills/dfd-assess/SKILL.md", "Un'altra skill DFD");
    p.run(&["install", "--harness", "codex"], 1);
    assert!(!p.path(".agents/skills/dfd-init/SKILL.md").exists());
    assert_eq!(
        fs::read_to_string(p.path(".claude/skills/dfd-assess/SKILL.md")).unwrap(),
        "Un'altra skill DFD"
    );
}

#[test]
fn unknown_risk_does_not_become_low_or_allow_specification() {
    let p = Project::new();
    p.init();
    p.feature("001-paypal");
    let output = p.run(&["assess", "001-paypal"], 2);
    assert_eq!(output["risk"], Value::Null);
    assert_eq!(output["missing"].as_array().unwrap().len(), 5);
    p.run(&["specify", "001-paypal"], 1);
    assert!(!p.path(".dfd/features/001-paypal/spec.md").exists());
}

#[test]
fn malformed_risk_does_not_modify_state() {
    let p = Project::new();
    p.init();
    p.feature("001-paypal");
    let state = p.read(".dfd/features/001-paypal/state.json");
    let mut risk = p.read(".dfd/features/001-paypal/risk.json");
    risk["dimensions"]["security"]["level"] = json!("loow");
    p.save(".dfd/features/001-paypal/risk.json", &risk);
    p.run(&["assess", "001-paypal"], 1);
    assert_eq!(p.read(".dfd/features/001-paypal/state.json"), state);
}

#[test]
fn independent_features_keep_state_separate() {
    let p = Project::new();
    p.init();
    p.feature("001-paypal");
    p.feature("002-refactor");
    let before = p.read(".dfd/features/002-refactor/state.json");
    p.risk("001-paypal");
    assert_eq!(p.read(".dfd/features/002-refactor/state.json"), before);
}

#[test]
fn specification_cannot_be_overwritten() {
    let p = Project::new();
    p.ready();
    let before = fs::read(p.path(".dfd/features/001-paypal/spec.md")).unwrap();
    assert_eq!(p.run(&["specify", "001-paypal"], 0)["resumed"], true);
    assert_eq!(
        fs::read(p.path(".dfd/features/001-paypal/spec.md")).unwrap(),
        before
    );
}

#[test]
fn baseline_gaps_outside_scope_do_not_block_pilot_design() {
    let p = Project::new();
    p.ready();
    let mut adoption = p.read(".dfd/domains/checkout/adoption.json");
    adoption["baseline_gaps"] = json!(["Mancano SLO di flussi legacy non modificati"]);
    p.save(".dfd/domains/checkout/adoption.json", &adoption);
    let result = p.run(&["review-design", "001-paypal"], 0);
    assert_eq!(result["gate"], "awaiting-human-review");
    assert_eq!(result["decisions"], json!([]));
    p.approve();
    assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "approved");
}

#[test]
fn changes_criteria_require_coverage_or_justified_exclusion() {
    let p = Project::new();
    p.ready();
    let mut catalog = p.read(".dfd/domains/checkout/criteria.json");
    catalog["criteria"][1]["scope"] = json!("changes");
    p.save(".dfd/domains/checkout/criteria.json", &catalog);
    assert_eq!(
        p.run(&["review-design", "001-paypal"], 2)["gate"],
        "blocked"
    );
    let mut design = p.read(".dfd/features/001-paypal/design.json");
    design["criteria"].as_array_mut().unwrap().push(json!({"id":"SLO-01","applicability":"excluded","justification":"Il flusso legacy non viene modificato"}));
    p.save(".dfd/features/001-paypal/design.json", &design);
    assert_eq!(
        p.run(&["review-design", "001-paypal"], 0)["gate"],
        "awaiting-human-review"
    );
}

#[test]
fn placeholder_design_blocks_approval() {
    let p = Project::new();
    p.ready();
    p.write(
        ".dfd/features/001-paypal/spec.md",
        "# [Nome Feature]\nTODO\n",
    );
    assert_eq!(
        p.run(&["review-design", "001-paypal"], 2)["gate"],
        "blocked"
    );
    p.run(
        &[
            "decide",
            "001-paypal",
            "--decision",
            "approved",
            "--reviewer",
            "TL",
            "--role",
            "tech-lead",
            "--note",
            "OK",
            "--human-confirmed",
        ],
        1,
    );
    assert_eq!(
        p.read(".dfd/features/001-paypal/state.json")["decisions"],
        json!([])
    );
}

#[test]
fn approval_requires_explicit_human_confirmation() {
    let p = Project::new();
    p.ready();
    // clap reports missing flag with exit 2.
    p.run(
        &[
            "decide",
            "001-paypal",
            "--decision",
            "approved",
            "--reviewer",
            "TL",
            "--role",
            "tech-lead",
            "--note",
            "OK",
        ],
        2,
    );
    assert_eq!(
        p.read(".dfd/features/001-paypal/state.json")["decisions"],
        json!([])
    );
}

#[test]
fn conditional_decision_records_owner_and_due_phase() {
    let p = Project::new();
    p.ready();
    p.run(
        &[
            "decide",
            "001-paypal",
            "--decision",
            "approved-with-conditions",
            "--reviewer",
            "TL",
            "--role",
            "architect",
            "--note",
            "Valido dopo test",
            "--human-confirmed",
        ],
        1,
    );
    let result = p.run(
        &[
            "decide",
            "001-paypal",
            "--decision",
            "approved-with-conditions",
            "--reviewer",
            "TL",
            "--role",
            "architect",
            "--note",
            "Valido dopo test",
            "--condition",
            "Test carico|QA|pre-release",
            "--human-confirmed",
        ],
        0,
    );
    assert_eq!(result["gate"], "approved-with-conditions");
    assert_eq!(result["decisions"][0]["conditions"][0]["owner"], "QA");
    assert_eq!(
        result["decisions"][0]["conditions"][0]["due_phase"],
        "pre-release"
    );
}

#[test]
fn document_changes_require_review_reconfirmation_and_keep_history() {
    for file in [
        ".dfd/features/001-paypal/spec.md",
        ".dfd/domains/checkout/dod.md",
        ".dfd/guardrails.md",
    ] {
        let p = Project::new();
        p.ready();
        p.approve();
        let mut body = fs::read_to_string(p.path(file)).unwrap();
        body.push_str("\nChiarimento del team.\n");
        p.write(file, &body);
        let result = p.run(&["status", "001-paypal"], 0);
        assert_eq!(result["gate"], "stale-review", "{file}");
        assert_eq!(result["decisions"].as_array().unwrap().len(), 1);
        p.approve();
        assert_eq!(
            p.run(&["status", "001-paypal"], 0)["decisions"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
    }
}

#[test]
fn semantic_notes_and_adoption_changes_invalidate_review() {
    let p = Project::new();
    p.ready();
    p.approve();
    p.write(
        ".dfd/features/001-paypal/review-notes.md",
        "Nuovo rilievo semantico",
    );
    assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "stale-review");
    p.approve();
    let mut adoption = p.read(".dfd/domains/checkout/adoption.json");
    adoption["next_steps"] = json!(["Nuovo passo"]);
    p.save(".dfd/domains/checkout/adoption.json", &adoption);
    assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "stale-review");
}

#[test]
fn changed_risk_requires_new_assessment_and_invalidates_review() {
    let p = Project::new();
    p.ready();
    p.approve();
    let mut risk = p.read(".dfd/features/001-paypal/risk.json");
    risk["dimensions"]["security"]["level"] = json!("high");
    p.save(".dfd/features/001-paypal/risk.json", &risk);
    assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "blocked");
    p.run(&["assess", "001-paypal"], 0);
    let result = p.run(&["status", "001-paypal"], 0);
    assert_eq!(result["route"], "full-complete");
    assert_eq!(result["gate"], "stale-review");
}

#[test]
fn date_with_offset_is_accepted_and_future_date_is_rejected() {
    let p = Project::new();
    p.ready();
    let mut design = p.read(".dfd/features/001-paypal/design.json");
    design["epic"]["approved_at"] = json!("2026-01-01T10:00:00+01:00");
    p.save(".dfd/features/001-paypal/design.json", &design);
    p.run(&["review-design", "001-paypal"], 0);
    design["epic"]["approved_at"] = json!("2999-01-01T00:00:00Z");
    p.save(".dfd/features/001-paypal/design.json", &design);
    p.run(&["review-design", "001-paypal"], 2);
}

#[test]
fn adoption_observation_requires_sources_and_domain_must_be_in_scope() {
    let p = Project::new();
    p.init();
    let mut adoption = p.read(".dfd/domains/checkout/adoption.json");
    adoption["areas"]["security"]["status"] = json!("observed");
    p.save(".dfd/domains/checkout/adoption.json", &adoption);
    p.run(
        &[
            "feature",
            "001-paypal",
            "--domain",
            "checkout",
            "--title",
            "PayPal",
            "--scope",
            "Nuovo pagamento",
        ],
        1,
    );
    adoption["areas"]["security"]["evidence"] = json!(["security-policy.md"]);
    adoption["pilot"] = json!(false);
    p.save(".dfd/domains/checkout/adoption.json", &adoption);
    p.run(
        &[
            "feature",
            "001-paypal",
            "--domain",
            "checkout",
            "--title",
            "PayPal",
            "--scope",
            "Nuovo pagamento",
        ],
        1,
    );
    assert!(!p.path(".dfd/features/001-paypal").exists());
}

#[test]
fn traversal_and_unowned_manifest_paths_are_rejected() {
    let p = Project::new();
    p.init();
    p.run(
        &[
            "feature",
            "../outside",
            "--domain",
            "checkout",
            "--title",
            "x",
            "--scope",
            "x",
        ],
        1,
    );
    p.write("owned.txt", "Preservare");
    let mut manifest = p.read(".dfd/install.json");
    manifest["files"]["owned.txt"] = json!("anything");
    p.save(".dfd/install.json", &manifest);
    p.run(&["install", "--harness", "claude-code"], 1);
    assert_eq!(
        fs::read_to_string(p.path("owned.txt")).unwrap(),
        "Preservare"
    );
}

#[test]
fn concurrent_mutation_lock_is_preserved() {
    let p = Project::new();
    p.write(".dfd/lock", "12345");
    p.run(&["init", "--harness", "codex", "--domain", "checkout"], 1);
    assert_eq!(fs::read_to_string(p.path(".dfd/lock")).unwrap(), "12345");
    assert!(!p.path(".dfd/config.json").exists());
}

#[cfg(unix)]
#[test]
fn symlinks_cannot_redirect_managed_writes_or_artifact_reads() {
    use std::os::unix::fs::symlink;
    let outside = tempfile::tempdir().unwrap();
    let p = Project::new();
    symlink(outside.path(), p.path(".dfd")).unwrap();
    p.run(&["init", "--harness", "codex", "--domain", "checkout"], 1);
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
    let p = Project::new();
    p.ready();
    fs::remove_file(p.path(".dfd/features/001-paypal/spec.md")).unwrap();
    fs::write(outside.path().join("spec.md"), "Esterno").unwrap();
    symlink(
        outside.path().join("spec.md"),
        p.path(".dfd/features/001-paypal/spec.md"),
    )
    .unwrap();
    p.run(&["status", "001-paypal"], 1);
    assert_eq!(
        fs::read_to_string(outside.path().join("spec.md")).unwrap(),
        "Esterno"
    );
}

#[test]
fn classification_is_proportional_and_new_service_uses_full() {
    use dfd_kit::{classify, model::*};
    let mut risk = Risk {
        schema_version: 1,
        dimensions: DIMENSIONS
            .iter()
            .map(|name| {
                (
                    name.to_string(),
                    RiskDimension {
                        level: Some(Level::Low),
                        rationale: "No impatto".into(),
                    },
                )
            })
            .collect(),
    };
    assert_eq!(
        classify(&risk, Kind::Feature).unwrap().route,
        Some(Route::Light)
    );
    risk.dimensions.get_mut("security").unwrap().level = Some(Level::Medium);
    assert_eq!(
        classify(&risk, Kind::Feature).unwrap().risk,
        Some(Level::Low)
    );
    risk.dimensions.get_mut("cost").unwrap().level = Some(Level::Medium);
    assert_eq!(
        classify(&risk, Kind::Feature).unwrap().route,
        Some(Route::FullProportional)
    );
    risk.dimensions.get_mut("reliability").unwrap().level = Some(Level::High);
    assert_eq!(
        classify(&risk, Kind::Feature).unwrap().route,
        Some(Route::FullComplete)
    );
    let mut risk = Risk {
        schema_version: 1,
        dimensions: DIMENSIONS
            .iter()
            .map(|name| {
                (
                    name.to_string(),
                    RiskDimension {
                        level: Some(Level::Low),
                        rationale: "No impatto".into(),
                    },
                )
            })
            .collect(),
    };
    let result = classify(&risk, Kind::NewService).unwrap();
    assert_eq!(result.risk, Some(Level::Low));
    assert_eq!(result.route, Some(Route::FullComplete));
    risk.dimensions.get_mut("cost").unwrap().rationale = " ".into();
    assert!(classify(&risk, Kind::Feature).unwrap().risk.is_none());
}

#[test]
fn setup_blocks_design_and_is_visible_without_features() {
    let p = Project::new();
    p.init();
    let setup = p.run(&["setup", "--domain", "checkout"], 2);
    assert_eq!(setup["gate"], "blocked");
    assert_eq!(p.run(&["status"], 0)["setups"][0], setup);
    p.feature("001-paypal");
    p.risk("001-paypal");
    assert_eq!(p.run(&["status", "001-paypal"], 0)["next"], "dfd-setup");
    p.run(&["specify", "001-paypal"], 1);
    assert!(!p.path(".dfd/features/001-paypal/design.json").exists());
    p.run(&["setup", "--domain", "unregistered"], 1);
}

#[test]
fn iterative_specification_preserves_work_and_exposes_questions() {
    let p = Project::new();
    p.ready();
    let mut design = p.read(".dfd/features/001-paypal/design.json");
    design["open_questions"] = json!(["Come gestire un timeout del provider?"]);
    p.save(".dfd/features/001-paypal/design.json", &design);
    let before = p.read(".dfd/features/001-paypal/state.json");
    let result = p.run(&["specify", "001-paypal"], 0);
    assert_eq!(result["open_questions"], design["open_questions"]);
    assert_eq!(p.read(".dfd/features/001-paypal/design.json"), design);
    assert_eq!(p.read(".dfd/features/001-paypal/state.json"), before);
    assert_eq!(
        p.run(&["review-design", "001-paypal"], 2)["gate"],
        "blocked"
    );
    design["open_questions"] = json!([]);
    design["test_plan"] = json!("Test di timeout del provider e retry");
    p.save(".dfd/features/001-paypal/design.json", &design);
    p.run(&["review-design", "001-paypal"], 0);
}

#[test]
fn partial_specification_is_preserved() {
    let p = Project::new();
    p.ready();
    let before = fs::read(p.path(".dfd/features/001-paypal/spec.md")).unwrap();
    fs::remove_file(p.path(".dfd/features/001-paypal/design.json")).unwrap();
    p.run(&["specify", "001-paypal"], 1);
    assert_eq!(
        fs::read(p.path(".dfd/features/001-paypal/spec.md")).unwrap(),
        before
    );
    assert!(!p.path(".dfd/features/001-paypal/design.json").exists());
}

#[test]
fn domain_changes_invalidate_its_reviews_and_local_templates_are_used() {
    for file in [
        "guardrails.md",
        "process.md",
        "templates/specifica_feature.md",
    ] {
        let p = Project::new();
        p.ready();
        p.run(&["init", "--harness", "codex", "--domain", "auth"], 0);
        p.approve();
        p.write(
            &format!(".dfd/domains/auth/{file}"),
            "# Altro dominio\nPolicy modificata\n",
        );
        assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "approved");
        p.write(
            &format!(".dfd/domains/checkout/{file}"),
            "# Policy del dominio\nNuova convenzione\n",
        );
        assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "stale-review");
        if file == "templates/specifica_feature.md" {
            p.feature("002-paypal");
            p.risk("002-paypal");
            p.run(&["specify", "002-paypal"], 0);
            assert_eq!(
                fs::read_to_string(p.path(".dfd/features/002-paypal/spec.md")).unwrap(),
                "# Policy del dominio\nNuova convenzione\n"
            );
        }
    }
}

#[test]
fn init_adds_missing_setup_files_and_preserves_local_changes() {
    let p = Project::new();
    p.ready();
    p.write(
        ".dfd/domains/checkout/templates/review_design.md",
        "# Checklist locale\n",
    );
    fs::remove_file(p.path(".dfd/domains/checkout/process.md")).unwrap();
    fs::remove_file(p.path(".dfd/domains/checkout/guardrails.md")).unwrap();
    p.init();
    assert!(p.path(".dfd/domains/checkout/process.md").is_file());
    assert!(p.path(".dfd/domains/checkout/guardrails.md").is_file());
    assert_eq!(
        fs::read_to_string(p.path(".dfd/domains/checkout/templates/review_design.md")).unwrap(),
        "# Checklist locale\n"
    );
    p.run(&["setup", "--domain", "checkout"], 2);
    p.write(
        ".dfd/domains/checkout/guardrails.md",
        "# Policy\nUsare provider esistente\n",
    );
    p.write(
        ".dfd/domains/checkout/process.md",
        "# Convenzioni\nTemplate adattati e checklist locali; owner Checkout\n",
    );
    p.run(&["setup", "--domain", "checkout"], 0);
    let mut catalog = p.read(".dfd/domains/checkout/criteria.json");
    catalog["criteria"].as_array_mut().unwrap().pop();
    p.save(".dfd/domains/checkout/criteria.json", &catalog);
    p.run(&["setup", "--domain", "checkout"], 2);
}

impl Project {
    fn file_evidence(&self, path: &str) -> Value {
        use sha2::{Digest, Sha256};
        let digest = Sha256::digest(fs::read(self.path(path)).unwrap());
        json!({"path":path,"sha256":digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>()})
    }
    fn developed(&self) {
        self.ready();
        self.approve();
        assert_eq!(self.run(&["status", "001-paypal"], 0)["next"], "dfd-plan");
        self.run(&["plan", "001-paypal"], 0);
        self.write("src/checkout.rs", "// Fixture di implementazione\n");
        self.write("tests/checkout.rs", "// Fixture di test\n");
        let mut evidence = self.read(".dfd/features/001-paypal/evidence.json");
        evidence["files"] = json!([
            self.file_evidence("src/checkout.rs"),
            self.file_evidence("tests/checkout.rs")
        ]);
        evidence["refactor_notes"] =
            json!("Contatore errori implementato; refactor eseguito e suite rieseguita");
        let mut checks = vec![];
        for (kind, exit, time) in [
            ("red", 1, "10:00:00"),
            ("green", 0, "10:01:00"),
            ("ci", 0, "10:02:00"),
        ] {
            let log = format!(".dfd/features/001-paypal/evidence/{kind}.log");
            self.write(&log, &format!("Fixture {kind}: risultato {exit}\n"));
            checks.push(json!({"kind":kind,"task":if kind == "ci" {None} else {Some("task-1")},"command":"cargo test","executed_at":format!("2026-01-01T{time}Z"),"exit_code":exit,"log":self.file_evidence(&log)}));
        }
        evidence["checks"] = json!(checks);
        self.save(".dfd/features/001-paypal/evidence.json", &evidence);
    }
}

#[test]
fn development_requires_current_human_design_approval() {
    let p = Project::new();
    p.ready();
    p.run(&["plan", "001-paypal"], 1);
    assert!(!p.path(".dfd/features/001-paypal/plan.json").exists());
    p.approve();
    p.write(
        ".dfd/features/001-paypal/spec.md",
        "# PayPal\nOBS-01: specifica aggiornata\n",
    );
    p.run(&["plan", "001-paypal"], 1);
    p.approve();
    p.run(&["plan", "001-paypal"], 0);
    assert_eq!(
        p.read(".dfd/features/001-paypal/plan.json")["tasks"][0]["criteria"],
        json!(["OBS-01"])
    );
    assert_eq!(
        p.run(&["verify", "001-paypal"], 2)["development"]["gate"],
        "blocked-evidence"
    );
}

#[test]
fn development_verifies_and_can_be_resumed_without_changing_work() {
    let p = Project::new();
    p.developed();
    let before = p.read(".dfd/features/001-paypal/evidence.json");
    assert_eq!(p.run(&["plan", "001-paypal"], 0)["resumed"], true);
    assert_eq!(p.read(".dfd/features/001-paypal/evidence.json"), before);
    let result = p.run(&["verify", "001-paypal"], 0);
    assert_eq!(result["phase"], "development-verified");
    assert_eq!(
        p.run(&["status", "001-paypal"], 0)["next"],
        "dfd-pre-release"
    );
    assert!(p
        .path(".dfd/features/001-paypal/development-review.md")
        .is_file());
}

#[test]
fn source_logs_and_plan_changes_invalidate_development() {
    for path in [
        "src/checkout.rs",
        ".dfd/features/001-paypal/evidence/green.log",
        ".dfd/features/001-paypal/plan.md",
    ] {
        let p = Project::new();
        p.developed();
        p.run(&["verify", "001-paypal"], 0);
        let mut body = fs::read_to_string(p.path(path)).unwrap();
        body.push_str("\nModifica successiva\n");
        p.write(path, &body);
        assert_eq!(
            p.run(&["status", "001-paypal"], 0)["development"]["gate"],
            "blocked-evidence"
        );
        assert_eq!(p.run(&["verify", "001-paypal"], 2)["phase"], "development");
    }
}

#[test]
fn development_rejects_incomplete_unknown_failed_or_unordered_evidence() {
    for mutation in [
        "no-red",
        "no-ci",
        "ci-failed",
        "unknown-task",
        "future",
        "unordered",
        "missing-file",
    ] {
        let p = Project::new();
        p.developed();
        let mut evidence = p.read(".dfd/features/001-paypal/evidence.json");
        match mutation {
            "no-red" => {
                evidence["checks"].as_array_mut().unwrap().remove(0);
            }
            "no-ci" => {
                evidence["checks"].as_array_mut().unwrap().pop();
            }
            "ci-failed" => evidence["checks"][2]["exit_code"] = json!(1),
            "unknown-task" => evidence["checks"][0]["task"] = json!("missing"),
            "future" => evidence["checks"][1]["executed_at"] = json!("2999-01-01T00:00:00Z"),
            "unordered" => evidence["checks"][0]["executed_at"] = json!("2026-01-01T10:03:00Z"),
            "missing-file" => {
                fs::remove_file(p.path("src/checkout.rs")).unwrap();
            }
            _ => unreachable!(),
        }
        p.save(".dfd/features/001-paypal/evidence.json", &evidence);
        p.run(&["verify", "001-paypal"], 2);
    }
}

#[test]
fn plan_refresh_preserves_tasks_and_requires_new_evidence() {
    let p = Project::new();
    p.developed();
    let before = p.read(".dfd/features/001-paypal/plan.json");
    p.write(
        ".dfd/domains/checkout/guardrails.md",
        "# Guardrail\nProvider esistente; retry idempotenti\n",
    );
    p.run(&["plan", "001-paypal", "--refresh"], 1);
    p.approve();
    p.run(&["plan", "001-paypal"], 2);
    p.run(&["plan", "001-paypal", "--refresh"], 0);
    let after = p.read(".dfd/features/001-paypal/plan.json");
    assert_eq!(before["tasks"], after["tasks"]);
    assert_ne!(before["design_fingerprint"], after["design_fingerprint"]);
    p.run(&["verify", "001-paypal"], 2);
}

#[test]
fn plan_questions_missing_coverage_and_partial_artifacts_block_development() {
    for mutation in ["questions", "coverage", "partial"] {
        let p = Project::new();
        p.developed();
        let mut plan = p.read(".dfd/features/001-paypal/plan.json");
        match mutation {
            "questions" => plan["open_questions"] = json!(["Quale contatore usare?"]),
            "coverage" => plan["tasks"] = json!([]),
            "partial" => {
                fs::remove_file(p.path(".dfd/features/001-paypal/plan.md")).unwrap();
            }
            _ => unreachable!(),
        }
        p.save(".dfd/features/001-paypal/plan.json", &plan);
        assert_eq!(
            p.run(&["verify", "001-paypal"], 2)["development"]["gate"],
            "blocked-plan"
        );
        if mutation == "partial" {
            p.run(&["plan", "001-paypal"], 1);
            assert_eq!(p.read(".dfd/features/001-paypal/plan.json"), plan);
        }
    }
}

#[test]
fn development_conditions_are_not_silently_resolved() {
    for (due, expected) in [("development", 2), ("pre-release", 0)] {
        let p = Project::new();
        p.developed();
        p.run(
            &[
                "decide",
                "001-paypal",
                "--decision",
                "approved-with-conditions",
                "--reviewer",
                "TL",
                "--role",
                "tech-lead",
                "--note",
                "Condizione esplicita",
                "--condition",
                &format!("Review del contatore|QA|{due}"),
                "--human-confirmed",
            ],
            0,
        );
        p.run(&["verify", "001-paypal"], expected);
    }
}

impl Project {
    fn pre_release_ready(&self) {
        self.developed();
        self.run(&["verify", "001-paypal"], 0);
        self.run(&["pre-release", "001-paypal"], 0);
        let mut rollout = self.read(".dfd/features/001-paypal/rollout.json");
        rollout["artifact"] = json!("registry/checkout:revision-123");
        rollout["revision"] = json!("revision-123");
        rollout["strategy_rationale"] =
            json!("Cambiamento limitato, rollout diretto entro i guardrail");
        rollout["steps"] = json!([{"id":"production","audience":"Tutti i tenant previsti","percentage":100,"duration_minutes":30,"promotion":"OBS-01 entro soglia per 30 minuti","stop":"Stop se aumentano gli errori"}]);
        rollout["criteria"][0]["signal"] = json!("Error count del nuovo flusso");
        rollout["criteria"][0]["threshold"] =
            json!("Nessun aumento rispetto alla baseline per 30 minuti");
        rollout["rollback_trigger"] = json!("Superamento della soglia OBS-01");
        rollout["rollback_procedure"] = json!("Ripristinare l'artefatto precedente");
        rollout["rollback_verification"] =
            json!("Walkthrough effettuato, errori tornano alla baseline");
        rollout["on_call"] = json!("checkout-oncall");
        rollout["communications"] = json!([{"audience":"Support e Ops","owner":"PM","informed_at":"2026-01-01T12:00:00Z","reference":"ticket-communication-123"}]);
        for name in ["alarms", "dashboards"] {
            rollout[name][0]["status"] = json!("excluded");
            rollout[name][0]["justification"] =
                json!("Percorso light: riusare monitoraggio esistente senza nuove risorse");
        }
        rollout["feature_flag"]["status"] = json!("excluded");
        rollout["feature_flag"]["justification"] =
            json!("Rollout diretto con ripristino dell'artefatto precedente");
        for test in rollout["non_functional_tests"].as_array_mut().unwrap() {
            test["applicability"] = json!("excluded");
            test["justification"] = json!(
                "Nessuna modifica a capacità, resilienza o controlli di sicurezza; policy light"
            );
        }
        self.save(".dfd/features/001-paypal/rollout.json", &rollout);
        self.write(".dfd/features/001-paypal/rollout.md", "# Rollout PayPal\nArtefatto revision-123.\nOBS-01: errore entro baseline; rollout diretto, rollback alla revisione precedente.\n");
    }
    fn approve_release(&self) -> Value {
        self.run(
            &[
                "decide",
                "001-paypal",
                "--stage",
                "release",
                "--decision",
                "approved",
                "--reviewer",
                "TL",
                "--role",
                "tech-lead",
                "--note",
                "Review di release effettuata",
                "--human-confirmed",
            ],
            0,
        )
    }
}

#[test]
fn pre_release_requires_current_development_and_preserves_partial_files() {
    let p = Project::new();
    p.ready();
    p.approve();
    p.run(&["pre-release", "001-paypal"], 1);
    assert!(!p.path(".dfd/features/001-paypal/rollout.json").exists());
    let p = Project::new();
    p.developed();
    p.write(
        ".dfd/features/001-paypal/rollout.md",
        "Contenuto da preservare\n",
    );
    p.run(&["pre-release", "001-paypal"], 1);
    assert_eq!(
        fs::read_to_string(p.path(".dfd/features/001-paypal/rollout.md")).unwrap(),
        "Contenuto da preservare\n"
    );
}

#[test]
fn release_review_requires_completion_and_human_decision_separate_from_design() {
    let p = Project::new();
    p.developed();
    p.run(&["pre-release", "001-paypal"], 0);
    assert_eq!(
        p.run(&["review-release", "001-paypal"], 2)["release"]["gate"],
        "blocked"
    );
    p.run(
        &[
            "decide",
            "001-paypal",
            "--stage",
            "release",
            "--decision",
            "approved",
            "--reviewer",
            "TL",
            "--role",
            "tech-lead",
            "--note",
            "OK",
            "--human-confirmed",
        ],
        1,
    );
    let p = Project::new();
    p.pre_release_ready();
    let before = p.read(".dfd/features/001-paypal/state.json")["decisions"].clone();
    let result = p.run(&["review-release", "001-paypal"], 0);
    assert_eq!(result["release"]["gate"], "awaiting-human-review");
    assert_eq!(result["release"]["checklist"], "review_release_light.md");
    p.run(
        &[
            "decide",
            "001-paypal",
            "--stage",
            "release",
            "--decision",
            "approved",
            "--reviewer",
            "TL",
            "--role",
            "tech-lead",
            "--note",
            "OK",
        ],
        2,
    );
    assert!(!p
        .path(".dfd/features/001-paypal/release-decisions.json")
        .exists());
    assert_eq!(p.approve_release()["release"]["gate"], "approved");
    assert_eq!(
        p.read(".dfd/features/001-paypal/state.json")["decisions"],
        before
    );
    assert_eq!(p.run(&["status", "001-paypal"], 0)["next"], "rollout");
    assert_eq!(p.run(&["status", "001-paypal"], 0)["gate"], "approved");
}

#[test]
fn rollout_resume_preserves_content_phase_and_review() {
    let p = Project::new();
    p.pre_release_ready();
    p.approve_release();
    let before = p.read(".dfd/features/001-paypal/state.json");
    let rollout = p.read(".dfd/features/001-paypal/rollout.json");
    assert_eq!(p.run(&["pre-release", "001-paypal"], 0)["resumed"], true);
    assert_eq!(p.read(".dfd/features/001-paypal/state.json"), before);
    assert_eq!(p.read(".dfd/features/001-paypal/rollout.json"), rollout);
    assert_eq!(
        p.run(&["review-release", "001-paypal"], 0)["release"]["gate"],
        "approved"
    );
    fs::remove_file(p.path(".dfd/features/001-paypal/rollout.md")).unwrap();
    p.run(&["pre-release", "001-paypal"], 1);
    assert_eq!(p.read(".dfd/features/001-paypal/rollout.json"), rollout);
}

#[test]
fn rollout_and_notes_changes_invalidate_release_review_without_losing_history() {
    for path in ["rollout.md", "release-notes.md"] {
        let p = Project::new();
        p.pre_release_ready();
        p.approve_release();
        p.write(
            &format!(".dfd/features/001-paypal/{path}"),
            "# Nuova nota\nOBS-01: verificare baseline con Ops.\n",
        );
        assert_eq!(
            p.run(&["status", "001-paypal"], 0)["release"]["gate"],
            "stale-review"
        );
        p.approve_release();
        assert_eq!(
            p.read(".dfd/features/001-paypal/release-decisions.json")["decisions"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
    }
}

#[test]
fn changed_source_blocks_release_and_updated_evidence_requires_rollout_refresh() {
    let p = Project::new();
    p.pre_release_ready();
    p.approve_release();
    p.write("src/checkout.rs", "// Nuova revisione verificata\n");
    assert_eq!(
        p.run(&["status", "001-paypal"], 0)["release"]["gate"],
        "blocked"
    );
    let mut evidence = p.read(".dfd/features/001-paypal/evidence.json");
    evidence["files"][0] = p.file_evidence("src/checkout.rs");
    p.save(".dfd/features/001-paypal/evidence.json", &evidence);
    p.run(&["verify", "001-paypal"], 0);
    assert_eq!(
        p.run(&["status", "001-paypal"], 0)["release"]["gate"],
        "blocked"
    );
    let before = p.read(".dfd/features/001-paypal/rollout.json");
    p.run(&["pre-release", "001-paypal", "--refresh"], 0);
    let after = p.read(".dfd/features/001-paypal/rollout.json");
    assert_eq!(before["steps"], after["steps"]);
    assert_ne!(
        before["development_fingerprint"],
        after["development_fingerprint"]
    );
    assert_eq!(
        p.run(&["review-release", "001-paypal"], 0)["release"]["gate"],
        "stale-review"
    );
}

#[test]
fn release_rejects_invalid_coverage_steps_tests_communications_and_questions() {
    for mutation in [
        "coverage",
        "unknown",
        "percentage",
        "canary",
        "questions",
        "communication",
        "test",
        "resource",
    ] {
        let p = Project::new();
        p.pre_release_ready();
        let mut rollout = p.read(".dfd/features/001-paypal/rollout.json");
        match mutation {
            "coverage" => rollout["criteria"] = json!([]),
            "unknown" => rollout["criteria"][0]["id"] = json!("SEC-99"),
            "percentage" => rollout["steps"][0]["percentage"] = json!(80),
            "canary" => rollout["strategy"] = json!("canary"),
            "questions" => rollout["open_questions"] = json!(["Quale soglia adottare?"]),
            "communication" => {
                rollout["communications"][0]["informed_at"] = json!("2999-01-01T00:00:00Z")
            }
            "test" => {
                rollout["non_functional_tests"][0]["applicability"] = json!("applicable");
            }
            "resource" => {
                rollout["alarms"][0]["status"] = json!("ready");
            }
            _ => unreachable!(),
        }
        p.save(".dfd/features/001-paypal/rollout.json", &rollout);
        assert_eq!(
            p.run(&["review-release", "001-paypal"], 2)["release"]["gate"],
            "blocked"
        );
    }
}

#[test]
fn planned_resources_need_conditional_decision_and_activation_evidence() {
    let p = Project::new();
    p.pre_release_ready();
    let mut rollout = p.read(".dfd/features/001-paypal/rollout.json");
    rollout["alarms"][0]["status"] = json!("planned");
    rollout["alarms"][0]["owner"] = json!("Ops");
    rollout["alarms"][0]["reference"] = json!("ticket-ops-123");
    rollout["alarms"][0]["activation_plan"] =
        json!("Ops attiva l'allarme prima del rollout e ne prova la notifica");
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    assert_eq!(
        p.run(&["review-release", "001-paypal"], 0)["release"]["pending_activation"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    p.run(
        &[
            "decide",
            "001-paypal",
            "--stage",
            "release",
            "--decision",
            "approved",
            "--reviewer",
            "TL",
            "--role",
            "tech-lead",
            "--note",
            "OK",
            "--human-confirmed",
        ],
        1,
    );
    let result = p.run(
        &[
            "decide",
            "001-paypal",
            "--stage",
            "release",
            "--decision",
            "approved-with-conditions",
            "--reviewer",
            "TL",
            "--role",
            "tech-lead",
            "--note",
            "Attivazione assegnata",
            "--condition",
            "Attivare allarme|Ops|rollout",
            "--human-confirmed",
        ],
        0,
    );
    assert_eq!(result["release"]["gate"], "approved-with-conditions");
    assert_eq!(
        p.run(&["status", "001-paypal"], 0)["next"],
        "resolve-release-conditions"
    );
    p.write(
        ".dfd/features/001-paypal/evidence/alarm.txt",
        "Fixture di verifica della notifica\n",
    );
    rollout["alarms"][0]["status"] = json!("ready");
    rollout["alarms"][0]["evidence"] =
        json!([p.file_evidence(".dfd/features/001-paypal/evidence/alarm.txt")]);
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    p.approve_release();
    p.write(
        ".dfd/features/001-paypal/evidence/alarm.txt",
        "Risorsa modificata\n",
    );
    assert_eq!(
        p.run(&["review-release", "001-paypal"], 2)["release"]["gate"],
        "blocked"
    );
}

#[test]
fn release_changes_requested_preserves_human_history_on_incomplete_preparation() {
    let p = Project::new();
    p.developed();
    p.run(&["pre-release", "001-paypal"], 0);
    let result = p.run(
        &[
            "decide",
            "001-paypal",
            "--stage",
            "release",
            "--decision",
            "changes-requested",
            "--reviewer",
            "TL",
            "--role",
            "architect",
            "--note",
            "Completare le soglie",
            "--human-confirmed",
        ],
        2,
    );
    assert_eq!(
        result["release"]["decisions"][0]["decision"],
        "changes-requested"
    );
}

#[test]
fn full_release_uses_full_checklist_and_requires_operational_resources() {
    let p = Project::new();
    p.pre_release_ready();
    let mut risk = p.read(".dfd/features/001-paypal/risk.json");
    risk["dimensions"]["security"]["level"] = json!("high");
    p.save(".dfd/features/001-paypal/risk.json", &risk);
    p.run(&["assess", "001-paypal"], 0);
    p.approve();
    let plan = p.run(&["plan", "001-paypal", "--refresh"], 0);
    let mut evidence = p.read(".dfd/features/001-paypal/evidence.json");
    evidence["design_fingerprint"] = plan["development"]["design_fingerprint"].clone();
    evidence["plan_hash"] = plan["development"]["plan_hash"].clone();
    p.save(".dfd/features/001-paypal/evidence.json", &evidence);
    p.run(&["verify", "001-paypal"], 0);
    p.run(&["pre-release", "001-paypal", "--refresh"], 0);
    let result = p.run(&["review-release", "001-paypal"], 2);
    assert_eq!(result["release"]["checklist"], "review_release.md");
    let mut rollout = p.read(".dfd/features/001-paypal/rollout.json");
    for name in ["alarms", "dashboards"] {
        rollout[name][0]["status"] = json!("ready");
        rollout[name][0]["owner"] = json!("Ops");
        rollout[name][0]["reference"] = json!("monitoring/checkout");
        rollout[name][0]["evidence"] =
            json!([p.file_evidence(".dfd/features/001-paypal/evidence/green.log")]);
    }
    rollout["strategy"] = json!("combined");
    let mut initial = rollout["steps"][0].clone();
    initial["id"] = json!("canary-internal");
    initial["percentage"] = json!(5);
    initial["audience"] = json!("Tenant interni");
    rollout["steps"].as_array_mut().unwrap().insert(0, initial);
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    p.run(&["review-release", "001-paypal"], 0);
    p.approve_release();
}

#[test]
fn nonfunctional_execution_requires_success_and_unchanged_log() {
    let p = Project::new();
    p.pre_release_ready();
    p.write(
        ".dfd/features/001-paypal/evidence/load.log",
        "Fixture di test di carico verde\n",
    );
    let mut rollout = p.read(".dfd/features/001-paypal/rollout.json");
    rollout["non_functional_tests"][0]["applicability"] = json!("applicable");
    rollout["non_functional_tests"][0]["execution"] = json!({"command":"load-test checkout","executed_at":"2026-01-01T11:00:00Z","exit_code":0,"log":p.file_evidence(".dfd/features/001-paypal/evidence/load.log")});
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    p.approve_release();
    p.write(
        ".dfd/features/001-paypal/evidence/load.log",
        "Test modificato\n",
    );
    p.run(&["review-release", "001-paypal"], 2);
    rollout["non_functional_tests"][0]["execution"]["log"] =
        p.file_evidence(".dfd/features/001-paypal/evidence/load.log");
    rollout["non_functional_tests"][0]["execution"]["exit_code"] = json!(1);
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    p.run(&["review-release", "001-paypal"], 2);
}

#[test]
fn design_conditions_due_before_release_must_be_reconfirmed() {
    let p = Project::new();
    p.pre_release_ready();
    p.run(
        &[
            "decide",
            "001-paypal",
            "--decision",
            "approved-with-conditions",
            "--reviewer",
            "TL",
            "--role",
            "tech-lead",
            "--note",
            "Walkthrough richiesto",
            "--condition",
            "Verificare rollback|Ops|pre-release",
            "--human-confirmed",
        ],
        0,
    );
    p.run(&["pre-release", "001-paypal", "--refresh"], 0);
    p.run(&["review-release", "001-paypal"], 2);
    p.approve();
    p.run(&["pre-release", "001-paypal", "--refresh"], 0);
    p.approve_release();
    let result = p.run(
        &[
            "decide",
            "001-paypal",
            "--stage",
            "release",
            "--decision",
            "approved-with-conditions",
            "--reviewer",
            "TL",
            "--role",
            "tech-lead",
            "--note",
            "Monitorare il rollout",
            "--condition",
            "Monitoraggio attivo|Ops|rollout",
            "--human-confirmed",
        ],
        0,
    );
    assert_eq!(result["release"]["next"], "rollout-with-conditions");
}

#[test]
fn release_checks_rollout_progression_and_observation_duration_independently() {
    for mutation in ["decreasing-exposure", "zero-duration"] {
        let p = Project::new();
        p.pre_release_ready();
        let mut rollout = p.read(".dfd/features/001-paypal/rollout.json");
        if mutation == "decreasing-exposure" {
            rollout["strategy"] = json!("segmented");
            let prototype = rollout["steps"][0].clone();
            rollout["steps"] = json!([]);
            for (index, percentage) in [10, 5, 100].iter().enumerate() {
                let mut step = prototype.clone();
                step["id"] = json!(format!("step-{index}"));
                step["percentage"] = json!(percentage);
                rollout["steps"].as_array_mut().unwrap().push(step);
            }
        } else {
            rollout["steps"][0]["duration_minutes"] = json!(0);
        }
        p.save(".dfd/features/001-paypal/rollout.json", &rollout);
        let result = p.run(&["review-release", "001-paypal"], 2);
        assert_eq!(result["release"]["gate"], "blocked");
    }
}

#[test]
fn ready_resource_requires_proof_even_with_complete_owner_and_reference() {
    let p = Project::new();
    p.pre_release_ready();
    let mut rollout = p.read(".dfd/features/001-paypal/rollout.json");
    rollout["alarms"][0]["status"] = json!("ready");
    rollout["alarms"][0]["owner"] = json!("Ops");
    rollout["alarms"][0]["reference"] = json!("monitoring/checkout");
    rollout["alarms"][0]["justification"] = json!("");
    // All other resource fields are valid; the only missing requirement is proof.
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    assert_eq!(
        p.run(&["review-release", "001-paypal"], 2)["release"]["gate"],
        "blocked"
    );
    p.write(
        ".dfd/features/001-paypal/evidence/alarm-ready.txt",
        "Fixture della risorsa pronta\n",
    );
    rollout["alarms"][0]["evidence"] =
        json!([p.file_evidence(".dfd/features/001-paypal/evidence/alarm-ready.txt")]);
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    assert_eq!(
        p.run(&["review-release", "001-paypal"], 0)["release"]["gate"],
        "awaiting-human-review"
    );
}

#[test]
fn release_requires_a_real_communication_record() {
    let p = Project::new();
    p.pre_release_ready();
    let mut rollout = p.read(".dfd/features/001-paypal/rollout.json");
    let communication = rollout["communications"].clone();
    rollout["communications"] = json!([]);
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    p.run(&["review-release", "001-paypal"], 2);
    rollout["communications"] = communication;
    p.save(".dfd/features/001-paypal/rollout.json", &rollout);
    p.run(&["review-release", "001-paypal"], 0);
}

#[test]
fn unsupported_json_schema_preserves_assessed_state() {
    let p = Project::new();
    p.ready();
    let state = p.read(".dfd/features/001-paypal/state.json");
    let mut risk = p.read(".dfd/features/001-paypal/risk.json");
    risk["schema_version"] = json!(2);
    p.save(".dfd/features/001-paypal/risk.json", &risk);
    p.run(&["assess", "001-paypal"], 1);
    assert_eq!(p.read(".dfd/features/001-paypal/state.json"), state);
}
