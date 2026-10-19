        self.mesh_runs.bytes()+self.marker_runs.bytes()
            +(self.faces.capacity() + self.edges.capacity() + self.controls.capacity())