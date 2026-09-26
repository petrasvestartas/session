use std::fs;
use std::path::Path;

/// Copy src/shaders/*.wgsl into OUT_DIR without comments, indentation or blank lines.
fn main() {
    println!("cargo:rerun-if-changed=src/shaders");
    let out = Path::new(&std::env::var("OUT_DIR").unwrap()).join("shaders");
    fs::create_dir_all(&out).unwrap();

    for entry in fs::read_dir("src/shaders").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "wgsl") {
            let source = fs::read_to_string(&path).unwrap();
            fs::write(out.join(path.file_name().unwrap()), minify(&source)).unwrap();
        }
    }
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
