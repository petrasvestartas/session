lor,q,0).rgb;
    if(cloud.w>0.){
        let extent=vec2i(textureDimensions(sdepth));
        let me=log_depth(d);
        var sum=0.;
        let taps=array(vec2i(-1,0),vec2i(1,0),vec2i(0,-1),vec2i(0,1));
        for(var k=0;k<4;k++){
            let tap=q+taps[k];
            if(any(tap<vec2i(0))||any(tap>=extent)){continue;}
            let nd=textureLoad(sdepth,tap,0);
            if(nd==0.){continue;}
            sum+=max(0.,me-log_depth(nd));
        }
        rgb*=max(exp(-sum*75.*cloud.w),0.25);
    }

    return Pixel(vec4f(rgb,1.),vec2u(0u),d);
}
s