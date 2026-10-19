    }
}

pub struct Prepared {pub source:std::rc::Rc<NurbsSurface>,pub grid:Grid}
impl Prepared {
    pub fn new(source:std::rc::Rc<NurbsSurface>)->Result<Self,&'static str> {let _=source.guid();let grid=Grid::new(&source)?;Ok(Self {source,grid})}
    pub fn mesh(&self)->Result<crate::prepared::PreparedMesh,&'static str> {
        let points=self.grid.samples.iter().map(|s|session_rust::Point::new(s.point[0],s.point[1],s.point[2])).collect();
        let faces=self.grid.indices.chunks_exact(3).map(|tri|tri.iter().map(|i|*i as usize).collect()).collect();
        let mut source=session_rust::Mesh::from_vertices_and_faces(points,faces);source.set_guid(self.source.guid().to_owned());source.name=self.source.name.clone();source.is_visible=self.source.is_visible;source.is_locked=self.source.is_locked;
        let colour=self.source.facecolors.first().cloned().unwrap_or_default().to_f32();source.set_objectcolor(session_rust::Color::new(colour[0],colour[1],colour[2],colour[3]));
        let vertices=self.grid.samples.iter().map(|s|[s.point[0] as f32,s.point[1] as f32,s.point[2] as f32,colour[0],colour[1],colour[2]]).collect();
        let normals=(0..self.grid.samples.len()).map(|i|self.grid.normal(i)).collect::<Result<Vec<_>,_>>()?;
        let mut display=crate::mesh::Mesh::new(vertices,self.grid.indices.clone())?.with_normals(normals)?;
        display.parts.faces=vec![0];display.parts.triangles=vec![0;self.grid.indices.len()/3];
        for boundary in &self.grid.boundaries {for &[a,b] in &boundary.segments {display.parts.edges.push((a as usize,b as usize));display.parts.endpoints.push([self.grid.samples[a as usize].point,self.grid.samples[b as usize].point]);}}
        Ok(crate::prepared::PreparedMesh {geometry:std::rc::Rc::new(source),display,source:None,model:session_rust::Xform::identity()})
    }
    pub fn edge_ids(&self)->impl Iterator<Item=usize>+'_ {self.grid.boundaries.iter().flat_map(|b|std::iter::repeat_n(b.edge,b.segments.len()))}
}

pub fn example()->NurbsSurface {
    let points=(0..3).flat_map(|i|(0..3).map(move|j|session_rust::Point::new(i as f64-1.,j as f64-1.,if i==1&&j==1 {1.2} else {0.}))).collect::<Vec<_>>();
    let mut source=NurbsSurface::create(false,false,2,2,3,3,&points).unwrap();source.name="Parametric roof".into();source.facecolors=vec![session_rust::Color::new(0.35,0.65,0.85,1.)];source
}
