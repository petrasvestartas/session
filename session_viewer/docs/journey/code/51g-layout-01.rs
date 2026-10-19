use session_rust::Xform;

pub struct Layout { pub uniform: [f32;24], pub extent: [u32;2], pub rect: [f64;4] }

// Measuring the projected pivot keeps a 96-pixel handle the same size while the camera zooms or changes projection.
pub fn world_per_pixel(matrix: &Xform, origin: [f64;3], height: f64) -> Option<f64> {
    if !height.is_finite() || height<=0. || origin.iter().any(|v| !v.is_finite()) { return None; }
    let m=&matrix.m;
    let w=m[3]*origin[0]+m[7]*origin[1]+m[11]*origin[2]+m[15];
    let vertical=(m[1]*m[1]+m[5]*m[5]+m[9]*m[9]).sqrt();
    let value=2.*w/(height*vertical);
    (w>0. && value.is_finite() && value>0.).then_some(value)
}

pub fn layout(matrix: &Xform, origin:[f64;3], scale:f64, size:[u32;2], active:f32) -> Option<Layout> {
    if size.contains(&0) || !scale.is_finite() || scale<=0. || !active.is_finite() { return None; }
    let model=&Xform::translation(origin[0],origin[1],origin[2])*&Xform::scale_xyz(scale,scale,scale);
    let matrix=matrix*&model;
    let rect=bounds(&matrix.m,size)?;
    let extent=std::array::from_fn(|i| ((rect[i+2]*2.).ceil() as u32).div_ceil(64).clamp(1,16)*64);
    let [x,y,width,height]=rect;
    let sx=size[0] as f64/width; let sy=size[1] as f64/height;
    let tx=(size[0] as f64-2.*x-width)/width; let ty=(2.*y+height-size[1] as f64)/height;
    let mut uniform=[0f32;24];
    for column in 0..4 { let at=column*4;
        uniform[at]=(sx*matrix.m[at]+tx*matrix.m[at+3]) as f32;
        uniform[at+1]=(sy*matrix.m[at+1]+ty*matrix.m[at+3]) as f32;
        uniform[at+2]=matrix.m[at+2] as f32; uniform[at+3]=matrix.m[at+3] as f32;
    }
    uniform[16]=(2.*x/size[0] as f64-1.) as f32; uniform[17]=(1.-2.*y/size[1] as f64) as f32;
    uniform[18]=(2.*width/size[0] as f64) as f32; uniform[19]=(-2.*height/size[1] as f64) as f32; uniform[20]=active;
    uniform.iter().all(|v| v.is_finite()).then_some(Layout {uniform,extent,rect})
}

fn bounds(m:&[f64;16], size:[u32;2]) -> Option<[f64;4]> {
    let radius=crate::gizmo::ARM+2.; let mut min=[f64::INFINITY;2]; let mut max=[f64::NEG_INFINITY;2];
    for corner in 0..8 {
        let p=std::array::from_fn::<_,3,_>(|i| if corner&(1<<i)==0 {-radius} else {radius});
        let c=std::array::from_fn::<_,4,_>(|i| m[i]*p[0]+m[4+i]*p[1]+m[8+i]*p[2]+m[12+i]);
        if c.iter().any(|v| !v.is_finite()) || c[3]<=0. { return None; }
        let screen=[(c[0]/c[3]*0.5+0.5)*size[0] as f64,(0.5-c[1]/c[3]*0.5)*size[1] as f64];
        for i in 0..2 { min[i]=min[i].min(screen[i]); max[i]=max[i].max(screen[i]); }
    }
    let x=(min[0]-2.).floor().max(0.); let y=(min[1]-2.).floor().max(0.);
    let width=(max[0]+2.).ceil().min(size[0] as f64)-x; let height=(max[1]+2.).ceil().min(size[1] as f64)-y;
    (width>0. && height>0.).then_some([x,y,width,height])
}
