use session_rust::NurbsSurface;

pub fn validate(source:&NurbsSurface)->Result<[[f64;2];2],&'static str> {
    if !(2..=3).contains(&source.m_dim) {return Err("Surface dimension must be two or three");}
    let count=source.m_cv_count[0].checked_mul(source.m_cv_count[1]).ok_or("Surface control count overflows")?;
    if count>65_536||source.m_cv_count.iter().any(|n|*n<2) {return Err("Surface control count exceeds the display budget");}
    let size=source.m_dim+usize::from(source.m_is_rat);
    if source.m_cv_stride.iter().any(|stride|*stride<size) {return Err("Surface control stride is too short");}
    let row_major=source.m_cv_stride[1].checked_mul(source.m_cv_count[1]).is_some_and(|span|source.m_cv_stride[0]>=span);
    let column_major=source.m_cv_stride[0].checked_mul(source.m_cv_count[0]).is_some_and(|span|source.m_cv_stride[1]>=span);
    if !row_major&&!column_major {return Err("Surface controls overlap in storage");}
    let end=(source.m_cv_count[0]-1).checked_mul(source.m_cv_stride[0]).and_then(|n|(source.m_cv_count[1]-1).checked_mul(source.m_cv_stride[1]).and_then(|m|n.checked_add(m))).and_then(|n|n.checked_add(size)).ok_or("Surface control range overflows")?;
    if end>source.m_cv.len() {return Err("Surface controls are incomplete");}
    for i in 0..source.m_cv_count[0] {for j in 0..source.m_cv_count[1] {
        let start=i*source.m_cv_stride[0]+j*source.m_cv_stride[1];let values=&source.m_cv[start..start+size];
        if values.iter().any(|v|!v.is_finite())||source.m_is_rat&&values[source.m_dim]<=0. {return Err("Surface controls need finite coordinates and positive weights");}
    }}
    let mut domains=[[0.;2];2];
    for dir in 0..2 {
        let order=source.m_order[dir];let count=source.m_cv_count[dir];let knots=&source.m_nurbsknot[dir];
        if !(2..=32).contains(&order)||order>count||knots.len()!=count+order-2||knots.iter().any(|v|!v.is_finite())||knots.windows(2).any(|pair|pair[0]>pair[1]) {return Err("Surface order or knots are invalid");}
        let (a,b)=source.domain(dir).ok_or("Surface domain is missing")?;
        if a>=b||!(b-a).is_finite() {return Err("Surface domain must have finite positive length");}domains[dir]=[a,b];
    }
    Ok(domains)
}

pub struct Sample {pub uv:[f64;2],pub point:[f64;3],pub normal:[f64;3]}
pub struct Boundary {pub edge:usize,pub segments:Vec<[u32;2]>}
pub struct Grid {pub steps:[usize;2],pub samples:Vec<Sample>,pub indices:Vec<u32>,pub boundaries:Vec<Boundary>}

pub fn border_parameters(source:&session_rust::NurbsSurface,edge:usize,steps:usize)->Option<Vec<f64>> {
    if edge>=4||steps==0 {return None;}
    let fixed=edge/2;let varying=1-fixed;let domain=source.domain(fixed)?;
    let curve=source.iso_curve(varying,if edge%2==0 {domain.0}else{domain.1})?;
    let mut turning=0.;let mut previous:Option<[f64;3]>=None;
    let control=|i:usize|{let at=i*curve.m_cv_stride;let weight=if curve.m_is_rat {curve.m_cv[at+curve.m_dim]}else{1.};std::array::from_fn::<_,3,_>(|axis|if axis<curve.m_dim {curve.m_cv[at+axis]/weight}else{0.})};
    for i in 1..curve.m_cv_count {
        let (a,b)=(control(i-1),control(i));let delta=std::array::from_fn::<_,3,_>(|axis|b[axis]-a[axis]);
        let length=delta.iter().map(|v|v*v).sum::<f64>().sqrt();if length<1e-12 {continue;}
        let unit=delta.map(|v|v/length);
        if let Some(before)=previous {turning+=(0..3).map(|axis|before[axis]*unit[axis]).sum::<f64>().clamp(-1.,1.).acos().to_degrees();}
        previous=Some(unit);
    }
    let count=((turning/4.-1e-6).ceil() as usize).max(curve.span_count()).clamp(1,256);
    let mut parameters:Vec<f64>=(0..=steps).map(|i|i as f64/steps as f64).chain((0..=count).map(|i|i as f64/count as f64)).collect();
    parameters.sort_by(f64::total_cmp);parameters.dedup_by(|a,b|(*a-*b).abs()<1e-12);Some(parameters)
}

impl Grid {
    pub fn new(source:&NurbsSurface)->Result<Self,&'static str> {
        if crate::math_quality::current().0.is_finite() {return quality_grid(source);}
        let domain=validate(source)?;let [nu,nv]=source.m_cv_count.map(|count|(count*4).clamp(16,96));let mut samples=Vec::with_capacity((nu+1)*(nv+1));
        for i in 0..=nu {for j in 0..=nv {
            let uv=[domain[0][0]+(domain[0][1]-domain[0][0])*i as f64/nu as f64,domain[1][0]+(domain[1][1]-domain[1][0])*j as f64/nv as f64];
            let point=source.point_at(uv[0],uv[1]).ok_or("Surface sample cannot be evaluated")?;let point=[point[0],point[1],point[2]];
            let normal=source.normal_at(uv[0],uv[1]);let normal=[normal[0],normal[1],normal[2]];
            if point.iter().chain(normal.iter()).any(|v|!v.is_finite()) {return Err("Surface sample must be finite");}
            samples.push(Sample {uv,point,normal});
        }}
        let mut indices=Vec::with_capacity(nu*nv*6);
        for i in 0..nu {for j in 0..nv {let a=(i*(nv+1)+j) as u32;let b=a+(nv+1) as u32;indices.extend_from_slice(&[a,b,b+1,a,b+1,a+1]);}}
        let mut boundaries=Vec::new();
        for edge in 0..4 {
            if source.is_closed(edge/2) {continue;}
            let points=match edge {0=>(0..=nv).collect::<Vec<_>>(),1=>(0..=nv).map(|j|nu*(nv+1)+j).collect(),2=>(0..=nu).map(|i|i*(nv+1)).collect(),_=>(0..=nu).map(|i|i*(nv+1)+nv).collect()};
            let varying=1-edge/2;let extent=domain[varying];let fixed=edge/2;
            let parameters=border_parameters(source,edge,points.len()-1).ok_or("Surface border curve cannot be evaluated")?;
            let mut refined=Vec::with_capacity(parameters.len());
            for t in parameters {
                let at=t*(points.len()-1) as f64;let nearest=at.round();
                if (at-nearest).abs()<1e-9 {refined.push(points[nearest as usize] as u32);continue;}
                let mut uv=[0.;2];uv[fixed]=domain[fixed][edge%2];uv[varying]=extent[0]+(extent[1]-extent[0])*t;
                let point=source.point_at(uv[0],uv[1]).ok_or("Surface border cannot be evaluated")?;let normal=source.normal_at(uv[0],uv[1]);
                let point=[point[0],point[1],point[2]];let normal=[normal[0],normal[1],normal[2]];
                if point.iter().chain(normal.iter()).any(|v|!v.is_finite()) {return Err("Surface border must be finite");}
                refined.push(samples.len() as u32);samples.push(Sample{uv,point,normal});
            }
            boundaries.push(Boundary {edge,segments:refined.windows(2).map(|pair|[pair[0],pair[1]]).collect()});
        }
        Ok(Self {steps:[nu,nv],samples,indices,boundaries})
    }

}

fn quality_grid(source:&NurbsSurface)->Result<Grid,&'static str> {
    validate(source)?;
    let mut us=crate::math_quality::surface_parameters(source,0)?;
    let mut vs=crate::math_quality::surface_parameters(source,1)?;
    let limit=crate::math_quality::SURFACE_STEPS;
    let tolerance=crate::math_quality::current().0;
    let point=|u,v|->Result<[f64;3],&'static str>{let p=source.point_at(u,v).ok_or("Surface sample cannot be evaluated")?;let p=[p[0],p[1],p[2]];if p.iter().all(|v|(*v as f32).is_finite()){Ok(p)}else{Err("Surface sample exceeds the display range")}};
    let mut settled=false;
    // Interior probes are an error heuristic, not a bound for arbitrary NURBS surfaces.
    for _ in 0..8 {
        if us.len()>limit+1||vs.len()>limit+1 {return Err("Surface refinement reached its finite grid budget");}
        let mut add_u=vec![false;us.len()-1];let mut add_v=vec![false;vs.len()-1];
        for (i,u) in us.windows(2).enumerate() {for (j,v) in vs.windows(2).enumerate() {
            let corners=[point(u[0],v[0])?,point(u[1],v[0])?,point(u[1],v[1])?,point(u[0],v[1])?];
            let mut refine=false;
            for s in [0.25,0.5,0.75] {for t in [0.25,0.5,0.75] {
                let actual=point(u[0]+(u[1]-u[0])*s,v[0]+(v[1]-v[0])*t)?;
                let weights=if t<=s {[1.-s,s-t,t,0.]} else {[1.-t,0.,s,t-s]};
                let deviation=(0..3).map(|axis|{let linear=(0..4).map(|i|weights[i]*corners[i][axis]).sum::<f64>();(actual[axis]-linear).powi(2)}).sum::<f64>().sqrt();
                refine|=deviation>tolerance;
            }}
            if refine {add_u[i]=true;add_v[j]=true;}
        }}
        if !add_u.iter().any(|v|*v) {settled=true;break;}
        if us.len()+add_u.iter().filter(|v|**v).count()>limit+1||vs.len()+add_v.iter().filter(|v|**v).count()>limit+1 {return Err("Surface refinement reached its finite grid budget");}
        for (values,flags) in [(&mut us,add_u),(&mut vs,add_v)] {
            let added:Vec<_>=values.windows(2).zip(flags).filter_map(|(v,add)|add.then_some(v[0]+(v[1]-v[0])*0.5)).collect();
            values.extend(added);values.sort_by(f64::total_cmp);values.dedup();
        }
    }
    if !settled {return Err("Surface refinement reached its finite pass budget");}
    let (nu,nv)=(us.len()-1,vs.len()-1);
    let mut samples=Vec::with_capacity((nu+1)*(nv+1));
    for &u in &us {for &v in &vs {
        let normal=source.normal_at(u,v);let normal=[normal[0],normal[1],normal[2]];
        if !normal.iter().all(|v|v.is_finite()) {return Err("Surface normal must be finite");}
        samples.push(Sample{uv:[u,v],point:point(u,v)?,normal});
    }}
    let mut indices=Vec::with_capacity(nu*nv*6);
    for i in 0..nu {for j in 0..nv {let a=(i*(nv+1)+j)as u32;let b=a+(nv+1)as u32;indices.extend_from_slice(&[a,b,b+1,a,b+1,a+1]);}}
    let mut boundaries=Vec::new();
    for edge in 0..4 {
        if source.is_closed(edge/2) {continue;}
        let points:Vec<usize>=match edge {0=>(0..=nv).collect(),1=>(0..=nv).map(|j|nu*(nv+1)+j).collect(),2=>(0..=nu).map(|i|i*(nv+1)).collect(),_=>(0..=nu).map(|i|i*(nv+1)+nv).collect()};
        boundaries.push(Boundary{edge,segments:points.windows(2).map(|p|[p[0]as u32,p[1]as u32]).collect()});
    }
    Ok(Grid{steps:[nu,nv],samples,indices,boundaries})
}
