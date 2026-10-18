pub fn screen(matrix:&[f64;16],point:[f64;3],size:[f64;2])->Option<[f64;2]> {
    crate::snap_math::Screen {matrix:*matrix,size,origin:[0.;3]}.unclipped(point)
}