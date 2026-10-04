        let meshes = scene.objects().iter().map(|object| GpuMesh::upload(&device, &object.mesh)).collect();
