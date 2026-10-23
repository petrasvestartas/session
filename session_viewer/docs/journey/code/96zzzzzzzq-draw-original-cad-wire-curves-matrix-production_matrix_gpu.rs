use crate::production_matrix::{self as matrix_rows, Rows};
use crate::production_ribbon as ribbon;
use std::cell::RefCell;
use wgpu::util::DeviceExt;
use wgpu::{BufferBindingType, ShaderStages};

fn storage(binding: u32, stages: ShaderStages) -> wgpu::BindGroupLayoutEntry {
    crate::tile_pipelines::entry(
        binding,
        stages,
        BufferBindingType::Storage { read_only: true },
    )
}

fn uniform(binding: u32, stages: ShaderStages) -> wgpu::BindGroupLayoutEntry {
    crate::tile_pipelines::entry(binding, stages, BufferBindingType::Uniform)
}

fn layouts(device: &wgpu::Device) -> [wgpu::BindGroupLayout; 4] {
    let vf = ShaderStages::VERTEX_FRAGMENT;
    let line = ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE;
    let texture = |binding, stages, sample_type, multisampled| {
        crate::text_gpu_support::texture_entry(binding, stages, sample_type, multisampled)
    };
    let entries = [
        vec![uniform(0, ShaderStages::VERTEX)],
        vec![uniform(0, line), uniform(1, line)],
        vec![
            storage(0, vf),
            storage(1, vf),
            texture(
                2,
                ShaderStages::FRAGMENT,
                wgpu::TextureSampleType::Depth,
                false,
            ),
            texture(
                3,
                ShaderStages::FRAGMENT,
                wgpu::TextureSampleType::Depth,
                true,
            ),
            texture(4, vf, wgpu::TextureSampleType::Uint, false),
            texture(5, vf, wgpu::TextureSampleType::Uint, true),
            storage(6, vf),
            storage(7, ShaderStages::FRAGMENT),
        ],
        vec![storage(0, vf)],
    ];
    entries.map(|entries| {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("production matrix layout"),
            entries: &entries,
        })
    })
}

fn buffer(
    device: &wgpu::Device,
    bytes: &[u8],
    minimum: usize,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    let zero = vec![0; minimum];
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("production matrix data"),
        contents: if bytes.is_empty() { &zero } else { bytes },
        usage: usage | wgpu::BufferUsages::COPY_DST,
    })
}

fn group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    resources: Vec<(u32, wgpu::BindingResource<'_>)>,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("production matrix bindings"),
        layout,
        entries: &resources
            .into_iter()
            .map(|(binding, resource)| wgpu::BindGroupEntry { binding, resource })
            .collect::<Vec<_>>(),
    })
}

fn pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    layouts: &[wgpu::BindGroupLayout; 4],
    format: wgpu::TextureFormat,
    samples: u32,
    picking: bool,
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("production matrix"),
        bind_group_layouts: &layouts.iter().map(Some).collect::<Vec<_>>(),
        immediate_size: 0,
    });
    let constants = [("SCENE_MSAA", f64::from(samples > 1))];
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("production matrix"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_matrix"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(if picking { "fs_id" } else { "fs_main" }),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &constants,
                ..Default::default()
            },
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: (!picking).then_some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::Always),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: samples,
            ..Default::default()
        },
        multiview_mask: None,
        cache: None,
    })
}

pub(crate) struct Frame {
    groups: [wgpu::BindGroup; 4],
}

pub(crate) struct Lane {
    layouts: [wgpu::BindGroupLayout; 4],
    shader: RefCell<Option<wgpu::ShaderModule>>,
    color: RefCell<Option<crate::sample_pipelines::Variants<wgpu::RenderPipeline>>>,
    pick: RefCell<Option<wgpu::RenderPipeline>>,
    samples: u32,
    format: wgpu::TextureFormat,
    matrix: wgpu::Buffer,
    line: wgpu::Buffer,
    pick_matrix: wgpu::Buffer,
    pick_line: wgpu::Buffer,
    objects: wgpu::Buffer,
    translations: wgpu::Buffer,
    segments: wgpu::Buffer,
    empty_depth: [wgpu::TextureView; 2],
    empty_primitive: [wgpu::TextureView; 2],
    rows: Rows,
    pub visible: bool,
    pub uploads: u64,
    pub uploaded_bytes: u64,
    pub anchor: [f64; 3],
    frame: RefCell<Option<Frame>>,
}

impl Lane {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let layouts = layouts(device);
        let ub = |size| buffer(device, &[], size, wgpu::BufferUsages::UNIFORM);
        let sb = |size| buffer(device, &[], size, wgpu::BufferUsages::STORAGE);
        let image = |format, samples| {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("unused production matrix attachment"),
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
            shader: RefCell::new(None),
            color: RefCell::new(None),
            pick: RefCell::new(None),
            samples: 1,
            format,
            matrix: ub(64),
            line: ub(80),
            pick_matrix: ub(64),
            pick_line: ub(80),
            objects: sb(96),
            translations: sb(16),
            segments: sb(64),

            empty_depth: [
                image(wgpu::TextureFormat::Depth32Float, 1),
                image(wgpu::TextureFormat::Depth32Float, 4),
            ],
            empty_primitive: [
                image(wgpu::TextureFormat::Rg16Uint, 1),
                image(wgpu::TextureFormat::Rg16Uint, 4),
            ],
            rows: Rows {
                words: Vec::new(),
                objects: Vec::new(),
                translations: Vec::new(),
                starts: Vec::new(),
            },
            visible: true,
            uploads: 0,
            uploaded_bytes: 0,
            anchor: [0.0; 3],
            frame: RefCell::new(None),
        }
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
                    label: Some("exact production matrix"),
                    source: wgpu::ShaderSource::Wgsl(matrix_rows::SOURCE.into()),
                })
            })
            .clone()
    }

    fn prepare_color(&self, device: &wgpu::Device) {
        let shader = self.shader(device);
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

    pub fn set_scene(&mut self,device:&wgpu::Device,scene:&crate::scene::Scene,anchor:[f64;3],selected:&[crate::scene::ObjectId])->Result<(), &'static str> {
        let rows=matrix_rows::rows(scene,anchor,selected,u64::from(device.limits().max_storage_buffer_binding_size))?;
        if rows.words!=self.rows.words {self.segments=buffer(device,&rows.words,64,wgpu::BufferUsages::STORAGE);self.uploads+=1;self.uploaded_bytes+=rows.words.len() as u64;}
        if rows.objects!=self.rows.objects {self.objects=buffer(device,&rows.objects,96,wgpu::BufferUsages::STORAGE);}
        if rows.translations!=self.rows.translations {self.translations=buffer(device,&rows.translations,16,wgpu::BufferUsages::STORAGE);}
        self.rows=rows;self.anchor=anchor;self.frame.borrow_mut().take();Ok(())
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
        let one = usize::from(samples == 1);
        let depths = if one == 1 {
            [depth, &self.empty_depth[1]]
        } else {
            [&self.empty_depth[0], depth]
        };
        let primitives = if one == 1 {
            [&tables.primitive, &self.empty_primitive[1]]
        } else {
            [&self.empty_primitive[0], &tables.primitive]
        };
        Frame {
            groups: [
                group(
                    device,
                    &self.layouts[0],
                    vec![(0, matrix.as_entire_binding())],
                ),
                group(
                    device,
                    &self.layouts[1],
                    vec![
                        (0, line.as_entire_binding()),
                        (1, clipping.as_entire_binding()),
                    ],
                ),
                group(
                    device,
                    &self.layouts[2],
                    vec![
                        (0, self.objects.as_entire_binding()),
                        (1, self.translations.as_entire_binding()),
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
                        (0, self.segments.as_entire_binding()),
                    ],
                ),
            ],
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
        if self.rows.starts.is_empty() {
            return;
        }
        self.prepare_color(device);
        let bytes = matrix
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect::<Vec<_>>();
        queue.write_buffer(&self.matrix, 0, &bytes);
        queue.write_buffer(
            &self.line,
            0,
            &ribbon::line_uniform(matrix, size, density, opacity, self.anchor),
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

    pub fn prepare_pick(
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
        if self.visible && !self.rows.starts.is_empty() {
            self.pick.borrow_mut().get_or_insert_with(|| {
                pipeline(
                    device,
                    &self.shader(device),
                    &self.layouts,
                    wgpu::TextureFormat::Rg32Uint,
                    1,
                    true,
                )
            });
        }
        let line = ribbon::line_uniform(matrix, window.full, density, 1.0, self.anchor);
        let (matrix, line) = ribbon::pick_frame(matrix, line, window);
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
        if !self.visible || self.rows.starts.is_empty() {
            return;
        }
        pass.set_pipeline(pipeline);
        for (index, group) in frame.groups.iter().enumerate() {
            pass.set_bind_group(index as u32, group, &[]);
        }
        for &start in &self.rows.starts {pass.draw(0..6,start..start+1);}
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        if let (Some(frame), Some(color)) =
            (self.frame.borrow().as_ref(), self.color.borrow().as_ref())
        {
            self.draw_with(pass, frame, &color.active);
        }
    }

    pub fn pick(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        frame: &Frame,
        mode: crate::pick_sources::Mode,
    ) {
        if matches!(
            mode,
            crate::pick_sources::Mode::Object
        ) {
            if let Some(pipeline) = self.pick.borrow().as_ref() {
                self.draw_with(pass, frame, pipeline);
            }
        }
    }

    pub fn usage(&self) -> [u64; 2] {
        [self.rows.starts.len() as u64, self.segments.size()]
    }
    pub fn clipping_bytes(&self) -> [u64; 2] {
        [self.objects.size(), self.rows.objects.capacity() as u64]
    }
}
