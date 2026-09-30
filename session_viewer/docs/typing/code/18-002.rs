
impl ArenaLane {
    /// Reproject the triangles for this camera; bin them into screen tiles too when `lists`.
    pub fn prepare_visibility(
        &mut self,
        ctx: &GpuCtx,
        encoder: &mut wgpu::CommandEncoder,
        binds: &Binds,
        matrix: [f32; 16],
        objects_revision: u64,
        lists: bool,
    ) {
        self.tiles.encode(
            ctx,
            encoder,
            super::triangle_tiles::TileInput {
                binds,
                geometry: [&self.verts.buf, &self.vids.buf, &self.faces.buf],
                table: &self.table.view,
                matrix,
                objects_revision,
            },
            lists,
        );
    }
}
