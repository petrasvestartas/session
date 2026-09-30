
/// Textures and tiles the ink bind group reads.
pub struct InkScene<'a> {
    pub tiles: &'a super::triangle_tiles::TriangleTiles, // screen tiles for visibility tests; register:tiles
    pub targets: &'a Targets,                            // depth and triangle id textures
}

/// Bind group 2 for ink lanes: rows, depth, triangle ids, tiles.
fn ink_instance_group(
    ctx: &GpuCtx,
    l: &Layouts,
    label: &str,
    buffers: [&wgpu::Buffer; 2],
    depths: [&wgpu::TextureView; 2],
    gradients: [&wgpu::TextureView; 2],
    tiles: &super::triangle_tiles::TriangleTiles, // register:tiles
) -> wgpu::BindGroup {
    let view = wgpu::BindingResource::TextureView;
    resource_group(ctx, &l.ink_instance, label, [
        (0, buffers[0].as_entire_binding()),
        (1, buffers[1].as_entire_binding()),
        (2, view(depths[0])),
        (3, view(depths[1])),
        (4, view(gradients[0])),
        (5, view(gradients[1])),
        (6, tiles.projected.as_entire_binding()), // register:tiles
        (7, tiles.buffer.as_entire_binding()),    // register:tiles
    ])
}

impl InstanceTable {
    /// Group 2 for ink lanes, with depth textures.
    pub fn ink_group(&self) -> &wgpu::BindGroup {
        self.ink_group
            .as_ref()
            .expect("the ink bind group is built with the GPU")
    }

    /// Rebuild the ink bind group.
    pub fn rebind_ink(&mut self, ctx: &GpuCtx, l: &Layouts, scene: &InkScene) {
        let t = scene.targets;
        self.ink_group = Some(ink_instance_group(
            ctx,
            l,
            "ink.instances.bind_group",
            [&self.buffer.buf, &self.translations.buf],
            [&t.depth_single, &t.depth_msaa],
            [&t.gradient_single, &t.gradient_msaa],
            scene.tiles, // register:tiles
        ));
    }

    /// An ink bind group over the pick pass's own depth textures.
    pub fn pick_group(
        &self,
        ctx: &GpuCtx,
        layouts: &Layouts,
        depths: [&wgpu::TextureView; 2],
        gradients: [&wgpu::TextureView; 2],
        tiles: &super::triangle_tiles::TriangleTiles, // register:tiles
    ) -> wgpu::BindGroup {
        ink_instance_group(
            ctx,
            layouts,
            "pick.instances",
            [&self.buffer.buf, &self.translations.buf],
            depths,
            gradients,
            tiles, // register:tiles
        )
    }
}
