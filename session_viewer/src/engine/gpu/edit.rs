//! Per-object edits: color/flag/transform updates, removal, and instance writes.

use crate::gpu_instance_groups::TEMPLATE_INSTANCE_BASE;
use super::types::*;

impl GpuSession {
    pub fn reset_color(&mut self, guid: &str, queue: &wgpu::Queue) -> bool {
        let id = match self.pick.instance_id(guid) { Some(i) => i, None => return false };
        let idx = id as usize;
        if idx >= self.instances_cpu.len() { return false; }
        self.instances_cpu[idx].color      = self.default_tints.get(guid).copied().unwrap_or([1.0; 4]);
        self.instances_cpu[idx].face_color = self.default_face_tints.get(guid).copied().unwrap_or([1.0; 4]);
        self.instances_cpu[idx].flags &= !InstanceData::FLAG_TINT_OVERRIDE;
        let offset = (idx as u64) * (std::mem::size_of::<InstanceData>() as u64);
        queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&self.instances_cpu[idx]));
        true
    }

    pub fn set_face_color(&mut self, guid: &str, color: [f32; 4], queue: &wgpu::Queue) -> bool {
        let id = match self.pick.instance_id(guid) { Some(i) => i, None => return false };
        let idx = id as usize;
        if idx >= self.instances_cpu.len() { return false; }
        self.instances_cpu[idx].face_color = color;
        let offset = (idx as u64) * (std::mem::size_of::<InstanceData>() as u64);
        queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&self.instances_cpu[idx]));
        true
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
        self.template_tri.clear();
        self.instance_groups.groups.clear();
        self.instance_groups.bump = 0;
    }

    pub(crate) fn write_instance(&mut self, instance_id: u32, color: [f32; 4], device: &wgpu::Device, queue: &wgpu::Queue) {
        self.write_instance_flags(instance_id, color, 0, device, queue);
    }

    pub(crate) fn write_instance_flags(&mut self, instance_id: u32, color: [f32; 4], flags: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.write_instance_model_flags(instance_id, color, flags, identity_matrix(), device, queue);
    }

    pub(crate) fn write_instance_model_flags(&mut self, instance_id: u32, color: [f32; 4], flags: u32, model: [[f32; 4]; 4], device: &wgpu::Device, queue: &wgpu::Queue) {
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

    pub(crate) fn grow_instance_buffer(&mut self, needed: u32, device: &wgpu::Device, queue: &wgpu::Queue) {
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
        self.bind_group_dirty = true;
        self.instance_groups.capacity = new_cap.saturating_sub(TEMPLATE_INSTANCE_BASE);
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

    pub fn set_color(&mut self, guid: &str, color: [f32; 4], queue: &wgpu::Queue) -> bool {
        let id = match self.pick.instance_id(guid) { Some(i) => i, None => return false };
        let idx = id as usize;
        if idx >= self.instances_cpu.len() { return false; }
        self.instances_cpu[idx].color = color;
        self.instances_cpu[idx].flags |= InstanceData::FLAG_TINT_OVERRIDE;
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
}
