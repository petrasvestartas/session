fn colour(input:Output)->vec4f {
    let ends=input.finite_axis_value;let delta=ends.zw-ends.xy;
    let span=length(delta);let direction=delta/max(span,1e-6);
    let along=dot(input.finite_pixel-ends.xy,direction);
    let distance=length(input.finite_pixel-ends.xy-direction*clamp(along,0.,span));
    var coverage=clamp(input.half_width+0.5-distance,0.,1.);
    if((input.flat_ends&1u)!=0u){coverage*=clamp(along+0.5,0.,1.);}
    if((input.flat_ends&2u)!=0u){coverage*=clamp(span-along+0.5,0.,1.);}
    return vec4f(input.colour.rgb,input.colour.a*coverage);
}
