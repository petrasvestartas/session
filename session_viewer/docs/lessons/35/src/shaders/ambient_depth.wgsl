@group(0) @binding(0) var linear: texture_2d<f32>;
@group(0) @binding(1) var radii: texture_2d<u32>;
@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = array<vec2<f32>, 3>(vec2<f32>(-1.0,-1.0), vec2<f32>(3.0,-1.0), vec2<f32>(-1.0,3.0));
    return vec4<f32>(p[i],0.0,1.0);
}
struct DepthRadius {
    @location(0) depth: f32,
    @location(1) radius: u32,
};
@fragment fn fs_reduce(@builtin(position) pixel: vec4<f32>) -> DepthRadius {
    let dims = vec2<i32>(textureDimensions(linear));
    let at = vec2<i32>(pixel.xy)*2;
    var nearest = 1e30;
    var radius = 0u;
    for (var i=0; i<4; i++) {
        let q = min(at+vec2<i32>(i%2,i/2),dims-1);
        let z = textureLoad(linear,q,0).x;
        if z>0.0 && z<nearest {
            nearest = z;
            radius = textureLoad(radii,q,0).x;
        }
    }
    return DepthRadius(select(nearest,0.0,nearest==1e30),radius);
}

@fragment fn fs_occupancy(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let dims = vec2<i32>(textureDimensions(linear));
    let at = vec2<i32>(pixel.xy);
    for (var i=0; i<25; i++) {
        let q = clamp(at+vec2<i32>(i%5-2,i/5-2),vec2<i32>(0),dims-1);
        if textureLoad(linear,q,0).x>0.0 { return 1.0; }
    }
    return 0.0;
}
