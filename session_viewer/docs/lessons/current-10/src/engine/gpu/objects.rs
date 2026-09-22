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
    pub place: Xform, // world placement
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
    pub rows: Vec<ObjectRow>, // one per object
}

/// Result of a `rebase_anchor` call.
pub struct Rebase {
    pub anchor: Point, // current scene origin
    pub moved: bool, // true when the table was rebuilt now
    pub pending: bool, // true when a rebuild waits on the throttle
}

/// A row with faces and its world box, for the inside test.
struct BoundedRow {
    row: u32, // object row
    lo: [f64; 3], // box minimum
    hi: [f64; 3], // box maximum
}

/// The row's box in world space.
fn world_box(r: &ObjectRow) -> AABB {
    r.bounds.transformed(&r.place)
}

/// Split a placement into matrix, translation and world box.
fn placed_row(local: &AABB, place: &Xform) -> ([f32; 16], [f64; 3], AABB) {
    let world = local.transformed(place);
    let mut model = place.to_f32();
    // translation goes in its own table
    model[12] = 0.0;
    model[13] = 0.0;
    model[14] = 0.0;
    (
        model,
        [place.m[12], place.m[13], place.m[14]],
        if world.is_valid() {
            world
        } else {
            AABB::empty()
        },
    )
}

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
    rows: Vec<Instance>, // the rows, as uploaded
    translation: Vec<[f64; 3]>, // exact world position per row
    local_bounds: Vec<AABB>, // box per row, in the object's own space
    widget: Option<u32>, // identity row the gumball draws with
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
            widget: None,
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

    /// An ink bind group over the pick pass's own depth textures.
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

    /// Append one upload's rows.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ObjectRows) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);

        // drop the widget row first; it must stay last
        if let Some(widget) = self.widget.take() {
            let keep = widget as usize;
            self.rows.truncate(keep);
            self.translation.truncate(keep);
            self.local_bounds.truncate(keep);
            self.world_bounds.truncate(keep);
            // rewind both buffers by re-appending the kept rows
            self.buffer.reset();
            self.translations.reset();

            if keep > 0 {
                let rows: Vec<Instance> = self.rows.clone();
                let anchored_rows: Vec<[f32; 4]> = match &self.last_origin {
                    Some(origin) => self
                        .translation
                        .iter()
                        .map(|t| anchored(*t, origin))
                        .collect(),
                    None => vec![[0.0f32; 4]; keep],
                };
                let grew = self.buffer.append(ctx, &rows);

                if self.translations.append(ctx, &anchored_rows) || grew {
                    self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
                }
            }
        }

        // first upload replaces the placeholder
        if self.translation.is_empty() {
            self.rows.clear();
            self.world_bounds.clear();
            self.local_bounds.clear();
            self.buffer.reset();
            self.translations.reset();
        }

        // row number of the first new object
        let base = self.translation.len() as u32;
        self.rows.reserve(up.rows.len());
        self.translation.reserve(up.rows.len());
        self.world_bounds.reserve(up.rows.len());
        self.local_bounds.reserve(up.rows.len());

        for (i, r) in up.rows.iter().enumerate() {
            let world = world_box(r);

            // rows with faces join the inside test
            if r.faces && world.is_valid() {
                let lo = world.min_point();
                let hi = world.max_point();
                self.bounded.push(BoundedRow {
                    row: base + i as u32,
                    lo: [lo[0], lo[1], lo[2]],
                    hi: [hi[0], hi[1], hi[2]],
                });
            }

            self.world_bounds.push(if world.is_valid() {
                world
            } else {
                AABB::empty()
            });
            self.local_bounds.push(r.bounds);
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
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        self.last_origin = Some(origin.clone());
        let mut rebased: Vec<[f32; 4]> = Vec::with_capacity(self.rows.len());

        for t in &self.translation {
            rebased.push(anchored(*t, origin));
        }

        rebased.resize(self.rows.len(), [0.0; 4]);
        self.translations.write_at(ctx, 0, &rebased);
    }

    /// Row `i`'s full matrix as the shader composes it.
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

    /// Row the gumball draws with: identity, appended once, last.
    pub fn widget_row(&mut self, ctx: &GpuCtx, l: &Layouts) -> (u32, bool) {
        if let Some(row) = self.widget {
            return (row, false);
        }

        let row = self.rows.len() as u32;
        // identity matrix, white tint
        self.rows.push(Instance {
            color: [1.0; 4],
            ..Instance::placeholder()
        });
        self.translation.push([0.0; 3]);
        self.local_bounds.push(AABB::empty());
        self.world_bounds.push(AABB::empty());
        // world position zero, relative to the origin
        let translation = match &self.last_origin {
            Some(origin) => anchored([0.0; 3], origin),
            None => [0.0; 4],
        };
        // append grows the buffers if needed
        let grew = self
            .buffer
            .append(ctx, std::slice::from_ref(&self.rows[row as usize]));
        let grew_t = self
            .translations
            .append(ctx, std::slice::from_ref(&translation));

        if grew || grew_t {
            self.group = instance_group(ctx, l, &self.buffer.buf, &self.translations.buf);
        }

        self.widget = Some(row);
        (row, grew || grew_t)
    }

    /// Move one object; writes only its row.
    pub fn set_placement(&mut self, ctx: &GpuCtx, row: u32, place: &Xform) -> bool {
        let i = row as usize;
        let (Some(instance), Some(local)) = (self.rows.get_mut(i), self.local_bounds.get(i)) else {
            return false;
        };
        let (model, translation, world) = placed_row(local, place);
        instance.model = model;
        self.translation[i] = translation;
        self.world_bounds[i] = world;

        // keep the inside-test box in step
        for b in &mut self.bounded {
            if b.row == row {
                let lo = world.min_point();
                let hi = world.max_point();
                b.lo = [lo[0], lo[1], lo[2]];
                b.hi = [hi[0], hi[1], hi[2]];
            }
        }

        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        let instance = *instance;
        self.buffer
            .write_at(ctx, row, std::slice::from_ref(&instance));

        if let Some(origin) = &self.last_origin {
            let t = anchored(self.translation[i], origin);
            self.translations
                .write_at(ctx, row, std::slice::from_ref(&t));
        }

        true
    }

    /// Set a row's box and spacing after its geometry changed.
    pub(crate) fn set_geometry_bounds(
        &mut self,
        ctx: &GpuCtx,
        row: u32,
        bounds: AABB,
        spacing: f32,
        place: &Xform,
    ) {
        self.local_bounds[row as usize] = bounds;
        self.rows[row as usize].spacing = spacing;
        self.set_placement(ctx, row, place);
    }

    /// Change display color without rebuilding geometry or placement.
    pub fn set_color(&mut self, ctx: &GpuCtx, row: u32, color: [u8; 3]) {
        if let Some(r) = self.rows.get_mut(row as usize) {
            r.color = [
                color[0] as f32 / 255.,
                color[1] as f32 / 255.,
                color[2] as f32 / 255.,
                1.,
            ];
            r.flags |= Instance::FLAG_COLOR;
            self.buffer.write_at(ctx, row, std::slice::from_ref(r));
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

        if bit & Instance::FLAG_HIDDEN != 0 {
            self.geometry_revision = self.geometry_revision.wrapping_add(1);
        }

        self.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }

    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
        self.widget = None;
        self.rows.clear();
        self.translation.clear();
        self.local_bounds.clear();
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
        self.local_bounds.shrink_to_fit();
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

    /// A move changes the translation and box, never the matrix.
    #[test]
    fn a_move_goes_into_the_translation_not_the_matrix() {
        let local = AABB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let place = Xform::translation(10.0, 20.0, 30.0);
        let (model, translation, world) = placed_row(&local, &place);

        assert_eq!([model[12], model[13], model[14]], [0.0, 0.0, 0.0]);
        assert_eq!(translation, [10.0, 20.0, 30.0]);
        assert_eq!(world.min_point(), Point::new(9.0, 19.0, 29.0));
        assert_eq!(world.max_point(), Point::new(11.0, 21.0, 31.0));
    }

    /// One row moves, its neighbour stays, the widget row stays last.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn one_row_moves_and_its_neighbour_does_not() {
        use crate::engine::gpu::{Gpu, Upload};
        let mut gpu = pollster::block_on(Gpu::new_headless(64, 64)).unwrap();
        let mut upload = Upload::default();

        for x in [0.0, 100.0] {
            let mut row = ObjectRow::new(Xform::translation(x, 0.0, 0.0), 0);
            row.bounds = AABB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
            row.faces = true;
            upload.obj.rows.push(row);
        }

        gpu.set_scene(&upload);
        assert_eq!(gpu.objects.len(), 2);

        let moved = Xform::translation(0.0, 50.0, 0.0);
        assert!(gpu.objects.set_placement(&gpu.ctx, 0, &moved));

        let first = gpu.objects.row_bounds(0).expect("row 0 has a box");
        let second = gpu.objects.row_bounds(1).expect("row 1 has a box");
        assert_eq!(first.min_point()[1], 49.0);
        assert_eq!(second.min_point()[0], 99.0);
        assert_eq!(second.min_point()[1], -1.0, "the neighbour did not move");

        // widget row comes after every object row
        let (widget, _) = gpu.objects.widget_row(&gpu.ctx, &gpu.layouts);
        assert_eq!(widget, 2);
        assert_eq!(gpu.objects.widget_row(&gpu.ctx, &gpu.layouts).0, widget);

        // a later upload drops and remints the widget row
        let mut more = Upload::default();
        more.obj
            .rows
            .push(ObjectRow::new(Xform::translation(200.0, 0.0, 0.0), 0));
        gpu.set_scene(&more);
        assert_eq!(gpu.objects.len(), 3, "two rows, one file's row, no widget");
        assert_eq!(gpu.objects.widget_row(&gpu.ctx, &gpu.layouts).0, 3);
    }

    /// A row with no box stays empty, never infinite.
    #[test]
    fn a_row_with_no_box_stays_empty() {
        let (_, _, world) = placed_row(&AABB::empty(), &Xform::translation(1.0, 0.0, 0.0));
        assert!(!world.is_valid());
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
    }
}
