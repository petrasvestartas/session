 let size=textureDimensions(primitives);
 if(any(vec2<u32>(p.xy)>=size)){discard;}
 let halves=textureLoad(primitives,vec2<i32>(p.xy),0).xy;
 let primitive=physical_address(halves);
 if(!clip_is_cap(primitive)||(radius.y>0.5&&(primitive&1u)==0u)){discard;}
 let plane=clip_cap_plane(primitive);
 if(plane>=clipping.count){discard;}
 let z=clip_plane_depth(plane,clip_ndc(p.xy,vec2<f32>(size),0.).xy);
 return Coverage(1.,z);
}
