
// `macro_rules!` makes a macro, code that writes code; it must come before the `mod` lines that use it.
/// A WGSL file from src/shaders as build.rs wrote it: no comments, indentation or blank lines.
macro_rules! shader {
    // `$name:literal` matches one string literal, such as "background.wgsl".
    ($name:literal) => {
        // `include_str!` pastes the file into the binary at compile time; OUT_DIR is the folder build.rs wrote.
        include_str!(concat!(env!("OUT_DIR"), "/shaders/", $name))
    };
}

// `mod engine;` makes src/engine/mod.rs part of this crate.
mod engine;
