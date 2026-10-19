fn geometry(start:vec3f,end:vec3f,colour:vec4f,width:f32,index:u32)->Output {
    let s=stroke_quad(view.transform*vec4f(start,1.),view.transform*vec4f(end,1.),width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,s.axis,s.pixel);
}

