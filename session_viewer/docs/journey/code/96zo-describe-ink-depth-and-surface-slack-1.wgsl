const DEPTH_REL_TOL:f32=1.9073486e-6;
const SLOPE_PX:f32=0.00390625;
const KINK:f32=0.03125;
const SLOPE_INVALID:f32=0.99951171875;
struct InkAxis{at:vec2<f32>,depth:f32,along:vec2<f32>,slope:f32,sag:vec2<f32>}
fn ink_depth(pixel:vec2<f32>,sample:u32)->f32{
    let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));
    if(any(at<vec2<i32>(0))||any(at>=size)){return 0.;}
    return 1.-textureLoad(finite_depth,at,0);
}
fn ink_sag_slack(sag: vec2<f32>, gradient: vec2<f32>) -> f32 {
    return sag.x + sag.y * (abs(gradient.x) + abs(gradient.y));
}
fn ink_tolerance(depth: f32, slope: f32, lever: f32) -> f32 {
    return abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (1.0 + lever);
}
fn ink_carry_visible(predicted: f32, z: f32, depth: f32, tolerance: f32) -> bool {
    if (z > depth + abs(depth) * DEPTH_REL_TOL) {
        return abs(predicted - depth) <= tolerance;
    }
    return predicted <= depth + tolerance;
}
