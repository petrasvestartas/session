//! GPU instancing: template registration + per-instance add/update/remove/pick.

use crate::gpu_instance_groups::{InstanceHandle, TemplateKey, TEMPLATE_INSTANCE_BASE};
use crate::pipelines::build_bind_group;
use super::types::*;

impl GpuSession {
    // ── Template instancing API ───────────────────────────────────────────────

    /// Upload template mesh geometry once. Idempotent — no-op if `key` is already registered.
    ///
    /// Obtain `verts` / `inds` from `gpu_adapters::mesh_to_vertices(mesh)`.
    /// `key` is the stable shape identifier ("bolt_m6", "chair_v1", …).
    pub fn register_template_mesh(
        &mut self,
        key: &TemplateKey,
        verts: &[MeshVertex],
        inds: &[u32],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        if self.template_tri.slot(&key.0).is_some() { return; }
        // instance_id = 0 is unused for template slots (draw_meshes never iterates template_tri).
        self.template_tri.allocate(&key.0, verts, Some(inds), 0, device, queue);
        let (_, needs_grow) = self.instance_groups.ensure_group(key.clone(), 16);
        if needs_grow {
            let required = TEMPLATE_INSTANCE_BASE + self.instance_groups.bump;
            self.grow_instance_buffer(required, device, queue);
        }
    }

    /// Add one instance of a registered template with the given world transform.
    /// Returns an `InstanceHandle` the caller must store for future `update_instance` /
    /// `set_instance_flag` / `remove_instance` calls.
    ///
    /// `model`:  column-major 4×4 world-space transform
    /// `color`:  RGBA tint `[0.0 .. 1.0]`
    /// `flags`:  `InstanceData::FLAG_*` bits (`0` = visible, not selected)
    pub fn add_instance(
        &mut self,
        key: &TemplateKey,
        guid: &str,
        model: [[f32; 4]; 4],
        color: [f32; 4],
        flags: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> InstanceHandle {
        let (_, needs_grow) = self.instance_groups.ensure_group(key.clone(), 16);
        if needs_grow {
            let required = TEMPLATE_INSTANCE_BASE + self.instance_groups.bump;
            self.grow_instance_buffer(required, device, queue);
        }
        let (slot, needs_grow2, new_bump) = self.instance_groups.alloc_in_group(key);
        if needs_grow2 {
            let required = TEMPLATE_INSTANCE_BASE + new_bump;
            self.grow_instance_buffer(required, device, queue);
        }
        let group = self.instance_groups.groups.get_mut(key).unwrap();
        let abs_idx = group.abs_index(slot);
        let mut data = InstanceData::new(abs_idx);
        data.model = model;
        data.color = color;
        data.flags = flags;
        group.instances_cpu[slot as usize] = data;
        group.guid_to_slot.insert(guid.to_string(), slot);
        group.live_count += 1;
        group.dirty = true;
        self.register_instance_pick_id(abs_idx, guid);
        InstanceHandle { group_key: key.clone(), slot_index: slot, guid: guid.to_string() }
    }

    /// Update transform, color, and flags of an existing instance.
    /// Writes 96 bytes to the GPU immediately — no dirty flag, no full re-upload.
    /// Returns `false` if the handle is stale.
    #[allow(dead_code)]
    pub fn update_instance(
        &mut self,
        handle: &InstanceHandle,
        model: [[f32; 4]; 4],
        color: [f32; 4],
        flags: u32,
        queue: &wgpu::Queue,
    ) -> bool {
        let group = match self.instance_groups.groups.get_mut(&handle.group_key) {
            Some(g) => g,
            None => return false,
        };
        let slot = handle.slot_index as usize;
        if slot >= group.instances_cpu.len() { return false; }
        let abs_idx = group.abs_index(handle.slot_index);
        let mut data = InstanceData::new(abs_idx);
        data.model = model;
        data.color = color;
        data.flags = flags;
        group.instances_cpu[slot] = data;
        let offset = (abs_idx as u64) * (std::mem::size_of::<InstanceData>() as u64);
        queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&data));
        true
    }

    /// Set or clear a single flag bit on an instance. Writes 96 bytes to the GPU immediately.
    /// Returns `false` if the handle is stale.
    #[allow(dead_code)]
    pub fn set_instance_flag(
        &mut self,
        handle: &InstanceHandle,
        flag: u32,
        on: bool,
        queue: &wgpu::Queue,
    ) -> bool {
        let group = match self.instance_groups.groups.get_mut(&handle.group_key) {
            Some(g) => g,
            None => return false,
        };
        let slot = handle.slot_index as usize;
        if slot >= group.instances_cpu.len() { return false; }
        if on { group.instances_cpu[slot].flags |= flag; }
        else   { group.instances_cpu[slot].flags &= !flag; }
        let abs_idx = group.abs_index(handle.slot_index);
        let offset = (abs_idx as u64) * (std::mem::size_of::<InstanceData>() as u64);
        queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&group.instances_cpu[slot]));
        true
    }

    /// Logically remove an instance: marks its slot `FLAG_HIDDEN` (96-byte GPU write),
    /// returns the slot to the free list for reuse, and unregisters it from the pick table.
    /// No GPU buffer compaction. O(1).
    #[allow(dead_code)]
    pub fn remove_instance(&mut self, handle: &InstanceHandle, queue: &wgpu::Queue) {
        let group = match self.instance_groups.groups.get_mut(&handle.group_key) {
            Some(g) => g,
            None => return,
        };
        if let Some(&slot) = group.guid_to_slot.get(&handle.guid) {
            let abs_idx = group.abs_index(slot);
            let mut hidden = InstanceData::new(abs_idx);
            hidden.flags = InstanceData::FLAG_HIDDEN;
            group.instances_cpu[slot as usize] = hidden;
            group.free_slots.push(slot);
            group.live_count = group.live_count.saturating_sub(1);
            group.guid_to_slot.remove(&handle.guid);
            let offset = (abs_idx as u64) * (std::mem::size_of::<InstanceData>() as u64);
            queue.write_buffer(&self.instance_buffer, offset, bytemuck::bytes_of(&hidden));
            self.unregister_instance_pick_id(abs_idx);
        }
    }

    /// CPU-side sphere-approximate ray pick across all live instances of a template.
    /// Returns `(guid, approx_distance)` pairs sorted nearest-first.
    ///
    /// `bbox_radius`: bounding sphere radius of the template mesh in world-space units
    /// (compute once from the mesh AABB at registration time and cache it).
    #[allow(dead_code)]
    pub fn pick_instance_group_by_ray(
        &self,
        key: &TemplateKey,
        ray_origin: [f32; 3],
        ray_dir: [f32; 3],
        bbox_radius: f32,
    ) -> Vec<(String, f32)> {
        let group = match self.instance_groups.groups.get(key) {
            Some(g) => g,
            None => return Vec::new(),
        };
        let mut hits: Vec<(String, f32)> = Vec::new();
        for (guid, &slot) in &group.guid_to_slot {
            let data = &group.instances_cpu[slot as usize];
            if (data.flags & InstanceData::FLAG_HIDDEN) != 0 { continue; }
            // Translation is in column 3 of the column-major model matrix.
            let cx = data.model[3][0];
            let cy = data.model[3][1];
            let cz = data.model[3][2];
            let ox = cx - ray_origin[0];
            let oy = cy - ray_origin[1];
            let oz = cz - ray_origin[2];
            let tca = ox * ray_dir[0] + oy * ray_dir[1] + oz * ray_dir[2];
            let d2  = ox*ox + oy*oy + oz*oz - tca*tca;
            let r2  = bbox_radius * bbox_radius;
            if d2 > r2 { continue; }
            let dist = tca - (r2 - d2).sqrt();
            if dist >= 0.0 { hits.push((guid.clone(), dist)); }
        }
        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        hits
    }

    /// If `instance_buffer` was replaced since the last call, builds and returns a
    /// new `wgpu::BindGroup` that references the new buffer.
    /// Returns `None` in the common case (no buffer replacement) — zero cost.
    /// Call once per frame before the geometry render pass opens.
    pub fn take_rebuilt_bind_group(
        &mut self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        camera_buf: &wgpu::Buffer,
    ) -> Option<wgpu::BindGroup> {
        if !self.bind_group_dirty { return None; }
        self.bind_group_dirty = false;
        Some(build_bind_group(device, layout, camera_buf, &self.instance_buffer))
    }

    fn register_instance_pick_id(&mut self, abs_id: u32, guid: &str) {
        let id = abs_id as usize;
        if id >= self.pick.instance_to_guid.len() {
            self.pick.instance_to_guid.resize(id + 1, None);
        }
        self.pick.instance_to_guid[id] = Some(guid.to_string());
        self.pick.guid_to_instance.insert(guid.to_string(), abs_id);
    }

    #[allow(dead_code)]
    fn unregister_instance_pick_id(&mut self, abs_id: u32) {
        let id = abs_id as usize;
        if id < self.pick.instance_to_guid.len() {
            if let Some(guid) = self.pick.instance_to_guid[id].take() {
                self.pick.guid_to_instance.remove(&guid);
            }
        }
    }
}
