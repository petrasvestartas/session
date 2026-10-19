    let centre=(position.xy/position.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;
    let offset = corners[index] * (radius + 0.70711);