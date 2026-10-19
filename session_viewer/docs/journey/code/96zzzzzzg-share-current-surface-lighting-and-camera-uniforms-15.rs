    let m=view.transform;
    let v=select(normalize(view.eye.xyz-input.world),normalize(vec3<f32>(m[0].z,m[1].z,m[2].z)),view.eye.w>0.);
    let n=select(normal,-normal,dot(normal,v)<0.);
    let brightness=select(1.,mix(0.72,1.,0.5+0.5*n.z),view.display.z>1.5);
    return vec4<f32>(input.colour*brightness,select(1.,view.display.y,closed));