    }

    pub fn bytes(&self) -> usize {
        self.ink.bytes()+self.dot_styles.capacity()*16+self.dots.capacity()*4+self.dot_colours.capacity()*16+self.pens.capacity()*std::mem::size_of::<crate::mesh_pens::Pen>()+self.faces.capacity() * std::mem::size_of::<usize>() + self.triangles.capacity() * 4
            + self.edges.capacity() * std::mem::size_of::<(usize, usize)>() + self.endpoints.capacity() * 48
    }
}
