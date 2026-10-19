use crate::{scene::{Scene, ObjectId}, snap::{Hit, Kind}};

pub enum Target { Point(Hit), Wire { ends: [[f64; 3]; 2], owner: ObjectId } }

pub fn collect(scene: &Scene) -> Vec<Target> {
    let mut out = Vec::new();
    for row in scene.points().iter().filter(|r| r.visible()) { let p = row.point(); point(&mut out,row.id,[p[0],p[1],p[2]],Kind::End); }
    for row in scene.lines().iter().filter(|r| r.visible()) { let ends = row.endpoints().map(|p| [p[0],p[1],p[2]]); polyline(&mut out,row.id,&ends); }
    for row in scene.paths().iter().filter(|r| r.visible()) {
        let points: Vec<_> = row.points().iter().map(|p| [p[0],p[1],p[2]]).collect();
        if let crate::chain::Source::Curve(curve, _) = &row.prepared.source {
            let (a,b) = curve.domain();
            for (t,kind) in [(a,Kind::End),(b,Kind::End),((a+b)*0.5,Kind::Mid)] { let p = row.model.transform_point(&curve.point_at(t)); point(&mut out,row.id,[p[0],p[1],p[2]],kind); }
            wires(&mut out,row.id,&points);
        } else { polyline(&mut out,row.id,&points); }
        if points.len()>=4 && (0..3).map(|i| (points[0][i]-points[points.len()-1][i]).powi(2)).sum::<f64>().sqrt()<=1e-9 {
            let corners=&points[..points.len()-1]; let center=std::array::from_fn(|i| corners.iter().map(|p| p[i]).sum::<f64>()/corners.len() as f64);
            point(&mut out,row.id,center,Kind::Center);
        }
    }
    for row in scene.objects().iter().filter(|r| r.visible()) {
        let mut seen = std::collections::HashSet::new();
        if let Some(source) = row.geometry() { for key in source.vertices() {
            if let Some(p) = source.vertex_point(key) { let p = row.model.transform_point(&p); point(&mut out,row.id,[p[0],p[1],p[2]],Kind::Vertex); seen.insert(key); }
        } }
        for (&(a,b),ends) in row.mesh.parts.edges.iter().zip(&row.mesh.parts.endpoints) {
            let ends=ends.map(|p| { let p=row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])); [p[0],p[1],p[2]] });
            for (key,p) in [(a,ends[0]),(b,ends[1])] { if seen.insert(key) { point(&mut out,row.id,p,Kind::Vertex); } }
            out.push(Target::Wire { ends,owner:row.id });
        }
    }
    out
}

fn point(out: &mut Vec<Target>, owner: ObjectId, point: [f64; 3], kind: Kind) { out.push(Target::Point(Hit { point,kind,owner:Some(owner) })); }
fn wires(out: &mut Vec<Target>, owner: ObjectId, points: &[[f64; 3]]) { for ends in points.windows(2) { out.push(Target::Wire { ends:[ends[0],ends[1]],owner }); } }
fn polyline(out: &mut Vec<Target>, owner: ObjectId, points: &[[f64; 3]]) {
    for (i,&p) in points.iter().enumerate() { point(out,owner,p,if i==0 || i+1==points.len() { Kind::End } else { Kind::Vertex }); }
    for p in points.windows(2) { point(out,owner,std::array::from_fn(|i| (p[0][i]+p[1][i])*0.5),Kind::Mid); }
    wires(out,owner,points);
}
