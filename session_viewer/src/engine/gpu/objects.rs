//! `ObjectRows` - the per-object columns ONE upload carries (a delta, dropped after upload) -
//! and `InstanceTable`, the ONE owner of the object rows the GPU reads: the rows, their f64
//! translations, the sparse bounded rows, the re-anchor, the inside test, the two buffers and
//! their bind group.

use super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group};
use super::instance::Instance;
use super::targets::Targets;
use crate::engine::pipelines::Layouts;
use crate::math::{Aabb, Mat4, mat_to_f32};
use session_rust::Point;

/// Re-anchor threshold band, world units: the table is rebased once the camera target drifts
/// a quarter of the view distance from the anchor, clamped to [MIN, MAX].
const REANCHOR_MIN: f64 = 1.0e3;
const REANCHOR_MAX: f64 = 1.0e5;

/// Re-anchors are throttled to this interval so a wheel-zoom gesture does not rebuild every tick.
const REANCHOR_THROTTLE_MS: f64 = 200.0;

/// One object as the walk reports it: its true placement, tint, flags, mesh-local box
/// (empty when the object has no volume the ink lanes care about) and vertex spacing.
#[derive(Clone)]
pub struct ObjectRow {
    pub place: Mat4,
    pub color: [f32; 4],
    pub flags: u32,
    pub bounds: Aabb,
    /// Meshes: the local vertex spacing; clouds: the point size in px. Read as a pen hint.
    pub spacing: f32,
    /// The row drew faces, so the per-frame inside test walks its box.
    pub faces: bool,
}

impl ObjectRow {
    /// A row with the file placement, white tint and no columns filled yet.
    pub fn new(place: Mat4, flags: u32) -> Self {
        Self {
            place,
            color: [1.0; 4],
            flags,
            bounds: Aabb::empty(),
            spacing: 0.0,
            faces: false,
        }
    }
}

/// The object rows of one upload - THIS upload's rows only; `Scene.bases.obj` numbers them.
#[derive(Default)]
pub struct ObjectRows {
    pub rows: Vec<ObjectRow>,
}

/// What one `rebase_anchor` call reports: the anchor in force, whether the table was just
/// rebuilt, and whether a rebuild is due but throttled (the caller asks for another frame).
pub struct Rebase {
    pub anchor: Point,
    pub moved: bool,
    pub pending: bool,
}

/// A row that drew faces and carries a world box. The inside test walks these only.
struct BoundedRow {
    row: u32,
    lo: [f64; 3],
    hi: [f64; 3],
}

/// The row's mesh-local box carried through its placement, in world units.
fn world_box(r: &ObjectRow) -> Aabb {
    r.bounds.placed(&r.place)
}

/// One row's GPU form under a placement: the model matrix with its translation column cleared,
/// the true f64 translation taken out of that column, and the row's own box placed into the
/// world. Split out of `set_placement` because it is the whole arithmetic of a move and a
/// device is not needed to check it.
fn placed_row(local: &Aabb, place: &Mat4) -> ([f32; 16], [f64; 3], Aabb) {
    let world = local.placed(place);
    let mut model = mat_to_f32(place);
    model[12] = 0.0;
    model[13] = 0.0;
    model[14] = 0.0;
    (
        model,
        [place[12], place[13], place[14]],
        if world.is_finite() {
            world
        } else {
            Aabb::empty()
        },
    )
}

/// The anchored translation the GPU reads: the true f64 world position minus the table's
/// anchor, narrowed once at the end. Subtracting first is what keeps a small move: a
/// millimetre is lost narrowing a world coordinate a kilometre out, and kept narrowing the
/// ten metres that remain after the anchor comes off.
///
/// It is a pure function of the base and the anchor, which fixes where a placement CHANGE
/// goes. `rebuild` recomputes every row from `translation`, so a delta added to the f32 this
/// returned is erased by the next re-anchor. An edit adds its increment to the f64 base.
/// `render_position` is the same boundary for geometry arriving from the kernel; this is the
/// boundary for placement, and an edit crosses it in the other direction.
fn anchored(t: [f64; 3], origin: &Point) -> [f32; 4] {
    [
        (t[0] - origin[0]) as f32,
        (t[1] - origin[1]) as f32,
        (t[2] - origin[2]) as f32,
        0.0,
    ]
}

/// The object rows as the GPU sees them, the TRUE f64 translation per row, and the sparse
/// bounded rows. The anchored translations live in their own 16 B/row buffer.
pub struct InstanceTable {
    geometry_revision: u64,
    rows: Vec<Instance>,
    /// The TRUE world translation per row, in f64. The GPU never sees it: `anchored` narrows
    /// it against the current anchor. Every change to a placement is applied HERE, so the
    /// error of an edit is the error of the edit, not of the position it happens at.
    translation: Vec<[f64; 3]>,
    /// Each row's box in its OWN space. `world_bounds` is this box under the current
    /// placement, so an edit that only moves the object recomputes the world box from here
    /// instead of walking the geometry again.
    local_bounds: Vec<Aabb>,
    bounded: Vec<BoundedRow>,
    /// Every row's world box, row order, so index = row; `Aabb::empty()` where not finite.
    world_bounds: Vec<Aabb>,
    last_origin: Option<Point>,
    buffer: GrowBuf,
    translations: GrowBuf,
    last_rebase_ms: f64,
    /// Group 2 of every instance-reading pipeline; rebuilt when either buffer grows.
    pub group: wgpu::BindGroup,
    pub ink_group: wgpu::BindGroup,
}

/// Group 2: the rows at binding 0, the anchored translations at binding 1.
fn instance_group(
    ctx: &GpuCtx,
    l: &Layouts,
    rows: &wgpu::Buffer,
    translations: &wgpu::Buffer,
) -> wgpu::BindGroup {
    bind_group(
        ctx,
        &l.instance,
        "instances.bind_group",
        &[rows, translations],
    )
}

/// The immutable physical depth bound beside each ink lane's instance columns.
pub struct InkScene<'a> {
    pub tiles: &'a super::triangle_tiles::TriangleTiles,
    pub targets: &'a Targets,
}

/// Group 2 for ink: the instance columns, the physical depth and gradient at both sample
/// counts, and the finite-visibility tables.
fn ink_instance_group(
    ctx: &GpuCtx,
    l: &Layouts,
    label: &str,
    buffers: [&wgpu::Buffer; 2],
    depths: [&wgpu::TextureView; 2],
    gradients: [&wgpu::TextureView; 2],
    tiles: &super::triangle_tiles::TriangleTiles,
) -> wgpu::BindGroup {
    let view = wgpu::BindingResource::TextureView;
    let entries = [
        buffers[0].as_entire_binding(),
        buffers[1].as_entire_binding(),
        view(depths[0]),
        view(depths[1]),
        view(gradients[0]),
        view(gradients[1]),
        tiles.projected.as_entire_binding(),
        tiles.buffer.as_entire_binding(),
    ];
    let entries: Vec<wgpu::BindGroupEntry> = entries
        .into_iter()
        .enumerate()
        .map(|(binding, resource)| wgpu::BindGroupEntry {
            binding: binding as u32,
            resource,
        })
        .collect();
    ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &l.ink_instance,
        entries: &entries,
    })
}

impl InstanceTable {
    /// Revision of placement, rebased translations or hidden state used by finite visibility.
    pub fn geometry_revision(&self) -> u64 {
        self.geometry_revision
    }

    /// Application-owned buffer allocation capacity in bytes; excludes driver overhead.
    pub fn allocated_bytes(&self) -> u64 {
        self.buffer.buf.size() + self.translations.buf.size()
    }

    /// One placeholder row in both tables, so the first frame binds real buffers.
    pub fn new(ctx: &GpuCtx, l: &Layouts, scene: &InkScene) -> Self {
        let buffer = GrowBuf::new(
            ctx,
            "instance.buffer",
            std::mem::size_of::<Instance>() as u64,
            ROWS,
        );
        let translations = GrowBuf::new(ctx, "instance.translations", 16, ROWS);
        let group = instance_group(ctx, l, &buffer.buf, &translations.buf);
        let t = scene.targets;
        let ink_group = ink_instance_group(
            ctx,
            l,
            "ink.instances.bind_group",
            [&buffer.buf, &translations.buf],
            [&t.depth_single, &t.depth_msaa],
            [&t.gradient_single, &t.gradient_msaa],
            scene.tiles,
        );

        Self {
            geometry_revision: 0,
            rows: vec![Instance::placeholder()],
            translation: Vec::new(),
            local_bounds: Vec::new(),
            bounded: Vec::new(),
            world_bounds: Vec::new(),
            last_origin: None,
            buffer,
            translations,
            last_rebase_ms: 0.0,
            group,
            ink_group,
        }
    }

    /// Refresh the depth and instance bindings after upload, resize, or release.
    pub fn rebind_ink(&mut self, ctx: &GpuCtx, l: &Layouts, scene: &InkScene) {
        let t = scene.targets;
        self.ink_group = ink_instance_group(
            ctx,
            l,
            "ink.instances.bind_group",
            [&self.buffer.buf, &self.translations.buf],
            [&t.depth_single, &t.depth_msaa],
            [&t.gradient_single, &t.gradient_msaa],
            scene.tiles,
        );
    }

    /// Bind pixel-center picking depth separately from the multisampled display depth.
    pub fn pick_group(
        &self,
        ctx: &GpuCtx,
        layouts: &Layouts,
        depths: [&wgpu::TextureView; 2],
        gradients: [&wgpu::TextureView; 2],
        tiles: &super::triangle_tiles::TriangleTiles,
    ) -> wgpu::BindGroup {
        ink_instance_group(
            ctx,
            layouts,
            "pick.instances",
            [&self.buffer.buf, &self.translations.buf],
            depths,
            gradients,
            tiles,
        )
    }

    /// Append one upload's rows: cast once, keep the f64 translation, note the bounded ones,
    /// send only the new rows. The next frame rebases the whole table.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ObjectRows) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        if self.translation.is_empty() {
            self.rows.clear();
            self.world_bounds.clear();
            self.local_bounds.clear();
            self.buffer.reset();
            self.translations.reset();
        }
        let base = self.translation.len() as u32;
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        self.world_bounds.reserve(up.rows.len());
        self.local_bounds.reserve(up.rows.len());
        for (i, r) in up.rows.iter().enumerate() {
            let world = world_box(r);
            if r.faces && world.is_finite() {
                let lo = [
                    world.min[0] as f64,
                    world.min[1] as f64,
                    world.min[2] as f64,
                ];
                let hi = [
                    world.max[0] as f64,
                    world.max[1] as f64,
                    world.max[2] as f64,
                ];
                self.bounded.push(BoundedRow {
                    row: base + i as u32,
                    lo,
                    hi,
                });
            }
            self.world_bounds.push(if world.is_finite() {
                world
            } else {
                Aabb::empty()
            });
            self.local_bounds.push(r.bounds);
            self.translation
                .push([r.place[12], r.place[13], r.place[14]]);
            let mut model = mat_to_f32(&r.place);
            model[12] = 0.0;
            model[13] = 0.0;
            model[14] = 0.0;
            self.rows.push(Instance {
                model,
                color: r.color,
                flags: r.flags,
                _pad0: 0.0,
                spacing: r.spacing,
                _pad: 0,
            });
        }
        if self.rows.is_empty() {
            self.rows.push(Instance::placeholder());
        }

        let fresh = &self.rows[self.buffer.len() as usize..];
        if fresh.is_empty() {
            return;
        }
        let zeros = vec![[0.0f32; 4]; fresh.len()];
        let grew = self.buffer.append(ctx, fresh);
        if self.translations.append(ctx, &zeros) || grew {
            self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
        }
        self.last_origin = None;
    }

    /// The anchor the table is rebased about. A rebuild runs only when the camera target
    /// strays past the band from the current anchor; `origin` and `view_dist` are world units.
    pub fn rebase_anchor(
        &mut self,
        ctx: &GpuCtx,
        origin: &Point,
        view_dist: f64,
        now: f64,
    ) -> Rebase {
        let thresh = (view_dist * 0.25).clamp(REANCHOR_MIN, REANCHOR_MAX);
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let d = [a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]];
                (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() > thresh
            }
        };
        let moved = need
            && (self.last_origin.is_none() || now - self.last_rebase_ms > REANCHOR_THROTTLE_MS);
        if moved {
            self.rebuild(ctx, origin);
            self.last_rebase_ms = now;
        }
        Rebase {
            // Safe in one step: `last_origin` being None makes `need` true and `moved` true,
            // so `rebuild` above has just filled it. Otherwise it was already filled.
            anchor: self.last_origin.clone().unwrap(),
            moved,
            pending: need && !moved,
        }
    }

    /// Rebase every row's translation around `origin` in f64, cast, and rewrite the 16 B/row
    /// translation table; the 96 B rows are not touched.
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        self.last_origin = Some(origin.clone());
        let mut rebased: Vec<[f32; 4]> = Vec::with_capacity(self.rows.len());
        for t in &self.translation {
            rebased.push(anchored(*t, origin));
        }
        rebased.resize(self.rows.len(), [0.0; 4]);
        self.translations.write_at(ctx, 0, &rebased);
    }

    /// Row `i`'s model as a shader composes it: rotation/scale plus the anchored translation.
    pub fn anchored_model(&self, i: u32) -> Option<[f32; 16]> {
        let mut model = self.rows.get(i as usize)?.model;
        if let (Some(t), Some(o)) = (self.translation.get(i as usize), &self.last_origin) {
            let a = anchored(*t, o);
            model[12] = a[0];
            model[13] = a[1];
            model[14] = a[2];
        }
        Some(model)
    }

    /// Per-frame refresh of `FLAG_INSIDE` over the bounded rows only; a row is written back
    /// only when its answer flips.
    pub fn update_inside(&mut self, ctx: &GpuCtx, eye: [f32; 3], scene: &Aabb) {
        if self.bounded.is_empty() {
            return;
        }
        let Some(origin) = self.last_origin.clone() else {
            return;
        };
        let ew = [
            origin[0] + eye[0] as f64,
            origin[1] + eye[1] as f64,
            origin[2] + eye[2] as f64,
        ];
        let in_scene = scene.contains(ew);
        for b in &self.bounded {
            let mut inside = in_scene;
            if inside {
                for (coordinate, (low, high)) in ew.iter().zip(b.lo.iter().zip(&b.hi)) {
                    if !(coordinate >= low && coordinate <= high) {
                        inside = false;
                        break;
                    }
                }
            }
            let Some(row) = self.rows.get_mut(b.row as usize) else {
                continue;
            };
            if (row.flags & Instance::FLAG_INSIDE != 0) == inside {
                continue;
            }
            row.flags ^= Instance::FLAG_INSIDE;
            self.buffer.write_at(ctx, b.row, std::slice::from_ref(row));
        }
    }

    /// Replace one row's placement and write back only that row.
    ///
    /// Two small writes - 96 B of instance and 16 B of anchored translation - so a drag frame
    /// costs the same whether the scene holds one object or a million. The true f64 translation
    /// is what changes; the anchored f32 the GPU reads is derived from it, because a delta
    /// written straight into the f32 is erased by the next re-anchor.
    ///
    /// The world box is recomputed from the row's own box, and `bounded` (the sparse list the
    /// inside test walks) is kept in step. Returns false when the row does not exist.
    pub fn set_placement(&mut self, ctx: &GpuCtx, row: u32, place: &Mat4) -> bool {
        let i = row as usize;
        let (Some(instance), Some(local)) = (self.rows.get_mut(i), self.local_bounds.get(i)) else {
            return false;
        };
        let (model, translation, world) = placed_row(local, place);
        instance.model = model;
        self.translation[i] = translation;
        self.world_bounds[i] = world;
        for b in &mut self.bounded {
            if b.row == row {
                b.lo = [world.min[0] as f64, world.min[1] as f64, world.min[2] as f64];
                b.hi = [world.max[0] as f64, world.max[1] as f64, world.max[2] as f64];
            }
        }
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        let instance = *instance;
        self.buffer.write_at(ctx, row, std::slice::from_ref(&instance));
        if let Some(origin) = &self.last_origin {
            let t = anchored(self.translation[i], origin);
            self.translations.write_at(ctx, row, std::slice::from_ref(&t));
        }
        true
    }

    /// Set or clear one flag bit on one row and write that row back.
    pub fn set_flag(&mut self, ctx: &GpuCtx, row: u32, bit: u32, on: bool) {
        let Some(r) = self.rows.get_mut(row as usize) else {
            return;
        };
        let was = r.flags & bit != 0;
        if was == on {
            return;
        }
        r.flags ^= bit;
        if bit & Instance::FLAG_HIDDEN != 0 {
            self.geometry_revision = self.geometry_revision.wrapping_add(1);
        }
        self.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }

    /// Forget every row; the buffers keep their capacity.
    pub fn reset(&mut self) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        self.rows.clear();
        self.translation.clear();
        self.local_bounds.clear();
        self.bounded.clear();
        self.world_bounds.clear();
        self.buffer.reset();
        self.translations.reset();
        self.last_origin = None;
    }

    /// Forget every row AND hand the memory back, both sides.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.reset();
        self.rows.shrink_to_fit();
        self.translation.shrink_to_fit();
        self.local_bounds.shrink_to_fit();
        self.bounded.shrink_to_fit();
        self.world_bounds.shrink_to_fit();
        self.rows.push(Instance::placeholder());
        self.buffer.release(ctx);
        self.translations.release(ctx);
        self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
    }

    /// One instance row as the GPU sees it.
    pub fn row(&self, i: u32) -> Option<&Instance> {
        self.rows.get(i as usize)
    }

    /// Rows in the table - the frame's object count.
    pub fn len(&self) -> u32 {
        self.rows.len() as u32
    }

    /// Row `row`'s world box, `None` when the row has no volume (or does not exist).
    pub fn row_bounds(&self, row: u32) -> Option<Aabb> {
        let b = *self.world_bounds.get(row as usize)?;
        b.is_finite().then_some(b)
    }

    /// Text has no solid volume; its shaped world box still supports fitting and annotations.
    pub fn set_text_bounds(&mut self, row: u32, bounds: Aabb) {
        if let Some(target) = self.world_bounds.get_mut(row as usize) {
            *target = bounds;
        }
    }

    /// The precise origin required to project source-world labels into the rebased frame.
    pub fn anchor(&self) -> [f64; 3] {
        match &self.last_origin {
            Some(point) => [point[0], point[1], point[2]],
            None => [0.0; 3],
        }
    }

    /// The anchor the rows are rebased about, as the shaders read it; zero before the first frame.
    pub fn anchor_f32(&self) -> [f32; 3] {
        match &self.last_origin {
            Some(origin) => [origin[0] as f32, origin[1] as f32, origin[2] as f32],
            None => [0.0; 3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Xform;

    /// A translated local box lands at the translated world position.
    #[test]
    fn world_box_translates() {
        let mut r = ObjectRow::new(Xform::translation(10.0, 20.0, 30.0).m, 0);
        r.bounds = Aabb {
            min: [0.0, 0.0, 0.0],
            max: [1.0, 2.0, 3.0],
        };
        let b = world_box(&r);
        assert_eq!(b.min, [10.0, 20.0, 30.0]);
        assert_eq!(b.max, [11.0, 22.0, 33.0]);
    }

    /// A row with no local box stays empty, translated or not.
    #[test]
    fn world_box_empty_stays_empty() {
        let r = ObjectRow::new(Xform::translation(10.0, 20.0, 30.0).m, 0);
        assert!(!world_box(&r).is_finite());
    }

    /// A millimetre move a kilometre out survives subtraction against a near anchor and does
    /// not survive narrowing the world coordinate itself. This is what the anchor is for.
    #[test]
    fn the_anchor_is_what_keeps_a_small_move() {
        let world = 1.0e6_f64;
        let step = 1.0e-3_f64;
        assert_eq!((world + step) as f32, world as f32);

        let near = Point::new(world - 1.0e4, 0.0, 0.0);
        let before = anchored([world, 0.0, 0.0], &near)[0];
        let after = anchored([world + step, 0.0, 0.0], &near)[0];
        assert_ne!(after, before);
    }

    /// A move rewrites the f64 translation and the world box, and leaves the model matrix's
    /// translation column at zero: the GPU adds the anchored translation itself, and a matrix
    /// carrying the position twice would draw the object at twice its distance.
    #[test]
    fn a_move_goes_into_the_translation_not_the_matrix() {
        let local = Aabb {
            min: [-1.0, -1.0, -1.0],
            max: [1.0, 1.0, 1.0],
        };
        let place = Xform::translation(10.0, 20.0, 30.0).m;
        let (model, translation, world) = placed_row(&local, &place);

        assert_eq!([model[12], model[13], model[14]], [0.0, 0.0, 0.0]);
        assert_eq!(translation, [10.0, 20.0, 30.0]);
        assert_eq!(world.min, [9.0, 19.0, 29.0]);
        assert_eq!(world.max, [11.0, 21.0, 31.0]);
    }

    /// A row with no volume keeps an empty box rather than an infinite one, so the inside test
    /// and the fit never walk a box that answers yes to everything.
    #[test]
    fn a_row_with_no_box_stays_empty() {
        let (_, _, world) = placed_row(&Aabb::empty(), &Xform::translation(1.0, 0.0, 0.0).m);
        assert!(!world.is_finite());
    }

    /// The anchored value is a pure function of the f64 base and the anchor, so a placement
    /// change written anywhere else is erased by the next re-anchor. An edit writes
    /// `translation`; it never adds its delta to the f32 the GPU was given.
    #[test]
    fn an_edit_written_past_the_base_does_not_survive_a_rebase() {
        let base = [1.0e4, 0.0, 0.0];
        let first = Point::new(0.0, 0.0, 0.0);
        let step = 0.25_f32;

        // An edit applied to the anchored value only.
        let edited = anchored(base, &first)[0] + step;
        assert_eq!(edited, 1.0e4 + 0.25);

        // The next re-anchor recomputes from `translation`, which never saw it.
        let second = Point::new(1.0e3, 0.0, 0.0);
        assert_eq!(anchored(base, &second)[0], 9.0e3);
    }
}
