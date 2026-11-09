// What the vertex shader hands the fragment shader.
struct Vertex {
    @builtin(position) position: vec4<f32>, // clip position
    @location(0) uv: vec2<f32>, // texture coordinate
    @location(1) color: vec4<f32>, // ink color
    @location(2) @interpolate(flat) clip: vec4<f32>, // screen box the label is cut to
    @location(3) @interpolate(flat) object: u32, // object row + 1, or 0
    @location(4) @interpolate(flat) selected: f32, // 1 when selected
    @location(5) @interpolate(flat) plate: f32, // 1 with a black plate behind the glyphs
    @location(7) @interpolate(flat) halo_width: f32, // black glyph halo radius, raster px
}

@group(0) @binding(0) var coverage_texture: texture_2d<f32>; // rasterized glyphs
@group(0) @binding(1) var coverage_sampler: sampler; // linear sampler

// Canvas clip space to the pick window; id pass only.
@group(1) @binding(0) var<uniform> pick: mat4x4<f32>;

// Pack the vertex attributes.
fn plane_vertex(position: vec4<f32>, uv: vec2<f32>, color: vec4<f32>, clip: vec4<f32>,
                object: u32, selected: f32, plate: f32, halo_width: f32) -> Vertex {
    var out: Vertex;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    out.object = object; out.selected = selected; out.plate = plate; out.halo_width = halo_width;
    return out;
}

@vertex
// Vertices arrive already in clip space.
fn vs_main(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
           @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
           @location(4) object: u32, @location(5) selected: f32, @location(6) plate: f32,
           @location(7) halo_width: f32) -> Vertex {
    return plane_vertex(position, uv, color, clip, object, selected, plate, halo_width);
}

@vertex
// The same, mapped into the pick window.
fn vs_id(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
         @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
         @location(4) object: u32, @location(5) selected: f32, @location(6) plate: f32,
         @location(7) halo_width: f32) -> Vertex {
    return plane_vertex(pick * position, uv, color, clip, object, selected, plate, halo_width);
}

// Signed distance to the rounded plate edge, in texture pixels.
fn plate_distance(uv: vec2<f32>) -> f32 {
    let half_size = vec2<f32>(textureDimensions(coverage_texture)) * 0.5;
    let radius = min(half_size.x, half_size.y);
    let q = abs((uv * 2.0 - 1.0) * half_size) - half_size + radius;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

// Dilate only the glyph coverage, never the label's bounding rectangle.
fn black_halo(uv: vec2<f32>, width: f32) -> f32 {
    var halo = textureSampleLevel(coverage_texture, coverage_sampler, uv, 0.0).r;
    if (width > 0.0) {
        let directions = array<vec2<f32>, 8>(
            vec2<f32>(1.0, 0.0), vec2<f32>(-1.0, 0.0),
            vec2<f32>(0.0, 1.0), vec2<f32>(0.0, -1.0),
            vec2<f32>(0.70710678, 0.70710678), vec2<f32>(-0.70710678, 0.70710678),
            vec2<f32>(0.70710678, -0.70710678), vec2<f32>(-0.70710678, -0.70710678),
        );
        let texel = width / vec2<f32>(textureDimensions(coverage_texture));
        for (var index = 0u; index < 8u; index += 1u) {
            let sample_uv = uv + directions[index] * texel;
            halo = max(halo, textureSampleLevel(coverage_texture, coverage_sampler, sample_uv, 0.0).r);
        }
    }
    return halo;
}

// Glyph color over a black plate, yellow when selected; the glyphs alone without a plate.
@fragment
fn fs_main(in: Vertex) -> @location(0) vec4<f32> {
    // outside the clip box
    if (in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) {
        discard;
    }

    let mask = textureSample(coverage_texture, coverage_sampler, in.uv).rg;
    let coverage = mask.r;
    var halo = mask.g;
    if (in.plate > 1.5) {
        halo = black_halo(in.uv, in.halo_width);
    }
    let distance = plate_distance(in.uv);
    let plate_coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    let background = vec3<f32>(in.selected, in.selected, 0.0);
    let plated = vec4<f32>(mix(background, in.color.rgb, coverage), in.color.a * plate_coverage);
    // no plate: the selected halo color first, then ink over it
    let halo_color = select(vec3<f32>(1.0), vec3<f32>(0.0), in.plate > 1.5);
    let bare = vec4<f32>(mix(halo_color, in.color.rgb, coverage), in.color.a * max(coverage, halo * (1.0 - coverage)));
    return select(bare, plated, in.plate >= 0.5 && in.plate < 1.5);
}

@fragment
// Object id inside the plate.
fn fs_id(in: Vertex) -> @location(0) vec2<u32> {
    if (in.object == 0u || in.position.x < in.clip.x || in.position.y < in.clip.y || in.position.x >= in.clip.z || in.position.y >= in.clip.w) {
        discard;
    }

    if (in.plate > 1.5) {
        if (black_halo(in.uv, in.halo_width) < 0.5) {
            discard;
        }
    } else if (plate_distance(in.uv) > 0.0) {
        discard;
    }

    return vec2<u32>(in.object, 0u);
}
