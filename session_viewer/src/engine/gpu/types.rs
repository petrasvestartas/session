//! GPU vertex/instance layout types, the pick table, and the GpuSession struct.

use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use crate::gpu_arena::GpuArena;
use crate::gpu_instance_groups::{InstanceGroupAllocator, TEMPLATE_INSTANCE_BASE};

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

/// Per-object GPU data. 112 bytes, 16-byte aligned.
/// `color` = tint for lines/points/cylinders; `face_color` = tint for mesh faces only.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct InstanceData {
    pub model:      [[f32; 4]; 4],
    pub color:      [f32; 4],
    pub face_color: [f32; 4],
    pub object_id:  u32,
    pub flags:      u32,
    pub _pad:       [u32; 2],
}

impl InstanceData {
    pub const FLAG_SELECTED:      u32 = 1 << 0;
    pub const FLAG_HIDDEN:        u32 = 1 << 1;
    pub const FLAG_SMOOTH:        u32 = 1 << 2;
    pub const FLAG_EDGES_HIDDEN:  u32 = 1 << 3;
    pub const FLAG_GLYPHS_HIDDEN: u32 = 1 << 4;
    /// When set, cylinder/sphere shaders use inst.tint instead of per-segment baked color.
    pub const FLAG_TINT_OVERRIDE: u32 = 1 << 5;

    pub fn new(instance_id: u32) -> Self {
        Self {
            model:      identity_matrix(),
            color:      [1.0, 1.0, 1.0, 1.0],
            face_color: [1.0, 1.0, 1.0, 1.0],
            object_id:  instance_id,
            flags:      0,
            _pad:       [0, 0],
        }
    }
}

/// Full 4×4 inverse for a column-major matrix. Returns identity on singular input.
pub(crate) fn mat4_inverse_cm(m: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
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

pub(crate) fn identity_matrix() -> [[f32; 4]; 4] {
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
    pub instance_to_guid: Vec<Option<String>>,
    pub guid_to_instance: HashMap<String, u32>,
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

pub(crate) const DEFAULT_TRI_VERTS: u32 = 65536;
pub(crate) const DEFAULT_TRI_INDS: u32 = 131072;
pub(crate) const DEFAULT_LINE_VERTS: u32 = 8192;
pub(crate) const DEFAULT_LINE_INDS: u32 = 16384;
pub(crate) const DEFAULT_POINT_VERTS: u32 = 4096;
// Instance buffer covers regular objects [0..TEMPLATE_INSTANCE_BASE) plus an
// initial template region of 128 slots. Regular scenes have far fewer than 16 384 objects.
pub(crate) const DEFAULT_INSTANCE_CAP: u32 = TEMPLATE_INSTANCE_BASE + 128;
pub(crate) const DEFAULT_SEGMENT_CAP: usize = 64;
pub(crate) const DEFAULT_GLYPH_CAP:   usize = 64;

pub struct GpuSession {
    pub tri:   GpuArena<MeshVertex>,
    pub line:  GpuArena<LineVertex>,
    pub point: GpuArena<PointVertex>,

    pub instance_buffer:   wgpu::Buffer,
    pub instance_capacity: u32,
    pub instances_cpu:     Vec<InstanceData>,

    pub pick: PickTable,
    pub default_tints: HashMap<String, [f32; 4]>,
    pub default_face_tints: HashMap<String, [f32; 4]>,

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

    // ── Template instancing ───────────────────────────────────────────────────

    /// Separate arena for template mesh geometry. Never iterated by `draw_meshes()`;
    /// only used by `draw_all_mesh()` → `draw_instance_groups()`.
    pub template_tri: GpuArena<MeshVertex>,
    /// All template instance groups, bump-allocated in the upper half of `instance_buffer`.
    pub instance_groups: InstanceGroupAllocator,
    /// Set `true` when `instance_buffer` is replaced so `lib.rs` rebuilds `bind_group`.
    pub bind_group_dirty: bool,
}
pub fn make_geom_bind_group(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, buf: &wgpu::Buffer) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("geom.bg"),
        layout,
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
    })
}
