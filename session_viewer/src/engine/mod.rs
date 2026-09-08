//! Prepared GPU resources, pipeline construction, shaped text and performance observations.
//! Source documents and geometry preparation belong to `app`; camera and interaction
//! coordinate these resources through `State`. Future editing tools do not own GPU internals.

pub mod gpu;
pub mod performance;
pub mod pipelines;

pub mod text;
