# 9.2 · Shared CAD boundaries across incident faces

**Steps:** 3 · **Source:** [app/walk/brep.rs](../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs)

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

## <a id="checkpoint-56-chains"></a>1. Connect CAD edge chains

### 1. `src/cad_edges.rs`

**Locate this existing block:**

```rust
    None
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    None
}

pub fn edge_chains(b: &BRep, fms: &[Mesh]) -> Vec<Option<EdgeChain>> {
    let mut out = Vec::with_capacity(b.m_edges.len());

    for (ei, e) in b.m_edges.iter().enumerate() {
        if e.degenerated {
            out.push(None);
            continue;
        }

        let uses = b.edge_faces(ei);
        let mut found: Option<EdgeChain> = None;

        for (k, u) in uses.iter().enumerate() {
            let eu = EdgeUse {
                edge: ei,
                face: u.index as usize,
                orientation: u.orientation,
            };
            let keys = match constrained_chain(&fms[eu.face], ei) {
                Some(keys) => keys,
                None => match iso_chain(b, &fms[eu.face], &eu) {
                    Some(keys) => keys,
                    None => continue,
                },
            };
            let mut other = None;

            for (j, candidate) in uses.iter().enumerate() {
                if j != k && candidate.index as usize != eu.face {
                    other = Some(candidate.index as usize);
                    break;
                }
            }

            found = Some(EdgeChain {
                edge: ei,
                face: eu.face,
                keys,
                other,
            });
            break;
        }

        out.push(found);
    }

    out
}
```

## <a id="checkpoint-56a-shared"></a>2. Mesh incident faces with shared boundary constraints

### 1. `src/cad_boundary.rs`

Create the file and type:

```rust
use std::rc::Rc;
use session_rust::{BRep,Mesh};

pub const QUALITY:(f64,f64)=(5.,0.001);
pub struct Prepared {pub source:Rc<BRep>,pub faces:Vec<Mesh>,pub chains:Vec<Option<crate::cad_edges::EdgeChain>>}
impl Prepared {
    pub fn new(source:Rc<BRep>)->Result<Self,&'static str> {
        if !source.is_valid() {return Err("BRep topology contains an invalid reference");}
        if source.m_faces.len()>65_536||source.m_edges.len()>262_144 {return Err("BRep topology exceeds the display budget");}
        for surface in &source.m_surfaces {crate::surface::validate(surface)?;}
        for curve in source.m_curves_3d.iter().chain(&source.m_curves_2d) {crate::curve::validate(curve)?;}
        let _=source.guid();let faces=if crate::math_quality::current().0.is_finite() {crate::math_cad::faces(&source)?} else {source.face_meshes_q(Some(QUALITY))};
        if faces.len()!=source.m_faces.len()||faces.iter().any(|face|face.face.is_empty()) {return Err("BRep face could not be tessellated");}
        let chains=crate::cad_edges::edge_chains(&source,&faces);
        for (index,(edge,chain)) in source.m_edges.iter().zip(&chains).enumerate() {
            if !edge.degenerated&&!source.edge_faces(index).is_empty()&&chain.is_none() {return Err("A CAD edge has no coherent face boundary");}
        }
        Ok(Self {source,faces,chains})
    }
    pub fn points(&self,edge:usize)->Option<Vec<[f64;3]>> {
        Some(self.display(edge)?.0)
    }
    pub fn display(&self,edge:usize)->Option<(Vec<[f64;3]>,Vec<usize>)> {
        let chain=self.chains.get(edge)?.as_ref()?;let face=self.faces.get(chain.face)?;
        if crate::math_quality::current().0.is_finite() {let display=crate::math_cad::boundary(&self.source,chain,face).ok()?;return Some((display.points,display.spans));}
        let coarse:Vec<[f64;3]>=chain.keys.iter().map(|key|face.vertex.get(key).map(|v|[v.x,v.y,v.z])).collect::<Option<_>>()?;
        let source=self.source.m_edges.get(edge)?;
        let Some(curve)=usize::try_from(source.curve_3d_index).ok().and_then(|index|self.source.m_curves_3d.get(index)) else {return Some((coarse,(0..chain.keys.len().saturating_sub(1)).collect()))};
        let count=crate::curve::sample(curve).ok()?.len().saturating_sub(1).max(1);
        let (start,end)=curve.domain();let period=end-start;
        if coarse.len()<2||!period.is_finite()||period<=0. {return None;}
        let mut parameters:Vec<f64>=coarse.iter().map(|&p|curve_parameter(curve,p,8*count)).collect();
        if curve.is_closed() {
            for k in 1..parameters.len() {
                while parameters[k]-parameters[k-1]>0.5*period {parameters[k]-=period;}
                while parameters[k]-parameters[k-1]< -0.5*period {parameters[k]+=period;}
            }
            let last=parameters.len()-1;
            if chain.keys.first()==chain.keys.last()&&last>1 {parameters[last]=parameters[0]+if parameters[last-1]>parameters[0] {period}else{-period};}
        }
        let wrap=|t:f64|if curve.is_closed(){start+(t-start).rem_euclid(period)}else{t.clamp(start,end)};
        let total:f64=parameters.windows(2).map(|p|(p[1]-p[0]).abs()).sum();
        let point=|t:f64|{let p=curve.point_at(wrap(t));[p[0],p[1],p[2]]};
        let mut points=vec![point(parameters[0])];let mut spans=Vec::new();
        for k in 0..parameters.len()-1 {
            let share=if total>0. {(parameters[k+1]-parameters[k]).abs()/total}else{0.};
            let steps=((count as f64*share).ceil() as usize).max(1);
            for j in 1..=steps {points.push(point(parameters[k]+(parameters[k+1]-parameters[k])*j as f64/steps as f64));spans.push(k);}
        }
        Some((points,spans))
    }
}

fn curve_parameter(curve: &session_rust::NurbsCurve, p: [f64; 3], count: usize) -> f64 {
    let (t0, t1) = curve.domain();
    let distance = |t: f64| {
        let q = curve.point_at(t);
        (0..3).map(|axis| (q[axis]-p[axis]).powi(2)).sum::<f64>()
    };
    let step = (t1 - t0) / count as f64;
    let nearest = (0..=count).min_by(|&i, &j| distance(t0 + step * i as f64).total_cmp(&distance(t0 + step * j as f64))).unwrap_or(0);
    let (mut lo, mut hi) = ((t0 + step * (nearest as f64 - 1.0)).max(t0), (t0 + step * (nearest as f64 + 1.0)).min(t1));
    let ratio = 0.5 * (5.0f64.sqrt() - 1.0);

    for _ in 0..60 {
        let (m1, m2) = (hi - ratio * (hi - lo), lo + ratio * (hi - lo));

        if distance(m1) < distance(m2) {
            hi = m2;
        } else {
            lo = m1;
        }
    }

    0.5 * (lo + hi)
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod math_quality;
pub mod math_view;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cad_boundary;

pub mod math_quality;
pub mod math_view;
pub mod math_cad;
```

### 3. `src/math_cad.rs`

Use the BRep faces for shaded triangles and its original 3D NURBS edge curves for visible boundaries. Associate curve segments with the face samples to obtain facing and sag offsets; triangle diagonals never become boundary lines. Apply one absolute chord target across the whole shape, then bound candidate memory and meshing work.

OCCT also prepares [shaded triangles and face-boundary segments separately](https://occt3d.com/dev/doc/refman/html/class_std_prs___shaded_shape.html), and recommends [consistent deflection across shape presentations](https://occt3d.com/dev/doc/refman/html/class_std_prs___tool_triangulated_shape.html). Here, physical camera scale supplies the requested display tolerance.

Create the file and type:

```rust
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
```

### 4. `Cargo.toml`

Bound mathematical display refinement while retaining original source identity.

**Locate this existing block:**

```toml
session_rust = { path = "../../../session_rust", default-features = false }
```

Edit the existing block to match this code; keep unchanged lines:

```toml
session_rust = { path = "../../../session_rust", default-features = false, features = ["viewer-display"] }
```

## <a id="checkpoint-56b-boundaries"></a>3. Prove straight and circular shared boundaries

### 1. `src/cad_boundary_tests.rs`

Create the file and type:

```rust
use crate::cad_boundary::Prepared;
use session_rust::BRep;
use std::rc::Rc;

#[test]
fn shared_box_edges_follow_original_face_samples_and_preserve_brep_source() {
    let source = Rc::new(BRep::create_box(4., 3., 2.));
    let _ = source.guid();
    let before = source.to_proto();
    let prepared = Prepared::new(source.clone()).unwrap();
    assert!(Rc::ptr_eq(&prepared.source, &source));
    assert_eq!(prepared.faces.len(), 6);
    assert_eq!(prepared.chains.len(), 12);
    for (edge, chain) in prepared.chains.iter().enumerate() {
        let chain = chain.as_ref().unwrap();
        assert_eq!(chain.edge, edge);
        assert!(chain.other.is_some());
        let points = prepared.points(edge).unwrap();
        assert!(points.len() >= 2);
        let other = &prepared.faces[chain.other.unwrap()];
        for p in points {
            assert!(
                other.vertex.values().any(|v| [v.x, v.y, v.z]
                    .iter()
                    .zip(p)
                    .all(|(a, b)| (a - b).abs() < 1e-10)),
                "Shared edge {edge} follows both incident faces"
            );
        }
    }
    assert_eq!(source.to_proto(), before);
    let weak = Rc::downgrade(&source);
    drop(source);
    assert!(weak.upgrade().is_some());
    drop(prepared);
    assert!(weak.upgrade().is_none());
}

#[test]
fn circular_boundary_samples_are_shared_by_cap_and_wall() {
    let source = Rc::new(BRep::create_cylinder(2., 3.));
    let _ = source.guid();
    let before = source.to_proto();
    let prepared = Prepared::new(source.clone()).unwrap();
    for edge in [0, 1] {
        let chain = prepared.chains[edge].as_ref().unwrap();
        let other = &prepared.faces[chain.other.unwrap()];
        let points = prepared.points(edge).unwrap();
        assert!(points.len() > 16);
        for p in points {
            assert!(
                other.vertex.values().any(|v| [v.x, v.y, v.z]
                    .iter()
                    .zip(p)
                    .all(|(a, b)| (a - b).abs() < 1e-9)),
                "Circle {edge} follows both incident faces"
            );
        }
    }
    assert_eq!(source.to_proto(), before);
}

#[test]
fn invalid_topology_and_control_storage_fail_before_meshing() {
    let mut source = BRep::create_box(4., 3., 2.);
    source.m_faces[0].surface_index = -1;
    assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source = BRep::create_box(4., 3., 2.);
    source.m_surfaces[0].m_cv.pop();
    assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source = BRep::create_box(4., 3., 2.);
    source.m_curves_3d[0].m_cv.pop();
    assert!(Prepared::new(Rc::new(source)).is_err());
    let mut source = BRep::create_box(4., 3., 2.);
    source.m_curves_2d[0].m_nurbsknot.reverse();
    source.m_curves_2d[0].m_nurbsknot[0] = f64::NAN;
    assert!(Prepared::new(Rc::new(source)).is_err());
}

fn adjacent() -> BRep {
    use session_rust::BRep;
    use session_rust::NurbsCurve;
    use session_rust::NurbsSurface;
    use session_rust::Point;
    use session_rust::brep::BRepOrientation;
    use session_rust::brep::BRepRef;

    let mut b = BRep::new();
    let mut surfaces = Vec::new();

    for face in 0..2 {
        let mut points = Vec::new();

        for i in 0..3 {
            for j in 0..2 {
                let z = if i != 1 {
                    0.0
                } else if j == 0 || face == 0 {
                    0.5
                } else {
                    4.0
                };
                points.push(Point::new(
                    i as f64 * 0.5,
                    j as f64
                        * if face == 0 {
                            1.0
                        } else {
                            -1.0
                        },
                    z,
                ));
            }
        }

        let surface = NurbsSurface::create(false, false, 2, 1, 3, 2, &points).unwrap();
        let si = b.add_surface(&surface);
        let corners = [
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];
        let mut vertices = Vec::new();

        for corner in &corners {
            vertices.push(
                b.add_vertex(
                    &surface
                        .point_at(corner[0], corner[1])
                        .unwrap(),
                    0.0,
                ),
            );
        }

        let mut edges = Vec::new();

        for side in 0..4 {
            let a = corners[side];
            let z = corners[(side + 1) % 4];
            let mut edge = 0;

            if side != 0 || face != 1 {
                let dir = if a[0] != z[0] {
                    0
                } else {
                    1
                };
                let mut curve = surface.iso_curve(dir, a[1 - dir]).unwrap();

                if a[dir] > z[dir] {
                    curve.reverse();
                }

                let ci = b.add_curve_3d(&curve);
                edge = b.add_edge(
                    ci as i32,
                    vertices[side] as i32,
                    vertices[(side + 1) % 4] as i32,
                );
            }

            let pc = NurbsCurve::create(
                false,
                1,
                &[
                    Point::new(a[0], a[1], 0.0),
                    Point::new(z[0], z[1], 0.0),
                ],
            );
            let ci = b.add_curve_2d(&pc);
            b.add_pcurve(edge, si, ci as i32, -1);
            edges.push(BRepRef::new(
                edge as i32,
                BRepOrientation::Forward,
            ));
        }

        let wi = b.add_wire(&edges);
        b.add_face(
            si as i32,
            &[BRepRef::new(
                wi as i32,
                BRepOrientation::Forward,
            )],
            1e-8,
        );
        surfaces.push(surface);
    }

    b
}

#[test]
fn unequal_independent_grids_become_a_coherent_curved_shared_boundary() {
    use session_rust::remesh_nurbssurface_grid::RemeshNurbsSurfaceGrid;
    let source = Rc::new(adjacent());
    let _ = source.guid();
    let before = source.to_proto();
    let points = |mesh: &session_rust::Mesh| {
        let mut p = mesh
            .vertex
            .values()
            .filter(|v| v.attributes.get("v") == Some(&0.))
            .map(|v| [v.x, v.y, v.z])
            .collect::<Vec<_>>();
        p.sort_by(|a, b| a[0].total_cmp(&b[0]));
        p.dedup();
        p
    };
    let a = RemeshNurbsSurfaceGrid::from_u_v_q(&source.m_surfaces[0], 0, 0, 20., 0.005);
    let b = RemeshNurbsSurfaceGrid::from_u_v_q(&source.m_surfaces[1], 0, 0, 20., 0.005);
    assert_ne!(points(&a).len(), points(&b).len());
    let prepared = Prepared::new(source.clone()).unwrap();
    let a = points(&prepared.faces[0]);
    let b = points(&prepared.faces[1]);
    assert_eq!(a, b);
    assert!(a.len() > 7);
    assert_eq!(source.to_proto(), before);
    let normal = |surface: &session_rust::NurbsSurface| surface.normal_at(0.5, 0.);
    assert_ne!(
        normal(&source.m_surfaces[0]).to_f32(),
        normal(&source.m_surfaces[1]).to_f32()
    );
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod cad_edges;

pub mod cad_boundary;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cad_edges;

pub mod cad_boundary;

#[cfg(test)]
mod cad_boundary_tests;
```

## Build & Run

![Recorded visual output for Lesson 9.2 (56b-boundaries)](../../screenshots/journey/56b-boundaries.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 9.2
npm --prefix ../session_tests run course -- save 9.2
```

Continue with [9.3 · Trimmed curved faces with real open holes](57-parameters.md).
