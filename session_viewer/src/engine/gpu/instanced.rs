use super::buffers::{GpuCtx, GrowBuf};
use std::ops::Range;

/// Slot value that keeps each row's own object row: slot 0, read by every plain draw.
pub const OWN_ROW: u32 = u32::MAX;

/// One instance slot: its object row, and what its triangles add to their arena index to name
/// their own triangle id, after every arena triangle and every earlier instance's.
pub type Slot = [u32; 2];

/// Texels per row of the slot table; the shaders split an index by it.
const TABLE_WIDTH: u32 = 1024;

/// One definition drawn once per instance: its shared rows and the slots of its instances.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Draw {
    pub faces: Range<u32>,   // solid face indices
    pub pipes: Range<u32>,   // edge segments
    pub ribbons: Range<u32>, // line segments
    pub slots: Range<u32>,   // instance slots, never slot 0
}

/// The slot at locations 0 and 1, stepped per instance: row and id base.
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

/// Where each instance's triangle ids lie, for the passes that read them: texel 0 holds the slot
/// count and the arena's triangle count, texel k slot k's row, first id, first definition
/// triangle and triangle count.
pub struct SlotTable {
    texture: wgpu::Texture, // Rgba32Uint, TABLE_WIDTH texels a row; one zero texel without instances
    pub view: wgpu::TextureView, // the texture, bound
}

impl SlotTable {
    /// One zero texel: no instances.
    pub fn new(ctx: &GpuCtx) -> Self {
        let texture = table_texture(ctx, 1, 1);
        Self {
            view: texture.create_view(&Default::default()),
            texture,
        }
    }

    /// Replace the table with `texels`, head first; the texture shrinks to one texel without slots.
    pub fn write(&mut self, ctx: &GpuCtx, texels: &[[u32; 4]]) {
        let count = texels.len() as u32;
        let (width, height) = if count <= 1 {
            (1, 1)
        } else {
            (TABLE_WIDTH, count.div_ceil(TABLE_WIDTH))
        };

        if self.texture.width() != width || self.texture.height() != height {
            self.texture = table_texture(ctx, width, height);
            self.view = self.texture.create_view(&Default::default());
        }

        let mut data = texels.to_vec();
        data.resize((width * height) as usize, [0; 4]);
        ctx.queue.write_texture(
            self.texture.as_image_copy(),
            bytemuck::cast_slice(&data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 16),
                rows_per_image: Some(height),
            },
            self.texture.size(),
        );
    }

    /// Bytes reserved on the GPU.
    pub fn allocated_bytes(&self) -> u64 {
        u64::from(self.texture.width()) * u64::from(self.texture.height()) * 16
    }
}

/// A `width` by `height` table of four u32 words a texel.
fn table_texture(ctx: &GpuCtx, width: u32, height: u32) -> wgpu::Texture {
    ctx.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("instance.table"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Uint, // row, first id, first triangle, triangles
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

/// The slots of instance `rows`, drawn by `draws`, with triangle ids after the first `arena`:
/// the vertex slots, the table texels and how many ids the instances take.
pub fn place(rows: &[u32], draws: &[Draw], arena: u32) -> (Vec<Slot>, Vec<[u32; 4]>, u32) {
    let mut slots = Vec::with_capacity(rows.len());
    let mut texels = Vec::with_capacity(rows.len() + 1);
    texels.push([rows.len() as u32, arena, 0, 0]);
    let mut start = arena;

    for draw in draws {
        let first = draw.faces.start / 3;
        let count = draw.faces.len() as u32 / 3;

        for k in draw.slots.clone() {
            let row = rows[k as usize - 1];
            slots.push([row, start.wrapping_sub(first)]);
            texels.push([row, start, first, count]);
            start += count;
        }
    }

    (slots, texels, start - arena)
}

impl super::Gpu {
    /// Draw each definition once per instance: `rows` are the instance rows in slot order, `draws`
    /// their ranges.
    pub fn set_instanced(&mut self, rows: &[u32], draws: &[Draw]) {
        self.arena.set_instanced(&self.ctx, rows, draws);
        let slots: Vec<Slot> = rows.iter().map(|&row| [row, 0]).collect();
        self.segments.slots.set(&self.ctx, &slots, draws);
        self.objects.geometry_changed();
    }
}

/// `range` cut to the first `len` rows; empty when nothing of it is left.
pub fn clamp(range: &Range<u32>, len: u32) -> Range<u32> {
    range.start.min(len)..range.end.min(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Instances number their triangles one after another, past the arena's: each slot's base
    /// turns a definition triangle into its own id, and the table names the slot back.
    #[test]
    fn instance_ids_follow_the_arena() {
        let draws = [
            Draw {
                faces: 30..36,
                slots: 1..3,
                ..Draw::default()
            },
            Draw {
                faces: 60..63,
                slots: 3..4,
                ..Draw::default()
            },
        ];
        let (slots, texels, count) = place(&[7, 8, 9], &draws, 100);
        assert_eq!(count, 2 + 2 + 1);
        assert_eq!(texels[0], [3, 100, 0, 0]);
        let slots_texels = [[7, 100, 10, 2], [8, 102, 10, 2], [9, 104, 20, 1]];
        assert_eq!(texels[1..], slots_texels);
        // triangle 11 of the definition, drawn by the second slot, is id 103 zero-based
        assert_eq!(slots[1], [8, 92]);
        assert_eq!(slots[1][1] + 11, 103);
        assert_eq!(slots[2][1] + 20, 104);
        let (slots, texels, count) = place(&[], &[], 100);
        assert!(slots.is_empty() && count == 0);
        assert_eq!(texels, [[0, 100, 0, 0]]);
    }
}
