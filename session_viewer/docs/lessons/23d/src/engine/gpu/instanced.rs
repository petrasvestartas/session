// --8<-- [start:instanced-gpu]
// Instancing = one definition's triangles uploaded once and drawn once per placed copy: 500 equal beams, one mesh.
// Slot = one placed copy, naming the instance row whose placement it takes; slot 0 is the arena drawing itself.
use super::buffers::GpuCtx;
use super::slots::{Change, Slot};
use std::ops::Range;

pub use super::slots::Definition;

impl super::Gpu {
    /// Draw each definition once per instance: everything again when `full`, else only the
    /// slots of the (definition, position) members `touched` since the last call, unless a run
    /// moves.
    pub fn set_instanced(
        &mut self,
        definitions: &[Definition],
        touched: &[(u32, u32)],
        full: bool,
    ) {
        let arena = self.arena.face_count() / 3;
        let change = if full {
            self.arena.space.reset(definitions, arena)
        } else {
            self.arena.space.update(definitions, touched, arena)
        };
        self.write_slots(&change);
    }

    /// Follow the arena's triangle count into the instance ids.
    pub fn follow_arena(&mut self) {
        let arena = self.arena.face_count() / 3;
        let change = self.arena.space.follow(arena);

        if change != Change::default() {
            self.write_slots(&change);
        }
    }

    /// Write what `change` names into both slot buffers and the table, and the draws.
    fn write_slots(&mut self, change: &Change) {
        let ctx = &self.ctx;
        let space = &self.arena.space;
        let end = space.end();
        // a closure: the texels of a slot range, read from `space` each time it is called
        let texels =
            |range: Range<u32>| -> Vec<[u32; 4]> { range.map(|at| space.texel(at)).collect() };
        let mut all = change.all;

        for range in &change.slots {
            // past the table's size: all of it again
            if all
                || !self
                    .arena
                    .table
                    .write_at(ctx, range.start, &texels(range.clone()))
            {
                all = true;
                break;
            }

            let first = range.start.max(1);
            let slots: Vec<Slot> = (first..range.end).map(|at| space.slot(at)).collect();
            self.arena.source_faces.slots.write(ctx, first, &slots);
            self.segments.slots.write(ctx, first, &slots);
        }

        if all {
            let slots: Vec<Slot> = (1..end).map(|at| space.slot(at)).collect();
            self.arena.source_faces.slots.write(ctx, 1, &slots);
            self.segments.slots.write(ctx, 1, &slots);
            self.arena.table.write(ctx, &texels(0..end));
        }

        let draws = space.draws();
        self.arena.source_faces.slots.set_draws(&draws);
        self.segments.slots.set_draws(&draws);
        self.arena.space.written = if all {
            u32::MAX
        } else {
            change.slots.iter().map(|range| range.len() as u32).sum()
        };
        // the tile lists hold the instances' triangles too, so they are built again
        self.arena.tiles.invalidate();
        self.objects.geometry_changed();
    }
}
// --8<-- [end:instanced-gpu]

// --8<-- [start:instanced-pass]
/// The pass that keeps the instance slots following the arena.
pub struct Instanced;

impl super::lane::Lane for Instanced {}

impl super::pass::Pass for Instanced {
    fn prepare(&mut self, g: &mut super::Gpu, _encoder: &mut wgpu::CommandEncoder) {
        g.follow_arena();
    }
}

/// The instancing pass.
pub fn pass(
    _ctx: &GpuCtx,
    _target: crate::engine::pipelines::Target,
) -> Box<dyn super::pass::Pass> {
    Box::new(Instanced)
}
// --8<-- [end:instanced-pass]
