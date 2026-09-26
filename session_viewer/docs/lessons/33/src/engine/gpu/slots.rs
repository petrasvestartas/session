// --8<-- [start:slots-layouts]
use super::buffers::{GpuCtx, GrowBuf};
use std::collections::HashMap;
use std::ops::Range;

/// A slot names the object row one copy of a shared mesh draws with; slot 0 holds this value,
/// which tells the shader "keep the vertex's own row", so every plain draw reads slot 0.
pub const OWN_ROW: u32 = u32::MAX;

/// [object row, id base]: the base turns a shared triangle's index into this copy's own triangle id.
/// A `type` alias only gives a name to an existing type.
pub type Slot = [u32; 2];

/// A texture row may hold at most 8192 texels in WebGPU, so slot k sits at (k % 1024, k / 1024).
const TABLE_WIDTH: u32 = 1024;

/// A definition is a mesh shared by many placed copies, its instances; one Draw repeats it per slot.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Draw {
    pub faces: Range<u32>,   // solid face indices
    pub pipes: Range<u32>,   // edge segments
    pub ribbons: Range<u32>, // line segments
    pub slots: Range<u32>,   // instance slots, never slot 0
}

/// Slot word 0 at shader location 0, word 1 at location 1.
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

/// draw_indexed(indices, 0, 3..5) draws the same triangles twice, as instances 3 and 4; a buffer
/// stepped per Instance moves one 8-byte slot per repetition instead of one per vertex.
pub fn slot_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &SLOT_FACES,
    }
}

/// For the ribbon shader of lesson 04b: the row alone.
pub fn row_slot_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &SLOT_ROW,
    }
}

/// Location 4, after the arena vertex (0-2) and its object row (3).
pub fn arena_slot_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: 8,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &SLOT_ARENA,
    }
}
// --8<-- [end:slots-layouts]

// --8<-- [start:slots]
/// Lesson 18a fills these; until then the buffer holds slot 0 alone and `draws` stays empty.
pub struct Slots {
    buf: GrowBuf,
    draws: Vec<Draw>,
}

impl Slots {
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

    /// Write `slots` from slot `at` in place; past the end the buffer grows.
    pub fn write(&mut self, ctx: &GpuCtx, at: u32, slots: &[Slot]) {
        let len = self.buf.len();
        // saturating_sub stops at 0 instead of wrapping a u32 round to 4 billion
        let inside = (len.saturating_sub(at) as usize).min(slots.len());
        self.buf.write_at(ctx, at, &slots[..inside]);

        if at > len {
            self.buf
                .append(ctx, &vec![[OWN_ROW, 0]; (at - len) as usize]);
        }

        self.buf.append(ctx, &slots[inside..]);
    }

    /// Draw `draws`, one per definition, from the slots as written.
    pub fn set_draws(&mut self, draws: &[Draw]) {
        self.draws.clear();
        self.draws.extend_from_slice(draws);
    }

    /// Drop the draws; the slots wait to be written again.
    pub fn clear_draws(&mut self) {
        self.draws.clear();
    }

    /// `slot` here is the vertex buffer number of the pipeline, not an instance slot.
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
// --8<-- [end:slots]

// --8<-- [start:slot-table]
/// A texture can hold plain numbers: Rgba32Uint = four u32 per texel, read in WGSL with textureLoad.
/// Texel 0 = [slot count, arena triangles, 0, 0]; texel k = [row, first id, first shared triangle, triangles].
/// Passes that start from a triangle id (visibility, picking) look its instance up here.
pub struct SlotTable {
    texture: wgpu::Texture, // one zero texel while there are no instances
    pub view: wgpu::TextureView,
}

impl SlotTable {
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
        data.resize((width * height) as usize, [0; 4]); // write_texture wants every texel of the rectangle
        ctx.queue.write_texture(
            self.texture.as_image_copy(),
            bytemuck::cast_slice(&data),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 16), // 16 bytes a texel
                rows_per_image: Some(height),
            },
            self.texture.size(),
        );
    }

    /// Overwrite texels from texel `first` in place; false when the table is too small for them.
    pub fn write_at(&self, ctx: &GpuCtx, first: u32, texels: &[[u32; 4]]) -> bool {
        let end = first + texels.len() as u32;
        let width = self.texture.width();

        if end > width * self.texture.height() || (end > 1 && width != TABLE_WIDTH) {
            return false;
        }

        let mut at = first;
        let mut rest = texels;

        // one write per texture row the run crosses
        while !rest.is_empty() {
            let (x, y) = (at % width, at / width);
            let (row, tail) = rest.split_at(((width - x) as usize).min(rest.len()));
            ctx.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    origin: wgpu::Origin3d { x, y, z: 0 },
                    ..self.texture.as_image_copy()
                },
                bytemuck::cast_slice(row),
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row.len() as u32 * 16),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d {
                    width: row.len() as u32,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );
            at += row.len() as u32;
            rest = tail;
        }

        true
    }

    /// Bytes reserved on the GPU.
    pub fn allocated_bytes(&self) -> u64 {
        u64::from(self.texture.width()) * u64::from(self.texture.height()) * 16
    }
}

/// TEXTURE_BINDING lets shaders read it, COPY_DST lets write_texture fill it.
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
        format: wgpu::TextureFormat::Rgba32Uint,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}
// --8<-- [end:slot-table]

// --8<-- [start:slot-space]
// The slot allocator: lesson 18a drives it; until then it holds slot 0 alone.

/// Triangle ids kept free past the arena's after it grew, so appending objects rarely renumbers.
fn headroom(arena: u32) -> u32 {
    arena / 64 + 1024
}

/// Triangle ids a run may keep spare past a quarter of its instances: each id costs the tile
/// tables 96 bytes, so a light definition grows in place and a heavy one reserves a quarter.
const SPARE_IDS: u32 = 4096;

/// Slots a full run of `len` instances of `triangles` triangles moves with.
fn grown_cap(len: u32, triangles: u32) -> u32 {
    len + len / 4 + (SPARE_IDS / triangles.max(1)).min(4)
}

/// One definition as the scene holds it: its instance rows in slot order and its shared rows.
pub struct Definition<'a> {
    pub key: u32,            // names the definition for as long as it lives
    pub rows: &'a [u32],     // instance rows; a member keeps its position until it leaves
    pub faces: Range<u32>,   // solid face indices
    pub pipes: Range<u32>,   // edge segments
    pub ribbons: Range<u32>, // line segments
}

/// A run of slots and triangle ids: one definition's instances and room to grow, or free.
#[derive(Clone, Debug, PartialEq)]
struct Run {
    key: Option<u32>, // the definition it holds; None = free
    start: u32,       // first slot
    cap: u32,         // slots in the run
    len: u32,         // slots in use, from the start
    ids: u32,         // first triangle id, past the base
    span: u32,        // triangle ids in the run
    first: u32,       // first definition triangle
    triangles: u32,   // triangles per instance
    draw: Draw,       // its shared rows; the slots follow the run
}

impl Run {
    /// A free run over slots `start..start + cap` and ids `ids..ids + span`.
    fn free(start: u32, cap: u32, ids: u32, span: u32) -> Self {
        Self {
            key: None,
            start,
            cap,
            len: 0,
            ids,
            span,
            first: 0,
            triangles: 0,
            draw: Draw::default(),
        }
    }

    /// The same room, freed.
    fn freed(&self) -> Self {
        Self::free(self.start, self.cap, self.ids, self.span)
    }
}

/// What an update changed on the GPU side.
#[derive(Debug, Default, PartialEq)]
pub struct Change {
    pub all: bool,              // every slot and texel: numbered again
    pub slots: Vec<Range<u32>>, // else the slots and texels to write in place, sorted
}

impl Change {
    /// Everything again.
    fn all() -> Self {
        Self {
            all: true,
            slots: Vec::new(),
        }
    }

    /// Slots written, for tests: None for everything.
    #[cfg(test)]
    fn written(&self) -> Option<u32> {
        (!self.all).then(|| self.slots.iter().map(|range| range.len() as u32).sum())
    }
}

/// The instance slots of every definition, one run each, in slot and triangle id order: a member
/// that joins or leaves writes its own slot, a full run moves into free room a quarter bigger,
/// and everything is numbered again only when the room is mostly free, the arena outgrows the
/// headroom, or the scene compacts.
#[derive(Default)]
pub struct SlotSpace {
    rows: Vec<u32>,   // row per slot; slot 0 keeps plain rows their own
    runs: Vec<Run>,   // cover slots 1.. without gaps
    base: u32,        // first instance triangle id, at or past the arena's
    arena: u32,       // arena triangles
    pub written: u32, // slots the last write sent, the head included; u32::MAX = all of them
}

impl SlotSpace {
    /// Slots in use or reserved, slot 0 included.
    pub fn end(&self) -> u32 {
        self.runs.last().map_or(1, |run| run.start + run.cap)
    }

    /// Triangle ids in use, the arena's first; None without instances.
    pub fn triangle_count(&self) -> Option<u32> {
        let last = self.runs.last()?;
        let any = self.runs.iter().any(|run| run.key.is_some() && run.len > 0);
        any.then(|| self.base + last.ids + last.span)
    }

    /// One draw per definition with instances, the oldest definition first wherever its run
    /// moved, so equal depths resolve as they did at load.
    pub fn draws(&self) -> Vec<Draw> {
        let mut held: Vec<&Run> = Vec::new();

        for run in self
            .runs
            .iter()
            .filter(|run| run.key.is_some() && run.len > 0)
        {
            held.insert(held.partition_point(|other| other.key < run.key), run);
        }

        held.iter()
            .map(|run| Draw {
                slots: run.start..run.start + run.len,
                ..run.draw.clone()
            })
            .collect()
    }

    /// Index of the run holding slot `slot`.
    fn run_at(&self, slot: u32) -> usize {
        self.runs.partition_point(|run| run.start + run.cap <= slot)
    }

    /// Slot `slot` as the vertex shaders read it: its row, and what its triangles add to their
    /// definition index to name their own id.
    pub fn slot(&self, slot: u32) -> Slot {
        let run = &self.runs[self.run_at(slot)];
        let start = self.base + run.ids + (slot - run.start) * run.triangles;
        [self.rows[slot as usize], start.wrapping_sub(run.first)]
    }

    /// Texel `slot` of the table: row, first id, first definition triangle and triangle count; a
    /// free slot names no triangle. Texel 0 holds the slot count and the arena's triangles.
    pub fn texel(&self, slot: u32) -> [u32; 4] {
        if slot == 0 {
            return [self.end() - 1, self.arena, 0, 0];
        }

        let run = &self.runs[self.run_at(slot)];
        let j = slot - run.start;
        let start = self.base + run.ids + j * run.triangles;
        let count = if run.key.is_some() && j < run.len {
            run.triangles
        } else {
            0
        };
        [self.rows[slot as usize], start, run.first, count]
    }

    /// Every definition packed tight from triangle id `base`.
    fn place_all(&mut self, definitions: &[Definition], base: u32) -> Change {
        self.runs.clear();
        self.rows.clear();
        self.rows.push(OWN_ROW);
        self.base = base;

        for definition in definitions {
            let len = definition.rows.len() as u32;

            if len > 0 {
                let at = self.allocate(len, definition.faces.len() as u32 / 3);
                self.hold(at, definition);
            }
        }

        Change::all()
    }

    /// Everything again after a load or a compaction, from the arena's `arena` triangles.
    pub fn reset(&mut self, definitions: &[Definition], arena: u32) -> Change {
        self.arena = arena;
        self.place_all(definitions, arena)
    }

    /// The arena holds `arena` triangles now: only the head texel changes while the instance ids
    /// still lie past it, else everything is numbered again with headroom.
    pub fn follow(&mut self, arena: u32) -> Change {
        if arena == self.arena {
            return Change::default();
        }

        self.arena = arena;

        // no slots: the shaders never read the head
        if self.runs.is_empty() {
            self.base = arena;
            return Change::default();
        }

        if arena <= self.base && self.base - arena <= 2 * headroom(arena) {
            return Change {
                all: false,
                slots: vec![0..1],
            };
        }

        let held: Vec<(u32, Vec<u32>, Draw)> = self
            .runs
            .iter()
            .filter_map(|run| {
                let rows = self.rows[run.start as usize..(run.start + run.len) as usize].to_vec();
                Some((run.key?, rows, run.draw.clone()))
            })
            .collect();
        let definitions: Vec<Definition> = held
            .iter()
            .map(|(key, rows, draw)| Definition {
                key: *key,
                rows,
                faces: draw.faces.clone(),
                pipes: draw.pipes.clone(),
                ribbons: draw.ribbons.clone(),
            })
            .collect();
        self.place_all(&definitions, arena + headroom(arena))
    }

    /// Take the scene's `definitions` over `arena` triangles; `touched` names each (definition,
    /// member position) written since the last update, the only slots written in place.
    pub fn update(
        &mut self,
        definitions: &[Definition],
        touched: &[(u32, u32)],
        arena: u32,
    ) -> Change {
        if self.follow(arena).all {
            return self.place_all(definitions, self.base);
        }

        let mut dirty: Vec<Range<u32>> = vec![0..1]; // runs, a few per update
        let mut members: Vec<u32> = Vec::new(); // single slots, one per touched member
        let live: HashMap<u32, usize> = (0..definitions.len())
            .map(|i| (definitions[i].key, i))
            .collect();

        // a run whose definition went, or was walked again into other faces, frees its room
        for run in &mut self.runs {
            let Some(key) = run.key else {
                continue;
            };
            let same = live.get(&key).is_some_and(|&i| {
                let faces = &definitions[i].faces;
                faces.start / 3 == run.first && faces.len() as u32 / 3 == run.triangles
            });

            if !same {
                dirty.push(run.start..run.start + run.cap);
                *run = run.freed();
            }
        }

        let held: HashMap<u32, usize> = self
            .runs
            .iter()
            .filter_map(|run| Some((run.key?, run.start as usize)))
            .collect();

        // members that joined, left or swapped places write their own slot
        for &(key, position) in touched {
            let (Some(&start), Some(&i)) = (held.get(&key), live.get(&key)) else {
                continue;
            };
            let run = &self.runs[self.run_at(start as u32)];
            let rows = definitions[i].rows;

            if position < run.cap && (position as usize) < rows.len() {
                let slot = run.start + position;
                self.rows[slot as usize] = rows[position as usize];
                members.push(slot);
            }
        }

        for definition in definitions {
            let len = definition.rows.len() as u32;
            let grow = held
                .get(&definition.key)
                .map(|&start| self.run_at(start as u32));

            if let Some(at) = grow {
                let run = &mut self.runs[at];

                if len <= run.cap {
                    if len < run.len {
                        dirty.push(run.start + len..run.start + run.len);
                    }

                    run.len = len;
                    run.draw = shared_draw(definition);
                    continue;
                }

                // full: the run moves
                dirty.push(run.start..run.start + run.cap);
                *run = run.freed();
            }

            if len == 0 {
                continue;
            }

            let triangles = definition.faces.len() as u32 / 3;
            let cap = if grow.is_some() {
                grown_cap(len, triangles)
            } else {
                len
            };
            let at = self.allocate(cap, triangles);
            let run = self.hold(at, definition);
            dirty.push(run);
        }

        self.merge_free();

        // mostly free room, in slots or in triangle ids: pack everything again
        let (mut free, mut used, mut free_ids, mut used_ids) = (0, 0, 0u64, 0u64);

        for run in &self.runs {
            let ids = u64::from(run.len) * u64::from(run.triangles);
            free += run.cap - run.len;
            used += run.len;
            free_ids += u64::from(run.span) - ids;
            used_ids += ids;
        }

        let spare_ids = u64::from(SPARE_IDS) * self.runs.len() as u64;

        if free > used + 256 || free_ids > used_ids / 2 + spare_ids {
            return self.place_all(definitions, self.base);
        }

        members.sort_unstable();
        members.dedup();
        dirty.extend(
            members
                .chunk_by(|a, b| a + 1 == *b)
                .map(|run| run[0]..run[0] + run.len() as u32),
        );
        Change {
            all: false,
            slots: coalesce(dirty, self.end()),
        }
    }

    /// Run `at` takes `definition`: its rows from the run's first slot. Returns the run's slots.
    fn hold(&mut self, at: usize, definition: &Definition) -> Range<u32> {
        let run = &mut self.runs[at];
        let len = definition.rows.len();
        run.key = Some(definition.key);
        run.len = len as u32;
        run.first = definition.faces.start / 3;
        run.triangles = definition.faces.len() as u32 / 3;
        run.draw = shared_draw(definition);
        let (start, end) = (run.start as usize, (run.start + run.cap) as usize);

        if self.rows.len() < end {
            self.rows.resize(end, 0);
        }

        self.rows[start..start + len].copy_from_slice(definition.rows);
        start as u32..end as u32
    }

    /// A free run of `cap` slots with `triangles` ids a slot: the first free run with room, cut
    /// to size, else new room past the end. Returns its index.
    fn allocate(&mut self, cap: u32, triangles: u32) -> usize {
        let span = cap * triangles;
        let fits = |run: &Run| run.key.is_none() && run.cap >= cap && run.span >= span;

        let Some(at) = self.runs.iter().position(fits) else {
            let (start, ids) = self
                .runs
                .last()
                .map_or((1, 0), |run| (run.start + run.cap, run.ids + run.span));
            self.runs.push(Run::free(start, cap, ids, span));
            return self.runs.len() - 1;
        };

        let hole = self.runs[at].clone();

        // the rest of the room stays free; ids without slots stay with the run
        if hole.cap > cap {
            self.runs[at] = Run::free(hole.start, cap, hole.ids, span);
            let rest = Run::free(
                hole.start + cap,
                hole.cap - cap,
                hole.ids + span,
                hole.span - span,
            );
            self.runs.insert(at + 1, rest);
        }

        at
    }

    /// Join neighbouring free runs and drop free room at the end.
    fn merge_free(&mut self) {
        let mut merged: Vec<Run> = Vec::with_capacity(self.runs.len());

        for run in self.runs.drain(..) {
            match merged.last_mut() {
                Some(last) if last.key.is_none() && run.key.is_none() => {
                    last.cap += run.cap;
                    last.span += run.span;
                }
                _ => merged.push(run),
            }
        }

        if merged.last().is_some_and(|run| run.key.is_none()) {
            merged.pop();
        }

        self.runs = merged;
        self.rows.truncate(self.end() as usize);
    }
}

/// The shared rows a definition draws per instance; its slots come from its run.
fn shared_draw(definition: &Definition) -> Draw {
    Draw {
        faces: definition.faces.clone(),
        pipes: definition.pipes.clone(),
        ribbons: definition.ribbons.clone(),
        slots: 0..0,
    }
}

/// `ranges` sorted, joined where they touch or overlap, and cut at `end`; few enough to insert.
fn coalesce(ranges: Vec<Range<u32>>, end: u32) -> Vec<Range<u32>> {
    let mut sorted: Vec<Range<u32>> = Vec::with_capacity(ranges.len());

    for range in ranges {
        sorted.insert(sorted.partition_point(|r| r.start <= range.start), range);
    }

    let mut out: Vec<Range<u32>> = Vec::with_capacity(sorted.len());

    for range in sorted {
        let range = range.start.min(end)..range.end.min(end);

        if range.is_empty() {
            continue;
        }

        match out.last_mut() {
            Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
            _ => out.push(range),
        }
    }

    out
}

/// `range` cut to the first `len` rows; empty when nothing of it is left.
pub fn clamp(range: &Range<u32>, len: u32) -> Range<u32> {
    range.start.min(len)..range.end.min(len)
}
// --8<-- [end:slot-space]

// --8<-- [start:slots-tests]
#[cfg(test)]
mod tests {
    use super::*;

    /// A definition of `triangles` triangles from face index `faces` with instance `rows`.
    fn definition(key: u32, rows: &[u32], faces: u32, triangles: u32) -> Definition<'_> {
        Definition {
            key,
            rows,
            faces: faces..faces + 3 * triangles,
            pipes: 0..0,
            ribbons: 0..0,
        }
    }

    /// Every used slot names its member and ids no other slot takes, past the arena's; the table
    /// is sorted by first id, as the shaders' binary search needs.
    fn check(space: &SlotSpace, members: &[(u32, Vec<u32>, u32)], arena: u32) {
        let mut ids = Vec::new();
        let texels: Vec<[u32; 4]> = (0..space.end()).map(|slot| space.texel(slot)).collect();
        assert_eq!(texels[0], [space.end() - 1, arena, 0, 0]);
        assert!(
            texels[1..].windows(2).all(|pair| pair[0][1] <= pair[1][1]),
            "sorted"
        );
        let draws = space.draws();

        for (key, rows, triangles) in members {
            let run = space
                .runs
                .iter()
                .find(|run| run.key == Some(*key))
                .expect("a run");
            let draw = draws
                .iter()
                .find(|d| d.slots.start == run.start)
                .expect("a draw");
            assert_eq!(draw.slots.len(), rows.len());

            for (position, row) in rows.iter().enumerate() {
                let slot = run.start + position as u32;
                let [texel_row, first, _, count] = texels[slot as usize];
                assert_eq!((texel_row, count), (*row, *triangles), "slot {slot}");
                assert_eq!(space.slot(slot)[0], *row);
                assert_eq!(space.slot(slot)[1].wrapping_add(run.first), first);
                ids.push(first..first + count);
            }
        }

        ids.sort_by_key(|range| range.start);
        assert!(
            ids.windows(2).all(|pair| pair[0].end <= pair[1].start),
            "ids overlap"
        );
        assert!(ids.first().is_none_or(|range| range.start >= arena));
        let count = space.triangle_count().unwrap_or(arena);
        assert!(ids.last().is_none_or(|range| range.end <= count));
    }

    /// Instances number their triangles one after another, past the arena's: each slot's base
    /// turns a definition triangle into its own id, and the table names the slot back.
    #[test]
    fn instance_ids_follow_the_arena() {
        let mut space = SlotSpace::default();
        let definitions = [definition(1, &[7, 8], 30, 2), definition(2, &[9], 60, 1)];
        assert!(space.reset(&definitions, 100).all);
        assert_eq!(space.triangle_count(), Some(105));
        let texels: Vec<[u32; 4]> = (0..4).map(|slot| space.texel(slot)).collect();
        let want = [
            [3, 100, 0, 0],
            [7, 100, 10, 2],
            [8, 102, 10, 2],
            [9, 104, 20, 1],
        ];
        assert_eq!(texels, want);
        // triangle 11 of the definition, drawn by the second slot, is id 103 zero-based
        assert_eq!(space.slot(2), [8, 92]);
        assert_eq!(space.slot(2)[1] + 11, 103);
        assert_eq!(space.slot(3)[1] + 20, 104);

        assert!(space.reset(&[], 100).all);
        assert_eq!(space.triangle_count(), None);
        assert_eq!(space.texel(0), [0, 100, 0, 0]);
    }

    /// Definitions of `members` (key, rows, triangles), the faces of key k at 30 * k.
    fn definitions(members: &[(u32, Vec<u32>, u32)]) -> Vec<Definition<'_>> {
        members
            .iter()
            .map(|(key, rows, triangles)| definition(*key, rows, 30 * *key, *triangles))
            .collect()
    }

    /// Members join and leave as the scene does, by swap removal: each edit writes its own slot
    /// and the head, a full run moves now and then, and the slots always match the members.
    #[test]
    fn edits_write_only_their_own_slots() {
        let mut members: Vec<(u32, Vec<u32>, u32)> = vec![
            (1, (0..40).collect(), 2),
            (2, (100..110).collect(), 5),
            (3, vec![200], 1),
        ];
        let mut space = SlotSpace::default();
        space.reset(&definitions(&members), 1000);
        check(&space, &members, 1000);
        let mut seed = 5u32;
        let mut next = |n: u32| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) % n
        };
        let mut row = 1000;
        let mut moves = 0;

        for step in 0..600 {
            let batch = next(3) as usize;
            let (key, rows, _) = &mut members[batch];
            let mut touched = Vec::new();

            // join, or leave by swap removal
            if next(2) == 0 || rows.len() < 2 {
                touched.push((*key, rows.len() as u32));
                rows.push(row);
                row += 1;
            } else {
                let at = next(rows.len() as u32) as usize;
                rows.swap_remove(at);

                if at < rows.len() {
                    touched.push((*key, at as u32));
                }
            }

            let change = space.update(&definitions(&members), &touched, 1000);
            check(&space, &members, 1000);

            match change.written() {
                // the head, the member's slot and the slot a leaver vacated
                Some(written) if written > 3 => moves += 1,
                Some(_) => {}
                None => panic!("step {step} numbered everything again"),
            }
        }

        // a full run moves to room a quarter bigger: rarely
        assert!(moves < 60, "{moves} moves");
    }

    /// A heavy definition joined one instance at a time never reserves much more than its
    /// instances use: every reserved id costs the tile tables 96 bytes.
    #[test]
    fn heavy_definitions_reserve_few_ids() {
        let triangles = 870_000;
        let mut rows: Vec<u32> = vec![0];
        let mut space = SlotSpace::default();
        space.reset(&[definition(1, &rows, 0, triangles)], 1000);

        for row in 1..40 {
            let touched = [(1, rows.len() as u32)];
            rows.push(row);
            space.update(&[definition(1, &rows, 0, triangles)], &touched, 1000);
            check(&space, &[(1, rows.clone(), triangles)], 1000);
            let used = rows.len() as u64 * u64::from(triangles);
            let reserved = u64::from(space.triangle_count().unwrap() - space.base);
            assert!(
                reserved <= used * 3 / 2 + u64::from(SPARE_IDS),
                "{row}: {reserved} ids for {used}"
            );
        }
    }

    /// The last instance of a definition frees its room for another; a new definition takes it.
    #[test]
    fn freed_room_is_taken_again() {
        let mut space = SlotSpace::default();
        let (a, b): (Vec<u32>, Vec<u32>) = ((0..8).collect(), (10..14).collect());
        space.reset(&[definition(1, &a, 0, 2), definition(2, &b, 30, 2)], 50);
        let change = space.update(&[definition(2, &b, 30, 2)], &[], 50);
        assert_eq!(change.written(), Some(9), "the head and the freed run");
        let c: Vec<u32> = (20..26).collect();
        let change = space.update(
            &[definition(2, &b, 30, 2), definition(3, &c, 60, 1)],
            &[],
            50,
        );
        assert_eq!(
            change.written(),
            Some(1 + 6),
            "the head and the room it took"
        );
        let run = space.runs.iter().find(|run| run.key == Some(3)).unwrap();
        assert_eq!(run.start, 1, "the freed room at the front");
        check(&space, &[(2, b.clone(), 2), (3, c.clone(), 1)], 50);
    }

    /// The arena grows: within the headroom only the head changes, past it everything is
    /// numbered again with room to spare.
    #[test]
    fn the_arena_grows_into_the_headroom() {
        let mut space = SlotSpace::default();
        let rows: Vec<u32> = (0..5).collect();
        space.reset(&[definition(1, &rows, 0, 3)], 100);
        let change = space.follow(200);
        assert!(change.all, "past the ids: numbered again");
        check(&space, &[(1, rows.clone(), 3)], 200);
        assert_eq!(space.base, 200 + headroom(200));

        for arena in [210, 500, 900] {
            let change = space.follow(arena);
            assert_eq!(change.slots, vec![0..1], "{arena}: the head alone");
            check(&space, &[(1, rows.clone(), 3)], arena);
        }

        assert!(space.follow(10_000).all);
        assert!(space.follow(10_000) == Change::default());
    }
}
// --8<-- [end:slots-tests]
