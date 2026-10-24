pub fn pixel_rays(inverse: &[f32; 16], size: (u32, u32)) -> [f32; 24] {
    // world point at a pixel and depth, as the shader's `world`
    let world = |x: f64, y: f64, z: f64| -> [f64; 3] {
        let ndc = [
            x / f64::from(size.0) * 2.0 - 1.0,
            1.0 - y / f64::from(size.1) * 2.0,
            z,
            1.0,
        ];
        let p: [f64; 4] = std::array::from_fn(|row| {
            (0..4)
                .map(|col| f64::from(inverse[col * 4 + row]) * ndc[col])
                .sum()
        });
        [p[0] / p[3], p[1] / p[3], p[2] / p[3]]
    };
    let near = [
        world(0.0, 0.0, 1.0),
        world(1.0, 0.0, 1.0),
        world(0.0, 1.0, 1.0),
    ];
    let half = [
        world(0.0, 0.0, 0.5),
        world(1.0, 0.0, 0.5),
        world(0.0, 1.0, 0.5),
    ];
    let ray: [[f64; 3]; 3] =
        std::array::from_fn(|i| std::array::from_fn(|k| half[i][k] - near[i][k]));
    let mut out = [0.0; 24];
    for (i, base) in [near, ray].iter().enumerate() {
        for k in 0..3 {
            out[i * 12 + k] = base[0][k] as f32;
            out[i * 12 + 4 + k] = (base[1][k] - base[0][k]) as f32;
            out[i * 12 + 8 + k] = (base[2][k] - base[0][k]) as f32;
        }
    }
    out
}


pub fn uniform(matrix:[f32;16],full:[u32;2],size:[u32;2],ground:[f32;2])->Option<[f32;80]>{
 uniform_with_ground(matrix,full,size,[size[0].div_ceil(2),size[1].div_ceil(2)],ground)
}
pub fn uniform_with_ground(matrix:[f32;16],full:[u32;2],size:[u32;2],ground_size:[u32;2],ground:[f32;2])->Option<[f32;80]>{
 let mvp=session_rust::Xform::from_matrix(matrix.map(f64::from));let inverse=crate::clipping_uniform::inverse(&mvp)?.m.map(|v|v as f32);if inverse.iter().any(|v|!v.is_finite()){return None;}
 let mut out=[0.;80];out[..16].copy_from_slice(&inverse);out[16..32].copy_from_slice(&mvp.m.map(|v|v as f32));out[32..40].copy_from_slice(&[ground[0],ground[1],full[0]as f32,full[1]as f32,ground_size[0]as f32,size[0]as f32,size[1]as f32,ground_size[1]as f32]);out[40..64].copy_from_slice(&pixel_rays(&inverse,(full[0],full[1])));Some(out)
}
