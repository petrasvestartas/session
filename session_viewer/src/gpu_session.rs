//! GPU mirror of Session. Three topology arenas + instance buffer + pick table.

use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;

use crate::gpu_adapters::{
    color_to_rgba_f32,
    line_endpoint_glyphs, line_to_segment,
    mesh_edges_to_segments, mesh_naked_edges_to_segments, mesh_to_vertices, mesh_vertex_glyphs,
    named_point_to_cross_vertices, obb_to_line_vertices, plane_to_axis_segments, plane_origin_glyph,
    point_to_vertex, pointcloud_to_cloud_points, pts_to_segments,
    polyline_endpoint_glyphs, polyline_to_segments,
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

/// One point-cloud point for the billboard circle pipeline. 32 bytes.
/// Layout matches cloud.wgsl CloudPoint struct (32 bytes: vec3+u32+vec4+f32+pad3).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CloudPoint {
    pub position:    [f32; 3],  // offset 0
    pub instance_id: u32,       // offset 12
    pub color:       [f32; 4],  // offset 16
    pub half_size:   f32,       // offset 32  (screen-space px radius)
    pub _pad:        [u32; 3],  // offset 36
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
    pub const FLAG_SMOOTH:   u32 = 1 << 2;

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

/// Full 4×4 inverse for a column-major matrix. Returns identity on singular input.
fn mat4_inverse_cm(m: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let f = |c: usize, r: usize| m[c][r];
    let (m00,m01,m02,m03) = (f(0,0),f(1,0),f(2,0),f(3,0));
    let (m10,m11,m12,m13) = (f(0,1),f(1,1),f(2,1),f(3,1));
    let (m20,m21,m22,m23) = (f(0,2),f(1,2),f(2,2),f(3,2));
    let (m30,m31,m32,m33) = (f(0,3),f(1,3),f(2,3),f(3,3));
    let a2323=m22*m33-m23*m32; let a1323=m21*m33-m23*m31; let a1223=m21*m32-m22*m31;
    let a0323=m20*m33-m23*m30; let a0223=m20*m32-m22*m30; let a0123=m20*m31-m21*m30;
    let a2313=m12*m33-m13*m32; let a1313=m11*m33-m13*m31; let a1213=m11*m32-m12*m31;
    let a2312=m12*m23-m13*m22; let a1312=m11*m23-m13*m21; let a1212=m11*m22-m12*m21;
    let a0313=m10*m33-m13*m30; let a0213=m10*m32-m12*m30; let a0312=m10*m23-m13*m20;
    let a0212=m10*m22-m12*m20; let a0113=m10*m31-m11*m30; let a0112=m10*m21-m11*m20;
    let det = m00*(m11*a2323-m12*a1323+m13*a1223)
            - m01*(m10*a2323-m12*a0323+m13*a0223)
            + m02*(m10*a1323-m11*a0323+m13*a0123)
            - m03*(m10*a1223-m11*a0223+m12*a0123);
    if det.abs() < 1e-30 { return identity_matrix(); }
    let id = 1.0 / det;
    let r = |a:f32,b:f32,c:f32,d:f32| (a-b+c-d)*id;
    let mut out = [[0.0f32; 4]; 4];
    out[0][0]=r(m11*a2323,m12*a1323,m13*a1223,0.0); out[0][1]=r(0.0,m01*a2323,m02*a1323,m03*a1223);
    out[0][2]=r(m01*a2313,m02*a1313,m03*a1213,0.0); out[0][3]=r(0.0,m01*a2312,m02*a1312,m03*a1212);
    out[1][0]=r(0.0,m10*a2323,m12*a0323,m13*a0223); out[1][1]=r(m00*a2323,m02*a0323,m03*a0223,0.0);
    out[1][2]=r(0.0,m00*a2313,m02*a0313,m03*a0213); out[1][3]=r(m00*a2312,m02*a0312,m03*a0212,0.0);
    out[2][0]=r(m10*a1323,m11*a0323,m13*a0123,0.0); out[2][1]=r(0.0,m00*a1323,m01*a0323,m03*a0123);
    out[2][2]=r(m00*a1313,m01*a0313,m03*a0113,0.0); out[2][3]=r(0.0,m00*a1312,m01*a0312,m03*a0112);
    out[3][0]=r(0.0,m10*a1223,m11*a0223,m12*a0123); out[3][1]=r(m00*a1223,m01*a0223,m02*a0123,0.0);
    out[3][2]=r(0.0,m00*a1213,m01*a0213,m02*a0113); out[3][3]=r(m00*a1212,m01*a0212,m02*a0112,0.0);
    // Convert from adjugate layout to column-major by transposing
    let mut t = [[0.0f32; 4]; 4];
    for c in 0..4 { for rr in 0..4 { t[c][rr] = out[rr][c]; } }
    t
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

const DEFAULT_TRI_VERTS: u32 = 65536;
const DEFAULT_TRI_INDS: u32 = 131072;
const DEFAULT_LINE_VERTS: u32 = 8192;
const DEFAULT_LINE_INDS: u32 = 16384;
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

    // Instanced sphere pipeline (Line/Polyline endpoints → sphere glyphs)
    pub sphere_vbo:        wgpu::Buffer,
    pub sphere_ibo:        wgpu::Buffer,
    pub glyphs_cpu:        Vec<GlyphPoint>,
    pub glyphs_gpu:        wgpu::Buffer,
    pub glyph_sphere_bg:   wgpu::BindGroup,
    pub guid_to_glyph:     HashMap<String, std::ops::Range<usize>>,
    pub glyphs_dirty:      bool,

    // Instanced cloud pipeline (PointCloud → billboard circles)
    pub clouds_cpu:        Vec<CloudPoint>,
    pub clouds_gpu:        wgpu::Buffer,
    pub cloud_bg:          wgpu::BindGroup,
    pub guid_to_cloud:     HashMap<String, std::ops::Range<usize>>,
    pub clouds_dirty:      bool,

    // Instanced cone pipeline (arrowheads — reuses cylinder template VBO/IBO)
    pub cones_cpu:         Vec<CylinderSegment>,
    pub cones_gpu:         wgpu::Buffer,
    pub cone_bg:           wgpu::BindGroup,
    pub guid_to_cone:      HashMap<String, std::ops::Range<usize>>,
    pub cones_dirty:       bool,

    /// Axis length for Plane objects (mm).
    pub plane_scale:       f32,
    /// Cached tessellated meshes for NurbsSurface picking (BVH pre-built at load).
    pub nurbs_pick_meshes: HashMap<String, session_rust::Mesh>,
    /// Cached NurbsSurface objects for analytical ray intersection.
    pub nurbs_surfaces: HashMap<String, session_rust::NurbsSurface>,
    /// Cached local-space BRep meshes (BVH pre-built) + world xform columns for picking.
    pub brep_pick_meshes: HashMap<String, (session_rust::Mesh, [[f32; 4]; 4])>,
    /// Cached polyline points for NurbsCurve viewport picking.
    pub nc_pick_pts: HashMap<String, Vec<[f32; 3]>>,
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

        let clouds_init = vec![CloudPoint { position: [0.0;3], instance_id: 0, color: [0.0;4], half_size: 5.0, _pad: [0;3] }; 64];
        let clouds_gpu = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("gpu_session.clouds"),
            contents: bytemuck::cast_slice(&clouds_init),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let cloud_bg = make_geom_bind_group(device, geom_bgl, &clouds_gpu);

        let cone_init = vec![CylinderSegment { p0: [0.0;3], radius: 0.0, p1: [0.0;3], instance_id: 0, color: [0.0;4] }; DEFAULT_SEGMENT_CAP];
        let cones_gpu = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("gpu_session.cones"),
            contents: bytemuck::cast_slice(&cone_init),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
        let cone_bg = make_geom_bind_group(device, geom_bgl, &cones_gpu);

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
            clouds_cpu: Vec::new(), clouds_gpu, cloud_bg,
            guid_to_cloud: HashMap::new(), clouds_dirty: false,
            cones_cpu: Vec::new(), cones_gpu, cone_bg,
            guid_to_cone: HashMap::new(), cones_dirty: false,
            plane_scale: 100.0,
            nurbs_pick_meshes: HashMap::new(),
            nurbs_surfaces: HashMap::new(),
            brep_pick_meshes: HashMap::new(),
            nc_pick_pts: HashMap::new(),
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
        if !curve.is_valid() { return; }
        let guid = curve.guid().to_string();
        let (pts, _) = curve.to_polyline_adaptive(session_rust::Tolerance::ANGULARDEFLECTION, 0.0, 0.0);
        if pts.len() < 2 { return; }
        let instance_id = self.pick.allocate(&guid);
        let segs = pts_to_segments(&pts, instance_id);
        let seg_start = self.segments_cpu.len();
        self.segments_cpu.extend(segs);
        self.guid_to_seg.insert(guid.clone(), seg_start..self.segments_cpu.len());
        self.segments_dirty = true;
        self.nc_pick_pts.insert(guid.clone(), pts.iter().map(|p| [p[0], p[1], p[2]]).collect());
        let gly_start = self.glyphs_cpu.len();
        for p in [pts.first().unwrap(), pts.last().unwrap()] {
            let center = [p[0], p[1], p[2]];
            self.glyphs_cpu.push(GlyphPoint { center, radius: 0.0,
                color: [1.0; 4], instance_id, _pad: [0; 3] });
        }
        self.guid_to_glyph.insert(guid.clone(), gly_start..self.glyphs_cpu.len());
        self.glyphs_dirty = true;
        let color = color_to_rgba_f32(curve.linecolors.get(0).unwrap_or(&session_rust::Color::white()));
        self.write_instance(instance_id, color, device, queue);
    }

    pub fn add_nurbssurface(&mut self, surface: &session_rust::NurbsSurface, device: &wgpu::Device, queue: &wgpu::Queue) {
        let guid = surface.guid().to_string();
        let mut mesh = session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid::from_u_v(
            surface.clone(), 32, 32);
        let (vs, is) = mesh_to_vertices(&mesh);
        if vs.is_empty() { return; }
        let instance_id = self.pick.allocate(&guid);
        self.tri.allocate(&guid, &vs, Some(&is), instance_id, device, queue);
        // Boundary edges: 4 iso-curves.
        // iso_curve(0,v) = u-direction curve at fixed v → use v-domain (dir=1) for parameter.
        // iso_curve(1,u) = v-direction curve at fixed u → use u-domain (dir=0) for parameter.
        let mut edge_segs: Vec<CylinderSegment> = Vec::new();
        let domains = [surface.domain(0), surface.domain(1)];
        for (iso_dir, param_dir, t_is_max) in [(0usize, 1usize, false), (0, 1, true), (1, 0, false), (1, 0, true)] {
            let t = if let Some((t0, t1)) = domains[param_dir] {
                if t_is_max { t1 } else { t0 }
            } else { continue };
            if let Some(crv) = surface.iso_curve(iso_dir, t) {
                let (pts, _) = crv.to_polyline_adaptive(
                    session_rust::Tolerance::ANGULARDEFLECTION * 0.1, 0.0, 0.0);
                edge_segs.extend(pts_to_segments(&pts, instance_id));
            }
        }
        if !edge_segs.is_empty() {
            let seg_start = self.segments_cpu.len();
            self.segments_cpu.extend(edge_segs);
            self.guid_to_seg.insert(guid.clone(), seg_start..self.segments_cpu.len());
            self.segments_dirty = true;
        }
        mesh.ensure_triangle_bvh();
        self.nurbs_pick_meshes.insert(guid.clone(), mesh);
        self.nurbs_surfaces.insert(guid.clone(), surface.clone());
        let surf_color = color_to_rgba_f32(surface.linecolors.get(0).unwrap_or(&session_rust::Color::white()));
        self.write_instance_flags(instance_id, surf_color, InstanceData::FLAG_SMOOTH, device, queue);
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
                let pts = pointcloud_to_cloud_points(pc, instance_id);
                if pts.is_empty() { self.pick.release(guid); return; }
                let start = self.clouds_cpu.len();
                self.clouds_cpu.extend(pts);
                self.guid_to_cloud.insert(guid.to_string(), start..self.clouds_cpu.len());
                self.clouds_dirty = true;
                self.write_instance(instance_id, [1.0, 1.0, 1.0, 1.0], device, queue);
            }
            Geometry::Mesh(m) => {
                let (vs, is) = mesh_to_vertices(m);
                self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                let segs = mesh_edges_to_segments(m, instance_id);
                if !segs.is_empty() {
                    let seg_start = self.segments_cpu.len();
                    self.segments_cpu.extend(segs);
                    self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                    self.segments_dirty = true;
                }
                let gly_start = self.glyphs_cpu.len();
                self.glyphs_cpu.extend(mesh_vertex_glyphs(m, instance_id));
                if self.glyphs_cpu.len() > gly_start {
                    self.guid_to_glyph.insert(guid.to_string(), gly_start..self.glyphs_cpu.len());
                    self.glyphs_dirty = true;
                }
                self.write_instance(instance_id, color_to_rgba_f32(m.objectcolor()), device, queue);
            }
            Geometry::Plane(pl) => {
                let segs = plane_to_axis_segments(pl, self.plane_scale, instance_id);
                let seg_start = self.segments_cpu.len();
                self.segments_cpu.extend(segs);
                self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                self.segments_dirty = true;
                let glyph = plane_origin_glyph(pl, instance_id);
                let gly_start = self.glyphs_cpu.len();
                self.glyphs_cpu.push(glyph);
                self.guid_to_glyph.insert(guid.to_string(), gly_start..self.glyphs_cpu.len());
                self.glyphs_dirty = true;
                self.write_instance(instance_id, [1.0, 1.0, 1.0, 1.0], device, queue);
            }
            Geometry::OBB(bb) => {
                let (vs, is) = obb_to_line_vertices(bb);
                self.line.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                self.write_instance(instance_id, [1.0, 1.0, 1.0, 1.0], device, queue);
            }
            Geometry::BRep(b) => {
                // Tessellate each BRep face independently (trimmed where needed, reversed
                // normals applied) so face boundaries are hard edges with correct shading.
                let mut vs: Vec<MeshVertex> = Vec::new();
                let mut is: Vec<u32> = Vec::new();
                for fm in b.face_meshes() {
                    let (fvs, fis) = mesh_to_vertices(&fm);
                    if fvs.is_empty() { continue; }
                    let offset = vs.len() as u32;
                    vs.extend(fvs);
                    is.extend(fis.iter().map(|&i| i + offset));
                }
                let mut bm = b.mesh();
                if !vs.is_empty() {
                    self.tri.allocate(guid, &vs, Some(&is), instance_id, device, queue);
                }
                let xf = b.xform.to_cols();
                // Edge segments stay in local space — inst.model applied in cylinder.wgsl
                let mut edge_segs: Vec<CylinderSegment> = Vec::new();
                if !b.m_curves_3d.is_empty() {
                    for curve in &b.m_curves_3d {
                        if !curve.is_valid() { continue; }
                        let (pts, _) = curve.to_polyline_adaptive(
                            session_rust::Tolerance::ANGULARDEFLECTION, 0.0, 0.0);
                        edge_segs.extend(pts_to_segments(&pts, instance_id));
                    }
                } else {
                    edge_segs.extend(mesh_naked_edges_to_segments(&bm, instance_id));
                }
                if edge_segs.is_empty() && vs.is_empty() { self.pick.release(guid); return; }
                if !edge_segs.is_empty() {
                    let seg_start = self.segments_cpu.len();
                    self.segments_cpu.extend(edge_segs);
                    self.guid_to_seg.insert(guid.to_string(), seg_start..self.segments_cpu.len());
                    self.segments_dirty = true;
                }
                let surf_color = color_to_rgba_f32(&b.surfacecolor);
                self.write_instance_model_flags(instance_id, surf_color, InstanceData::FLAG_SMOOTH, xf, device, queue);
                bm.ensure_triangle_bvh();
                self.brep_pick_meshes.insert(guid.to_string(), (bm, xf));
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
        if let Some(r) = self.guid_to_cloud.remove(guid) {
            let n = r.end - r.start;
            self.clouds_cpu.drain(r.start..r.end);
            for range in self.guid_to_cloud.values_mut() {
                if range.start >= r.start + n { range.start -= n; range.end -= n; }
            }
            self.clouds_dirty = true;
        }
        if let Some(r) = self.guid_to_cone.remove(guid) {
            let n = r.end - r.start;
            self.cones_cpu.drain(r.start..r.end);
            for range in self.guid_to_cone.values_mut() {
                if range.start >= r.start + n { range.start -= n; range.end -= n; }
            }
            self.cones_dirty = true;
        }
        self.pick.release(guid);
        self.nurbs_pick_meshes.remove(guid);
        self.nurbs_surfaces.remove(guid);
        self.brep_pick_meshes.remove(guid);
        self.nc_pick_pts.remove(guid);
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
        self.clouds_cpu.clear();
        self.guid_to_cloud.clear();
        self.clouds_dirty = true;
        self.cones_cpu.clear();
        self.guid_to_cone.clear();
        self.cones_dirty = true;
        self.nurbs_pick_meshes.clear();
        self.nurbs_surfaces.clear();
        self.brep_pick_meshes.clear();
        self.nc_pick_pts.clear();
    }

    fn write_instance(&mut self, instance_id: u32, color: [f32; 4], device: &wgpu::Device, queue: &wgpu::Queue) {
        self.write_instance_flags(instance_id, color, 0, device, queue);
    }

    fn write_instance_flags(&mut self, instance_id: u32, color: [f32; 4], flags: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.write_instance_model_flags(instance_id, color, flags, identity_matrix(), device, queue);
    }

    fn write_instance_model_flags(&mut self, instance_id: u32, color: [f32; 4], flags: u32, model: [[f32; 4]; 4], device: &wgpu::Device, queue: &wgpu::Queue) {
        let id = instance_id as usize;
        if id >= self.instances_cpu.len() {
            self.instances_cpu.resize(id + 1, InstanceData::new(0));
        }
        let mut data = InstanceData::new(instance_id);
        data.color = color;
        data.flags = flags;
        data.model = model;
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

    /// Ray-test cached NurbsSurface tessellations (BVH pre-built at load). Returns (guid, hit_point) pairs.
    pub fn pick_nurbssurfaces(
        &mut self,
        origin: &session_rust::Point,
        direction: &session_rust::Vector,
    ) -> Vec<(String, session_rust::Point)> {
        let dir_len = (direction[0]*direction[0] + direction[1]*direction[1] + direction[2]*direction[2]).sqrt();
        if dir_len <= 0.0 { return Vec::new(); }
        let du = session_rust::Vector::new(direction[0]/dir_len, direction[1]/dir_len, direction[2]/dir_len);
        let far = 1e6f32;
        let end = session_rust::Point::new(origin[0]+du[0]*far, origin[1]+du[1]*far, origin[2]+du[2]*far);
        let ray = session_rust::Line::from_points(origin, &end);
        let mut hits: Vec<(String, session_rust::Point, f32)> = Vec::new();
        for (guid, mesh) in &mut self.nurbs_pick_meshes {
            if let Some(p) = mesh.ray_cast_bvh(&ray, 1e-6) {
                let dx = p[0]-origin[0]; let dy = p[1]-origin[1]; let dz = p[2]-origin[2];
                let dist = (dx*dx + dy*dy + dz*dz).sqrt();
                hits.push((guid.clone(), p, dist));
            }
        }
        hits.sort_by(|a,b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().map(|(g,p,_)| (g,p)).collect()
    }

    /// Ray-test pre-tessellated BRep meshes (BVH pre-built at load). Returns (guid, hit_point) pairs.
    pub fn pick_breps(
        &mut self,
        origin: &session_rust::Point,
        direction: &session_rust::Vector,
    ) -> Vec<(String, session_rust::Point)> {
        let dir_len = (direction[0]*direction[0] + direction[1]*direction[1] + direction[2]*direction[2]).sqrt();
        if dir_len <= 0.0 { return Vec::new(); }
        let du = session_rust::Vector::new(direction[0]/dir_len, direction[1]/dir_len, direction[2]/dir_len);
        let far = 1e6f32;
        let mut hits: Vec<(String, session_rust::Point, f32)> = Vec::new();
        for (guid, (mesh, xf)) in &mut self.brep_pick_meshes {
            // Full 4×4 matrix inverse — correct for rotation, translation, AND scale.
            let inv = mat4_inverse_cm(xf);
            let inv_xp = |p: [f32;3]| -> [f32;3] {
                let w = inv[0][0]*p[0]+inv[1][0]*p[1]+inv[2][0]*p[2]+inv[3][0];
                let x = inv[0][1]*p[0]+inv[1][1]*p[1]+inv[2][1]*p[2]+inv[3][1];
                let y = inv[0][2]*p[0]+inv[1][2]*p[1]+inv[2][2]*p[2]+inv[3][2];
                let hw = inv[0][3]*p[0]+inv[1][3]*p[1]+inv[2][3]*p[2]+inv[3][3];
                let s = if hw.abs() > 1e-30 { 1.0 / hw } else { 1.0 };
                [w*s, x*s, y*s]
            };
            let lo = inv_xp([origin[0], origin[1], origin[2]]);
            let end_w = [origin[0]+du[0]*far, origin[1]+du[1]*far, origin[2]+du[2]*far];
            let le = inv_xp(end_w);
            let local_ray = session_rust::Line::from_points(
                &session_rust::Point::new(lo[0], lo[1], lo[2]),
                &session_rust::Point::new(le[0], le[1], le[2]),
            );
            if let Some(lp) = mesh.ray_cast_bvh(&local_ray, 1e-6) {
                // Transform hit back to world space
                let wp = [
                    xf[0][0]*lp[0]+xf[1][0]*lp[1]+xf[2][0]*lp[2]+xf[3][0],
                    xf[0][1]*lp[0]+xf[1][1]*lp[1]+xf[2][1]*lp[2]+xf[3][1],
                    xf[0][2]*lp[0]+xf[1][2]*lp[1]+xf[2][2]*lp[2]+xf[3][2],
                ];
                let p = session_rust::Point::new(wp[0], wp[1], wp[2]);
                let dx = wp[0]-origin[0]; let dy = wp[1]-origin[1]; let dz = wp[2]-origin[2];
                let dist = (dx*dx + dy*dy + dz*dz).sqrt();
                hits.push((guid.clone(), p, dist));
            }
        }
        hits.sort_by(|a,b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().map(|(g,p,_)| (g,p)).collect()
    }

    /// Ray-test cached NurbsCurve polylines. Returns (guid, closest_point) pairs within pick_radius.
    pub fn pick_nurbscurves(
        &self,
        origin: &session_rust::Point,
        direction: &session_rust::Vector,
        pick_radius: f32,
    ) -> Vec<(String, session_rust::Point)> {
        let ox = origin[0]; let oy = origin[1]; let oz = origin[2];
        let dx = direction[0]; let dy = direction[1]; let dz = direction[2];
        let mut hits: Vec<(String, session_rust::Point, f32)> = Vec::new();
        for (guid, pts) in &self.nc_pick_pts {
            let mut best_t = f32::MAX;
            for seg in pts.windows(2) {
                let [ax, ay, az] = seg[0];
                let [bx, by, bz] = seg[1];
                let wx = ax - ox; let wy = ay - oy; let wz = az - oz;
                let abx = bx - ax; let aby = by - ay; let abz = bz - az;
                let ab2 = abx*abx + aby*aby + abz*abz;
                if ab2 < 1e-10 { continue; }
                let d_dot_ab = dx*abx + dy*aby + dz*abz;
                let w_dot_ab = wx*abx + wy*aby + wz*abz;
                let w_dot_d  = wx*dx + wy*dy + wz*dz;
                let d2 = dx*dx + dy*dy + dz*dz;
                let denom = d2*ab2 - d_dot_ab*d_dot_ab;
                let (s, t) = if denom.abs() < 1e-10 {
                    (0.0f32, w_dot_ab / ab2)
                } else {
                    let s = (d_dot_ab*w_dot_ab - ab2*w_dot_d) / denom;
                    let t = (d_dot_ab*s - w_dot_ab) / (-ab2);
                    (s.max(0.0), t.clamp(0.0, 1.0))
                };
                let px = ox + dx*s - (ax + abx*t);
                let py = oy + dy*s - (ay + aby*t);
                let pz = oz + dz*s - (az + abz*t);
                let dist = (px*px + py*py + pz*pz).sqrt();
                if dist < pick_radius && s < best_t {
                    best_t = s;
                    let cx = ax + abx*t; let cy2 = ay + aby*t; let cz2 = az + abz*t;
                    hits.retain(|(g, _, _)| g != guid);
                    hits.push((guid.clone(), session_rust::Point::new(cx, cy2, cz2), s));
                }
            }
        }
        hits.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        hits.into_iter().map(|(g, p, _)| (g, p)).collect()
    }

    pub fn draw_clouds<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if self.clouds_cpu.is_empty() { return; }
        pass.draw(0..6, 0..self.clouds_cpu.len() as u32);
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
        if self.clouds_dirty {
            let needed = (self.clouds_cpu.len().max(1) * std::mem::size_of::<CloudPoint>()) as u64;
            if needed > self.clouds_gpu.size() {
                let new_size = (self.clouds_gpu.size() * 2).max(needed);
                self.clouds_gpu = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gpu_session.clouds"),
                    size: new_size,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.cloud_bg = make_geom_bind_group(device, geom_bgl, &self.clouds_gpu);
            }
            if !self.clouds_cpu.is_empty() {
                queue.write_buffer(&self.clouds_gpu, 0, bytemuck::cast_slice(&self.clouds_cpu));
            }
            self.clouds_dirty = false;
        }
        if self.cones_dirty {
            let needed = (self.cones_cpu.len().max(1) * std::mem::size_of::<CylinderSegment>()) as u64;
            if needed > self.cones_gpu.size() {
                let new_size = (self.cones_gpu.size() * 2).max(needed);
                self.cones_gpu = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gpu_session.cones"),
                    size: new_size,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.cone_bg = make_geom_bind_group(device, geom_bgl, &self.cones_gpu);
            }
            if !self.cones_cpu.is_empty() {
                queue.write_buffer(&self.cones_gpu, 0, bytemuck::cast_slice(&self.cones_cpu));
            }
            self.cones_dirty = false;
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
