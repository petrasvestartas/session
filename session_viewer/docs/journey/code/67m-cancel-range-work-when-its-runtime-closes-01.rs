impl Drop for Runtime {
    fn drop(&mut self) {
        crate::stream::reset_range_gate();
        self.request.borrow_mut().cancel();
        self.reload.borrow_mut().cancel();
