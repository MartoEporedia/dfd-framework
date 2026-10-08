use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "dfd",
    version,
    about = "DFD Kit: adozione, rischio e review negli harness esistenti"
)]
pub struct Cli {
    /// Repository adottante (default: directory corrente).
    #[arg(long, global = true, default_value = ".")]
    pub project: PathBuf,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Harness {
    ClaudeCode,
    Opencode,
    Copilot,
    Codex,
}

impl Harness {
    pub fn name(self) -> &'static str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::Opencode => "opencode",
            Self::Copilot => "copilot",
            Self::Codex => "codex",
        }
    }
    pub fn directory(self) -> &'static str {
        match self {
            Self::ClaudeCode => ".claude/skills",
            Self::Opencode => ".opencode/skills",
            Self::Copilot => ".github/skills",
            Self::Codex => ".agents/skills",
        }
    }
    pub const ALL: [Self; 4] = [Self::ClaudeCode, Self::Opencode, Self::Copilot, Self::Codex];
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Installa le skill o cambia l'adapter attivo, preservando lo stato.
    Install {
        #[arg(long, value_enum)]
        harness: Harness,
    },
    /// Inizializza un dominio pilota, senza migrare il legacy.
    Init {
        #[arg(long, value_enum)]
        harness: Harness,
        #[arg(long)]
        domain: String,
        #[arg(long, value_enum, default_value = "brownfield")]
        mode: crate::model::Mode,
    },
    /// Verifica la Fase 0 del dominio, prerequisito del design.
    Setup {
        #[arg(long)]
        domain: String,
    },
    /// Crea una feature con titolo e ambito espliciti.
    Feature {
        id: String,
        #[arg(long)]
        domain: String,
        #[arg(long)]
        title: String,
        #[arg(long)]
        scope: String,
        #[arg(long, value_enum, default_value = "feature")]
        kind: crate::model::Kind,
    },
    /// Classifica risk.json; i dati mancanti restano da chiarire.
    Assess { id: String },
    /// Crea o riprende la specifica iterativa, senza sovrascrivere.
    Specify { id: String },
    /// Verifica il design e produce il report strutturale.
    ReviewDesign { id: String },
    /// Crea o riprende il piano TDD dopo l'approvazione del design.
    Plan {
        id: String,
        /// Riallinea il piano al design approvato, preservando i task.
        #[arg(long)]
        refresh: bool,
    },
    /// Verifica le evidenze di sviluppo e la prontezza per la pre-release.
    Verify { id: String },
    /// Crea o riprende rollout e preparazione operativa dopo lo sviluppo.
    PreRelease {
        id: String,
        /// Riallinea i riferimenti allo sviluppo verificato, preservando il piano.
        #[arg(long)]
        refresh: bool,
    },
    /// Verifica la preparazione del rilascio e il gate di review.
    ReviewRelease { id: String },
    /// Registra una decisione umana dichiarata; non autentica il reviewer.
    Decide {
        id: String,
        #[arg(long, value_enum, default_value = "design")]
        stage: crate::model::ReviewStage,
        #[arg(long, value_enum)]
        decision: crate::model::Verdict,
        #[arg(long)]
        reviewer: String,
        #[arg(long, value_enum)]
        role: crate::model::Role,
        #[arg(long)]
        note: String,
        /// Condizione: testo|responsabile|fase. Ripetibile.
        #[arg(long)]
        condition: Vec<String>,
        /// Conferma che la decisione è stata espressa dal responsabile umano.
        #[arg(long, required = true)]
        human_confirmed: bool,
    },
    /// Mostra stato, gate e lacune; senza ID elenca le feature.
    Status { id: Option<String> },
    /// Controlla le skill incorporate e le loro risorse.
    Doctor,
}
