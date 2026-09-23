use super::buffers::{GpuCtx, GrowBuf, ROWS, bind_group};
use super::instance::Instance;
use super::targets::Targets;
use crate::engine::pipelines::Layouts;
use session_rust::{AABB, Point, Xform};

/// Smallest camera drift, world units, that moves the anchor.
const REANCHOR_MIN: f64 = 1.0e3;

/// Largest camera drift, world units, before the anchor must move.
const REANCHOR_MAX: f64 = 1.0e5;

/// Least time between two anchor moves, ms.
const REANCHOR_THROTTLE_MS: f64 = 200.0;

/// One object row as the CPU builds it.
#[derive(Clone)]
pub struct ObjectRow {
    pub place: Xform,
    pub color: [f32; 4], // rgba tint
    pub flags: u32, // Instance::FLAG_* bits
    pub bounds: AABB, // box in the object's own space
    pub spacing: f32, // vertex spacing, or point size for clouds
    pub faces: bool, // true when the object drew faces
}

impl ObjectRow {
    /// A row with a placement and flags, everything else empty.
    pub fn new(place: Xform, flags: u32) -> Self {
        Self {
            place,
            color: [1.0; 4],
            flags,
            bounds: AABB::empty(),
            spacing: 0.0,
            faces: false,
        }
    }
}

/// Object rows of one upload.
#[derive(Default)]
pub struct ObjectRows {
    pub rows: Vec<ObjectRow>,
}

/// Result of a `rebase_anchor` call.
pub struct Rebase {
    pub anchor: Point, // current scene origin
    pub moved: bool, // true when the table was rebuilt now
    pub pending: bool, // true when a rebuild waits on the throttle
}

/// A row with faces and its world box, for the inside test.
struct BoundedRow {
    row: u32,
    lo: [f64; 3], // box minimum
    hi: [f64; 3], // box maximum
}

/// The row's box in world space.
fn world_box(r: &ObjectRow) -> AABB {
    r.bounds.transformed(&r.place)
}

// --8<-- [start:step-18a]
/// Translation relative to the origin, as the GPU reads it.
fn anchored(t: [f64; 3], origin: &Point) -> [f32; 4] {
    [
        (t[0] - origin[0]) as f32,
        (t[1] - origin[1]) as f32,
        (t[2] - origin[2]) as f32,
        0.0,
    ]
}

/// The object rows on the GPU and their exact positions on the CPU.
pub struct InstanceTable {
    geometry_revision: u64, // bumps when anything moves or hides
    rows: Vec<Instance>,
    translation: Vec<[f64; 3]>, // exact world position per row
    // --8<-- [end:step-18a]
    bounded: Vec<BoundedRow>, // rows with faces, for the inside test
    world_bounds: Vec<AABB>, // box per row in world space
    last_origin: Option<Point>, // origin the GPU positions are measured from
    buffer: GrowBuf, // Instance rows on the GPU
    translations: GrowBuf, // positions minus the scene origin, on the GPU
    last_rebase_ms: f64, // when the origin last moved
    pub group: wgpu::BindGroup, // group 2: rows and translations
    pub ink_group: wgpu::BindGroup, // group 2 for ink, with depth textures
}

/// Bind group 2: rows at binding 0, translations at 1.
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

/// Textures and tiles the ink bind group reads.
pub struct InkScene<'a> {
    // --8<-- [start:step-18b]
    pub tiles: &'a super::triangle_tiles::TriangleTiles, // screen tiles for visibility tests
    pub targets: &'a Targets, // depth and gradient textures
}

/// Bind group 2 for ink lanes: rows, depth, gradient, tiles.
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
    pub fn geometry_revision(&self) -> u64 {
        self.geometry_revision
    }

// --8<-- [end:step-18b]
    pub fn allocated_bytes(&self) -> u64 {
        self.buffer.buf.size() + self.translations.buf.size()
    }

    pub fn new(ctx: &GpuCtx, l: &Layouts, scene: &InkScene) -> Self {
        let buffer = GrowBuf::new(
            ctx,
            "instance.buffer",
            std::mem::size_of::<Instance>() as u64,
            ROWS,
        );
        let translations = GrowBuf::new(ctx, "instance.translations", 16, ROWS);
        let group = instance_group(ctx, l, &buffer.buf, &translations.buf);
        // --8<-- [start:step-18c]
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
            // --8<-- [end:step-18c]
            rows: vec![Instance::placeholder()],
            translation: Vec::new(),
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

    /// Rebuild the ink bind group.
    pub fn rebind_ink(&mut self, ctx: &GpuCtx, l: &Layouts, scene: &InkScene) {
        // --8<-- [start:step-18d]
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
        // --8<-- [end:step-18d]
    }

    /// An ink bind group over the pick pass's own depth textures.
    pub fn pick_group(
        &self,
        ctx: &GpuCtx,
        layouts: &Layouts,
        depths: [&wgpu::TextureView; 2],
        gradients: [&wgpu::TextureView; 2],
        // --8<-- [start:step-18e]
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

    /// Append one upload's rows.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ObjectRows) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);

        // first upload replaces the placeholder
// --8<-- [end:step-18e]
        if self.translation.is_empty() {
            self.rows.clear();
            self.world_bounds.clear();
            self.buffer.reset();
            self.translations.reset();
        }

        // row number of the first new object
        let base = self.translation.len() as u32;
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        self.world_bounds.reserve(up.rows.len());

        for (i, r) in up.rows.iter().enumerate() {
            let world = world_box(r);

            // rows with faces join the inside test
            if r.faces && world.is_valid() {
                let lo = [
                    world.min_point()[0],
                    world.min_point()[1],
                    world.min_point()[2],
                ];
                let hi = [
                    world.max_point()[0],
                    world.max_point()[1],
                    world.max_point()[2],
                ];
                self.bounded.push(BoundedRow {
                    row: base + i as u32,
                    lo,
                    hi,
                });
            }

            self.world_bounds.push(if world.is_valid() {
                world
            } else {
                AABB::empty()
            });
            self.translation
                .push([r.place.m[12], r.place.m[13], r.place.m[14]]);
            // matrix without translation
            let mut model = r.place.to_f32();
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

        // rows not yet on the GPU
        let fresh = &self.rows[self.buffer.len() as usize..];

        if fresh.is_empty() {
            return;
        }

        // translations are filled by the next rebase
        let zeros = vec![[0.0f32; 4]; fresh.len()];
        let grew = self.buffer.append(ctx, fresh);

        if self.translations.append(ctx, &zeros) || grew {
            self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
        }

        self.last_origin = None;
    }

    /// Move the scene origin when the camera drifted far enough.
    pub fn rebase_anchor(
        &mut self,
        ctx: &GpuCtx,
        origin: &Point,
        view_dist: f64,
        now: f64,
    ) -> Rebase {
        // a quarter of the view distance, clamped
        let thresh = (view_dist * 0.25).clamp(REANCHOR_MIN, REANCHOR_MAX);
        // drifted past the threshold?
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let d = [a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]];
                (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() > thresh
            }
        };
        // rebuild now unless throttled
        let moved = need
            && (self.last_origin.is_none() || now - self.last_rebase_ms > REANCHOR_THROTTLE_MS);

        if moved {
            self.rebuild(ctx, origin);
            self.last_rebase_ms = now;
        }

        Rebase {
            // set by rebuild above or earlier
            anchor: self.last_origin.clone().unwrap(),
            moved,
            pending: need && !moved,
        }
    }

    /// Recompute every relative translation for a new origin.
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        // --8<-- [start:step-18f]
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        self.last_origin = Some(origin.clone());
        let mut rebased: Vec<[f32; 4]> = Vec::with_capacity(self.rows.len());

        for t in &self.translation {
            rebased.push(anchored(*t, origin));
        }

        rebased.resize(self.rows.len(), [0.0; 4]);
        self.translations.write_at(ctx, 0, &rebased);
        // --8<-- [end:step-18f]
    }

    /// Row `i`'s full matrix as the shader composes it.
    pub fn anchored_model(&self, i: u32) -> Option<[f32; 16]> {
        let mut model = self.rows.get(i as usize)?.model;

        if let (Some(t), Some(o)) = (self.translation.get(i as usize), &self.last_origin) {
            // --8<-- [start:step-18g]
            let a = anchored(*t, o);
            model[12] = a[0];
            model[13] = a[1];
            model[14] = a[2];
            // --8<-- [end:step-18g]
        }

        Some(model)
    }

    /// Set FLAG_INSIDE on rows whose box contains the eye.
    pub fn update_inside(&mut self, ctx: &GpuCtx, eye: [f32; 3], scene: &AABB) {
        if self.bounded.is_empty() {
            return;
        }

        let Some(origin) = self.last_origin.clone() else {
            return;
        };
        // eye in world space
        let ew = [
            origin[0] + eye[0] as f64,
            origin[1] + eye[1] as f64,
            origin[2] + eye[2] as f64,
        ];
        let in_scene = scene.contains(&Point::new(ew[0], ew[1], ew[2]));

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

            // write only when the answer changed
            if (row.flags & Instance::FLAG_INSIDE != 0) == inside {
                continue;
            }

            row.flags ^= Instance::FLAG_INSIDE;
            self.buffer.write_at(ctx, b.row, std::slice::from_ref(row));
        }
    }

    /// Set or clear one flag bit on one row.
    pub fn set_flag(&mut self, ctx: &GpuCtx, row: u32, bit: u32, on: bool) {
        let Some(r) = self.rows.get_mut(row as usize) else {
            return;
        };
        let was = r.flags & bit != 0;

        if was == on {
            return;
        }

        r.flags ^= bit;
// --8<-- [start:step-18h]

        if bit & Instance::FLAG_HIDDEN != 0 {
            self.geometry_revision = self.geometry_revision.wrapping_add(1);
        }

        self.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }

    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        // --8<-- [end:step-18h]
        self.rows.clear();
        self.translation.clear();
        self.bounded.clear();
        self.world_bounds.clear();
        self.buffer.reset();
        self.translations.reset();
        self.last_origin = None;
    }

    /// Forget every row and free the memory.
    pub fn release(&mut self, ctx: &GpuCtx, l: &Layouts) {
        self.reset();
        self.rows.shrink_to_fit();
        self.translation.shrink_to_fit();
        self.bounded.shrink_to_fit();
        self.world_bounds.shrink_to_fit();
        self.rows.push(Instance::placeholder());
        self.buffer.release(ctx);
        self.translations.release(ctx);
        self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
    }

    /// Row `i` as uploaded.
    pub fn row(&self, i: u32) -> Option<&Instance> {
        self.rows.get(i as usize)
    }

    /// Row count.
    pub fn len(&self) -> u32 {
        self.rows.len() as u32
    }

    /// World box of a row, None when it has none.
    pub fn row_bounds(&self, row: u32) -> Option<AABB> {
        let b = *self.world_bounds.get(row as usize)?;
        b.is_valid().then_some(b)
    }

    /// Set the world box of a text row.
    pub fn set_text_bounds(&mut self, row: u32, bounds: AABB) {
        if let Some(target) = self.world_bounds.get_mut(row as usize) {
            *target = bounds;
        }
    }

    /// Scene origin in f64.
    pub fn anchor(&self) -> [f64; 3] {
        match &self.last_origin {
            Some(point) => [point[0], point[1], point[2]],
            None => [0.0; 3],
        }
    }

    /// Scene origin in f32; zero before the first frame.
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

    #[test]
    fn world_box_translates() {
        // --8<-- [start:step-18i]
        let mut r = ObjectRow::new(Xform::translation(10.0, 20.0, 30.0), 0);
        r.bounds = AABB::new(0.5, 1.0, 1.5, 0.5, 1.0, 1.5);
        let b = world_box(&r);
        assert_eq!(b.min_point(), Point::new(10.0, 20.0, 30.0));
        assert_eq!(b.max_point(), Point::new(11.0, 22.0, 33.0));
    }

    /// A row with no local box stays empty, translated or not.
    #[test]
    fn world_box_empty_stays_empty() {
        let r = ObjectRow::new(Xform::translation(10.0, 20.0, 30.0), 0);
        assert!(!world_box(&r).is_valid());
    }

    /// A tiny move far from zero survives only relative to a near origin.
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

    /// A move written to the f32 value is lost at the next rebase.
    #[test]
    fn an_edit_written_past_the_base_does_not_survive_a_rebase() {
        let base = [1.0e4, 0.0, 0.0];
        let first = Point::new(0.0, 0.0, 0.0);
        let step = 0.25_f32;

        // edit the relative value only
        let edited = anchored(base, &first)[0] + step;
        assert_eq!(edited, 1.0e4 + 0.25);

        // the rebase never saw the edit
        let second = Point::new(1.0e3, 0.0, 0.0);
        assert_eq!(anchored(base, &second)[0], 9.0e3);
        // --8<-- [end:step-18i]
    }
}
