    let l=normalize(v+vec3<f32>(0.,0.,0.35));let h=normalize(l+v);
    let wrap=clamp((dot(n,l)+0.5)/1.5,0.,1.);
    let lit=min(0.40+0.55*wrap+pow(max(dot(n,h),0.),32.)*0.15,1.);
    let sky=mix(0.72,1.,0.5+0.5*n.z);
    let brightness=select(1.,select(lit,sky,view.display.z>1.5),view.display.z>0.5);
    let back=!source_front&&view.display.w>0.5;
    let base=select(input.colour,vec3<f32>(0.80,0.05,0.05),back);
    return vec4<f32>(base*brightness,select(1.,view.display.y,closed));