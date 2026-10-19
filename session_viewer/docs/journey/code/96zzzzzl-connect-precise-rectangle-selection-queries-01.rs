use crate::scene::{Scene,ObjectId};

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rectangle {pub bounds:[f64;4],pub crossing:bool}

impl Rectangle {
    pub fn new(from:[f64;2],to:[f64;2])->Option<Self> {
        if from.into_iter().chain(to).any(|value|!value.is_finite()) {return None;}
        let bounds=[from[0].min(to[0]),from[1].min(to[1]),from[0].max(to[0]),from[1].max(to[1])];
        (bounds[0]<bounds[2]&&bounds[1]<bounds[3]).then_some(Self {bounds,crossing:to[0]<from[0]})
    }

    pub fn contains(&self,point:[f64;2])->bool {
        point.iter().all(|value|value.is_finite())&&point[0]>=self.bounds[0]&&point[0]<=self.bounds[2]&&point[1]>=self.bounds[1]&&point[1]<=self.bounds[3]
    }

    pub fn crosses(&self,a:[f64;2],b:[f64;2])->bool {
        if a.into_iter().chain(b).any(|value|!value.is_finite()) {return false;}
        let [left,top,right,bottom]=self.bounds;let [dx,dy]=[b[0]-a[0],b[1]-a[1]];let (mut low,mut high)=(0_f64,1_f64);
        for (p,q) in [(-dx,a[0]-left),(dx,right-a[0]),(-dy,a[1]-top),(dy,bottom-a[1])] {
            if p==0. {if q<0. {return false;}continue;}
            let t=q/p;if p<0. {low=low.max(t);} else {high=high.min(t);}
        }
        low<=high
    }
}

pub fn screen(matrix:&[f64;16],point:[f64;3],size:[f64;2])->Option<[f64;2]> {
    crate::snap_math::Screen {matrix:*matrix,size,origin:[0.;3]}.unclipped(point)
}

fn matches(view:&crate::snap_math::Screen,rectangle:Rectangle,strokes:Vec<Vec<[f64;3]>>)->bool {
    if strokes.is_empty() {return false;}
    if !rectangle.crossing {return strokes.iter().flatten().all(|point|view.unclipped(*point).is_some_and(|point|rectangle.contains(point)));}
    strokes.iter().any(|stroke| {
        let points=stroke.iter().map(|point|view.unclipped(*point)).collect::<Vec<_>>();
        points.iter().flatten().any(|point|rectangle.contains(*point))||points.windows(2).any(|pair|matches!(pair,[Some(a),Some(b)] if rectangle.crosses(*a,*b)))
    })
}

pub fn found(scene:&Scene,matrix:&[f64;16],size:[f64;2],rectangle:Rectangle)->Vec<ObjectId> {found_in(scene,&crate::snap_math::Screen{matrix:*matrix,size,origin:[0.;3]},rectangle)}
pub fn found_in(scene:&Scene,view:&crate::snap_math::Screen,rectangle:Rectangle)->Vec<ObjectId> {
    let origin=view.origin;let local=crate::snap_math::Screen{matrix:view.matrix,size:view.size,origin:[0.;3]};
    let placed=|model:&session_rust::Xform,p:[f64;3]|{let model=crate::render_space::placed(model,origin);let p=model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]));[p[0],p[1],p[2]]};
    let eligible=|id:ObjectId|scene.pick_id(id.value()).is_some();let mut found=Vec::new();
    for row in scene.objects().iter().filter(|row|eligible(row.id)) {
        let mut strokes=row.mesh.parts.endpoints.iter().map(|ends|ends.iter().map(|point|placed(&row.model,*point)).collect()).collect::<Vec<_>>();
        if let Some(source)=row.geometry() {strokes.extend(source.vertices().into_iter().filter_map(|key|source.vertex_point(key)).map(|point|vec![placed(&row.model,[point[0],point[1],point[2]])]));}
        if matches(&local,rectangle,strokes) {found.push(row.id);}
    }
    for row in scene.lines().iter().filter(|row|eligible(row.id)) {if matches(&local,rectangle,vec![row.endpoints_at(origin).map(|point|[point[0],point[1],point[2]]).to_vec()]) {found.push(row.id);}}
    for row in scene.paths().iter().filter(|row|eligible(row.id)) {if matches(&local,rectangle,vec![row.points_at(origin).into_iter().map(|point|[point[0],point[1],point[2]]).collect()]) {found.push(row.id);}}
    for row in scene.points().iter().filter(|row|eligible(row.id)) {let point=row.point_at(origin);if matches(&local,rectangle,vec![vec![[point[0],point[1],point[2]]]]) {found.push(row.id);}}
    for row in scene.texts().iter().filter(|row|eligible(row.id)) {if crate::scene_label_screen::matches_in(row,view,rectangle) {found.push(row.id);}}
    for row in scene.clouds().iter().filter(|row|eligible(row.id)) {let mut points=row.prepared.source.coords().chunks_exact(3).map(|p|local.unclipped(placed(&row.model,[p[0],p[1],p[2]])));let hit=if rectangle.crossing {points.any(|p|p.is_some_and(|p|rectangle.contains(p)))} else {points.all(|p|p.is_some_and(|p|rectangle.contains(p)))};if hit {found.push(row.id);}}
    for row in scene.sheets().iter().filter(|row|eligible(row.id)) {let strokes=row.prepared.pages().iter().flat_map(|page|page.coordinates.chunks_exact(6).map(|p|vec![placed(&row.model,[p[0],p[1],p[2]]),placed(&row.model,[p[3],p[4],p[5]])]).collect::<Vec<_>>()).collect();if matches(&local,rectangle,strokes) {found.push(row.id);}}
    found.sort_unstable_by_key(|id|id.value());found.dedup();found
}
