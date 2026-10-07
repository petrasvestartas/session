    Ok(vector.map(|v| v / length))
}

pub fn matrix(model: &session_rust::Xform) -> Result<[f32; 16], &'static str> {
    if !crate::placement::valid(&model.m) { return Err("Normal placement needs a finite affine matrix"); }
    let column = |index: usize| [model.m[index * 4], model.m[index * 4 + 1], model.m[index * 4 + 2]];
    let cross = |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let [a, b, c] = [column(0), column(1), column(2)];
    let columns = [cross(b, c), cross(c, a), cross(a, b)];
    let scale = columns.iter().flatten().map(|v| v.abs()).fold(0.0, f64::max);
    let mut matrix = [0.0; 16];
    if scale > 0.0 {
        for col in 0..3 { for row in 0..3 { matrix[col * 4 + row] = (columns[col][row] / scale) as f32; } }
    }
    matrix[15] = 1.0; Ok(matrix)
}