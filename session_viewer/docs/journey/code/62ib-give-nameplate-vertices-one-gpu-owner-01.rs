use crate::text_gpu_support::{GpuCtx,GrowBuf,VERTS};
use crate::text_gpu_support::{Pipeline,Target};

pub(super) struct Rectangle {
    pub(super) bounds: [f32; 4],
    pub(super) clip: [f32; 4],
    pub(super) rounded: bool,
    pub(super) depth: Option<f32>,
    pub(super) object: Option<crate::text::TextObject>,
}
pub(super) struct Plates {
    vertices: GrowBuf,
    pipeline: Pipeline,
    id_pipeline: Pipeline,
    physical_vertices: u32,
}
