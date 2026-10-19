use crate::{snap::{Hit, Kind, Settings}, snap_sources::Target, snap_math::Screen};

pub struct Index { targets: Vec<Target>, screen: Option<Screen>, bins: crate::snap_bins::Bins, pub builds: u64, pub visits: usize }

impl Index {
    pub fn new(scene: &crate::scene::Scene) -> Self { Self { targets:crate::snap_sources::collect(scene),screen:None,bins:Default::default(),builds:0,visits:0 } }
    pub fn query(&mut self, screen: Screen, ray: &crate::camera::Ray, cursor: [f64; 2], modes: u8, from: Option<[f64; 3]>) -> Option<Hit> {
        if cursor.iter().chain(screen.size.iter()).any(|v| !v.is_finite()) || screen.size.iter().any(|v| *v<=0.) { return None; }
        if self.screen.as_ref()!=Some(&screen) {
            self.bins=Default::default(); self.builds+=1;
            for (id,target) in self.targets.iter().enumerate() { match target {
                Target::Point(hit) => { if let Some(p)=screen.point(hit.point) { self.bins.insert(id,Some([p,p])); } }
                Target::Wire { ends,.. } => self.bins.insert(id,screen.point(ends[0]).zip(screen.point(ends[1])).map(|(a,b)| [a,b])),
            } }
            self.screen=Some(screen.clone());
        }
        let ids=self.bins.near(cursor,12.); self.visits=ids.len(); let mut winner=None;
        for id in ids { match &self.targets[id] {
            Target::Point(hit) => choose(&mut winner,*hit,&screen,cursor,modes),
            Target::Wire { ends,owner } => {
                choose(&mut winner,Hit { point:crate::snap_math::near(ends[0],ends[1],ray),kind:Kind::Near,owner:Some(*owner) },&screen,cursor,modes);
                if let Some(p)=from.and_then(|from| crate::snap_math::perpendicular(ends[0],ends[1],from)) {
                    choose(&mut winner,Hit { point:p,kind:Kind::Perp,owner:Some(*owner) },&screen,cursor,modes);
                }
            }
        } }
        winner.map(|(hit,_)| hit)
    }
}

// A preferred kind wins inside the aperture before distance breaks ties between candidates of that same kind.
fn choose(winner: &mut Option<(Hit,f64)>, hit: Hit, screen: &Screen, cursor: [f64; 2], modes: u8) {
    if modes & hit.kind.mode()==0 { return; }
    let Some(p)=screen.point(hit.point) else { return; }; let distance=(p[0]-cursor[0]).hypot(p[1]-cursor[1]);
    if distance>12. { return; }
    if winner.is_none_or(|(best,d)| hit.kind<best.kind || (hit.kind==best.kind && distance<d)) { *winner=Some((hit,distance)); }
}

pub fn fallback(settings: Settings, camera: &crate::camera::Camera, screen: [f32; 2]) -> Option<Hit> {
    let point=crate::tool_coords::on_xy(camera,screen)?;
    Some(if settings.grid { Hit { point:crate::snap_math::grid(point,[0.;3],[[1.,0.,0.],[0.,1.,0.]],settings.step)?,kind:Kind::Grid,owner:None } }
        else { Hit { point,kind:Kind::Near,owner:None } })
}
