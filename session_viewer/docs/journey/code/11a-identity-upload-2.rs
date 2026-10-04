        self.meshes = scene.objects().iter().map(|object| GpuMesh::upload(&self.device, &object.mesh)).collect();
