//! File-format -> Session (.pb) converters.
//!
//! Every foreign file format the project reads lives here, NOT in the kernels: `session_py`,
//! `session_rust` and `session_cpp` speak `.pb` (and their own JSON) and nothing else. One `[[bin]]`
//! per format turns a file into a `.pb` that all three kernels load through `Session::pb_load`.
//!
//! Keeping the parsers out of the kernels means a format is written and fixed ONCE instead of three
//! times in three languages, and the kernels stop carrying dependencies a format drags in
//! (`pdf_import` compiles MuPDF's C sources; `session_rust` must stay pure Rust so the viewer keeps
//! building for wasm32, where there is no filesystem to read from anyway).
pub mod obj;
pub mod ply;
pub mod xyz;
