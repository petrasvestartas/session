    #[cfg(test)]
    pub(crate) fn geometry_for_test(&self)->Option<&Rc<crate::gpu_geometry::GpuGeometry>> {self.mesh.as_ref().map(|m|&m.geometry)}
    pub fn rebase_face(