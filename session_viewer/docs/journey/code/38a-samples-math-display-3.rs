use session_rust::{NurbsCurve, NurbsSurface, Xform};
use std::cell::Cell;

pub const CURVE_SEGMENTS: usize = 1024;
pub const SURFACE_STEPS: usize = 128;
pub const BREP_WORK: usize = 16_384;
pub const OBJECT_BYTES: usize = 16 * 1024 * 1024;
pub const SCENE_BYTES: usize = 64 * 1024 * 1024;
pub const TRANSIENT_BYTES: usize = 32 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quality(pub f64);
impl Default for Quality { fn default() -> Self { Self(f64::INFINITY) } }
impl Quality {
    pub fn for_error(error: f64, previous: Self) -> Self {
        if !error.is_finite() || error <= 0. { return previous; }
        if previous.0 <= error && error < previous.0 * 2. { return previous; }
        Self(2f64.powf((error.log2() * 2.).floor().clamp(-80., 80.) / 2.))
    }
}

thread_local! {
    static QUALITY: Cell<Quality> = Cell::new(Quality(f64::INFINITY));
    static FAILED: Cell<bool> = const { Cell::new(false) };
    static CURVES: Cell<usize> = const { Cell::new(0) };
}
pub fn current() -> Quality { QUALITY.get() }
pub fn fail() { FAILED.set(true); }
pub fn using<T>(quality: Quality, prepare: impl FnOnce() -> Result<T, &'static str>) -> Result<T, &'static str> {
    struct Restore(Quality, bool, usize);
    impl Drop for Restore { fn drop(&mut self) { QUALITY.set(self.0); FAILED.set(self.1); CURVES.set(self.2); } }
    let _restore = Restore(QUALITY.replace(quality), FAILED.replace(false), CURVES.replace(0));
    let result = prepare()?;
    if FAILED.get() { Err("Mathematical display reached its finite budget") } else { Ok(result) }
}
pub fn charge_curves(segments:usize)->Result<(), &'static str> {
    let next=CURVES.get().saturating_add(segments);
    if next>8192 {fail();return Err("Object curve segments exceed the display budget");}
    CURVES.set(next);Ok(())
}

pub fn curve_supported(curve: &NurbsCurve) -> bool {
    (2..=3).contains(&curve.m_dim) && (2..=16).contains(&curve.m_order)
        && curve.m_cv_count>=curve.m_order && curve.m_cv_count <= 256 && curve.m_cv.len() <= 2048 && curve.m_nurbsknot.len() <= 512
        && curve.m_cv_stride>=curve.m_dim+usize::from(curve.m_is_rat)
        && curve.m_cv_count.saturating_sub(1).checked_mul(curve.m_cv_stride).and_then(|n|n.checked_add(curve.m_dim+usize::from(curve.m_is_rat))).is_some_and(|n|n<=curve.m_cv.len())
        && curve.name.len()<=4096 && (!curve.has_guid()||curve.guid().len()<=4096)
        && colours_supported(curve.pointcolors.iter().chain(&curve.linecolors))
        && (0..curve.m_cv_count).all(|i|curve.get_cv_4d(i).and_then(control).is_some())
}
pub fn surface_supported(surface: &NurbsSurface) -> bool {
    surface.m_dim==3 && (0..2).all(|i|(2..=16).contains(&surface.m_order[i])&&surface.m_cv_count[i]>=surface.m_order[i]&&surface.m_cv_count[i]<=64)
        && surface.m_cv.len() <= 8192
        && surface.m_cv_count[0].saturating_sub(1).checked_mul(surface.m_cv_stride[0])
            .and_then(|a|surface.m_cv_count[1].saturating_sub(1).checked_mul(surface.m_cv_stride[1]).and_then(|b|a.checked_add(b)))
            .and_then(|n|n.checked_add(3+usize::from(surface.m_is_rat))).is_some_and(|n|n<=surface.m_cv.len())
        && surface.m_nurbsknot.iter().all(|knots| knots.len() <= 128)
        && surface.m_mesh.is_none() && surface.name.len()<=4096
        && (!surface.has_guid()||surface.guid().len()<=4096)
        && colours_supported(surface.pointcolors.iter().chain(&surface.linecolors).chain(&surface.facecolors))
        && (0..surface.m_cv_count[0]).all(|i|(0..surface.m_cv_count[1]).all(|j|surface.get_cv_4d(i,j).and_then(control).is_some()))
}
pub fn control((x,y,z,w):(f64,f64,f64,f64))->Option<[f64;3]> {
    if ![x,y,z,w].iter().all(|v|v.is_finite())||w<=1e-14 {return None;}
    let p=[x/w,y/w,z/w];p.iter().all(|v|v.is_finite()).then_some(p)
}
pub fn colours_supported<'a>(colours:impl Iterator<Item=&'a session_rust::Color>)->bool {
    let mut count=0usize;let mut bytes=0usize;
    for colour in colours {
        count+=1;bytes=bytes.saturating_add(colour.name.len()).saturating_add(if colour.has_guid(){colour.guid().len()}else{0});
        if count>512||bytes>16*1024 {return false;}
    }true
}

fn point(curve: &NurbsCurve, t: f64) -> [f64; 3] {
    let p = curve.point_at(t); [p[0], p[1], p[2]]
}
fn distance(p: [f64; 3], a: [f64; 3], b: [f64; 3]) -> f64 {
    let d: [f64; 3] = std::array::from_fn(|i| b[i] - a[i]);
    let q: [f64; 3] = std::array::from_fn(|i| p[i] - a[i]);
    let length = d.iter().map(|v| v*v).sum::<f64>();
    let t = ((0..3).map(|i| d[i]*q[i]).sum::<f64>() / length.max(1e-300)).clamp(0., 1.);
    (0..3).map(|i| (q[i]-t*d[i]).powi(2)).sum::<f64>().sqrt()
}
fn flat(curve: &NurbsCurve, a: [f64; 3], b: [f64; 3], tolerance: f64) -> bool {
    (0..curve.m_cv_count).all(|i| {
        let Some(p) = curve.get_cv_4d(i).and_then(control) else { return false; };
        distance(p,a,b) <= tolerance
    })
}

pub fn parameters(curve: &NurbsCurve, quality: Quality, budget: usize) -> Result<Vec<f64>, &'static str> {
    if !curve_supported(curve) || !quality.0.is_finite() || quality.0 <= 0. {
        return Err("Curve refinement input exceeds the display budget");
    }
    let (start,end) = crate::curve::validate(curve)?;
    let budget = budget.min(CURVE_SEGMENTS);
    let spans = curve.get_span_vector();
    if budget == 0 || spans.len().saturating_sub(1) > budget { return Err("Curve knot spans exceed the display budget"); }
    let mut out = vec![start];
    let mut stack = Vec::new();
    for (index, span) in spans.windows(2).enumerate() {
        let mut piece = curve.clone();
        if !piece.trim(span[0],span[1]) { return Err("Curve span cannot be trimmed for display"); }
        stack.push((piece,span[0],span[1],0));
        let remaining = spans.len()-index-2;
        while let Some((piece,a,b,depth)) = stack.pop() {
            if flat(&piece,point(curve,a),point(curve,b),quality.0) { out.push(b); continue; }
            if depth >= 16 || out.len()+stack.len()+remaining >= budget {
                return Err("Curve refinement reached its finite segment budget");
            }
            let mid = a+(b-a)*0.5;
            let (left,right) = piece.split(mid);
            let epsilon = (end-start).abs()*1e-10;
            if (left.domain().0-a).abs()>epsilon || (left.domain().1-mid).abs()>epsilon
                || (right.domain().0-mid).abs()>epsilon || (right.domain().1-b).abs()>epsilon {
                return Err("Curve subdivision did not preserve its parameter interval");
            }
            stack.push((right,mid,b,depth+1)); stack.push((left,a,mid,depth+1));
        }
    }
    if out.last()!=Some(&end) { return Err("Curve sampling did not cover its complete domain"); }
    Ok(out)
}

pub fn curve_points(curve: &NurbsCurve) -> Result<Vec<[f64;3]>, &'static str> {
    parameters(curve,current(),CURVE_SEGMENTS)?.into_iter().map(|t| {
        let p=point(curve,t);
        if p.iter().all(|v| (*v as f32).is_finite()) { Ok(p) } else { Err("Curve sample exceeds the display range") }
    }).collect()
}

pub fn surface_parameters(surface: &NurbsSurface, direction: usize) -> Result<Vec<f64>, &'static str> {
    if !surface_supported(surface) { return Err("Surface refinement input exceeds the display budget"); }
    let domain=surface.domain(direction).ok_or("Missing surface domain")?;
    let other=surface.domain(1-direction).ok_or("Missing surface domain")?;
    let mut out=vec![domain.0,domain.1];
    for i in 0..=4 {
        let curve=surface.iso_curve(direction,other.0+(other.1-other.0)*i as f64/4.).ok_or("Surface iso-curve cannot be evaluated")?;
        out.extend(parameters(&curve,Quality(current().0*0.5),SURFACE_STEPS)?);
        out.sort_by(f64::total_cmp);out.dedup_by(|a,b|(*a-*b).abs()<(domain.1-domain.0)*1e-12);
        if out.len()>SURFACE_STEPS+1 { return Err("Surface grid reached its finite interval budget"); }
    }
    Ok(out)
}

pub struct Projection { matrix: [f64;16], size: [f64;2], min_w:f64, max_w:f64, max_xy:[f64;2] }
impl Projection {
    pub fn new(camera:&crate::camera::Camera,model:&Xform,size:[u32;2])->Self {
        Self::from_matrix(&camera.view_projection(),model,size)
    }
    pub fn from_matrix(view_projection:&Xform,model:&Xform,size:[u32;2])->Self {
        let matrix=(view_projection*model).m;
        Self {matrix,size:size.map(|n| n as f64*0.5),min_w:f64::INFINITY,max_w:0.,max_xy:[0.;2]}
    }
    pub fn point(&mut self,p:[f64;3])->Result<(), &'static str> {
        let q:[f64;4]=std::array::from_fn(|i| (0..3).map(|j|self.matrix[j*4+i]*p[j]).sum::<f64>()+self.matrix[12+i]);
        if !q.iter().all(|v|v.is_finite()) || q[3]<=1e-12 {return Err("Source hull crosses the camera near plane");}
        self.min_w=self.min_w.min(q[3]);self.max_w=self.max_w.max(q[3]);
        for axis in 0..2 {self.max_xy[axis]=self.max_xy[axis].max(q[axis].abs());}Ok(())
    }
    pub fn curve(&mut self,curve:&NurbsCurve)->Result<(), &'static str> {
        if !curve_supported(curve) {return Err("Curve controls exceed the refinement budget");}
        for i in 0..curve.m_cv_count {
            let p=curve.get_cv_4d(i).and_then(control).ok_or("Curve weights cannot bound the display")?;self.point(p)?;
        }Ok(())
    }
    pub fn surface(&mut self,surface:&NurbsSurface)->Result<(), &'static str> {
        if !surface_supported(surface) {return Err("Surface controls exceed the refinement budget");}
        for i in 0..surface.m_cv_count[0] {for j in 0..surface.m_cv_count[1] {
            let p=surface.get_cv_4d(i,j).and_then(control).ok_or("Surface weights cannot bound the display")?;self.point(p)?;
        }}Ok(())
    }
    pub fn quality(&self,previous:Quality)->Result<Quality, &'static str> {
        let mut norm=0.;
        for axis in 0..2 {for j in 0..3 {
            let derivative=(self.matrix[j*4+axis].abs()*self.max_w+self.max_xy[axis]*self.matrix[j*4+3].abs())/(self.min_w*self.min_w);
            norm+=(self.size[axis]*derivative).powi(2);
        }}
        if !norm.is_finite()||norm<=0. {return Err("Camera has no finite display scale");}
        Ok(Quality::for_error(0.35/norm.sqrt(),previous))
    }
}

#[cfg(test)]
mod native_projection_tests {
    use super::*;
    #[test]
    fn explicit_projection_matches_camera_and_physical_density() {
        let camera=crate::camera::Camera::default();let model=Xform::identity();
        let mut a=Projection::new(&camera,&model,[640,480]);
        let mut b=Projection::from_matrix(&camera.view_projection(),&model,[640,480]);
        let mut dense=Projection::from_matrix(&camera.view_projection(),&model,[1280,960]);
        for point in [[0.,0.,0.],[0.1,0.1,0.1]] {a.point(point).unwrap();b.point(point).unwrap();dense.point(point).unwrap();}
        let ordinary=a.quality(Quality::default()).unwrap();
        assert_eq!(ordinary,b.quality(Quality::default()).unwrap());
        assert!((dense.quality(Quality::default()).unwrap().0*2.-ordinary.0).abs()<=ordinary.0*1e-12);
    }
}
