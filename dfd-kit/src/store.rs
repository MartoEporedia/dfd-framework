use crate::{Error, Result};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
};

pub struct Store {
    pub root: PathBuf,
}

impl Store {
    pub fn new(root: &Path) -> Result<Self> {
        let root = root.canonicalize()?;
        if !root.is_dir() {
            return Err(Error::message(
                "La directory del progetto deve già esistere.",
            ));
        }
        Ok(Self { root })
    }

    pub fn path(&self, relative: &str) -> Result<PathBuf> {
        let rel = Path::new(relative);
        if relative.is_empty()
            || relative.contains('\\')
            || rel.components().any(|c| !matches!(c, Component::Normal(_)))
        {
            return Err(Error::message(crate::localized_format!(
                "Percorso non consentito: {relative}",
                "Path not allowed: {relative}",
                relative = relative
            )));
        }
        let mut current = self.root.clone();
        for part in rel.components() {
            current.push(part.as_os_str());
            match fs::symlink_metadata(&current) {
                Ok(meta) if meta.file_type().is_symlink() => {
                    return Err(Error::message(crate::localized_format!(
                        "Symlink non consentito: {}",
                        "Symlink not allowed: {}",
                        current.display()
                    )))
                }
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
        }
        Ok(current)
    }

    pub fn read(&self, relative: &str) -> Result<String> {
        let path = self.path(relative)?;
        fs::read_to_string(path).map_err(|error| {
            Error::message(crate::localized_format!(
                "Lettura {relative}: {error}",
                "Reading {relative}: {error}",
                relative = relative,
                error = error
            ))
        })
    }
    pub fn json<T: DeserializeOwned>(&self, relative: &str) -> Result<T> {
        let value: serde_json::Value = serde_json::from_str(&self.read(relative)?)?;
        if value["schema_version"] != 1 {
            return Err(Error::message(crate::localized_format!(
                "Schema non supportato: {relative}",
                "Unsupported schema: {relative}",
                relative = relative
            )));
        }
        serde_json::from_value(value).map_err(|error| {
            Error::message(crate::localized_format!(
                "JSON non valido in {relative}: {error}",
                "Invalid JSON in {relative}: {error}",
                relative = relative,
                error = error
            ))
        })
    }
    pub fn write(&self, relative: &str, content: &[u8], exclusive: bool) -> Result<()> {
        let path = self.path(relative)?;
        let parent = path
            .parent()
            .ok_or_else(|| Error::message("Percorso senza parent"))?;
        fs::create_dir_all(parent)?;
        // Recheck after directory creation, before persistence.
        self.path(relative)?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
        temporary.write_all(content)?;
        temporary.as_file().sync_all()?;
        if exclusive {
            temporary.persist_noclobber(&path).map_err(|error| {
                Error::message(crate::localized_format!(
                    "File esistente o scrittura fallita, preservato {relative}: {error}",
                    "Existing file or failed write, preserved {relative}: {error}",
                    relative = relative,
                    error = error
                ))
            })?;
        } else {
            temporary.persist(&path).map_err(|error| {
                Error::message(crate::localized_format!(
                    "Scrittura {relative}: {error}",
                    "Writing {relative}: {error}",
                    relative = relative,
                    error = error
                ))
            })?;
        }
        Ok(())
    }
    pub fn save<T: Serialize>(&self, relative: &str, value: &T, exclusive: bool) -> Result<()> {
        let mut bytes = serde_json::to_vec_pretty(value)?;
        bytes.push(b'\n');
        self.write(relative, &bytes, exclusive)
    }
    pub fn hash(&self, relative: &str) -> Result<String> {
        Ok(hash(&fs::read(self.path(relative)?)?))
    }
    pub fn lock(&self) -> Result<Lock> {
        let path = self.path(".dfd/lock")?;
        fs::create_dir_all(path.parent().unwrap())?;
        let mut file = OpenOptions::new().write(true).create_new(true).open(&path)
            .map_err(|error| Error::message(crate::localized_format!("Impossibile acquisire .dfd/lock: {error}. Verificare che non ci siano altri comandi DFD attivi.", "Cannot acquire .dfd/lock: {error}. Check that no other DFD commands are running.", error = error)))?;
        writeln!(file, "{}", std::process::id())?;
        Ok(Lock(path))
    }
}

pub fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
pub struct Lock(PathBuf);
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
