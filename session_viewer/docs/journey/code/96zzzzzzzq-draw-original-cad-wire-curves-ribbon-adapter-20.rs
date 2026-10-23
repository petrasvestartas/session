    pub fn take(&mut self, view: crate::pick_view::View) -> Option<Result<Reply, &'static str>> {
        self.take_mapped(view,|pixel|pixel)
    }

    pub fn take_mapped(&mut self, view: crate::pick_view::View, remap: impl Fn([u32;2])->[u32;2]) -> Option<Result<Reply, &'static str>> {