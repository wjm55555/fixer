use std::env;
use std::path::{Path, PathBuf};

fn collect_c_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_c_files(&path, files);
            } else if path.extension().map(|e| e == "c").unwrap_or(false) {
                files.push(path);
            }
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let c_root = manifest_dir.join("../../inputs/translate_gci/src/src");

    println!("cargo:rerun-if-changed={}", c_root.display());

    let mut c_files = Vec::new();
    collect_c_files(&c_root, &mut c_files);

    let mut build = cc::Build::new();
    build.include(&c_root);
    build.include(c_root.join("bytecode-generator"));
    build.include(c_root.join("virtual-machine"));
    build.include(c_root.join("lexer"));
    build.include(c_root.join("parser"));
    build.include(c_root.join("garbage-collector"));
    build.include(c_root.join("utils"));
    build.include(c_root.join("data-types"));
    for file in &c_files {
        println!("cargo:rerun-if-changed={}", file.display());
        build.file(file);
    }

    build.compile("libgci_c");
}
