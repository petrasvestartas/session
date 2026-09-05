//! engine — the reusable, scene-agnostic viewer core (ARCHITECTURE.md §9).
//! Grows into: gpu/ · pipelines/ · camera · pick · text · gumball/. App-specific code lives
//! in `app/` (added when the first scene/CLI/tool chapter needs it), never here.

pub mod gpu;
pub mod pipelines;
pub mod performance;
