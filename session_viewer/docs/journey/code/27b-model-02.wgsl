    let world = model * vec4<f32>(position, 1.0);
    output.position = transform * world;
    output.colour = colour;
    output.world = world.xyz;
