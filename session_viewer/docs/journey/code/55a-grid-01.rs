}

pub struct Sample {pub uv:[f64;2],pub point:[f64;3],pub normal:[f64;3]}
pub struct Boundary {pub edge:usize,pub segments:Vec<[u32;2]>}
pub struct Grid {pub steps:[usize;2],pub samples:Vec<Sample>,pub indices:Vec<u32>,pub boundaries:Vec<Boundary>}

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
            boundaries.push(Boundary {edge,segments:points.windows(2).map(|pair|[pair[0] as u32,pair[1] as u32]).collect()});
        }
        Ok(Self {steps:[nu,nv],samples,indices,boundaries})
    }

}
