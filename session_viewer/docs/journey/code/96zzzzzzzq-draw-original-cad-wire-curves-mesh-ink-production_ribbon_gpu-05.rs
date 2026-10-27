        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) -> Frame {
        if (self.visible||self.edges_visible) && !self.rows.owners.is_empty() {
            self.pick.borrow_mut().get_or_insert_with(|| {
                pipeline(
                    device,
