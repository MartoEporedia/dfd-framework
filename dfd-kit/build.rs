use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut files: Vec<PathBuf> = [
        "fondamenta.md",
        "processo-e2e.md",
        "rischio.md",
        "raci.md",
        "adozione.md",
    ]
    .iter()
    .map(PathBuf::from)
    .collect();
    for entry in fs::read_dir(root.join("templates")).unwrap() {
        let entry = entry.unwrap().path();
        if entry.extension().is_some_and(|ext| ext == "md") {
            files.push(entry.strip_prefix(&root).unwrap().to_owned());
        }
    }
    for entry in fs::read_dir(root.join("skills")).unwrap() {
        let skill = entry.unwrap().path().join("SKILL.md");
        if skill.is_file() {
            files.push(skill.strip_prefix(&root).unwrap().to_owned());
        }
    }
    files.sort();
    let mut generated = String::from("pub const FILES: &[(&str, &str)] = &[\n");
    for file in files {
        println!("cargo:rerun-if-changed={}", file.display());
        let relative = file.to_str().unwrap().replace('\\', "/");
        let absolute = root.join(file);
        generated.push_str(&format!("({relative:?}, include_str!({absolute:?})),\n"));
    }
    generated.push_str("];\n");
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=skills");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("bundle.rs"),
        generated,
    )
    .unwrap();
}
