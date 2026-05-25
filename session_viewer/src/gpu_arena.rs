//! Game-engine-style arena allocator over a wgpu vertex (and optional index)
//! buffer. One arena per topology. Adding a geometry appends (or fills a hole
//! from the free list); removing marks a range free without shifting data;
//! updating writes in place via `queue.write_buffer`. No buffer recreation
//! per change; growth is amortized 2x.

use bytemuck::Pod;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Range;

/// One slot in an arena: the vertex range, optional index range, and the
/// `instance_id` pointing into the GpuSession instance buffer.
#[derive(Clone, Debug)]
pub struct ArenaSlot {
    pub vertex_range: Range<u32>,
    pub index_range: Option<Range<u32>>,
    pub instance_id: u32,
}

/// Free range stored as `[start, end)`. Best-fit allocation walks this list.
#[derive(Clone, Copy, Debug)]
struct FreeRange {
    start: u32,
    end: u32,
}

impl FreeRange {
    fn len(self) -> u32 {
        self.end - self.start
    }
}

/// Free-list/cursor allocator over a wgpu::Buffer.
///
/// `vbo` (vertex buffer) is always present. `ibo` (index buffer) is `Some`
/// for topologies that use indices (triangles, line lists).
pub struct GpuArena<V: Pod> {
    pub label: String,
    pub vbo: wgpu::Buffer,
    pub ibo: Option<wgpu::Buffer>,

    pub capacity_verts: u32,
    pub capacity_inds: u32,

    /// Bump pointer — used until exhausted, then fall back to free list.
    cursor_verts: u32,
    cursor_inds: u32,

    free_verts: Vec<FreeRange>,
    free_inds: Vec<FreeRange>,

    /// guid → slot. Lookup for updates/deletes/picking.
    slots: HashMap<String, ArenaSlot>,

    _marker: PhantomData<V>,
}

impl<V: Pod> GpuArena<V> {
    pub fn new(
        device: &wgpu::Device,
        label: &str,
        capacity_verts: u32,
        capacity_inds: u32,
    ) -> Self {
        let with_indices = capacity_inds > 0;
        let vbo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{label}.vbo")),
            size: (capacity_verts as u64) * (std::mem::size_of::<V>() as u64),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let ibo = with_indices.then(|| {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{label}.ibo")),
                size: (capacity_inds as u64) * (std::mem::size_of::<u32>() as u64),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        });
        Self {
            label: label.to_string(),
            vbo,
            ibo,
            capacity_verts,
            capacity_inds,
            cursor_verts: 0,
            cursor_inds: 0,
            free_verts: Vec::new(),
            free_inds: Vec::new(),
            slots: HashMap::new(),
            _marker: PhantomData,
        }
    }

    /// Allocate vertex (and optional index) ranges, upload data, register slot.
    /// Grows the underlying buffer 2x if exhausted.
    pub fn allocate(
        &mut self,
        guid: &str,
        verts: &[V],
        inds: Option<&[u32]>,
        instance_id: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> ArenaSlot {
        let v_count = verts.len() as u32;
        let i_count = inds.map(|s| s.len() as u32).unwrap_or(0);

        let vertex_range = self.allocate_vertex_range(v_count, device, queue);
        let index_range = if let Some(inds) = inds {
            Some(self.allocate_index_range(i_count, device, queue, inds))
        } else {
            None
        };

        queue.write_buffer(
            &self.vbo,
            (vertex_range.start as u64) * (std::mem::size_of::<V>() as u64),
            bytemuck::cast_slice(verts),
        );

        let slot = ArenaSlot { vertex_range, index_range, instance_id };
        self.slots.insert(guid.to_string(), slot.clone());
        slot
    }

    fn allocate_vertex_range(
        &mut self,
        n: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Range<u32> {
        if let Some(idx) = self.free_verts.iter().position(|r| r.len() >= n) {
            let r = self.free_verts[idx];
            let start = r.start;
            let end = start + n;
            if r.len() == n {
                self.free_verts.remove(idx);
            } else {
                self.free_verts[idx] = FreeRange { start: end, end: r.end };
            }
            return start..end;
        }
        if self.cursor_verts + n <= self.capacity_verts {
            let start = self.cursor_verts;
            self.cursor_verts += n;
            return start..(start + n);
        }
        self.grow_vertex_buffer(self.cursor_verts + n, device, queue);
        let start = self.cursor_verts;
        self.cursor_verts += n;
        start..(start + n)
    }

    fn allocate_index_range(
        &mut self,
        n: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        inds: &[u32],
    ) -> Range<u32> {
        let ibo = self
            .ibo
            .as_ref()
            .expect("allocate_index_range called on arena without ibo");
        if let Some(idx) = self.free_inds.iter().position(|r| r.len() >= n) {
            let r = self.free_inds[idx];
            let start = r.start;
            let end = start + n;
            if r.len() == n {
                self.free_inds.remove(idx);
            } else {
                self.free_inds[idx] = FreeRange { start: end, end: r.end };
            }
            queue.write_buffer(
                ibo,
                (start as u64) * (std::mem::size_of::<u32>() as u64),
                bytemuck::cast_slice(inds),
            );
            return start..end;
        }
        if self.cursor_inds + n <= self.capacity_inds {
            let start = self.cursor_inds;
            self.cursor_inds += n;
            queue.write_buffer(
                ibo,
                (start as u64) * (std::mem::size_of::<u32>() as u64),
                bytemuck::cast_slice(inds),
            );
            return start..(start + n);
        }
        self.grow_index_buffer(self.cursor_inds + n, device, queue);
        let ibo = self.ibo.as_ref().unwrap();
        let start = self.cursor_inds;
        self.cursor_inds += n;
        queue.write_buffer(
            ibo,
            (start as u64) * (std::mem::size_of::<u32>() as u64),
            bytemuck::cast_slice(inds),
        );
        start..(start + n)
    }

    fn grow_vertex_buffer(
        &mut self,
        needed: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let mut new_cap = (self.capacity_verts.max(1)) * 2;
        while new_cap < needed {
            new_cap *= 2;
        }
        let new_vbo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{}.vbo", self.label)),
            size: (new_cap as u64) * (std::mem::size_of::<V>() as u64),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(&format!("{}.grow", self.label)),
        });
        encoder.copy_buffer_to_buffer(
            &self.vbo,
            0,
            &new_vbo,
            0,
            (self.cursor_verts as u64) * (std::mem::size_of::<V>() as u64),
        );
        queue.submit(std::iter::once(encoder.finish()));
        self.vbo = new_vbo;
        self.capacity_verts = new_cap;
    }

    fn grow_index_buffer(
        &mut self,
        needed: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let mut new_cap = (self.capacity_inds.max(1)) * 2;
        while new_cap < needed {
            new_cap *= 2;
        }
        let new_ibo = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{}.ibo", self.label)),
            size: (new_cap as u64) * (std::mem::size_of::<u32>() as u64),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        if let Some(old_ibo) = self.ibo.as_ref() {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&format!("{}.grow_ibo", self.label)),
            });
            encoder.copy_buffer_to_buffer(
                old_ibo,
                0,
                &new_ibo,
                0,
                (self.cursor_inds as u64) * (std::mem::size_of::<u32>() as u64),
            );
            queue.submit(std::iter::once(encoder.finish()));
        }
        self.ibo = Some(new_ibo);
        self.capacity_inds = new_cap;
    }

    /// Mark a slot's ranges free. Does not shift other data.
    pub fn free(&mut self, guid: &str) {
        if let Some(slot) = self.slots.remove(guid) {
            self.free_verts.push(FreeRange {
                start: slot.vertex_range.start,
                end: slot.vertex_range.end,
            });
            if let Some(ir) = slot.index_range {
                self.free_inds.push(FreeRange {
                    start: ir.start,
                    end: ir.end,
                });
            }
        }
    }

    /// Iterate slots for drawing.
    pub fn iter_slots(&self) -> impl Iterator<Item = (&String, &ArenaSlot)> {
        self.slots.iter()
    }

    pub fn clear(&mut self) {
        self.slots.clear();
        self.free_verts.clear();
        self.free_inds.clear();
        self.cursor_verts = 0;
        self.cursor_inds = 0;
    }
}

