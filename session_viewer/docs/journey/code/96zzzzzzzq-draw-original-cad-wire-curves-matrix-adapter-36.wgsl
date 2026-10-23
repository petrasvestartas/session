    let distance = plate_distance(in.uv);
    let plate_coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    let background = vec3<f32>(in.selected, in.selected, 0.0);
    let plated = vec4<f32>(mix(background, in.color.rgb, coverage), in.color.a * plate_coverage);
    let bare = vec4<f32>(in.color.rgb, in.color.a * coverage);
    return select(bare, plated, in.plate >= 0.5);
}

@fragment
