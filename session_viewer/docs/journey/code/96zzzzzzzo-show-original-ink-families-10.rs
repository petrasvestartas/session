        self.target=target.filter(|t|t.valid(scene));
        self.edge=self.target.is_some_and(|t|matches!(t.part,Part::BRepEdge(_)|Part::SurfaceEdge(_))||matches!(t.part,Part::MeshEdge(_,_))&&!scene.objects().iter().find(|r|r.id==t.owner).is_some_and(|r|matches!(&r.original,Some(session_rust::Geometry::Plane(_)|session_rust::Geometry::OBB(_)))));
        self.mesh=None;