// A build script: cargo compiles and runs it before the crate, and the crate reads what it wrote to OUT_DIR.
use std::fs;
use std::path::Path;

/// Copy src/shaders/*.wgsl into OUT_DIR with `#include "file"` lines expanded, without comments,
/// indentation or blank lines.
fn main() {
    println!("cargo:rerun-if-changed=src/shaders"); // cargo reads this line: run the script again only when src/shaders changes
    let dir = Path::new("src/shaders");
    // `unwrap` takes the value or stops with a panic, which is fine in a build script
    let out = Path::new(&std::env::var("OUT_DIR").unwrap()).join("shaders");
    fs::create_dir_all(&out).unwrap();

    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        // `|e| e == "wgsl"` is a closure: a small unnamed function passed as a value
        if path.extension().is_some_and(|e| e == "wgsl") {
            let source = expand(dir, &path, &mut Vec::new());
            fs::write(out.join(path.file_name().unwrap()), minify(&source)).unwrap();
        }
    }
}

/// The file with each `#include "file"` line replaced by that file, every file once per shader.
fn expand(dir: &Path, path: &Path, seen: &mut Vec<String>) -> String {
    let name = path.file_name().unwrap().to_string_lossy().into_owned();
    if seen.contains(&name) {
        return String::new();
    }
    seen.push(name);
    let source = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut text = String::with_capacity(source.len());

    for line in source.lines() {
        let include = line
            .trim()
            .strip_prefix("#include \"")
            .and_then(|rest| rest.strip_suffix('"'));
        match include {
            Some(file) => text += &expand(dir, &dir.join(file), seen),
            None => {
                text += line;
                text.push('\n');
            }
        }
    }
    text
}

/// WGSL with `//` comments cut and each line's whitespace collapsed; line breaks kept.
fn minify(source: &str) -> String {
    assert!(!source.contains("/*"), "block comments are not stripped");
    let mut text = String::with_capacity(source.len());

    for line in source.lines() {
        let code = line.split("//").next().unwrap_or("");
        let words: Vec<&str> = code.split_whitespace().collect();
        if !words.is_empty() {
            text += &words.join(" ");
            text.push('\n');
        }
    }
    text
}
