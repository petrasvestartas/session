//! Render-pass draw calls, one per primitive pipeline.

use super::types::*;

impl GpuSession {
    /// Draw regular meshes then all template instance groups. Replaces `draw_meshes` in lib.rs.
    /// Both use the same mesh pipeline and bind group 0 — no pipeline switch needed.
    pub fn draw_all_mesh<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        self.draw_meshes(pass);
        self.draw_instance_groups(pass);
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

    fn draw_instance_groups<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        let ibo = match self.template_tri.ibo.as_ref() { Some(b) => b, None => return };
        if self.instance_groups.groups.is_empty() { return; }
        pass.set_vertex_buffer(0, self.template_tri.vbo.slice(..));
        pass.set_index_buffer(ibo.slice(..), wgpu::IndexFormat::Uint32);
        for group in self.instance_groups.groups.values() {
            if group.live_count == 0 { continue; }
            let slot = match self.template_tri.slot(&group.template_key.0) {
                Some(s) => s,
                None => continue,
            };
            let ir = match slot.index_range.clone() { Some(r) => r, None => continue };
            // Draw all capacity slots. Hidden ones discard immediately in mesh.wgsl line 57
            // (`if (in.flags & 2u) != 0u { discard; }`), so no geometry cost for empty slots.
            pass.draw_indexed(
                ir,
                slot.vertex_range.start as i32,
                group.first_instance()..(group.first_instance() + group.capacity),
            );
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
    pub fn draw_clouds<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if self.clouds_cpu.is_empty() { return; }
        pass.draw(0..6, 0..self.clouds_cpu.len() as u32);
    }
}
