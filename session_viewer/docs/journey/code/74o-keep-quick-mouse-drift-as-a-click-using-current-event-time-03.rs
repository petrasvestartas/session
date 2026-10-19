    pub fn cancel_pointer(&mut self,id:i32) {if self.active.as_ref().is_some_and(|d|d.id==id) {self.cancel();}}
}

// A quick press may wobble without moving geometry; a longer held press uses the narrower threshold for intentional dragging.
pub fn drags(moved:f64,held:f64)->bool {moved>10. || (moved>4. && held>=200.)}

