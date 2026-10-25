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
