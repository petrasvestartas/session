//! Reusable, scene-agnostic viewer engine: GPU mirror + render pipelines.
//!
//! Nothing in here knows about the demo scene, CLI vocabulary, or tree group
//! conventions — those live in the application layer. The crate root re-exports
//! these modules under their historical names (`gpu_session`, `pipelines`, …) so
//! call sites use short paths.

pub mod gpu;
pub mod pipelines;
