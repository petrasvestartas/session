#[derive(Default)]
pub struct Recovery { reduced: bool, requested: bool }

impl Recovery {
    pub fn adopted() -> Self { Self { reduced: true, requested: false } }
    pub fn reduced(&self) -> bool { self.reduced }

    pub fn request(&mut self, device_lost: bool) -> bool {
        if !device_lost || self.reduced || self.requested { return false; }
        self.requested = true; true
    }
}
