# 9.1 · Original surfaces, analytic normals and trim boundaries

**Steps:** 11 · **Source:** [app/walk/brep.rs](../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">(u,v) Outer Loop + Inner Hole Trim Loop</text>
  <rect x="64" y="48" width="212" height="106" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="170" cy="101" r="28" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
  <line x1="64" y1="48" x2="142" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="276" y1="48" x2="198" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="64" y1="154" x2="142" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="276" y1="154" x2="198" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Crease Angle &gt; θ: Split Normals N1, N2 vs Smooth N</text>
  <line x1="426" y1="138" x2="496" y2="88" stroke="#111111" stroke-width="2.5"/>
  <line x1="496" y1="88" x2="566" y2="138" stroke="#111111" stroke-width="2.5"/>
  <line x1="496" y1="88" x2="464" y2="44" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <line x1="496" y1="88" x2="528" y2="44" stroke="#2563eb" stroke-width="2.2" marker-end="url(#arrB)"/>
  <path d="M 596 138 Q 646 72 696 138" fill="none" stroke="#111111" stroke-width="2.5"/>
  <line x1="646" y1="105" x2="646" y2="48" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Crisp Crease Shading + Boundary Wire Edges</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,128 934,62 934,146" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <polygon points="934,62 1022,128 934,146" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <ellipse cx="904" cy="112" rx="12" ry="8" fill="#ffffff" stroke="#dc2626" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-55-validation"></a>1. Validate surface control storage

### 1. `src/surface.rs`

Create the file and type:

```rust
use session_rust::NurbsSurface;

pub fn validate(source: &NurbsSurface) -> Result<[[f64; 2]; 2], &'static str> {
    if !(2..=3).contains(&source.m_dim) {
        return Err("Surface dimension must be two or three");
    }
    let count = source.m_cv_count[0]
        .checked_mul(source.m_cv_count[1])
        .ok_or("Surface control count overflows")?;
    if count > 65_536 || source.m_cv_count.iter().any(|n| *n < 2) {
        return Err("Surface control count exceeds the display budget");
    }
    let size = source.m_dim + usize::from(source.m_is_rat);
    if source
        .m_cv_stride
        .iter()
        .any(|stride| *stride < size)
    {
        return Err("Surface control stride is too short");
    }
    let row_major = source.m_cv_stride[1]
        .checked_mul(source.m_cv_count[1])
        .is_some_and(|span| source.m_cv_stride[0] >= span);
    let column_major = source.m_cv_stride[0]
        .checked_mul(source.m_cv_count[0])
        .is_some_and(|span| source.m_cv_stride[1] >= span);
    if !row_major && !column_major {
        return Err("Surface controls overlap in storage");
    }
    let end = (source.m_cv_count[0] - 1)
        .checked_mul(source.m_cv_stride[0])
        .and_then(|n| {
            (source.m_cv_count[1] - 1)
                .checked_mul(source.m_cv_stride[1])
                .and_then(|m| n.checked_add(m))
        })
        .and_then(|n| n.checked_add(size))
        .ok_or("Surface control range overflows")?;
    if end > source.m_cv.len() {
        return Err("Surface controls are incomplete");
    }
    for i in 0..source.m_cv_count[0] {
        for j in 0..source.m_cv_count[1] {
            let start = i * source.m_cv_stride[0] + j * source.m_cv_stride[1];
            let values = &source.m_cv[start..start + size];
            if values.iter().any(|v| !v.is_finite())
                || source.m_is_rat && values[source.m_dim] <= 0.
            {
                return Err(
                    "Surface controls need finite coordinates and positive weights",
                );
            }
        }
    }
    let mut domains = [[0.; 2]; 2];
    for dir in 0..2 {
        let order = source.m_order[dir];
        let count = source.m_cv_count[dir];
        let knots = &source.m_nurbsknot[dir];
        if !(2..=32).contains(&order)
            || order > count
            || knots.len() != count + order - 2
            || knots.iter().any(|v| !v.is_finite())
            || knots
                .windows(2)
                .any(|pair| pair[0] > pair[1])
        {
            return Err("Surface order or knots are invalid");
        }
        let (a, b) = source
            .domain(dir)
            .ok_or("Surface domain is missing")?;
        if a >= b || !(b - a).is_finite() {
            return Err("Surface domain must have finite positive length");
        }
        domains[dir] = [a, b];
    }
    Ok(domains)
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod shortcut_focus_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod shortcut_focus_tests;

pub mod surface;
```

## <a id="checkpoint-55a-grid"></a>2. Keep UV coordinates beside world samples

### 1. `src/surface.rs`

**Locate this existing block:**

```rust
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
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
```

## <a id="checkpoint-55b-normals"></a>3. Use analytic normals and finite singular-point fallbacks

### 1. `src/surface.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn normal(&self, index: usize) -> Result<[f32; 3], &'static str> {
        let sample = self
            .samples
            .get(index)
            .ok_or("Missing surface sample")?;
        if sample
            .normal
            .iter()
            .map(|v| v * v)
            .sum::<f64>()
            > 1e-24
        {
            return crate::normals::unit(sample.normal.map(|v| v as f32));
        }
        let [nu, nv] = self.steps;
        let grid_count = (nu + 1) * (nv + 1);
        let index = if index < grid_count {
            index
        } else {
            (0..grid_count)
                .min_by(|&a, &b| {
                    let distance = |i: usize| {
                        (0..2)
                            .map(|axis| {
                                (self.samples[i].uv[axis] - sample.uv[axis]).powi(2)
                            })
                            .sum::<f64>()
                    };
                    distance(a).total_cmp(&distance(b))
                })
                .unwrap()
        };
        let i = index / (nv + 1);
        let j = index % (nv + 1);
        for a in [i.saturating_sub(1), i.min(nu - 1)] {
            for b in [j.saturating_sub(1), j.min(nv - 1)] {
                let p = self.samples[a * (nv + 1) + b].point;
                let u = self.samples[(a + 1) * (nv + 1) + b].point;
                let v = self.samples[a * (nv + 1) + b + 1].point;
                let u =
                    session_rust::Vector::new(u[0] - p[0], u[1] - p[1], u[2] - p[2]);
                let v =
                    session_rust::Vector::new(v[0] - p[0], v[1] - p[1], v[2] - p[2]);
                let n = u.cross(&v);
                if n.magnitude() > 1e-12 {
                    return crate::normals::unit([
                        n[0] as f32,
                        n[1] as f32,
                        n[2] as f32,
                    ]);
                }
            }
        }
        Ok([0., 0., 1.])
    }
}
```

## <a id="checkpoint-55c-display"></a>4. Prepare derived surface display without losing its source

### 1. `src/surface.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

pub struct Prepared {
    pub source: std::rc::Rc<NurbsSurface>,
    pub grid: Grid,
}
impl Prepared {
    pub fn new(source: std::rc::Rc<NurbsSurface>) -> Result<Self, &'static str> {
        let _ = source.guid();
        let grid = Grid::new(&source)?;
        Ok(Self { source, grid })
    }
    pub fn mesh(&self) -> Result<crate::prepared::PreparedMesh, &'static str> {
        let points = self
            .grid
            .samples
            .iter()
            .map(|s| session_rust::Point::new(s.point[0], s.point[1], s.point[2]))
            .collect();
        let faces = self
            .grid
            .indices
            .chunks_exact(3)
            .map(|tri| tri.iter().map(|i| *i as usize).collect())
            .collect();
        let mut source = session_rust::Mesh::from_vertices_and_faces(points, faces);
        source.set_guid(self.source.guid().to_owned());
        source.name = self.source.name.clone();
        source.is_visible = self.source.is_visible;
        source.is_locked = self.source.is_locked;
        let colour = self
            .source
            .facecolors
            .first()
            .cloned()
            .unwrap_or_default()
            .to_f32();
        source.set_objectcolor(session_rust::Color::new(
            colour[0], colour[1], colour[2], colour[3],
        ));
        let vertices = self
            .grid
            .samples
            .iter()
            .map(|s| {
                [
                    s.point[0] as f32,
                    s.point[1] as f32,
                    s.point[2] as f32,
                    colour[0],
                    colour[1],
                    colour[2],
                ]
            })
            .collect();
        let normals = (0..self.grid.samples.len())
            .map(|i| self.grid.normal(i))
            .collect::<Result<Vec<_>, _>>()?;
        let mut display = crate::mesh::Mesh::new(vertices, self.grid.indices.clone())?
            .with_normals(normals)?;
        display.parts.faces = vec![0];
        display.parts.triangles = vec![0; self.grid.indices.len() / 3];
        for boundary in &self.grid.boundaries {
            for &[a, b] in &boundary.segments {
                display
                    .parts
                    .edges
                    .push((a as usize, b as usize));
                display.parts.endpoints.push([
                    self.grid.samples[a as usize].point,
                    self.grid.samples[b as usize].point,
                ]);
            }
        }
        Ok(crate::prepared::PreparedMesh {
            geometry: std::rc::Rc::new(source),
            display,
            source: None,
            model: session_rust::Xform::identity(),
        })
    }
    pub fn edge_ids(&self) -> impl Iterator<Item = usize> + '_ {
        self.grid
            .boundaries
            .iter()
            .flat_map(|b| std::iter::repeat_n(b.edge, b.segments.len()))
    }
}

pub fn example() -> NurbsSurface {
    let points = (0..3)
        .flat_map(|i| {
            (0..3).map(move |j| {
                session_rust::Point::new(
                    i as f64 - 1.,
                    j as f64 - 1.,
                    if i == 1 && j == 1 {
                        1.2
                    } else {
                        0.
                    },
                )
            })
        })
        .collect::<Vec<_>>();
    let mut source = NurbsSurface::create(false, false, 2, 2, 3, 3, &points).unwrap();
    source.name = "Parametric roof".into();
    source.facecolors = vec![session_rust::Color::new(
        0.35, 0.65, 0.85, 1.,
    )];
    source
}
```

## <a id="checkpoint-55d-source-row"></a>5. Attach surface ownership to existing scene rows

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .iter_mut()
            .find(|r| r.id == target.owner)
        {
            let mut source = (**row
```

Replace it with:

```rust
            .iter_mut()
            .find(|r| r.id == target.owner)
        {
            if row.surface.is_some() {
                return Err(
                    "CAD surface part editing connects after source parameter mapping",
                );
            }
            let mut source = (**row
```

**Locate this block:**

```rust
        Ok(id)
    }

    pub(crate) fn copy_row(
        &mut self,
        id: ObjectId,
```

Replace it with:

```rust
        Ok(id)
    }

    pub fn insert_surface(
        &mut self,
        source: Rc<session_rust::NurbsSurface>,
    ) -> Result<ObjectId, &'static str> {
        let prepared = Rc::new(crate::surface::Prepared::new(source)?);
        let mesh = prepared.mesh()?;
        let id = self.insert(mesh)?;
        self.objects
            .iter_mut()
            .find(|row| row.id == id)
            .unwrap()
            .surface = Some(prepared);
        Ok(id)
    }

    pub(crate) fn copy_row(
        &mut self,
        id: ObjectId,
```

**Locate this block:**

```rust
        let label = metadata.name.clone();
        let node_guid = guid.clone();
        self.objects.push(Object {
            id,
            guid,
            metadata,
```

Replace it with:

```rust
        let label = metadata.name.clone();
        let node_guid = guid.clone();
        self.objects.push(Object {
            surface: None,
            id,
            guid,
            metadata,
```

**Locate this block:**

```rust
pub struct Object {
    pub id: ObjectId,
    pub guid: String,
    pub metadata: Rc<crate::row_metadata::RowMetadata>,
```

Replace it with:

```rust
pub struct Object {
    pub surface: Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
    pub metadata: Rc<crate::row_metadata::RowMetadata>,
```

### 2. `src/document.rs`

**Locate this existing block:**

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    if !scene.points().is_empty() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    if scene
        .objects()
        .iter()
        .any(|row| row.surface.is_some())
    {
        return Err("This checkpoint cannot save CAD surface objects yet");
    }
    if !scene.points().is_empty() {
```

### 3. `src/part_preview.rs`

**Locate this existing block:**

```rust
    {
        let mesh = row
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    {
        if row.surface.is_some() {
            return Err(
                "CAD surface part editing connects after source parameter mapping",
            );
        }
        let mesh = row
```

### 4. `src/math_view.rs`

Refine the shaded surface grid and its four mathematical borders together. The triangle indices fill the face; the border list supplies the visible line segments. Internal triangulation edges stay out of that list. Retain the original surface so a later camera change can prepare a finer presentation without changing its document row.

**Locate this existing block:**

```rust
}
enum Prepared {
    Curve(crate::chain::PreparedChain),
}
impl Prepared {
    fn upload_bytes(&self)->usize {
        match self {
            Self::Curve(prepared)=>prepared.points.len().saturating_mul(256),
        }
    }
}
struct Request { id:ObjectId, source:Source, model:Xform }
struct Entry { key:(u32,u8,usize), _source:Source, target:Quality, accepted:Quality, rejected:Option<Quality>, error:Option<&'static str>, data:Option<Prepared>, bytes:usize }

#[derive(Default)]
pub struct Presentation {
    entries:Vec<Entry>, dirty:bool, scheduled:bool,
    pub limited:Option<&'static str>, pub bytes:usize, pub generations:u64, pub uploaded:bool,
}

impl Source {
    fn key(&self,id:ObjectId)->(u32,u8,usize) {
        match self {
            Self::Curve(source)=>(Scene::math_id(id),0,Rc::as_ptr(source) as usize),
        }
    }
    fn projection(&self,projection:&mut Projection)->Result<(), &'static str> {
        match self {
            Self::Curve(source)=>projection.curve(source),
            
        }
    }
    fn initial_chord(&self,current:Quality)->Option<f64> {
        if current.0.is_finite() {return None;}
        None
    }
    fn reserve(&self)->usize {
        match self {
            Self::Curve(_)=>2*1024*1024,
        }
    }
    fn prepare(&self,requested:Quality)->Result<Prepared,&'static str> {
        quality::using(requested,||match self {
            Self::Curve(source)=>Ok(Prepared::Curve(crate::chain::PreparedChain::curve(source.clone())?)),
            
            
        })
    }
}

fn requests(scene:&Scene)->Result<Vec<Request>,&'static str> {
    if scene.objects().len()+scene.paths().len()+scene.lines().len()+scene.points().len()>1024 {
        return Err("Presentation row count exceeds the refinement budget");
    }
    let display_units=scene.objects().iter().fold(0usize,|n,row|n.saturating_add(row.mesh.vertices().len()).saturating_add(row.mesh.indices().len()))
        .saturating_add(scene.paths().iter().map(|row|row.prepared.points.len()).sum::<usize>());
    let names=scene.objects().iter().map(|row|row.guid.len()).sum::<usize>()
        .saturating_add(scene.paths().iter().map(|row|row.guid.len()).sum::<usize>());
    if display_units.saturating_mul(256)>16*1024*1024 || names>256*1024 {
        return Err("Scene upload scratch exceeds the refinement budget");
    }
    let mut out=Vec::new();
    for row in scene.paths() {
        if let crate::chain::Source::Curve(source,_)=&row.prepared.source {
            if out.len()>=128 {return Err("Mathematical source count exceeds the refinement budget");}
            out.push(Request{id:row.id,source:Source::Curve(source.clone()),model:row.model.clone()});
        }
    }
    
    Ok(out)
}

impl Presentation {
    pub fn qualities(&self)->Vec<(u32,Option<f64>,Option<f64>,bool)> {
        self.entries.iter().map(|entry|(entry.key.0,entry.target.0.is_finite().then_some(entry.target.0),entry.accepted.0.is_finite().then_some(entry.accepted.0),entry.error.is_some())).collect()
    }
    pub fn invalidate(&mut self,scene:&Scene) {
        self.dirty=true;
        match requests(scene) {
            Ok(requests)=>self.entries.retain(|entry|requests.iter().any(|request|request.source.key(request.id)==entry.key)),
            Err(error)=>{self.entries.clear();self.limited=Some(error);}
        }
        self.bytes=self.entries.iter().map(|entry|entry.bytes).sum();
    }

    #[cfg(target_arch="wasm32")]
    pub fn upload(&mut self,renderer:&mut crate::renderer::Renderer,scene:&Scene,selected:Option<ObjectId>,fault:crate::gpu_fault::Fault) {
        self.invalidate(scene);
        crate::browser_upload::prepare(renderer,scene,selected,fault);
    }

    pub fn scene(&self,source:&Scene)->Scene {
        let mut view=source.clone();
        for row in view.math_paths_mut() {
            if let crate::chain::Source::Curve(source,_)=&row.prepared.source {
                let key=(Scene::math_id(row.id),0,Rc::as_ptr(source) as usize);
                if let Some(Prepared::Curve(prepared))=self.entries.iter().find(|entry|entry.key==key).and_then(|entry|entry.data.as_ref()) {
                    row.prepared=prepared.clone();
                }
            }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Surface(Rc<session_rust::NurbsSurface>),
}
enum Prepared {
    Curve(crate::chain::PreparedChain),
    Surface(Rc<crate::surface::Prepared>,Rc<crate::mesh::Mesh>),
}
impl Prepared {
    fn upload_bytes(&self)->usize {
        match self {
            Self::Curve(prepared)=>prepared.points.len().saturating_mul(256),
            Self::Surface(_,mesh)=>mesh.vertices().len().saturating_add(mesh.indices().len()).saturating_mul(256),
        }
    }
}
struct Request { id:ObjectId, source:Source, model:Xform }
struct Entry { key:(u32,u8,usize), _source:Source, target:Quality, accepted:Quality, rejected:Option<Quality>, error:Option<&'static str>, data:Option<Prepared>, bytes:usize }

#[derive(Default)]
pub struct Presentation {
    entries:Vec<Entry>, dirty:bool, scheduled:bool,
    pub limited:Option<&'static str>, pub bytes:usize, pub generations:u64, pub uploaded:bool,
}

impl Source {
    fn key(&self,id:ObjectId)->(u32,u8,usize) {
        match self {
            Self::Curve(source)=>(Scene::math_id(id),0,Rc::as_ptr(source) as usize),
            Self::Surface(source)=>(Scene::math_id(id),1,Rc::as_ptr(source) as usize),
        }
    }
    fn projection(&self,projection:&mut Projection)->Result<(), &'static str> {
        match self {
            Self::Curve(source)=>projection.curve(source),
            Self::Surface(source)=>projection.surface(source),
            
        }
    }
    fn initial_chord(&self,current:Quality)->Option<f64> {
        if current.0.is_finite() {return None;}
        None
    }
    fn reserve(&self)->usize {
        match self {
            Self::Curve(_)=>2*1024*1024,
            Self::Surface(_)=>quality::OBJECT_BYTES,
        }
    }
    fn prepare(&self,requested:Quality)->Result<Prepared,&'static str> {
        quality::using(requested,||match self {
            Self::Curve(source)=>Ok(Prepared::Curve(crate::chain::PreparedChain::curve(source.clone())?)),
            
            Self::Surface(source)=>{
                let prepared=Rc::new(crate::surface::Prepared::new(source.clone())?);
                let mesh=Rc::new(prepared.mesh()?.display);Ok(Prepared::Surface(prepared,mesh))
            }
            
            
        })
    }
}

fn requests(scene:&Scene)->Result<Vec<Request>,&'static str> {
    if scene.objects().len()+scene.paths().len()+scene.lines().len()+scene.points().len()>1024 {
        return Err("Presentation row count exceeds the refinement budget");
    }
    let display_units=scene.objects().iter().fold(0usize,|n,row|n.saturating_add(row.mesh.vertices().len()).saturating_add(row.mesh.indices().len()))
        .saturating_add(scene.paths().iter().map(|row|row.prepared.points.len()).sum::<usize>());
    let names=scene.objects().iter().map(|row|row.guid.len()).sum::<usize>()
        .saturating_add(scene.paths().iter().map(|row|row.guid.len()).sum::<usize>());
    if display_units.saturating_mul(256)>16*1024*1024 || names>256*1024 {
        return Err("Scene upload scratch exceeds the refinement budget");
    }
    let mut out=Vec::new();
    for row in scene.paths() {
        if let crate::chain::Source::Curve(source,_)=&row.prepared.source {
            if out.len()>=128 {return Err("Mathematical source count exceeds the refinement budget");}
            out.push(Request{id:row.id,source:Source::Curve(source.clone()),model:row.model.clone()});
        }
    }
    
    for row in scene.objects() {
        if let Some(surface)=&row.surface {
            if out.len()>=128 {return Err("Mathematical source count exceeds the refinement budget");}
            out.push(Request{id:row.id,source:Source::Surface(surface.source.clone()),model:row.model.clone()});
        }
        
    }
    
    Ok(out)
}

impl Presentation {
    pub fn qualities(&self)->Vec<(u32,Option<f64>,Option<f64>,bool)> {
        self.entries.iter().map(|entry|(entry.key.0,entry.target.0.is_finite().then_some(entry.target.0),entry.accepted.0.is_finite().then_some(entry.accepted.0),entry.error.is_some())).collect()
    }
    pub fn invalidate(&mut self,scene:&Scene) {
        self.dirty=true;
        match requests(scene) {
            Ok(requests)=>self.entries.retain(|entry|requests.iter().any(|request|request.source.key(request.id)==entry.key)),
            Err(error)=>{self.entries.clear();self.limited=Some(error);}
        }
        self.bytes=self.entries.iter().map(|entry|entry.bytes).sum();
    }

    #[cfg(target_arch="wasm32")]
    pub fn upload(&mut self,renderer:&mut crate::renderer::Renderer,scene:&Scene,selected:Option<ObjectId>,fault:crate::gpu_fault::Fault) {
        self.invalidate(scene);
        crate::browser_upload::prepare(renderer,scene,selected,fault);
    }

    pub fn scene(&self,source:&Scene)->Scene {
        let mut view=source.clone();
        for row in view.math_paths_mut() {
            if let crate::chain::Source::Curve(source,_)=&row.prepared.source {
                let key=(Scene::math_id(row.id),0,Rc::as_ptr(source) as usize);
                if let Some(Prepared::Curve(prepared))=self.entries.iter().find(|entry|entry.key==key).and_then(|entry|entry.data.as_ref()) {
                    row.prepared=prepared.clone();
                }
            }
        }
        
        for row in view.objects_mut() {
            if let Some(surface)=&row.surface {
                let key=(Scene::math_id(row.id),1,Rc::as_ptr(&surface.source) as usize);
                if let Some(Prepared::Surface(prepared,mesh))=self.entries.iter().find(|entry|entry.key==key).and_then(|entry|entry.data.as_ref()) {
                    row.surface=Some(prepared.clone());row.mesh=mesh.clone();
                }
            }
```

## <a id="checkpoint-55e-identities"></a>6. Resolve surface face and domain edges

### 1. `src/pick_sources.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .iter()
                .zip(&row.mesh.parts.endpoints)
            {
                let ids = append(
                    &mut sources.edges,
                    Kind::Edge,
                    Target {
                        owner: row.id,
                        part: Part::MeshEdge(a, b),
                    },
```

Replace it with:

```rust
                .iter()
                .zip(&row.mesh.parts.endpoints)
            {
                let part = surface_edges
                    .as_mut()
                    .and_then(|edges| edges.next())
                    .map_or(Part::MeshEdge(a, b), Part::SurfaceEdge);
                let ids = append(
                    &mut sources.edges,
                    Kind::Edge,
                    Target {
                        owner: row.id,
                        part,
                    },
```

**Locate this block:**

```rust
                    },
                )?;
            }
            for (&(a, b), ends) in row
                .mesh
                .parts
```

Replace it with:

```rust
                    },
                )?;
            }
            let mut surface_edges = row
                .surface
                .as_ref()
                .map(|surface| surface.edge_ids());
            for (&(a, b), ends) in row
                .mesh
                .parts
```

**Locate this block:**

```rust
    Face(usize),
    MeshEdge(usize, usize),
    PathEdge(usize),
    Curve,
```

Replace it with:

```rust
    Face(usize),
    SurfaceEdge(usize),
    MeshEdge(usize, usize),
    PathEdge(usize),
    Curve,
```

### 2. `src/selection.rs`

**Locate this existing block:**

```rust
            Part::Face(key) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.faces.contains(&key)),
            Part::MeshEdge(a, b) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.edges.contains(&(a, b))),
            Part::PathEdge(index) => scene.paths().iter().any(|r| r.id == self.owner && matches!(&r.prepared.source,
                crate::chain::Source::Polyline(line) if line.get_points().windows(2).nth(index).is_some_and(|pair| pair[0].to_f32() != pair[1].to_f32()))),
            Part::Curve => scene.lines().iter().any(|r| r.id == self.owner) || scene.paths().iter().any(|r| r.id == self.owner && !matches!(r.prepared.source, crate::chain::Source::Polyline(_))),
            Part::Control(index) => scene.paths().iter().any(|r| r.id == self.owner && r.controls_visible && matches!(&r.prepared.source,
                crate::chain::Source::Curve(curve, _) if crate::controls::Control::new(std::rc::Rc::clone(curve), index).is_ok())),
        }
    }

    pub fn describe(self) -> String {
        match self.part {
            Part::Object => format!("Object {}", self.owner.value()),
            Part::Face(key) => format!(
                "Object {} · source face {key}",
                self.owner.value()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Part::Face(key) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.faces.contains(&key)),
            Part::SurfaceEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.surface.as_ref().is_some_and(|s|s.grid.boundaries.iter().any(|b|b.edge==edge))),
            Part::MeshEdge(a, b) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.edges.contains(&(a, b))),
            Part::PathEdge(index) => scene.paths().iter().any(|r| r.id == self.owner && matches!(&r.prepared.source,
                crate::chain::Source::Polyline(line) if line.get_points().windows(2).nth(index).is_some_and(|pair| pair[0].to_f32() != pair[1].to_f32()))),
            Part::Curve => scene.lines().iter().any(|r| r.id == self.owner) || scene.paths().iter().any(|r| r.id == self.owner && !matches!(r.prepared.source, crate::chain::Source::Polyline(_))),
            Part::Control(index) => scene.paths().iter().any(|r| r.id == self.owner && r.controls_visible && matches!(&r.prepared.source,
                crate::chain::Source::Curve(curve, _) if crate::controls::Control::new(std::rc::Rc::clone(curve), index).is_ok())),
        }
    }

    pub fn describe(self) -> String {
        match self.part {
            Part::Object => format!("Object {}", self.owner.value()),
            Part::Face(key) => format!(
                "Object {} · source face {key}",
                self.owner.value()
            ),
            Part::SurfaceEdge(edge) => format!(
                "Object {} · surface domain edge {edge}",
                self.owner.value()
```

### 3. `src/highlight.rs`

**Locate this existing block:**

```rust
                }
                Part::MeshEdge(a, b) => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
                Part::SurfaceEdge(edge) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let surface = row.surface.as_ref().unwrap();
                    let boundary = surface
                        .grid
                        .boundaries
                        .iter()
                        .find(|b| b.edge == edge)
                        .unwrap();
                    for ends in &boundary.segments {
                        let ends = ends.map(|index| {
                            let p = surface.grid.samples[index as usize].point;
                            row.model
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32()
                        });
                        strokes.push(crate::stroke::Stroke {
                            start: ends[0],
                            end: ends[1],
                            colour: yellow,
                            width: 3.,
                        });
                    }
                }
                Part::MeshEdge(a, b) => {
```

## <a id="checkpoint-55f-ownership"></a>7. Account for shared source and UV cache owners

### 1. `src/memory.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

pub fn surfaces<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 5] {
    let mut sources = HashSet::new();
    let mut prepared = HashSet::new();
    let mut usage = [0; 5];
    for scene in scenes {
        for row in scene.objects() {
            if let Some(surface) = row.surface.as_ref() {
                usage[0] += 1;
                let source = &surface.source;
                if sources.insert(Rc::as_ptr(source)) {
                    usage[3] += source.m_cv.capacity() * 8
                        + source
                            .m_nurbsknot
                            .iter()
                            .map(|knots| knots.capacity() * 8)
                            .sum::<usize>()
                        + source.facecolors.capacity()
                            * std::mem::size_of::<session_rust::Color>()
                        + source.linecolors.capacity()
                            * std::mem::size_of::<session_rust::Color>()
                        + source.name.capacity();
                }
                if prepared.insert(Rc::as_ptr(surface)) {
                    usage[4] += surface.grid.samples.capacity()
                        * std::mem::size_of::<crate::surface::Sample>()
                        + surface.grid.indices.capacity() * 4
                        + surface.grid.boundaries.capacity()
                            * std::mem::size_of::<crate::surface::Boundary>()
                        + surface
                            .grid
                            .boundaries
                            .iter()
                            .map(|b| b.segments.capacity() * 8)
                            .sum::<usize>();
                }
            }
        }
    }
    usage[1] = sources.len();
    usage[2] = prepared.len();
    usage
}
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-curve-cpu",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-surface-cpu",
        &serde_json::to_string(&crate::memory::surfaces(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    let surfaces=editor.scene.objects().iter().filter_map(|row|row.surface.as_ref().map(|s|serde_json::json!({"id":row.id.value(),"source":s.source.guid(),"controls":s.source.m_cv_count,"steps":s.grid.steps,"samples":s.grid.samples.len(),"faces":row.mesh.parts.faces,"edges":s.grid.boundaries.iter().map(|b|b.edge).collect::<Vec<_>>()}))).collect::<Vec<_>>();
    canvas.set_attribute(
        "data-surfaces",
        &serde_json::to_string(&surfaces).unwrap(),
    )?;
    canvas.set_attribute(
        "data-curve-cpu",
```

## <a id="checkpoint-55g-surface"></a>8. Draw and pick the parametric roof

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                self.objects.clear();
                self.selected = None;
            }
```

Replace it with:

```rust
                self.objects.clear();
                self.selected = None;
            }
            Action::AddSurface => {
                let source = std::rc::Rc::new(crate::surface::example());
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_surface(source).map(|_| ())
                    })?;
            }
```

**Locate this block:**

```rust
    AddNested,
    AddLine,
    AddPoint,
    AddCurve,
```

Replace it with:

```rust
    AddNested,
    AddSurface,
    AddLine,
    AddPoint,
    AddCurve,
```

### 2. `src/commands.rs`

**Locate this existing block:**

```rust
    ("Example Box", || Action::AddBox),
    ("Example Nested", || Action::AddNested),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ("Example Box", || Action::AddBox),
    ("Example Surface", || Action::AddSurface),
    ("Example Nested", || Action::AddNested),
```

### 3. `src/surface_tests.rs`

Create the file and type:

```rust
use crate::surface::{Grid, validate};
use session_rust::{NurbsSurface, Point};

fn plane() -> NurbsSurface {
    NurbsSurface::create(
        false,
        false,
        1,
        1,
        2,
        2,
        &[
            Point::new(-1., -2., 0.),
            Point::new(-1., 2., 0.),
            Point::new(1., -2., 0.),
            Point::new(1., 2., 0.),
        ],
    )
    .unwrap()
}

#[test]
fn parametric_grid_retains_exact_uvs_analytic_normals_and_four_original_boundaries() {
    let source = plane();
    let before = source.to_proto();
    let grid = Grid::new(&source).unwrap();
    assert_eq!(grid.steps, [16, 16]);
    assert_eq!(grid.samples.len(), 289);
    assert_eq!(grid.indices.len(), 16 * 16 * 6);
    assert_eq!(grid.samples[0].point, [-1., -2., 0.]);
    assert_eq!(grid.samples[288].point, [1., 2., 0.]);
    assert_eq!(grid.samples[144].point, [0.; 3]);
    assert_eq!(grid.samples[144].uv, [0.5, 0.5]);
    assert_eq!(grid.normal(144).unwrap(), [0., 0., 1.]);
    assert_eq!(
        grid.boundaries
            .iter()
            .map(|b| b.edge)
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    assert!(
        grid.boundaries
            .iter()
            .all(|b| b.segments.len() == 16)
    );
    assert_eq!(source.to_proto(), before);
    assert!(
        grid.indices
            .iter()
            .all(|i| (*i as usize) < grid.samples.len())
    );
}

#[test]
fn rational_sampling_preserves_weights_and_original_nonunit_parameter_domains() {
    let mut source = plane();
    source.to_rational();
    assert!(source.set_cv_4d(1, 1, 2., 4., 0., 2.));
    source.m_nurbsknot[0] = vec![2., 9.];
    source.m_nurbsknot[1] = vec![-3., 5.];
    let before = source.to_proto();
    let grid = Grid::new(&source).unwrap();
    assert_eq!(grid.samples[0].uv, [2., -3.]);
    assert_eq!(grid.samples.last().unwrap().uv, [9., 5.]);
    assert!(
        (grid.samples[144].point[0] - 0.2).abs() < 1e-12
            && (grid.samples[144].point[1] - 0.4).abs() < 1e-12
    );
    for sample in &grid.samples {
        let point = source
            .point_at(sample.uv[0], sample.uv[1])
            .unwrap();
        assert_eq!(sample.point, [point[0], point[1], point[2]]);
    }
    assert_eq!(
        source.get_cv_4d(1, 1),
        Some((2., 4., 0., 2.))
    );
    assert_eq!(source.to_proto(), before);
}

#[test]
fn malformed_controls_weights_knots_and_overflow_are_refused_before_evaluation() {
    let mut source = plane();
    source.m_cv.pop();
    assert!(validate(&source).is_err());
    let mut source = plane();
    source.m_cv_stride = [usize::MAX, 3];
    assert!(validate(&source).is_err());
    let mut source = plane();
    source.m_cv_stride = [3, 3];
    assert!(validate(&source).is_err());
    let mut source = plane();
    source.m_cv[0] = f64::NAN;
    assert!(validate(&source).is_err());
    let mut source = plane();
    source.m_nurbsknot[0] = vec![2., 1.];
    assert!(validate(&source).is_err());
    let mut source = plane();
    source.m_nurbsknot[0] = vec![1., 1.];
    assert!(validate(&source).is_err());
    let mut source = plane();
    source.to_rational();
    source.m_cv[3] = 0.;
    assert!(validate(&source).is_err());
    let mut source = plane();
    source.m_cv_count = [usize::MAX, usize::MAX];
    assert!(validate(&source).is_err());
    let mut source = plane();
    source.m_order[0] = 33;
    assert!(validate(&source).is_err());
}

#[test]
fn source_owned_surface_draws_one_face_and_only_original_domain_edges() {
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::Close)
        .unwrap();
    e.apply(crate::editor::Action::AddSurface)
        .unwrap();
    let row = &e.scene.objects()[0];
    let id = row.id;
    let surface = row.surface.as_ref().unwrap();
    assert_eq!(
        row.metadata.source_guid,
        surface.source.guid()
    );
    assert_eq!(row.mesh.parts.faces, vec![0]);
    assert!(
        row.mesh
            .parts
            .triangles
            .iter()
            .all(|slot| *slot == 0)
    );
    assert_eq!(row.mesh.parts.endpoints.len(), 64);
    assert_eq!(surface.grid.samples[144].uv, [0.5, 0.5]);
    let sources = crate::pick_sources::Sources::new(&e.scene).unwrap();
    assert_eq!(sources.faces.len(), 1);
    assert_eq!(sources.edges.len(), 64);
    assert!(sources.edges.iter().all(|target| matches!(
        target.part,
        crate::pick_sources::Part::SurfaceEdge(0..=3)
    )));
    assert!(
        crate::document::snapshot(&e.scene)
            .unwrap_err()
            .contains("CAD surface")
    );
    assert_eq!(
        crate::memory::surfaces([&e.scene])[..3],
        [1, 1, 1]
    );
    let before = e.scene.objects()[0]
        .surface
        .as_ref()
        .unwrap()
        .source
        .to_proto();
    e.select_objects([id], false);
    e.apply(crate::editor::Action::Translate([
        0.123456789012345,
        0.,
        0.,
    ]))
    .unwrap();
    assert_eq!(
        e.scene.objects()[0]
            .surface
            .as_ref()
            .unwrap()
            .source
            .to_proto(),
        before
    );
    e.apply(crate::editor::Action::Undo).unwrap();
    assert_eq!(
        e.scene.placement(id).unwrap().m,
        session_rust::Xform::identity().m
    );
    e.apply(crate::editor::Action::Undo).unwrap();
    assert!(e.scene.objects().is_empty());
    e.apply(crate::editor::Action::Redo).unwrap();
    assert_eq!(e.scene.objects()[0].id, id);
}

#[test]
fn metadata_and_display_owners_do_not_pin_original_surface_after_close() {
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::Close)
        .unwrap();
    e.apply(crate::editor::Action::AddSurface)
        .unwrap();
    let row = &e.scene.objects()[0];
    let weak = std::rc::Rc::downgrade(&row.surface.as_ref().unwrap().source);
    let metadata = row.metadata.clone();
    let display = row.mesh.clone();
    assert_eq!(
        crate::memory::surfaces(e.presentation_scenes())[1..3],
        [1, 1]
    );
    e.apply(crate::editor::Action::Close)
        .unwrap();
    assert!(weak.upgrade().is_none());
    assert_eq!(metadata.name, "Parametric roof");
    assert_eq!(display.indices().len(), 16 * 16 * 6);
    assert_eq!(
        crate::memory::surfaces(e.presentation_scenes()),
        [0; 5]
    );
}

#[test]
fn transposed_padded_control_layout_samples_without_rewriting_original_storage() {
    let source = plane();
    let mut transposed = source.clone();
    transposed.m_cv_stride.swap(0, 1);
    let before = transposed.to_proto();
    let grid = Grid::new(&transposed).unwrap();
    assert_eq!(grid.samples[16].point, [1., -2., 0.]);
    assert_eq!(grid.samples[272].point, [-1., 2., 0.]);
    assert_eq!(transposed.to_proto(), before);
    let mut padded = plane();
    padded.m_cv_stride = [12, 6];
    padded.m_cv = vec![0.; 21];
    for (i, j) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
        assert!(padded.set_cv(i, j, &source.get_cv(i, j).unwrap()));
    }
    let before = padded.to_proto();
    assert_eq!(
        Grid::new(&padded).unwrap().samples[144].point,
        [0.; 3]
    );
    assert_eq!(padded.to_proto(), before);
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
mod shortcut_focus_tests;

pub mod surface;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod shortcut_focus_tests;

pub mod surface;

#[cfg(test)]
mod surface_tests;
```

### 5. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(e.scene.placement(id).unwrap().m, original);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(e.scene.placement(id).unwrap().m, original);
}

#[test]
fn actual_surface_pick_keeps_one_cad_face_and_four_domain_edge_ids() {
    use crate::pick_sources::{Mode, Part};
    let mut scene = Scene::demo();
    scene.clear();
    let source = Rc::new(crate::surface::example());
    let _ = source.guid();
    let before = source.to_proto();
    let id = scene.insert_surface(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    let camera = crate::camera::Camera::default();
    let transform = camera.uniform();
    let gpu = renderer.stats();
    let finish_at = |renderer: &mut Renderer| {
        renderer
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        renderer
            .take_pick(transform)
            .unwrap()
            .unwrap()
            .and_then(|hit| renderer.resolve_pick(&scene, hit))
    };
    renderer.set_pick_mode(Mode::Face);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    assert_eq!(
        finish_at(&mut renderer).unwrap().part,
        Part::Face(0)
    );
    renderer.set_pick_mode(Mode::Edge);
    for (point, edge) in [
        ([-1., 0., 0.], 0),
        ([1., 0., 0.], 1),
        ([0., -1., 0.], 2),
        ([0., 1., 0.], 3),
    ] {
        let point =
            camera
                .view_projection()
                .transform_point(&session_rust::Point::new(
                    point[0], point[1], point[2],
                ));
        renderer
            .request_pick([point[0] as f32, point[1] as f32], transform)
            .unwrap();
        let target = finish_at(&mut renderer).unwrap();
        assert_eq!(target.owner, id);
        assert_eq!(target.part, Part::SurfaceEdge(edge));
    }
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(source.to_proto(), before);
}
```

## <a id="checkpoint-55za-read-original-edge-use-samples"></a>9. Read edge-use samples

### 1. `src/cad_edges.rs`

Create the file and type:

```rust
use session_rust::Mesh;
use session_rust::brep::{BRep, BRepOrientation};

fn sample_order(a: &f64, b: &f64) -> std::cmp::Ordering {
    a.partial_cmp(b)
        .unwrap_or(std::cmp::Ordering::Equal)
}

fn parameter_order(a: &(f64, usize), b: &(f64, usize)) -> std::cmp::Ordering {
    sample_order(&a.0, &b.0).then(a.1.cmp(&b.1))
}

fn total_parameter_order(a: &(f64, usize), b: &(f64, usize)) -> std::cmp::Ordering {
    a.0.total_cmp(&b.0).then(a.1.cmp(&b.1))
}

fn same_parameter(a: &mut (f64, usize), b: &mut (f64, usize)) -> bool {
    a.0 == b.0
}

fn parse_sample_index(value: &str) -> Option<usize> {
    value.parse().ok()
}

pub struct EdgeUse {
    pub edge: usize,
    pub face: usize,
    pub orientation: BRepOrientation,
}

fn pcurve_ends(b: &BRep, eu: &EdgeUse) -> Option<([f64; 2], [f64; 2])> {
    let ci = b.pcurve_index(eu.edge, eu.face, eu.orientation);

    if ci < 0 {
        return None;
    }

    let c = b.m_curves_2d.get(ci as usize)?;

    if c.degree() != 1 || c.is_rational() || c.cv_count() != 2 {
        return None;
    }

    let p0 = c.get_cv(0)?;
    let p1 = c.get_cv(c.cv_count().checked_sub(1)?)?;
    Some(([p0[0], p0[1]], [p1[0], p1[1]]))
}

fn sample_values(fm: &Mesh, name: &str) -> Vec<f64> {
    let mut vals = Vec::new();

    for vertex in fm.vertex.values() {
        if let Some(value) = vertex.attributes.get(name) {
            vals.push(*value);
        }
    }

    vals.sort_unstable_by(sample_order);
    vals.dedup();
    vals
}

fn nearest_sample(vals: &[f64], target: f64, wrap: Option<(f64, f64)>) -> Option<f64> {
    let mut best: Option<(f64, f64)> = None;

    for &v in vals {
        let mut d = (v - target).abs();

        if let Some((start, end)) = wrap {
            d = d.min((v - (target - (end - start))).abs());
        }

        if match best {
            Some((bd, _)) => d < bd,
            None => true,
        } {
            best = Some((d, v));
        }
    }

    Some(best?.1)
}

pub struct EdgeChain {
    pub edge: usize,
    pub face: usize,
    pub keys: Vec<usize>,
    pub other: Option<usize>,
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod surface_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod surface_tests;

pub mod cad_edges;
```

## <a id="checkpoint-55zb-follow-original-isoparametric-boundaries"></a>10. Follow isoparametric boundaries

### 1. `src/cad_edges.rs`

**Locate this existing block:**

```rust
}

pub struct EdgeChain {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn iso_chain(b: &BRep, fm: &Mesh, eu: &EdgeUse) -> Option<Vec<usize>> {
    let e = b.m_edges.get(eu.edge)?;

    if e.degenerated {
        return None;
    }

    let (p0, p1) = pcurve_ends(b, eu)?;

    if !p0.into_iter().chain(p1).all(f64::is_finite)
        || (p0[0] != p1[0] && p0[1] != p1[1])
    {
        return None;
    }

    let fixed = if (p1[0] - p0[0]).abs() <= (p1[1] - p0[1]).abs() {
        0
    } else {
        1
    };
    let (fixed_name, free_name) = if fixed == 0 {
        ("u", "v")
    } else {
        ("v", "u")
    };
    let vals = sample_values(fm, fixed_name);

    if vals.is_empty() {
        return None;
    }

    let face = b.m_faces.get(eu.face)?;
    let srf = b
        .m_surfaces
        .get(face.surface_index as usize)?;
    let wrap = if srf.is_closed(fixed) {
        srf.domain(fixed)
    } else {
        None
    };
    let at = nearest_sample(&vals, p0[fixed], wrap)?;
    let wrapped_start = match wrap {
        Some((start, end)) => p0[fixed] - (end - start) == at,
        None => false,
    };

    if at != p0[fixed] && !wrapped_start {
        return None;
    }

    let mut on_line: Vec<(f64, usize)> = Vec::new();

    for (&key, vd) in fm.vertex.iter() {
        let (Some(&f), Some(&t)) = (
            vd.attributes.get(fixed_name),
            vd.attributes.get(free_name),
        ) else {
            continue;
        };
        let lo = p0[1 - fixed].min(p1[1 - fixed]);
        let hi = p0[1 - fixed].max(p1[1 - fixed]);

        if f == at && t >= lo && t <= hi {
            on_line.push((t, key));
        }
    }

    if on_line.len() < 2 {
        return None;
    }

    on_line.sort_unstable_by(parameter_order);
    let mut keys = Vec::with_capacity(on_line.len());

    for (_, key) in on_line {
        keys.push(key);
    }

    if e.start_vertex == e.end_vertex {
        keys.push(keys[0]);
    }

    Some(keys)
}

pub struct EdgeChain {
```

## <a id="checkpoint-55zc-follow-original-constrained-trim-labels"></a>11. Follow constrained trim labels

### 1. `src/cad_edges.rs`

**Locate this existing block:**

```rust
    pub other: Option<usize>,
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub other: Option<usize>,
}

fn constrained_chain(fm: &Mesh, edge: usize) -> Option<Vec<usize>> {
    let prefix = format!("brep_edge/{edge}/");
    let mut uses = std::collections::BTreeMap::<
        usize,
        std::collections::BTreeMap<usize, usize>,
    >::new();

    for (&key, vertex) in &fm.vertex {
        for name in vertex.attributes.keys() {
            let Some(suffix) = name.strip_prefix(&prefix) else {
                continue;
            };
            let Some((use_id, sample)) = suffix.split_once('/') else {
                continue;
            };
            let (Ok(use_id), Ok(sample)) = (
                use_id.parse::<usize>(),
                sample.parse::<usize>(),
            ) else {
                continue;
            };
            let value = uses
                .entry(use_id)
                .or_default()
                .entry(sample)
                .or_insert(key);
            *value = (*value).min(key);
        }
    }

    for (&use_id, samples) in &uses {
        if samples.len() < 2 {
            continue;
        }

        let mut keys = Vec::with_capacity(samples.len());

        for (expected, (&sample, &key)) in samples.iter().enumerate() {
            if expected != sample {
                return None;
            }

            keys.push(key);
        }

        let interval_prefix = format!("brep_edge_interval/{edge}/{use_id}/");
        let mut ordered = Vec::with_capacity(keys.len());

        for (sample, key) in keys.into_iter().enumerate() {
            ordered.push((sample as f64, key));
        }

        for (&key, vertex) in &fm.vertex {
            for name in vertex.attributes.keys() {
                if let Some(sample) = name
                    .strip_prefix(&interval_prefix)
                    .and_then(parse_sample_index)
                    && let Some(&t) = vertex.attributes.get(name)
                    && sample + 1 < samples.len()
                    && t.is_finite()
                    && t > 0.0
                    && t < 1.0
                {
                    ordered.push((sample as f64 + t, key));
                }
            }
        }

        ordered.sort_unstable_by(total_parameter_order);
        ordered.dedup_by(same_parameter);
        let mut keys = Vec::with_capacity(ordered.len());

        for (_, key) in ordered {
            keys.push(key);
        }

        return Some(keys);
    }

    None
}
```

## Build & Run

![Recorded visual output for Lesson 9.1 (55zc-follow-original-constrained-trim-labels)](../../screenshots/journey/55zc-follow-original-constrained-trim-labels.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 9.1
npm --prefix ../session_tests run course -- save 9.1
```

Continue with [9.2 · Shared CAD boundaries across incident faces](56-chains.md).
