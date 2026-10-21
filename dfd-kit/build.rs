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
    for entry in fs::read_dir(root.join("examples")).unwrap() {
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
    for file in &files {
        println!("cargo:rerun-if-changed={}", file.display());
        let relative = file.to_str().unwrap().replace('\\', "/");
        let absolute = root.join(file);
        let body = without_navigation(&fs::read_to_string(&absolute).unwrap());
        generated.push_str(&format!("({relative:?}, {body:?}),\n"));
    }
    generated.push_str("];\n");
    generated.push_str("pub const FILES_EN: &[(&str, &str)] = &[\n");
    for file in &files {
        let relative = file.to_str().unwrap().replace('\\', "/");
        let absolute = root.join("en").join(&relative);
        println!("cargo:rerun-if-changed={}", absolute.display());
        let body = without_navigation(&fs::read_to_string(&absolute).unwrap());
        generated.push_str(&format!("({relative:?}, {body:?}),\n"));
    }
    generated.push_str("];\n");
    println!("cargo:rerun-if-changed=en");
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=skills");
    println!("cargo:rerun-if-changed=examples");
    fs::write(
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("bundle.rs"),
        generated,
    )
    .unwrap();
}

// Source-only language navigation must not point outside the installed locale tree.
fn without_navigation(body: &str) -> String {
    let mut skip_blank = false;
    let mut result = String::new();
    for line in body.lines() {
        if line.starts_with("<!-- dfd-language -->") {
            skip_blank = true;
            continue;
        }
        if skip_blank && line.is_empty() {
            skip_blank = false;
            continue;
        }
        skip_blank = false;
        result.push_str(line);
        result.push('\n');
    }
    result
}
