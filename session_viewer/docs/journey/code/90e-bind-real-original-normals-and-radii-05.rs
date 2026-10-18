}



fn source_at(pixel:vec2<f32>)->vec2<i32>{return clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);}
fn source_radius(pixel:vec2<f32>)->f32{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 let cap=textureLoad(source_sections,clamp(at,vec2<i32>(0),vec2<i32>(textureDimensions(source_sections))-1),0).y;
 return select(radius,ambient.params.y,radius==0.0&&cap>=0.5);
}
fn normal_of(pixel:vec2<f32>,p:vec3<f32>,sample:i32)->vec3<f32>{
 let at=source_at(pixel);let radius=textureLoad(source_radii,at,0).x;
 if(radius>0.0){let n=decode_normal(textureLoad(source_normals,at,0).xy*2.0-1.0);return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let cap=textureLoad(source_sections,clamp(at,vec2<i32>(0),vec2<i32>(textureDimensions(source_sections))-1),0).y;
 if(cap>=0.5){let plane=(u32(cap)-1u)&7u;let n=-clipping.planes[plane].xyz;return select(-n,n,dot(n,-ray_direction(pixel))>0.0);}
 let a=surface(pixel+vec2<f32>(1.,0.),sample);let b=surface(pixel-vec2<f32>(1.,0.),sample);let c=surface(pixel+vec2<f32>(0.,1.),sample);let d=surface(pixel-vec2<f32>(0.,1.),sample);
 let dx=select(p-b.xyz,a.xyz-p,a.w>0.&& (b.w==0.||distance(a.xyz,p)<distance(b.xyz,p)));let dy=select(p-d.xyz,c.xyz-p,c.w>0.&&(d.w==0.||distance(c.xyz,p)<distance(d.xyz,p)));let n=cross(dx,dy);let normal=n/max(length(n),1e-12);return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.);
}
