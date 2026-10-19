        candidates
    }
    pub fn stops(&self) -> (Vec<(Owner, Cutter)>, Vec<(Owner, Rc<session_rust::Mesh>)>) {
        let mut cutters = Vec::new();
        let mut meshes = Vec::new();
        if self.distance.is_none() {
            for owner in &self.boundaries {
                if let Some(plane) = source_cut::as_plane(&owner.source)
                    .and_then(|p| source_cut::plane_moved(&p, &owner.model))
                {
                    cutters.push((owner.clone(), Cutter::Plane(plane)))
                } else if let Some(curve) = source_cut::as_curve(&owner.source) {
                    cutters.push((
                        owner.clone(),
                        Cutter::Curve(curve.transformed(&owner.model)),
                    ))
                } else {
                    match &owner.source {
                        Geometry::Mesh(mesh) => meshes.push((owner.clone(), mesh.clone())),
                        Geometry::BRep(brep) => {
                            for mesh in brep.face_meshes_q(Some(crate::cad_boundary::QUALITY)) {
                                meshes.push((owner.clone(), Rc::new(mesh)))
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        (cutters, meshes)
    }
    pub fn start(&mut self, scene: &Scene) -> Result<(), String> {
        if self.distance.is_none() && self.boundaries.is_empty() {
            return Err("Select boundary objects, or type Distance and a length".into());
        }
        for owner in &self.boundaries {
            owner.valid(scene).map_err(str::to_owned)?
        }
        let candidates = Self::candidates(scene);
        let (cutters, meshes) = self.stops();
        self.cutters = cutters;
        self.meshes = meshes;
        self.last = None;
        self.candidates = candidates;
        self.curves = true;
        self.asking = false;
        self.hover = None;
        Ok(())
    }
    pub fn new(
        scene: &Scene,
        before: Vec<ObjectId>,
        distance: Option<f64>,
        asking: bool,
    ) -> Result<Self, String> {
        let mut result = Self {
            before: before.clone(),
            boundaries: Vec::new(),
            curves: false,
            distance,
            asking,
            candidates: Vec::new(),
            hover: None,
            error: String::new(),
            cutters: Vec::new(),
            meshes: Vec::new(),
            last: None,
        };
        if distance.is_some() {
            result.start(scene)?
        } else {
            for id in before {
                let _ = result.pick(scene, Some(id));
            }
        }
        Ok(result)
    }
}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
