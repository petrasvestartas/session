fn colour(rgba:vec4<f32>,radius:f32,delta:vec2<f32>)->vec4<f32> {
    let coverage=clamp(radius+0.5-length(delta),0.0,1.0);
    return vec4<f32>(rgba.rgb,rgba.a*coverage);
}
