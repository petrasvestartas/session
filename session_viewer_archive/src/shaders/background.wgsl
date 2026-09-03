// Arctic horizon-gradient background: drawn first in the geometry pass (no depth
// write, Always compare) so geometry covers it. Bright band at the horizon,
// slightly cooler/darker below and above — gives the white scene a volumetric feel.

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    let xy = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u));
    var o: VsOut;
    o.pos = vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
    o.uv  = xy; // 0..2 across the triangle; visible region is 0..1, y up
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let t = in.uv.y; // 0 = bottom, 1 = top
    let below   = vec3<f32>(0.82, 0.84, 0.87);
    let horizon = vec3<f32>(0.99, 0.99, 1.00);
    let above   = vec3<f32>(0.90, 0.92, 0.95);
    var c: vec3<f32>;
    if (t < 0.45) {
        c = mix(below, horizon, smoothstep(0.0, 0.45, t));
    } else {
        c = mix(horizon, above, smoothstep(0.45, 1.0, t));
    }
    return vec4<f32>(c, 1.0);
}
