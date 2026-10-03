use crate::{gpu_geometry::GpuGeometry, mesh::Mesh};
use std::{collections::HashMap, rc::{Rc, Weak}};

#[derive(Default)]
pub struct GeometryCache {
    entries: HashMap<*const Mesh, Weak<GpuGeometry>>,
    pub uploads: usize,
}

impl GeometryCache {
    pub fn get(&mut self, device: &wgpu::Device, source: &Rc<Mesh>) -> Rc<GpuGeometry> {
        let key = Rc::as_ptr(source);
        if let Some(geometry) = self.entries.get(&key).and_then(Weak::upgrade) {
            return geometry;
        }
        let geometry = Rc::new(GpuGeometry::upload(device, Rc::clone(source)));
        self.entries.insert(key, Rc::downgrade(&geometry));
        self.uploads += 1;
        geometry
    }

    pub fn prune(&mut self) {
        self.entries.retain(|_, geometry| geometry.strong_count() > 0);
    }

    pub fn entries(&self) -> usize { self.entries.len() }
}
