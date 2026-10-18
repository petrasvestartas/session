fn marker_radius(position:vec4f,size:f32)->f32 {
    if(size>=0.){return size*view.density*0.5;}
    let scale=length(vec3f(view.transform[0].y,view.transform[1].y,view.transform[2].y));
    let spacing=-size*scale*view.size.y*0.5/position.w;
    let radius=view.density*0.5;
    return max(0.5,radius*clamp(spacing/max(6.*radius,1e-6),0.15,1.));
}
