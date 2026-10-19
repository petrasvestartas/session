struct StrokeQuad {
    position:vec4f, axis:vec4f, pixel:vec2f, depths:vec2f, radius:f32, flat_ends:u32,
}
fn stroke_screen(p:vec4f)->vec2f {return (p.xy/p.w*vec2f(0.5,-0.5)+0.5)*view.size;}
fn stroke_quad(start:vec4f,end:vec4f,width:f32,index:u32)->StrokeQuad {
    var a=start;var b=end;
    var flat=u32(near_distance(a)<0.)|(u32(near_distance(b)<0.)<<1u);
    let hidden=StrokeQuad(vec4f(2.,2.,2.,1.),vec4f(0.),vec2f(0.),vec2f(0.),0.,0u);
    if(flat==3u||all(a==b)){return hidden;}
    a=near_endpoint(a,b);b=near_endpoint(b,a);
    if(a.w<=0.||b.w<=0.){return hidden;}
    let delta=(b.xy/b.w-a.xy/a.w)*view.size;
    let length2=dot(delta,delta);var direction=vec2f(1.,0.);
    if(length2>0.){direction=delta*inverseSqrt(length2);}else{
        a=select(a,b,b.z/b.w>a.z/a.w);b=a;flat=0u;
    }
    let corners=array<vec2f,6>(vec2f(0.,-1.),vec2f(1.,-1.),vec2f(0.,1.),vec2f(0.,1.),vec2f(1.,-1.),vec2f(1.,1.));
    let q=corners[index];let radius=width*view.density*0.5;let extent=radius+0.70711;
    var p=select(a,b,q.x>0.5);
    let offset=(vec2f(-direction.y,direction.x)*q.y+direction*(2.*q.x-1.))*extent;
    p=vec4f(p.xy+offset*2./view.size*p.w,p.zw);
    return StrokeQuad(p,vec4f(stroke_screen(a),stroke_screen(b)),stroke_screen(p),vec2f(a.z/a.w,b.z/b.w),radius,flat);
}
