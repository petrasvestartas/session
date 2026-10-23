use crate::{
    production_joined as rows,
    production_ribbon_gpu::{buffer, group, pipeline, storage, uniform},
    scene::{ObjectId, Scene},
    sheet_source::Prepared,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};
pub const SOURCE: &str = include_str!("production_sheet.wgsl");
struct Geometry {
    _page: crate::sheet_stream::DisplayPage,
    source: Rc<Prepared>,
    segments: wgpu::Buffer,
    edges: wgpu::Buffer,
    count: u32,
}
struct Draw {
    id: ObjectId,
    geometry: Rc<Geometry>,
    objects: wgpu::Buffer,
    translations: wgpu::Buffer,
    selection: wgpu::Buffer,
    picks: wgpu::Buffer,
    bytes: Vec<u8>,
}
pub(crate) struct Frame {
    groups: Vec<[wgpu::BindGroup; 4]>,
}
pub struct Lane {
    layouts: [wgpu::BindGroupLayout; 4],
    shader: RefCell<Option<wgpu::ShaderModule>>,
    color: RefCell<Option<crate::sample_pipelines::Variants<wgpu::RenderPipeline>>>,
    pick: RefCell<Option<wgpu::RenderPipeline>>,
    format: wgpu::TextureFormat,
    samples: u32,
    matrix: wgpu::Buffer,
    line: wgpu::Buffer,
    pick_matrix: wgpu::Buffer,
    pick_line: wgpu::Buffer,
    slot: wgpu::Buffer,
    empty_depth: [wgpu::TextureView; 2],
    empty_primitive: [wgpu::TextureView; 2],
    frame: RefCell<Option<Frame>>,
    draws: Vec<Draw>,
    cache: HashMap<(usize, u32), Weak<Geometry>>,
    pub uploads: u64,
    pub uploaded_bytes: u64,
    anchor: [f64; 3],
    highlight: Option<crate::pick_sources::Target>,
}
impl Lane {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let mut layouts = crate::production_ribbon_gpu::layouts(device);
        layouts[3] = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("production sheet page and owner ABI"),
            entries: &[
                storage(0, wgpu::ShaderStages::VERTEX),
                storage(1, wgpu::ShaderStages::VERTEX),
                uniform(2, wgpu::ShaderStages::VERTEX),
                uniform(3, wgpu::ShaderStages::FRAGMENT),
            ],
        });
        let ub = |size| buffer(device, &[], size, wgpu::BufferUsages::UNIFORM);
        let image = |format, samples| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("unused production sheet attachment"),
                    size: wgpu::Extent3d {
                        width: 1,
                        height: 1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: samples,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING
                        | wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                })
                .create_view(&Default::default())
        };
        Self {
            layouts,
            shader: Default::default(),
            color: Default::default(),
            pick: Default::default(),
            format,
            samples: 1,
            matrix: ub(64),
            line: ub(80),
            pick_matrix: ub(64),
            pick_line: ub(80),
            slot: buffer(
                device,
                &[255, 255, 255, 255, 0, 0, 0, 0],
                8,
                wgpu::BufferUsages::VERTEX,
            ),
            empty_depth: [
                image(wgpu::TextureFormat::Depth32Float, 1),
                image(wgpu::TextureFormat::Depth32Float, 4),
            ],
            empty_primitive: [
                image(wgpu::TextureFormat::Rg16Uint, 1),
                image(wgpu::TextureFormat::Rg16Uint, 4),
            ],
            frame: Default::default(),
            draws: Vec::new(),
            cache: HashMap::new(),
            uploads: 0,
            uploaded_bytes: 0,
            anchor: [0.0; 3],
            highlight: None,
        }
    }
    fn geometry(
        &mut self,
        device: &wgpu::Device,
        source: &Rc<Prepared>,
        page: crate::sheet_stream::DisplayPage,
        range: std::ops::Range<u32>,
    ) -> Rc<Geometry> {
        let key = (page.key(), range.start);
        if let Some(geometry) = self.cache.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let mut segments = Vec::with_capacity(range.len() * 52);
        let mut edges = Vec::with_capacity(range.len() * 4);
        for i in range.clone() {
            let i = i as usize;
            let p = &page.display[i * 6..i * 6 + 6];
            rows::segment_bytes(
                &crate::production_curve_walk::StrokeSegment {
                    segment: crate::production_curve_walk::CylinderSegment {
                        p0: [p[0], p[1], p[2]],
                        radius: page.radius(i),
                        p1: [p[3], p[4], p[5]],
                        instance_id: 0,
                        color: page.colors[i],
                        facing: u32::MAX,
                    },
                    previous: u32::MAX,
                    next: u32::MAX,
                    sag: 0.0,
                },
                &mut segments,
            );
            edges.extend(page.entities[i].to_ne_bytes());
        }
        let geometry = Rc::new(Geometry {
            _page: page,
            source: source.clone(),
            segments: buffer(device, &segments, 52, wgpu::BufferUsages::STORAGE),
            edges: buffer(device, &edges, 4, wgpu::BufferUsages::STORAGE),
            count: range.end - range.start,
        });
        self.uploads += 1;
        self.uploaded_bytes += (segments.len() + edges.len()) as u64;
        self.cache.insert(key, Rc::downgrade(&geometry));
        geometry
    }
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        selected: &[ObjectId],
        bases: &HashMap<(ObjectId, usize), u32>,
    ) -> Result<(), &'static str> {
        self.set_anchored(device, queue, scene, selected, bases, [0.0; 3])
    }
    pub fn set_anchored(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        scene: &Scene,
        selected: &[ObjectId],
        bases: &HashMap<(ObjectId, usize), u32>,
        anchor: [f64; 3],
    ) -> Result<(), &'static str> {
        let mut old = std::mem::take(&mut self.draws);
        let limit = device
            .limits()
            .max_buffer_size
            .min(u64::from(device.limits().max_storage_buffer_binding_size));
        for row in scene.sheets().iter().filter(|row| scene.visible(row.id)) {
            for page in row.prepared.pages() {
                for range in chunks(page.rows.end - page.rows.start, limit)? {
                    let base = bases
                        .get(&(row.id, page.key()))
                        .ok_or("Missing original sheet source addresses")?
                        .checked_add(range.start)
                        .ok_or("Sheet source address overflow")?;
                    let tag = crate::sub_id::encode(crate::sub_id::Kind::Edge, base as usize)?;
                    if range.end > range.start {
                        crate::sub_id::encode(
                            crate::sub_id::Kind::Edge,
                            (base + range.end - range.start - 1) as usize,
                        )?;
                    }
                    let geometry = self.geometry(device, &row.prepared, page.clone(), range);
                    let (objects, translations) = rows::object(
                        scene,
                        row.id,
                        &row.model,
                        anchor,
                        selected.contains(&row.id),
                        32,
                    );
                    let selection = self
                        .highlight
                        .filter(|t| t.owner == row.id)
                        .and_then(|t| {
                            if let crate::pick_sources::Part::SheetEntity(edge) = t.part {
                                Some([0, edge, u32::MAX, u32::MAX])
                            } else {
                                None
                            }
                        })
                        .unwrap_or([u32::MAX; 4]);
                    let owner = crate::feature_owner::display_owner(scene, row.id)
                        .map_or(0, ObjectId::value);
                    let picks = [owner, tag, 0, 0];
                    let mut bytes = objects.clone();
                    bytes.extend(&translations);
                    bytes.extend(selection.into_iter().flat_map(u32::to_ne_bytes));
                    bytes.extend(picks.into_iter().flat_map(u32::to_ne_bytes));
                    let draw = if let Some(i) = old
                        .iter()
                        .position(|d| d.id == row.id && Rc::ptr_eq(&d.geometry, &geometry))
                    {
                        let mut draw = old.swap_remove(i);
                        if draw.bytes != bytes {
                            queue.write_buffer(&draw.objects, 0, &objects);
                            queue.write_buffer(&draw.translations, 0, &translations);
                            queue.write_buffer(&draw.selection, 0, &bytes[112..128]);
                            queue.write_buffer(&draw.picks, 0, &bytes[128..144]);
                            draw.bytes = bytes;
                        }
                        draw
                    } else {
                        Draw {
                            id: row.id,
                            geometry,
                            objects: buffer(device, &objects, 96, wgpu::BufferUsages::STORAGE),
                            translations: buffer(
                                device,
                                &translations,
                                16,
                                wgpu::BufferUsages::STORAGE,
                            ),
                            selection: buffer(
                                device,
                                &bytes[112..128],
                                16,
                                wgpu::BufferUsages::UNIFORM,
                            ),
                            picks: buffer(
                                device,
                                &bytes[128..144],
                                16,
                                wgpu::BufferUsages::UNIFORM,
                            ),
                            bytes,
                        }
                    };
                    self.draws.push(draw);
                }
            }
        }
        drop(old);
        self.cache.retain(|_, owner| owner.strong_count() > 0);
        self.anchor = anchor;
        self.frame.borrow_mut().take();
        Ok(())
    }
    pub fn set_highlight(
        &mut self,
        queue: &wgpu::Queue,
        target: Option<crate::pick_sources::Target>,
    ) {
        self.highlight = target;
        for draw in &mut self.draws {
            let selection = target
                .filter(|t| t.owner == draw.id)
                .and_then(|t| {
                    if let crate::pick_sources::Part::SheetEntity(edge) = t.part {
                        Some([0, edge, u32::MAX, u32::MAX])
                    } else {
                        None
                    }
                })
                .unwrap_or([u32::MAX; 4]);
            let bytes = selection
                .into_iter()
                .flat_map(u32::to_ne_bytes)
                .collect::<Vec<_>>();
            queue.write_buffer(&draw.selection, 0, &bytes);
            draw.bytes[112..128].copy_from_slice(&bytes);
        }
    }
    pub fn camera(&self, _camera: &crate::camera::Camera) {}
    pub fn view(
        &self,
        _queue: &wgpu::Queue,
        _transform: &[f32; 16],
        _size: [u32; 2],
        _density: f32,
    ) {
    }
    pub fn set_samples(&mut self, _device: &wgpu::Device, samples: u32) {
        assert!(matches!(samples, 1 | 4));
        self.samples = samples;
    }
    fn shader(&self, device: &wgpu::Device) -> wgpu::ShaderModule {
        self.shader
            .borrow_mut()
            .get_or_insert_with(|| {
                device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("actual production sheet ribbon"),
                    source: wgpu::ShaderSource::Wgsl(SOURCE.into()),
                })
            })
            .clone()
    }
    fn pipeline(&self, device: &wgpu::Device, picking: bool) {
        if self.draws.is_empty() {
            return;
        }
        let shader = self.shader(device);
        if picking {
            self.pick.borrow_mut().get_or_insert_with(|| {
                pipeline(
                    device,
                    &shader,
                    &self.layouts,
                    wgpu::TextureFormat::Rg32Uint,
                    1,
                    true,
                )
            });
        } else {
            let mut color = self.color.borrow_mut();
            if let Some(color) = color.as_mut() {
                color.select(self.samples, || {
                    pipeline(
                        device,
                        &shader,
                        &self.layouts,
                        self.format,
                        self.samples,
                        false,
                    )
                });
            } else {
                *color = Some(crate::sample_pipelines::Variants::new(
                    pipeline(
                        device,
                        &shader,
                        &self.layouts,
                        self.format,
                        self.samples,
                        false,
                    ),
                    self.samples,
                ));
            }
        }
    }
    fn bindings(
        &self,
        device: &wgpu::Device,
        matrix: &wgpu::Buffer,
        line: &wgpu::Buffer,
        clipping: &wgpu::Buffer,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
        samples: u32,
    ) -> Frame {
        let depths = if samples == 1 {
            [depth, &self.empty_depth[1]]
        } else {
            [&self.empty_depth[0], depth]
        };
        let primitives = if samples == 1 {
            [&tables.primitive, &self.empty_primitive[1]]
        } else {
            [&self.empty_primitive[0], &tables.primitive]
        };
        let camera = group(
            device,
            &self.layouts[0],
            vec![(0, matrix.as_entire_binding())],
        );
        let view = group(
            device,
            &self.layouts[1],
            vec![
                (0, line.as_entire_binding()),
                (1, clipping.as_entire_binding()),
            ],
        );
        Frame {
            groups: self
                .draws
                .iter()
                .map(|draw| {
                    [
                        camera.clone(),
                        view.clone(),
                        group(
                            device,
                            &self.layouts[2],
                            vec![
                                (0, draw.objects.as_entire_binding()),
                                (1, draw.translations.as_entire_binding()),
                                (2, wgpu::BindingResource::TextureView(depths[0])),
                                (3, wgpu::BindingResource::TextureView(depths[1])),
                                (4, wgpu::BindingResource::TextureView(primitives[0])),
                                (5, wgpu::BindingResource::TextureView(primitives[1])),
                                (6, tables.projected.as_entire_binding()),
                                (7, tables.tiles.as_entire_binding()),
                            ],
                        ),
                        group(
                            device,
                            &self.layouts[3],
                            vec![
                                (0, draw.geometry.segments.as_entire_binding()),
                                (1, draw.geometry.edges.as_entire_binding()),
                                (2, draw.selection.as_entire_binding()),
                                (3, draw.picks.as_entire_binding()),
                            ],
                        ),
                    ]
                })
                .collect(),
        }
    }
    pub fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        matrix: [f32; 16],
        size: [u32; 2],
        density: f32,
        opacity: f32,
        clipping: &wgpu::Buffer,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) {
        if self.draws.is_empty() {
            return;
        }
        self.pipeline(device, false);
        queue.write_buffer(
            &self.matrix,
            0,
            &matrix
                .into_iter()
                .flat_map(f32::to_ne_bytes)
                .collect::<Vec<_>>(),
        );
        queue.write_buffer(
            &self.line,
            0,
            &rows::line_uniform(matrix, size, density, opacity, self.anchor),
        );
        *self.frame.borrow_mut() = Some(self.bindings(
            device,
            &self.matrix,
            &self.line,
            clipping,
            tables,
            depth,
            self.samples,
        ));
    }
    pub(crate) fn prepare_pick(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        matrix: [f32; 16],
        window: crate::pick_window::Window,
        density: f32,
        clipping: &wgpu::Buffer,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) -> Frame {
        self.pipeline(device, true);
        let line = rows::line_uniform(matrix, window.full, density, 1.0, self.anchor);
        let (matrix, line) = rows::pick_frame(matrix, line, window);
        queue.write_buffer(
            &self.pick_matrix,
            0,
            &matrix
                .into_iter()
                .flat_map(f32::to_ne_bytes)
                .collect::<Vec<_>>(),
        );
        queue.write_buffer(&self.pick_line, 0, &line);
        self.bindings(
            device,
            &self.pick_matrix,
            &self.pick_line,
            clipping,
            tables,
            depth,
            1,
        )
    }
    fn draw_with(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        frame: &Frame,
        pipeline: &wgpu::RenderPipeline,
    ) {
        pass.set_pipeline(pipeline);
        pass.set_vertex_buffer(0, self.slot.slice(..));
        for (draw, groups) in self.draws.iter().zip(&frame.groups) {
            for (index, group) in groups.iter().enumerate() {
                pass.set_bind_group(index as u32, group, &[]);
            }
            pass.draw(0..draw.geometry.count * 6, 0..1);
        }
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if let (Some(frame), Some(color)) =
            (self.frame.borrow().as_ref(), self.color.borrow().as_ref())
        {
            self.draw_with(pass, frame, &color.active);
        }
    }
    pub(crate) fn pick(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        frame: &Frame,
        mode: crate::pick_sources::Mode,
    ) {
        if matches!(
            mode,
            crate::pick_sources::Mode::Object | crate::pick_sources::Mode::Edge
        ) {
            if let Some(pipeline) = self.pick.borrow().as_ref() {
                self.draw_with(pass, frame, pipeline);
            }
        }
    }
    pub fn usage(&self) -> [u64; 5] {
        let mut unique = std::collections::HashSet::new();
        let bytes = self
            .draws
            .iter()
            .filter(|d| unique.insert(Rc::as_ptr(&d.geometry)))
            .map(|d| d.geometry.segments.size() + d.geometry.edges.size())
            .sum::<u64>();
        [
            self.draws.len() as u64,
            unique.len() as u64,
            bytes + self.draws.len() as u64 * 144 + 296,
            self.uploads,
            self.uploaded_bytes,
        ]
    }
    pub fn sources(&self) -> impl Iterator<Item = &Rc<Prepared>> {
        self.draws.iter().map(|d| &d.geometry.source)
    }
}
pub fn chunks(count: u32, limit: u64) -> Result<Vec<std::ops::Range<u32>>, &'static str> {
    let size = (limit / 52).min(u64::from(u32::MAX / 6)) as u32;
    if size == 0 {
        return Err("Device cannot hold a production sheet segment");
    }
    let mut out = Vec::new();
    let mut start = 0;
    while start < count {
        let end = start + size.min(count - start);
        out.push(start..end);
        start = end;
    }
    Ok(out)
}
