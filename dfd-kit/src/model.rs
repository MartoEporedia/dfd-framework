use chrono::{DateTime, Utc};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const DIMENSIONS: [&str; 5] = [
    "security",
    "reliability",
    "cost",
    "business",
    "architecture",
];
pub const AREAS: [&str; 6] = [
    "process",
    "observability",
    "security",
    "reliability",
    "cost",
    "culture",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Brownfield,
    Greenfield,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    Feature,
    Refactor,
    Incident,
    NewService,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Level {
    Low,
    Medium,
    High,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Route {
    Light,
    FullProportional,
    FullComplete,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Verdict {
    Approved,
    ApprovedWithConditions,
    ChangesRequested,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    TechLead,
    Architect,
    Reviewer,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AdoptionStatus {
    Unknown,
    Gap,
    Observed,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CriterionScope {
    Changes,
    Baseline,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Applicability {
    Applicable,
    Excluded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LifecycleMode {
    DevelopmentOnly,
    ReleasePreparation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lifecycle {
    pub schema_version: u32,
    pub mode: LifecycleMode,
    pub ci_required: bool,
    pub rationale: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema_version: u32,
    pub mode: Mode,
    pub domains: Vec<String>,
    pub created_at: DateTime<Utc>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema_version: u32,
    pub version: String,
    pub harness: String,
    pub files: BTreeMap<String, String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Area {
    pub status: AdoptionStatus,
    pub evidence: Vec<String>,
    pub notes: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Adoption {
    pub schema_version: u32,
    pub pilot: bool,
    pub current_level: u8,
    pub target_level: u8,
    pub areas: BTreeMap<String, Area>,
    pub baseline_gaps: Vec<String>,
    pub next_steps: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiskDimension {
    pub level: Option<Level>,
    pub rationale: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Risk {
    pub schema_version: u32,
    pub dimensions: BTreeMap<String, RiskDimension>,
}
#[derive(Debug, Serialize)]
pub struct Classification {
    pub risk: Option<Level>,
    pub route: Option<Route>,
    pub missing: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub schema_version: u32,
    pub id: String,
    pub title: String,
    pub domain: String,
    pub kind: Kind,
    pub scope: String,
    pub phase: String,
    pub created_at: DateTime<Utc>,
    pub risk: Option<Level>,
    pub route: Option<Route>,
    pub assessment_hash: Option<String>,
    pub decisions: Vec<Decision>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub selective_review: bool,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Condition {
    pub text: String,
    pub owner: String,
    pub due_phase: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decision {
    pub decision: Verdict,
    pub reviewer: String,
    pub role: Role,
    pub note: String,
    pub conditions: Vec<Condition>,
    pub recorded_at: DateTime<Utc>,
    pub fingerprint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileEvidence>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub criteria: Vec<Criterion>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Criterion {
    pub id: String,
    pub statement: String,
    pub scope: CriterionScope,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Epic {
    pub reference: String,
    pub approved_by: String,
    pub approved_at: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CriterionReference {
    pub id: String,
    pub applicability: Applicability,
    #[serde(default)]
    pub implementation: String,
    #[serde(default)]
    pub verification: String,
    #[serde(default)]
    pub justification: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Design {
    pub schema_version: u32,
    pub owner: String,
    pub objective: String,
    pub business_metrics: Vec<String>,
    pub epic: Epic,
    pub criteria: Vec<CriterionReference>,
    pub test_plan: String,
    pub guardrails: String,
    pub open_questions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub schema_version: u32,
    pub design_fingerprint: String,
    pub tasks: Vec<Task>,
    pub open_questions: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub id: String,
    pub owner: String,
    pub description: String,
    pub criteria: Vec<String>,
    pub implementation: String,
    pub verification: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileEvidence {
    pub path: String,
    pub sha256: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CheckKind {
    Red,
    Green,
    Suite,
    Ci,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestEvidence {
    pub task: Option<String>,
    pub kind: CheckKind,
    pub command: String,
    pub executed_at: String,
    pub exit_code: i32,
    pub log: FileEvidence,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub schema_version: u32,
    pub design_fingerprint: String,
    pub plan_hash: String,
    pub files: Vec<FileEvidence>,
    pub checks: Vec<TestEvidence>,
    pub refactor_notes: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewStage {
    Design,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RolloutStrategy {
    Direct,
    Canary,
    Segmented,
    Combined,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RolloutStep {
    pub id: String,
    pub audience: String,
    pub percentage: u8,
    pub duration_minutes: u32,
    pub promotion: String,
    pub stop: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseCriterion {
    pub id: String,
    pub signal: String,
    pub threshold: String,
    pub verification: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Preparation {
    Planned,
    Ready,
    Excluded,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedResource {
    pub name: String,
    pub owner: String,
    pub status: Preparation,
    pub reference: String,
    pub activation_plan: String,
    pub justification: String,
    pub evidence: Vec<FileEvidence>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionEvidence {
    pub command: String,
    pub executed_at: String,
    pub exit_code: i32,
    pub log: FileEvidence,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NonFunctionalTest {
    pub area: String,
    pub applicability: Applicability,
    pub justification: String,
    pub execution: Option<ExecutionEvidence>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Communication {
    pub audience: String,
    pub owner: String,
    pub informed_at: String,
    pub reference: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rollout {
    pub schema_version: u32,
    pub development_fingerprint: String,
    pub owner: String,
    pub artifact: String,
    pub revision: String,
    pub environment: String,
    pub strategy: RolloutStrategy,
    pub strategy_rationale: String,
    pub steps: Vec<RolloutStep>,
    pub criteria: Vec<ReleaseCriterion>,
    pub alarms: Vec<PreparedResource>,
    pub dashboards: Vec<PreparedResource>,
    pub feature_flag: PreparedResource,
    pub rollback_trigger: String,
    pub rollback_procedure: String,
    pub rollback_verification: String,
    pub on_call: String,
    pub communications: Vec<Communication>,
    pub non_functional_tests: Vec<NonFunctionalTest>,
    pub open_questions: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseHistory {
    pub schema_version: u32,
    pub decisions: Vec<Decision>,
}

fn is_false(value: &bool) -> bool {
    !value
}
