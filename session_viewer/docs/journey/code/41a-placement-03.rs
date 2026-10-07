use crate::normals::matrix;
use session_rust::Xform;

fn vector(matrix: &[f32; 16], v: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|row| (0..3).map(|col| matrix[col * 4 + row] as f64 * v[col]).sum())
}

#[test]
fn normal_cofactors_keep_transformed_tangents_perpendicular_and_ignore_translation() {
    let mut model = Xform::identity(); model.m[0] = 2.0; model.m[4] = 1.0; model.m[5] = 3.0; model.m[10] = 4.0;
    let normal = vector(&matrix(&model).unwrap(), [1.0, 1.0, 1.0]);
    for tangent in [[1.0, -3.0, 0.0], [2.0, 0.0, -4.0]] {
        let dot: f64 = (0..3).map(|axis| normal[axis] * tangent[axis]).sum(); assert!(dot.abs() < 1e-6);
    }
    let before = matrix(&model).unwrap(); model.m[12] = 123.456; model.m[13] = -987.654; assert_eq!(matrix(&model).unwrap(), before);
}

#[test]
fn reflections_flattening_and_extreme_scale_keep_finite_oriented_normals() {
    let mut reflected = Xform::identity(); reflected.m[0] = -2.0;
    assert!(vector(&matrix(&reflected).unwrap(), [0.0, 0.0, 1.0])[2] < 0.0);
    let mut flat = Xform::identity(); flat.m[10] = 0.0;
    assert_eq!(vector(&matrix(&flat).unwrap(), [0.0, 0.0, 1.0]), [0.0, 0.0, 1.0]);
    flat.m[5] = 0.0; assert_eq!(vector(&matrix(&flat).unwrap(), [1.0; 3]), [0.0; 3]);
    let mut huge = Xform::identity(); for index in [0, 5, 10] { huge.m[index] = f32::MAX as f64; }
    let result = matrix(&huge).unwrap(); assert!(result.iter().all(|v| v.is_finite())); assert_eq!([result[0], result[5], result[10]], [1.0; 3]);
    huge.m[0] = f64::NAN; assert!(matrix(&huge).is_err());
    let mut perspective = Xform::identity(); perspective.m[3] = 0.1; assert!(matrix(&perspective).is_err());
}
