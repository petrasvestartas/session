// Arctic/outline composite: resolved scene color x blurred AO -> swapchain, plus
// an object boundary drawn from the ID mask (surface geometry only — see
// mask.wgsl). The horizon gradient is drawn as the geometry-pass background (not
// here), because the edit and gumball overlay passes clear the depth buffer
// before this pass runs.

struct Arctic {
    proj:      mat4x4<f32>,
    inv_proj:  mat4x4<f32>,
    kernel:    array<vec4<f32>, 32>,
    radius_ws: f32,
    bias_ws:   f32,
    intensity: f32,
    flags:     u32,   // bit0 = gradient, bit1 = outline
    ao_mode:   u32,
    outline_px: f32,
    _pad1:     u32,
    _pad2:     u32,
}

@group(0) @binding(0) var t_color: texture_2d<f32>;
@group(0) @binding(1) var t_ao: texture_2d<f32>;
@group(0) @binding(2) var t_mask: texture_2d<f32>;
@group(0) @binding(3) var<uniform> u: Arctic;

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
    var rgb = col * ao;

    if ((u.flags & 2u) != 0u) {
        // Union-silhouette boundary: one border around all surface objects
        // together, never between touching parts. The mask holds MSAA-resolved
        // FRACTIONAL coverage, so the band strength follows the true anti-aliased
        // silhouette: each tap contributes coverage × a smoothstep distance
        // falloff, and the result fades out inside the object by (1 - coverage).
        let dims = vec2<i32>(textureDimensions(t_mask));
        let c_center = textureLoad(t_mask, clamp(px, vec2<i32>(0), dims - vec2<i32>(1)), 0).r;
        if (c_center < 0.999) {
            let r = clamp(u.outline_px, 1.0, 8.0);
            let ri = i32(ceil(r)) + 1;
            var a = 0.0;
            for (var dy = -ri; dy <= ri; dy++) {
                for (var dx = -ri; dx <= ri; dx++) {
                    let q = clamp(px + vec2<i32>(dx, dy), vec2<i32>(0), dims - vec2<i32>(1));
                    let c = textureLoad(t_mask, q, 0).r;
                    if (c > 0.0) {
                        let w = 1.0 - smoothstep(r - 0.75, r + 0.75, length(vec2<f32>(f32(dx), f32(dy))));
                        a = max(a, c * w);
                    }
                }
            }
            a = a * (1.0 - c_center);
            rgb = mix(rgb, vec3<f32>(0.10, 0.10, 0.12), 0.9 * a);
        }
    }

    // ±0.5/255 dither breaks up 8-bit swapchain banding on the wide soft gradients.
    let dither = (ign(in.pos.xy) - 0.5) * (1.0 / 255.0);
    return vec4<f32>(rgb + vec3<f32>(dither), 1.0);
}
