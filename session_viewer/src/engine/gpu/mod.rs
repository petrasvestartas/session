pub mod arena; // register:arena
pub mod backdrop; // register:backdrop
pub mod buffers; // register:buffers
pub mod clip; // register:clip
pub mod cloud; // register:cloud
pub mod device; // register:device
pub mod faces; // register:faces
pub mod frame; // register:frame
pub mod glyphs; // register:glyphs
pub mod hull; // register:hull
pub mod instance; // register:instance
pub mod instanced; // register:instanced
pub mod lod; // register:lod
pub mod objects;
pub mod slots; // register:slots

pub mod lane; // register:lane
pub mod matrices; // register:matrices
#[cfg(target_arch = "wasm32")]
mod pacing;
pub mod pass; // register:pass
pub(crate) mod patch; // register:patch
pub mod pick; // register:pick
pub mod planes; // register:planes
pub mod present; // register:present
pub mod render; // register:render
pub mod segments; // register:segments
pub mod splat; // register:splat
mod ssao; // register:ssao
pub mod surface_outline; // register:surface_outline
pub mod targets; // register:targets
pub mod text; // register:text
pub mod text_outline; // register:text_outline
pub mod timing; // register:timing
mod triangle_tiles; // register:triangle_tiles
pub mod ui; // register:ui
pub mod upload; // register:upload
pub mod vectors; // register:vectors
pub mod view; // register:view
mod widget; // register:widget
mod widget_mesh; // register:widget_mesh

mod context;
pub use context::Gpu;
#[cfg(test)]
pub(crate) use context::lane_shaders;

pub use cloud::{CloudDraw, LodNode, NO_NORMALS}; // register:clouds
pub use frame::FrameInput;
pub use glyphs::GlyphPoint; // register:markers
pub use instance::Instance;
pub use objects::{ObjectRow, Rebase};
pub use pick::Pick; // register:shell
pub use segments::CylinderSegment; // register:strokes
pub use upload::Upload;
pub use view::View;
