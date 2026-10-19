#[derive(Clone, PartialEq)]
pub struct Screen { pub matrix: [f64; 16], pub size: [f64; 2] }

impl Screen {
    pub fn point(&self, point: [f64; 3]) -> Option<[f64; 2]> {
        if self.size.iter().any(|v| !v.is_finite() || *v <= 0.) { return None; }
        let clip: [f64; 4] = std::array::from_fn(|row| point.iter().enumerate().fold(self.matrix[12+row], |sum,(i,v)| sum+self.matrix[i*4+row]*v));
        if clip.iter().any(|v| !v.is_finite()) || clip[3] <= 1e-9 || clip[2] < 0. || clip[2] > clip[3] { return None; }
        let pixel=[(clip[0]/clip[3]+1.)*self.size[0]/2., (1.-clip[1]/clip[3])*self.size[1]/2.];
        pixel.iter().all(|v| v.is_finite()).then_some(pixel)
    }
}

// Round the plane's two coordinates around its origin; retain the signed height along its normal.
pub fn grid(point: [f64; 3], origin: [f64; 3], axes: [[f64; 3]; 2], step: f64) -> Option<[f64; 3]> {
    crate::tool_coords::checked(point).ok()?;
    crate::tool_coords::checked(origin).ok()?;
    let [x,y] = axes;
    if !step.is_finite() || step <= 0. || x.iter().chain(y.iter()).any(|v| !v.is_finite())
        || (dot(x,x)-1.).abs()>1e-9 || (dot(y,y)-1.).abs()>1e-9 || dot(x,y).abs()>1e-9 { return None; }
    let z=[x[1]*y[2]-x[2]*y[1], x[2]*y[0]-x[0]*y[2], x[0]*y[1]-x[1]*y[0]];
    let d=sub(point,origin); let mut local=[dot(d,x),dot(d,y),dot(d,z)];
    for value in &mut local[..2] { let units=*value/step; if units.is_finite() { *value=units.round()*step; } }
    crate::tool_coords::checked(std::array::from_fn(|i| origin[i]+local[0]*x[i]+local[1]*y[i]+local[2]*z[i])).ok()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 { a.iter().zip(b).map(|(a,b)| a*b).sum() }
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { std::array::from_fn(|i| a[i]-b[i]) }
fn along(a: [f64; 3], u: [f64; 3], t: f64) -> [f64; 3] { std::array::from_fn(|i| a[i]+u[i]*t) }

// Minimize the distance between a source segment and the current ray, then clamp the result to the segment.
pub fn near(a: [f64; 3], b: [f64; 3], ray: &crate::camera::Ray) -> [f64; 3] {
    let u = sub(b,a); let w = sub(a,std::array::from_fn(|i| ray.origin[i])); let d = std::array::from_fn(|i| ray.direction[i]);
    let (uu,ud,dd) = (dot(u,u),dot(u,d),dot(d,d)); let denominator = uu*dd-ud*ud;
    let t = if denominator > 1e-12*uu*dd { ((ud*dot(d,w)-dd*dot(u,w))/denominator).clamp(0.,1.) } else { 0. };
    along(a,u,t)
}

pub fn perpendicular(a: [f64; 3], b: [f64; 3], from: [f64; 3]) -> Option<[f64; 3]> {
    let u = sub(b,a); let length = dot(u,u); if length <= 1e-24 { return None; }
    let t = dot(sub(from,a),u)/length;
    (t > 1e-9 && t < 1.-1e-9).then(|| along(a,u,t))
}
