
impl Index {
    pub fn new(scene: &crate::scene::Scene) -> Self { Self { targets:crate::snap_sources::collect(scene),screen:None,bins:Default::default(),builds:0,visits:0 } }
    pub fn excluding(mut self, ids: &[crate::scene::ObjectId]) -> Self {
        self.targets.retain(|target| match target { Target::Point(hit)=>hit.owner.is_none_or(|id| !ids.contains(&id)), Target::Wire { owner,.. }=>!ids.contains(owner) }); self
    }
    pub fn query(&mut self, screen: Screen, ray: &crate::camera::Ray, cursor: [f64; 2], modes: u8, from: Option<[f64; 3]>) -> Option<Hit> {
        if cursor.iter().chain(screen.size.iter()).any(|v| !v.is_finite()) || screen.size.iter().any(|v| *v<=0.) { return None; }
        if self.screen.as_ref()!=Some(&screen) {
