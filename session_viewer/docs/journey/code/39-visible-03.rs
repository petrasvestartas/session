        for object in self.objects.iter().filter(|row| row.visible()) {
            for vertex in object.mesh.vertices() {