//! Static, offline localization. Machine contracts and user values are never translated.
use crate::{store::Store, Error, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{cell::Cell, collections::BTreeMap, sync::OnceLock};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    It,
    En,
}

thread_local! { static CURRENT: Cell<Language> = const { Cell::new(Language::It) }; }

pub struct Scope(Language);
impl Scope {
    pub fn enter(language: Language) -> Self {
        Self(CURRENT.with(|current| current.replace(language)))
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        CURRENT.with(|current| current.set(self.0));
    }
}
pub fn current() -> Language {
    CURRENT.with(Cell::get)
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Preference {
    schema_version: u32,
    language: Language,
}

pub fn project_language(store: &Store) -> Result<Language> {
    if !store.path(".dfd/language.json")?.exists() {
        return Ok(Language::It);
    }
    Ok(store.json::<Preference>(".dfd/language.json")?.language)
}
pub fn save(store: &Store, language: Language) -> Result<()> {
    store.save(
        ".dfd/language.json",
        &Preference {
            schema_version: 1,
            language,
        },
        false,
    )
}

fn catalog() -> &'static BTreeMap<String, String> {
    static CATALOG: OnceLock<BTreeMap<String, String>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../i18n/en.json")).expect("valid embedded translations")
    })
}

/// Translate static CLI-owned text only, before inserting user-provided values.
pub fn text(source: &str) -> String {
    if current() == Language::En {
        if let Some(value) = catalog().get(source) {
            return value.clone();
        }
    }
    source.to_owned()
}

/// Both format strings are literals, so Rust checks placeholders at compile time.
#[macro_export]
macro_rules! localized_format {
    ($it:literal, $en:literal $(, $($args:tt)*)?) => {
        if $crate::i18n::current() == $crate::i18n::Language::En {
            format!($en $(, $($args)*)?)
        } else {
            format!($it $(, $($args)*)?)
        }
    };
}

pub fn files() -> &'static [(&'static str, &'static str)] {
    match current() {
        Language::It => crate::bundle::FILES,
        Language::En => crate::bundle::FILES_EN,
    }
}

/// Parse selection before clap renders help; strict parsing still belongs to clap.
pub fn from_args(args: &[std::ffi::OsString]) -> Result<Language> {
    let mut language = None;
    let mut project = std::path::PathBuf::from(".");
    for (index, arg) in args.iter().enumerate().skip(1) {
        let value = arg.to_string_lossy();
        if value == "--" {
            break;
        }
        if value == "--lang" {
            language = args
                .get(index + 1)
                .map(|v| v.to_string_lossy().into_owned());
        }
        if let Some(value) = value.strip_prefix("--lang=") {
            language = Some(value.to_owned());
        }
        if value == "--project" {
            if let Some(value) = args.get(index + 1) {
                project = value.into();
            }
        }
        if let Some(value) = value.strip_prefix("--project=") {
            project = value.into();
        }
    }
    if let Some(language) = language {
        return match language.as_str() {
            "en" => Ok(Language::En),
            "it" => Ok(Language::It),
            _ => Err(Error::message("Lingua non supportata; usare it o en.")),
        };
    }
    // Help and doctor remain usable outside a project.
    if !project.exists() {
        return Ok(Language::It);
    }
    project_language(&Store::new(&project)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting_preserves_user_values_and_restores_language_scope() {
        assert_eq!(current(), Language::It);
        {
            let _scope = Scope::enter(Language::En);
            let user_text = "Soddisfare OBS-01\n{value}\n## Ambito e test";
            assert_eq!(
                crate::localized_format!(
                    "Evidenza assente: {}.",
                    "Missing evidence: {}.",
                    user_text
                ),
                format!("Missing evidence: {user_text}.")
            );
            assert_eq!(
                text("Titolo e ambito sono obbligatori."),
                "Title and scope are required."
            );
            assert_eq!(
                text("Testo utente non presente nel catalogo"),
                "Testo utente non presente nel catalogo"
            );
            {
                let _italian = Scope::enter(Language::It);
                assert_eq!(
                    text("Titolo e ambito sono obbligatori."),
                    "Titolo e ambito sono obbligatori."
                );
            }
            assert_eq!(current(), Language::En);
        }
        assert_eq!(current(), Language::It);
    }
}
