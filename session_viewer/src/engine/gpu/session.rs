//! GpuSession lifecycle: construction, full rebuild, and per-frame buffer flush.

use std::collections::HashMap;
use crate::gpu_arena::GpuArena;
use crate::gpu_instance_groups::{InstanceGroupAllocator, TEMPLATE_INSTANCE_BASE};
use super::types::*;

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

        let template_tri = GpuArena::<MeshVertex>::new(device, "gpu_session.template_tri", 8_192, 32_768);

        Self {
            tri, line, point,
            instance_buffer, instance_capacity: DEFAULT_INSTANCE_CAP, instances_cpu: Vec::new(),
            pick: PickTable::new(), default_tints: HashMap::new(), default_face_tints: HashMap::new(),
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
            template_tri,
            instance_groups: InstanceGroupAllocator::new(),
            bind_group_dirty: false,
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
        // Flush dirty template instance groups into the upper half of instance_buffer.
        for group in self.instance_groups.groups.values_mut() {
            if !group.dirty { continue; }
            let offset = (TEMPLATE_INSTANCE_BASE + group.block_start) as u64
                         * std::mem::size_of::<InstanceData>() as u64;
            queue.write_buffer(&self.instance_buffer, offset, bytemuck::cast_slice(&group.instances_cpu));
            group.dirty = false;
        }
    }
}
