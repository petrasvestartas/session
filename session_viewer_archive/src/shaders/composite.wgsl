// Arctic composite: resolved scene color x blurred AO -> swapchain. The horizon
// gradient is drawn as the geometry-pass background (not here), because the edit
// and gumball overlay passes clear the depth buffer before this pass runs.

@group(0) @binding(0) var t_color: texture_2d<f32>;
@group(0) @binding(1) var t_ao: texture_2d<f32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    let xy = vec2<f32>(f32((vi << 1u) & 2u), f32(vi & 2u));
    var o: VsOut;
    o.pos = vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
    return o;
}

// Interleaved gradient noise for output dithering.
fn ign(p: vec2<f32>) -> f32 {
    return fract(52.9829189 * fract(0.06711056 * p.x + 0.00583715 * p.y));
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let px = vec2<i32>(in.pos.xy);
    let col = textureLoad(t_color, px, 0).rgb;
    let ao = textureLoad(t_ao, px, 0).r;
    // ±0.5/255 dither breaks up 8-bit swapchain banding on the wide soft gradients.
    let dither = (ign(in.pos.xy) - 0.5) * (1.0 / 255.0);
    return vec4<f32>(col * ao + vec3<f32>(dither), 1.0);
}
