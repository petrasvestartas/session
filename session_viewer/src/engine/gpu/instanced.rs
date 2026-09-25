use super::buffers::{GpuCtx, GrowBuf};
use std::ops::Range;

/// Slot value that keeps each row's own object row: slot 0, read by every plain draw.
pub const OWN_ROW: u32 = u32::MAX;

/// One instance slot: its object row, and the triangle id + 1 its turned triangles report, an
/// empty one of its definition's hidden rows.
pub type Slot = [u32; 2];

/// One definition drawn once per instance: its shared rows and the slots of its instances.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Draw {
    pub faces: Range<u32>,   // solid face indices
    pub pipes: Range<u32>,   // edge segments
    pub ribbons: Range<u32>, // line segments
    pub slots: Range<u32>,   // instance slots, never slot 0
}

/// The slot at locations 0 and 1, stepped per instance: row and empty triangle.
const SLOT_FACES: [wgpu::VertexAttribute; 2] = [
    wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Uint32,
    },
    wgpu::VertexAttribute {
        offset: 4,
        shader_location: 1,
        format: wgpu::VertexFormat::Uint32,
    },
];

/// The row alone at location 0, for ribbons.
const SLOT_ROW: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Uint32,
}];

/// The same at location 4, after the arena's vertex attributes.
const SLOT_ARENA: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 4,
    format: wgpu::VertexFormat::Uint32,
}];

/// The slot buffer as the pulled face shaders read it, at locations 0 and 1.
pub fn slot_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &SLOT_FACES,
    }
}

/// The slot buffer as the ribbon shader reads it, the row at location 0.
pub fn row_slot_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &SLOT_ROW,
    }
}

/// The slot buffer beside the arena's vertex and row buffers, at location 4.
pub fn arena_slot_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &SLOT_ARENA,
    }
}

/// The instance slots of one lane and the draws that read them; slot 0 is `OWN_ROW`.
pub struct Slots {
    buf: GrowBuf,     // one Slot per instance
    draws: Vec<Draw>, // one per definition
}

impl Slots {
    /// Only slot 0, so plain draws keep their rows.
    pub fn new(ctx: &GpuCtx) -> Self {
        let mut buf = GrowBuf::new(ctx, "instance.slots", 8, super::buffers::VERTS);
        buf.append(ctx, &[[OWN_ROW, 0]]);
        Self {
            buf,
            draws: Vec::new(),
        }
    }

    /// Replace every slot after slot 0 and the draws that read them.
    pub fn set(&mut self, ctx: &GpuCtx, rows: &[Slot], draws: &[Draw]) {
        self.buf.reset();
        self.buf.append(ctx, &[[OWN_ROW, 0]]);
        self.buf.append(ctx, rows);
        self.draws = draws.to_vec();
    }

    /// Back to slot 0 alone; `free` also shrinks the buffer.
    pub fn clear(&mut self, ctx: &GpuCtx, free: bool) {
        if free {
            self.buf.release(ctx);
        }

        self.set(ctx, &[], &[]);
    }

    /// Drop the draws; the slots wait to be written again.
    pub fn clear_draws(&mut self) {
        self.draws.clear();
    }

    /// The slot buffer, bound as vertex buffer `slot` of every draw that reads it.
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>, slot: u32) {
        pass.set_vertex_buffer(slot, self.buf.buf.slice(..));
    }

    /// The draws, one per definition.
    pub fn draws(&self) -> &[Draw] {
        &self.draws
    }

    /// True when some definition is drawn per instance.
    pub fn any(&self) -> bool {
        !self.draws.is_empty()
    }

    /// Instances drawn, over every definition.
    pub fn instances(&self) -> u32 {
        self.draws.iter().map(|d| d.slots.len() as u32).sum()
    }

    /// Bytes reserved on the GPU.
    pub fn allocated_bytes(&self) -> u64 {
        self.buf.buf.size()
    }
}

impl super::Gpu {
    /// Draw each definition once per instance: `rows` are the instance rows, `draws` their ranges.
    pub fn set_instanced(&mut self, rows: &[Slot], draws: &[Draw]) {
        self.arena.source_faces.slots.set(&self.ctx, rows, draws);
        self.segments.slots.set(&self.ctx, rows, draws);
        self.objects.geometry_changed();
    }
}

/// `range` cut to the first `len` rows; empty when nothing of it is left.
pub fn clamp(range: &Range<u32>, len: u32) -> Range<u32> {
    range.start.min(len)..range.end.min(len)
}
