use session_rust::{BRep,Mesh,NurbsCurve};

pub struct Boundary {pub points:Vec<[f64;3]>,pub spans:Vec<usize>,pub sags:Vec<f32>}

pub fn supported(source:&BRep)->bool {
    source.surfacecolor.name.len().saturating_add(if source.surfacecolor.has_guid(){source.surfacecolor.guid().len()}else{0})<=256
        && source.m_faces.len()<=64 && source.m_edges.len()<=256 && source.m_surfaces.len()<=64
        && source.m_curves_3d.len()+source.m_curves_2d.len()<=512
        && source.m_vertices.len()<=512 && source.m_wires.len()<=128
        && source.m_shells.len()<=64 && source.m_solids.len()<=64
        && source.m_edges.iter().map(|edge|edge.pcurves.len()).sum::<usize>()<=1024
        && source.m_wires.iter().map(|wire|wire.edges.len()).sum::<usize>()<=2048
        && source.m_faces.iter().map(|face|face.wires.len()).sum::<usize>()<=256
        && source.m_shells.iter().map(|shell|shell.faces.len()).sum::<usize>()<=256
        && source.m_solids.iter().map(|solid|solid.shells.len()).sum::<usize>()<=128
        && source.m_surfaces.iter().all(crate::math_quality::surface_supported)
        && source.m_curves_3d.iter().chain(&source.m_curves_2d).all(crate::math_quality::curve_supported)
        && source_bytes(source)<=512*1024
        && source_bytes(source).saturating_mul(20).saturating_add(crate::math_quality::BREP_WORK*1024).saturating_add(8192*128)<=crate::math_quality::TRANSIENT_BYTES
}

fn source_bytes(source:&BRep)->usize {
    let guid=if source.has_guid(){source.guid().len()}else{0};
    fn table<T>(values:&Vec<T>)->usize {values.capacity().saturating_mul(std::mem::size_of::<T>()+64)}
    let topology=[table(&source.m_vertices),table(&source.m_edges),table(&source.m_wires),table(&source.m_faces),table(&source.m_shells),table(&source.m_solids)].into_iter().fold(0usize,usize::saturating_add);
    let mut bytes=4096usize.saturating_add(topology).saturating_add(source.name.capacity()).saturating_add(guid);
    let colour_bytes=|colour:&session_rust::Color|256usize.saturating_add(colour.name.capacity()).saturating_add(if colour.has_guid(){colour.guid().len()}else{0});
    bytes=bytes.saturating_add(colour_bytes(&source.surfacecolor));
    for edge in &source.m_edges {bytes=bytes.saturating_add(edge.pcurves.capacity().saturating_mul(64));}
    for wire in &source.m_wires {bytes=bytes.saturating_add(wire.edges.capacity().saturating_mul(32));}
    for face in &source.m_faces {bytes=bytes.saturating_add(face.wires.capacity().saturating_mul(32));}
    for shell in &source.m_shells {bytes=bytes.saturating_add(shell.faces.capacity().saturating_mul(32));}
    for solid in &source.m_solids {bytes=bytes.saturating_add(solid.shells.capacity().saturating_mul(32));}
    for vertex in &source.m_vertices {
        let point=&vertex.point;
        let metadata=point.name.capacity().saturating_add(if point.has_guid(){point.guid().len()}else{0}).saturating_add(colour_bytes(&point.pointcolor));
        if metadata>384 {return usize::MAX;}
        bytes=bytes.saturating_add(metadata);
    }
    for face in &source.m_faces {if let Some(colour)=&face.facecolor {bytes=bytes.saturating_add(colour_bytes(colour));}}
    bytes=bytes.saturating_add(source.m_surfaces.capacity().saturating_sub(source.m_surfaces.len()).saturating_mul(std::mem::size_of::<session_rust::NurbsSurface>()));
    for curves in [&source.m_curves_3d,&source.m_curves_2d] {bytes=bytes.saturating_add(curves.capacity().saturating_sub(curves.len()).saturating_mul(std::mem::size_of::<NurbsCurve>()));}
    for curve in source.m_curves_3d.iter().chain(&source.m_curves_2d) {
        bytes=bytes.saturating_add(std::mem::size_of::<NurbsCurve>()+64+curve.name.capacity()+if curve.has_guid(){curve.guid().len()}else{0});
        bytes=bytes.saturating_add(curve.m_cv.capacity().saturating_add(curve.m_nurbsknot.capacity()).saturating_mul(8));
        bytes=bytes.saturating_add(curve.pointcolors.capacity().saturating_add(curve.linecolors.capacity()).saturating_mul(std::mem::size_of::<session_rust::Color>()));
        for colour in curve.pointcolors.iter().chain(&curve.linecolors) {bytes=bytes.saturating_add(colour_bytes(colour));}
    }
    for surface in &source.m_surfaces {
        bytes=bytes.saturating_add(std::mem::size_of::<session_rust::NurbsSurface>()+64+surface.name.capacity()+if surface.has_guid(){surface.guid().len()}else{0});
        bytes=bytes.saturating_add(surface.m_nurbsknot.iter().fold(surface.m_cv.capacity(),|n,knots|n.saturating_add(knots.capacity())).saturating_mul(8));
        bytes=bytes.saturating_add(surface.pointcolors.capacity().saturating_add(surface.linecolors.capacity()).saturating_add(surface.facecolors.capacity()).saturating_mul(std::mem::size_of::<session_rust::Color>()));
        for colour in surface.pointcolors.iter().chain(&surface.linecolors).chain(&surface.facecolors) {bytes=bytes.saturating_add(colour_bytes(colour));}
    }
    bytes
}

pub fn diagonal(source:&BRep)->Result<f64,&'static str> {
    let mut lo=[f64::INFINITY;3];let mut hi=[f64::NEG_INFINITY;3];
    for surface in &source.m_surfaces {
        if !crate::math_quality::surface_supported(surface) {return Err("CAD surface controls exceed the refinement budget");}
        for i in 0..surface.m_cv_count[0] {for j in 0..surface.m_cv_count[1] {
            let p=surface.get_cv_4d(i,j).and_then(crate::math_quality::control).ok_or("Missing CAD control")?;
            for axis in 0..3 {lo[axis]=lo[axis].min(p[axis]);hi[axis]=hi[axis].max(p[axis]);}
        }}
    }
    let diagonal=(0..3).map(|i|(hi[i]-lo[i]).powi(2)).sum::<f64>().sqrt();
    if !diagonal.is_finite()||diagonal<=0. {return Err("CAD bounds cannot set display quality");}
    Ok(diagonal)
}

pub fn faces(source:&BRep)->Result<Vec<Mesh>,&'static str> {
    if !supported(source) {return Err("CAD topology exceeds the refinement budget");}
    if source.m_faces.is_empty() {return Ok(Vec::new());}
    let sag=crate::math_quality::current().0/diagonal(source)?;
    if !sag.is_finite()||sag<1e-6 {return Err("CAD requested sag exceeds the kernel refinement range");}
    source.face_meshes_q_bounded((20.,sag),crate::math_quality::BREP_WORK).ok_or("CAD face meshing reached its finite work budget")
}

fn parameter(curve:&NurbsCurve,p:[f64;3])->f64 {
    let (start,end)=curve.domain();let step=(end-start)/512.;
    let distance=|t:f64|{let q=curve.point_at(t);(0..3).map(|i|(q[i]-p[i]).powi(2)).sum::<f64>()};
    let nearest=(0..=512).min_by(|a,b|distance(start+step*(*a as f64)).total_cmp(&distance(start+step*(*b as f64)))).unwrap_or(0);
    let (mut lo,mut hi)=((start+step*(nearest as f64-1.)).max(start),(start+step*(nearest as f64+1.)).min(end));
    let ratio=0.5*(5f64.sqrt()-1.);
    for _ in 0..48 {let(a,b)=(hi-ratio*(hi-lo),lo+ratio*(hi-lo));if distance(a)<distance(b){hi=b;}else{lo=a;}}
    (lo+hi)*0.5
}
fn distance(p:[f64;3],a:[f64;3],b:[f64;3])->f64 {
    let d:[f64;3]=std::array::from_fn(|i|b[i]-a[i]);let q:[f64;3]=std::array::from_fn(|i|p[i]-a[i]);
    let t=((0..3).map(|i|q[i]*d[i]).sum::<f64>()/d.iter().map(|v|v*v).sum::<f64>().max(1e-300)).clamp(0.,1.);
    (0..3).map(|i|(q[i]-t*d[i]).powi(2)).sum::<f64>().sqrt()
}
pub fn boundary(source:&BRep,chain:&crate::cad_edges::EdgeChain,mesh:&Mesh)->Result<Boundary,&'static str> {
    if chain.keys.len()<2||chain.keys.len()>crate::math_quality::CURVE_SEGMENTS {return Err("CAD boundary exceeds the display budget");}
    let coarse=chain.keys.iter().map(|key|mesh.vertex.get(key).map(|p|[p.x,p.y,p.z]).ok_or("Missing CAD boundary vertex")).collect::<Result<Vec<_>,_>>()?;
    let edge=source.m_edges.get(chain.edge).ok_or("Missing CAD edge")?;
    let curve=usize::try_from(edge.curve_3d_index).ok().and_then(|i|source.m_curves_3d.get(i)).ok_or("Original CAD edge has no exact 3D curve")?;
    let sampled=crate::math_quality::parameters(curve,crate::math_quality::current(),crate::math_quality::CURVE_SEGMENTS-chain.keys.len()+1)?;
    let (start,end)=curve.domain();let period=end-start;
    let mut parameters:Vec<f64>=coarse.iter().map(|p|parameter(curve,*p)).collect();
    if curve.is_closed() {
        for k in 1..parameters.len() {while parameters[k]-parameters[k-1]>period*0.5 {parameters[k]-=period;}while parameters[k]-parameters[k-1]< -period*0.5 {parameters[k]+=period;}}
        let last=parameters.len()-1;
        if chain.keys.first()==chain.keys.last()&&last>1 {parameters[last]=parameters[0]+if parameters[last-1]>parameters[0]{period}else{-period};}
    }
    let point=|t:f64|{let t=if curve.is_closed(){start+(t-start).rem_euclid(period)}else{t.clamp(start,end)};let p=curve.point_at(t);[p[0],p[1],p[2]]};
    let mut points=vec![point(parameters[0])];let mut spans=Vec::new();let mut sags=Vec::new();
    for k in 0..parameters.len()-1 {
        let(a,b)=(parameters[k],parameters[k+1]);let mut cuts=vec![b];
        for &t in &sampled {let t=if curve.is_closed(){t+((a-t)/period).round()*period}else{t};if t>a.min(b)&&t<a.max(b){cuts.push(t);}}
        cuts.sort_by(f64::total_cmp);if b<a{cuts.reverse();}cuts.dedup_by(|a,b|(*a-*b).abs()<period*1e-12);
        for t in cuts {
            if spans.len()>=crate::math_quality::CURVE_SEGMENTS {return Err("CAD boundary reached its finite segment budget");}
            let p=point(t);if !p.iter().all(|v|(*v as f32).is_finite()){return Err("CAD boundary sample exceeds the display range");}
            let sag=distance(*points.last().unwrap(),coarse[k],coarse[k+1]).max(distance(p,coarse[k],coarse[k+1]));
            points.push(p);spans.push(k);sags.push((sag*2.)as f32);
        }
    }
    crate::math_quality::charge_curves(spans.len())?;
    Ok(Boundary{points,spans,sags})
}
