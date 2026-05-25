//! GPU mirror of Session. Three topology arenas + instance buffer + pick table.

use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;

use crate::gpu_adapters::{
    color_to_rgba_f32, color_to_rgba_u8,
    line_endpoint_glyphs, line_to_segment, mesh_to_vertices, named_point_to_cross_vertices,
    obb_to_line_vertices, plane_to_mesh_vertices, point_to_vertex,
    pointcloud_to_point_vertices, polyline_endpoint_glyphs, polyline_to_segments,
};
use crate::gpu_arena::GpuArena;

// ── Vertex types & instance types ────────────────────────────────────────────

/// Triangle-arena vertex. 28 bytes: position + normal + RGBA8.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [u8; 4],
}

impl MeshVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Unorm8x4];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Line-arena vertex. 16 bytes: position + RGBA8.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct LineVertex {
    pub position: [f32; 3],
    pub color: [u8; 4],
}

impl LineVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Unorm8x4];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Point-arena vertex. 16 bytes: position + RGBA8.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PointVertex {
    pub position: [f32; 3],
    pub color: [u8; 4],
}

impl PointVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Unorm8x4];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Template vertex for instanced cylinder/sphere geometry. 24 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TemplateVertex {
    pub position: [f32; 3],
    pub normal:   [f32; 3],
}

impl TemplateVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// One cylinder segment stored in the GPU segment storage buffer. 32 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CylinderSegment {
    pub p0:          [f32; 3],
    pub radius:      f32,       // 0.0 = use shader default CYLINDER_RADIUS
    pub p1:          [f32; 3],
    pub instance_id: u32,
    pub color:       [f32; 4],  // if alpha > 0, overrides inst.tint; else falls back to inst.tint
}

/// One sphere glyph point stored in the GPU glyph storage buffer. 48 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GlyphPoint {
    pub center:      [f32; 3],
    pub radius:      f32,
    pub color:       [f32; 4],
    pub instance_id: u32,
    pub _pad:        [u32; 3],
}

// ── InstanceData ─────────────────────────────────────────────────────────────

/// Per-object GPU data. 96 bytes, 16-byte aligned.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InstanceData {
    pub model: [[f32; 4]; 4],
    pub color: [f32; 4],
    pub object_id: u32,
    pub flags: u32,
    pub _pad: [u32; 2],
}

impl InstanceData {
    pub const FLAG_SELECTED: u32 = 1 << 0;
    pub const FLAG_HIDDEN:   u32 = 1 << 1;

    pub fn new(instance_id: u32) -> Self {
        Self {
            model: identity_matrix(),
            color: [1.0, 1.0, 1.0, 1.0],
            object_id: instance_id,
            flags: 0,
            _pad: [0, 0],
        }
    }
}

fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

// ── PickTable ─────────────────────────────────────────────────────────────────

#[derive(Default, Debug)]
pub struct PickTable {
    instance_to_guid: Vec<Option<String>>,
    guid_to_instance: HashMap<String, u32>,
    free_instance_ids: Vec<u32>,
}

impl PickTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(&mut self, guid: &str) -> u32 {
        if let Some(&existing) = self.guid_to_instance.get(guid) {
            return existing;
        }
        let id = if let Some(reused) = self.free_instance_ids.pop() {
            self.instance_to_guid[reused as usize] = Some(guid.to_string());
            reused
        } else {
            let id = self.instance_to_guid.len() as u32;
            self.instance_to_guid.push(Some(guid.to_string()));
            id
        };
        self.guid_to_instance.insert(guid.to_string(), id);
        id
    }

    pub fn release(&mut self, guid: &str) {
        if let Some(id) = self.guid_to_instance.remove(guid) {
            self.instance_to_guid[id as usize] = None;
            self.free_instance_ids.push(id);
        }
    }

    pub fn instance_id(&self, guid: &str) -> Option<u32> {
        self.guid_to_instance.get(guid).copied()
    }

    pub fn clear(&mut self) {
        self.instance_to_guid.clear();
        self.guid_to_instance.clear();
        self.free_instance_ids.clear();
    }
}

// ── GpuSession ───────────────────────────────────────────────────────────────

const DEFAULT_TRI_VERTS: u32 = 4096;
const DEFAULT_TRI_INDS: u32 = 8192;
const DEFAULT_LINE_VERTS: u32 = 4096;
const DEFAULT_LINE_INDS: u32 = 8192;
const DEFAULT_POINT_VERTS: u32 = 4096;
const DEFAULT_INSTANCE_CAP: u32 = 1024;
const DEFAULT_SEGMENT_CAP: usize = 64;
const DEFAULT_GLYPH_CAP:   usize = 64;

pub struct GpuSession {
    pub tri:   GpuArena<MeshVertex>,
    pub line:  GpuArena<LineVertex>,
    pub point: GpuArena<PointVertex>,

    pub instance_buffer:   wgpu::Buffer,
    pub instance_capacity: u32,
    pub instances_cpu:     Vec<InstanceData>,

    pub pick: PickTable,

    // Instanced cylinder pipeline (Line / Polyline → tubes)
    pub cylinder_vbo:      wgpu::Buffer,
    pub cylinder_ibo:      wgpu::Buffer,
    pub segments_cpu:      Vec<CylinderSegment>,
    pub segments_gpu:      wgpu::Buffer,
    pub segment_bg:        wgpu::BindGroup,
    pub guid_to_seg:       HashMap<String, std::ops::Range<usize>>,
    pub segments_dirty:    bool,

    // Instanced sphere pipeline (PointCloud → sphere glyphs)
    pub sphere_vbo:        wgpu::Buffer,
    pub sphere_ibo:        wgpu::Buffer,
    pub glyphs_cpu:        Vec<GlyphPoint>,
    pub glyphs_gpu:        wgpu::Buffer,
    pub glyph_sphere_bg:   wgpu::BindGroup,
    pub guid_to_glyph:     HashMap<String, std::ops::Range<usize>>,
    pub glyphs_dirty:      bool,
}

impl GpuSession {
    pub fn new(device: &wgpu::Device, geom_bgl: &wgpu::BindGroupLayout) -> Self {
        use wgpu::util::DeviceExt;
        use crate::gpu_adapters::{unit_cylinder_template, unit_sphere_template};

        let tri   = GpuArena::<MeshVertex>::new(device, "gpu_session.tri",   DEFAULT_TRI_VERTS,   DEFAULT_TRI_INDS);
        let line  = GpuArena::<LineVertex>::new(device, "gpu_session.line",  DEFAULT_LINE_VERTS,  DEFAULT_LINE_INDS);
        let point = GpuArena::<PointVertex>::new(device, "gpu_session.point", DEFAULT_POINT_VERTS, 0);

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gpu_session.instances"),
            size: (DEFAULT_INSTANCE_CAP as u64) * (std::mem::size_of::<InstanceData>() as u64),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Cylinder template
        let (cyl_v, cyl_i) = unit_cylinder_template();
        let cylinder_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cylinder.template.vbo"),
            contents: bytemuck::cast_slice(&cyl_v),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let cylinder_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cylinder.template.ibo"),
            contents: bytemuck::cast_slice(&cyl_i),
            usage: wgpu::BufferUsages::INDEX,
        });
        let seg_init = vec![CylinderSegment { p0: [0.0;3], radius: 0.0, p1: [0.0;3], instance_id: 0, color: [0.0;4] }; DEFAULT_SEGMENT_CAP];
        let segments_gpu = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("gpu_session.segments"),
            contents: bytemuck::cast_slice(&seg_init),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let segment_bg = make_geom_bind_group(device, geom_bgl, &segments_gpu);

        // Sphere template
        let (sph_v, sph_i) = unit_sphere_template();
        let sphere_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sphere.template.vbo"),
            contents: bytemuck::cast_slice(&sph_v),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let sphere_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("sphere.template.ibo"),
            contents: bytemuck::cast_slice(&sph_i),
            usage: wgpu::BufferUsages::INDEX,
        });
        let gly_init = vec![GlyphPoint { center: [0.0;3], radius: 0.0, color: [0.0;4], instance_id: 0, _pad: [0;3] }; DEFAULT_GLYPH_CAP];
        let glyphs_gpu = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("gpu_session.glyphs"),
            contents: bytemuck::cast_slice(&gly_init),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let glyph_sphere_bg = make_geom_bind_group(device, geom_bgl, &glyphs_gpu);

        Self {
            tri, line, point,
            instance_buffer, instance_capacity: DEFAULT_INSTANCE_CAP, instances_cpu: Vec::new(),
            pick: PickTable::new(),
            cylinder_vbo, cylinder_ibo,
            segments_cpu: Vec::new(), segments_gpu, segment_bg,
            guid_to_seg: HashMap::new(), segments_dirty: false,
            sphere_vbo, sphere_ibo,
            glyphs_cpu: Vec::new(), glyphs_gpu, glyph_sphere_bg,
            guid_to_glyph: HashMap::new(), glyphs_dirty: false,
        }
    }

    pub fn rebuild_from(&mut self, session: &session_rust::session::Session, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.clear();
        for (guid, geom) in &session.lookup {
            self.add_geometry(guid, geom, device, queue);
        }
        for nc in &session.objects.nurbscurves {
            self.add_nurbscurve(nc, device, queue);
        }
        for ns in &session.objects.nurbssurfaces {
            self.add_nurbssurface(ns, device, queue);
        }
    }

    pub fn add_nurbscurve(&mut self, curve: &session_rust::NurbsCurve, device: &wgpu::Device, queue: &wgpu::Queue) {
        let guid = curve.guid().to_string();
        let (pts, _) = curve.to_polyline_adaptive(session_rust::Tolerance::ANGULARDEFLECTION, 0.0, 0.0);
        if pts.len() < 2 { return; }
        let instance_id = self.pick.allocate(&guid);
        let color = color_to_rgba_u8(curve.linecolors.get(0).unwrap_or(&session_rust::Color::black()));
        let verts: Vec<LineVertex> = pts.iter().map(|p| LineVertex { position: [p[0], p[1], p[2]], color }).collect();
        let n = verts.len();
        let mut inds: Vec<u32> = Vec::with_capacity(n.saturating_sub(1) * 2);
        for i in 0..n.saturating_sub(1) { inds.push(i as u32); inds.push((i+1) as u32); }
        self.line.allocate(&guid, &verts, Some(&inds), instance_id, device, queue);
        self.write_instance(instance_id, color_to_rgba_f32(curve.linecolors.get(0).unwrap_or(&session_rust::Color::black())), device, queue);
    }

    pub fn add_nurbssurface(&mut self, surface: &session_rust::NurbsSurface, device: &wgpu::Device, queue: &wgpu::Queue) {
        let guid = surface.guid().to_string();
        let mesh = surface.mesh();
        let (vs, is) = mesh_to_vertices(&mesh);
        if vs.is_empty() { return; }
        let instance_id = self.pick.allocate(&guid);
        self.tri.allocate(&guid, &vs, Some(&is), instance_id, device, queue);
        self.write_instance(instance_id, color_to_rgba_f32(surface.linecolors.get(0).unwrap_or(&session_rust::Color::white())), device, queue);
    }

    pub fn add_geometry(&mut self, guid: &str, geom: &session_rust::session::Geometry, device: &wgpu::Device, queue: &wgpu::Queue) {
        use session_rust::session::Geometry;
        let instance_id = self.pick.allocate(guid);

        match geom {
            Geometry::Point(p) => {
                if !p.name.is_empty() && p.name != "my_point" {
                    // Named point → 3-axis crosshair in line arena + text label.
                    let (vs, is) = named_point_to_cross_vertices(p);
                    self.line.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                    self.write_instance(instance_id, color_to_rgba_f32(&p.pointcolor), device, queue);
                } else {
                    let vs = point_to_vertex(p);
                    self.point.allocate(guid, &vs, None, instance_id, device, queue);
                    self.write_instance(instance_id, color_to_rgba_f32(&p.pointcolor), device, queue);
                }
            }
            Geometry::Line(l) => {
                let seg = line_to_segment(l, instance_id);
                let seg_start = self.segments_cpu.len();
                self.segments_cpu.push(seg);
                self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                self.segments_dirty = true;
                let gly_start = self.glyphs_cpu.len();
                self.glyphs_cpu.extend(line_endpoint_glyphs(l, instance_id));
                self.guid_to_glyph.insert(guid.to_string(), gly_start..self.glyphs_cpu.len());
                self.glyphs_dirty = true;
                self.write_instance(instance_id, color_to_rgba_f32(&l.linecolor), device, queue);
            }
            Geometry::Polyline(pl) => {
                let segs = polyline_to_segments(pl, instance_id);
                if segs.is_empty() { self.pick.release(guid); return; }
                let seg_start = self.segments_cpu.len();
                self.segments_cpu.extend(segs);
                self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                self.segments_dirty = true;
                let gly_start = self.glyphs_cpu.len();
                self.glyphs_cpu.extend(polyline_endpoint_glyphs(pl, instance_id));
                self.guid_to_glyph.insert(guid.to_string(), gly_start..self.glyphs_cpu.len());
                self.glyphs_dirty = true;
                self.write_instance(instance_id, color_to_rgba_f32(&pl.linecolor), device, queue);
            }
            Geometry::PointCloud(pc) => {
                let verts = pointcloud_to_point_vertices(pc);
                if verts.is_empty() { self.pick.release(guid); return; }
                self.point.allocate(guid, &verts, None, instance_id, device, queue);
                self.write_instance(instance_id, [1.0, 1.0, 1.0, 1.0], device, queue);
            }
            Geometry::Mesh(m) => {
                let (vs, is) = mesh_to_vertices(m);
                self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                self.write_instance(instance_id, color_to_rgba_f32(m.objectcolor()), device, queue);
            }
            Geometry::Plane(pl) => {
                let (vs, is) = plane_to_mesh_vertices(pl, 1.0);
                self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                self.write_instance(instance_id, color_to_rgba_f32(&pl.linecolor), device, queue);
            }
            Geometry::OBB(bb) => {
                let (vs, is) = obb_to_line_vertices(bb);
                self.line.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                self.write_instance(instance_id, [1.0, 1.0, 1.0, 1.0], device, queue);
            }
            Geometry::BRep(b) => {
                let m = b.mesh();
                let (vs, is) = mesh_to_vertices(&m);
                if !vs.is_empty() {
                    self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                    self.write_instance(instance_id, color_to_rgba_f32(&b.surfacecolor), device, queue);
                } else {
                    self.pick.release(guid);
                }
            }
            Geometry::Element(e) => {
                use session_rust::element::ElementGeometry;
                match e.geometry() {
                    ElementGeometry::Mesh(m) => {
                        let (vs, is) = mesh_to_vertices(m);
                        if !vs.is_empty() {
                            self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                            self.write_instance(instance_id, color_to_rgba_f32(m.objectcolor()), device, queue);
                        } else {
                            self.pick.release(guid);
                        }
                    }
                    ElementGeometry::BRep(b) => {
                        let m = b.mesh();
                        let (vs, is) = mesh_to_vertices(&m);
                        if !vs.is_empty() {
                            self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                            self.write_instance(instance_id, color_to_rgba_f32(&b.surfacecolor), device, queue);
                        } else {
                            self.pick.release(guid);
                        }
                    }
                    ElementGeometry::None => {
                        self.pick.release(guid);
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, guid: &str) {
        self.tri.free(guid);
        self.line.free(guid);
        self.point.free(guid);
        if let Some(r) = self.guid_to_seg.remove(guid) {
            let n = r.end - r.start;
            self.segments_cpu.drain(r.start..r.end);
            for range in self.guid_to_seg.values_mut() {
                if range.start >= r.start + n { range.start -= n; range.end -= n; }
            }
            self.segments_dirty = true;
        }
        if let Some(r) = self.guid_to_glyph.remove(guid) {
            let n = r.end - r.start;
            self.glyphs_cpu.drain(r.start..r.end);
            for range in self.guid_to_glyph.values_mut() {
                if range.start >= r.start + n { range.start -= n; range.end -= n; }
            }
            self.glyphs_dirty = true;
        }
        self.pick.release(guid);
    }

    pub fn clear(&mut self) {
        self.tri.clear();
        self.line.clear();
        self.point.clear();
        self.pick.clear();
        self.instances_cpu.clear();
        self.segments_cpu.clear();
        self.guid_to_seg.clear();
        self.segments_dirty = true;
        self.glyphs_cpu.clear();
        self.guid_to_glyph.clear();
        self.glyphs_dirty = true;
    }

    fn write_instance(&mut self, instance_id: u32, color: [f32; 4], device: &wgpu::Device, queue: &wgpu::Queue) {
        let id = instance_id as usize;
        if id >= self.instances_cpu.len() {
            self.instances_cpu.resize(id + 1, InstanceData::new(0));
        }
        let mut data = InstanceData::new(instance_id);
        data.color = color;
        self.instances_cpu[id] = data;
        if instance_id >= self.instance_capacity {
            self.grow_instance_buffer(instance_id + 1, device, queue);
        }
        let offset = (id as u64) * (std::mem::size_of::<InstanceData>() as u64);
        queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&data));
    }

    fn grow_instance_buffer(&mut self, needed: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut new_cap = self.instance_capacity.max(1) * 2;
        while new_cap < needed { new_cap *= 2; }
        let new_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gpu_session.instances"),
            size: (new_cap as u64) * (std::mem::size_of::<InstanceData>() as u64),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let bytes_to_copy = (self.instance_capacity as u64) * (std::mem::size_of::<InstanceData>() as u64);
        if bytes_to_copy > 0 {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("instances.grow") });
            encoder.copy_buffer_to_buffer(&self.instance_buffer, 0, &new_buffer, 0, bytes_to_copy);
            queue.submit(std::iter::once(encoder.finish()));
        }
        self.instance_buffer = new_buffer;
        self.instance_capacity = new_cap;
    }

    pub fn update_transform(&mut self, guid: &str, model: [[f32; 4]; 4], queue: &wgpu::Queue) -> bool {
        let id = match self.pick.instance_id(guid) { Some(i) => i, None => return false };
        let idx = id as usize;
        if idx >= self.instances_cpu.len() { return false; }
        self.instances_cpu[idx].model = model;
        let offset = (idx as u64) * (std::mem::size_of::<InstanceData>() as u64);
        queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&self.instances_cpu[idx]));
        true
    }

    pub fn set_flag(&mut self, guid: &str, flag: u32, on: bool, queue: &wgpu::Queue) -> bool {
        let id = match self.pick.instance_id(guid) { Some(i) => i, None => return false };
        let idx = id as usize;
        if idx >= self.instances_cpu.len() { return false; }
        if on { self.instances_cpu[idx].flags |= flag; } else { self.instances_cpu[idx].flags &= !flag; }
        let offset = (idx as u64) * (std::mem::size_of::<InstanceData>() as u64);
        queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&self.instances_cpu[idx]));
        true
    }

    pub fn draw_meshes<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        let ibo = match self.tri.ibo.as_ref() { Some(b) => b, None => return };
        pass.set_vertex_buffer(0, self.tri.vbo.slice(..));
        pass.set_index_buffer(ibo.slice(..), wgpu::IndexFormat::Uint32);
        for (_, slot) in self.tri.iter_slots() {
            if let Some(ir) = slot.index_range.clone() {
                pass.draw_indexed(ir, slot.vertex_range.start as i32, slot.instance_id..(slot.instance_id+1));
            }
        }
    }

    pub fn draw_lines<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_vertex_buffer(0, self.line.vbo.slice(..));
        if let Some(ibo) = self.line.ibo.as_ref() {
            pass.set_index_buffer(ibo.slice(..), wgpu::IndexFormat::Uint32);
        }
        for (_, slot) in self.line.iter_slots() {
            match slot.index_range.clone() {
                Some(ir) => pass.draw_indexed(ir, slot.vertex_range.start as i32, slot.instance_id..(slot.instance_id+1)),
                None     => pass.draw(slot.vertex_range.clone(), slot.instance_id..(slot.instance_id+1)),
            }
        }
    }

    pub fn draw_points<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_vertex_buffer(0, self.point.vbo.slice(..));
        for (_, slot) in self.point.iter_slots() {
            pass.draw(slot.vertex_range.clone(), slot.instance_id..(slot.instance_id+1));
        }
    }

    pub fn draw_cylinders<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if self.segments_cpu.is_empty() { return; }
        pass.set_vertex_buffer(0, self.cylinder_vbo.slice(..));
        pass.set_index_buffer(self.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..crate::gpu_adapters::N_CYL_INDICES, 0, 0..self.segments_cpu.len() as u32);
    }

    pub fn draw_spheres<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if self.glyphs_cpu.is_empty() { return; }
        pass.set_vertex_buffer(0, self.sphere_vbo.slice(..));
        pass.set_index_buffer(self.sphere_ibo.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..crate::gpu_adapters::N_SPHERE_INDICES, 0, 0..self.glyphs_cpu.len() as u32);
    }

    pub fn flush_geometry(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, geom_bgl: &wgpu::BindGroupLayout) {
        if self.segments_dirty {
            let needed = (self.segments_cpu.len().max(1) * std::mem::size_of::<CylinderSegment>()) as u64;
            if needed > self.segments_gpu.size() {
                let new_size = (self.segments_gpu.size() * 2).max(needed);
                self.segments_gpu = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gpu_session.segments"),
                    size: new_size,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.segment_bg = make_geom_bind_group(device, geom_bgl, &self.segments_gpu);
            }
            if !self.segments_cpu.is_empty() {
                queue.write_buffer(&self.segments_gpu, 0, bytemuck::cast_slice(&self.segments_cpu));
            }
            self.segments_dirty = false;
        }
        if self.glyphs_dirty {
            let needed = (self.glyphs_cpu.len().max(1) * std::mem::size_of::<GlyphPoint>()) as u64;
            if needed > self.glyphs_gpu.size() {
                let new_size = (self.glyphs_gpu.size() * 2).max(needed);
                self.glyphs_gpu = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gpu_session.glyphs"),
                    size: new_size,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.glyph_sphere_bg = make_geom_bind_group(device, geom_bgl, &self.glyphs_gpu);
            }
            if !self.glyphs_cpu.is_empty() {
                queue.write_buffer(&self.glyphs_gpu, 0, bytemuck::cast_slice(&self.glyphs_cpu));
            }
            self.glyphs_dirty = false;
        }
    }
}

pub fn make_geom_bind_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, buf: &wgpu::Buffer) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("geom.bg"),
        layout,
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
    })
}
