 axis, sample);
        }
        gradient = finite_projected[triangle].gradient.xy;
    }
    if (any(abs(gradient) >= vec2<f32>(SLOPE_INVALID))) {
        return ink_axis_visible(pixel, axis, sample);
    }
    let delta = axis.at - pixel;
    let predicted = z + dot(gradient, delta);
    let tolerance = ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)) + ink_sag_slack(axis.sag, gradient);
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}

fn finite_ink_axis(pixel:vec2<f32>,ends:vec4<f32>,depths:vec2<f32>)->InkAxis{
 let delta=ends.zw-ends.xy;let len=length(delta);let along=delta/max(len,1e-6);
 let distance=clamp(dot(pixel-ends.xy,along),0.,len);let at=ends.xy+along*distance;
 let depth=select(depths.x,(depths.x*(len-distance)+depths.y*distance)/max(len,1e-6),len>1e-6);
 return InkAxis(at,depth,along,(depths.y-depths.x)/max(len,1e-3),vec2<f32>(0.));
}
fn ink_exact_floor(axis:InkAxis,triangle:ProjectedTriangle)->f32{
 return axis.depth+abs(axis.depth)*DEPTH_REL_TOL+ink_sag_slack(axis.sag,triangle.gradient.xy);
}
fn finite_triangle_hides(primitive:u32,axis:InkAxis)->bool{
 if(primitive==0u||primitive>arrayLength(&finite_projected)){return false;}
 let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,axis.at);
 return hit.y>0.5&&hit.x>ink_exact_floor(axis,triangle)&&!(clip_active()&&clip_cut_ndc(0u,clip_ndc(axis.at,view.size,1.-hit.x)));
}
fn finite_source_hidden(pixel:vec2<f32>,axis:InkAxis)->bool{
 let fringe=ink_primitive(pixel,0u);if(finite_triangle_hides(fringe,axis)){return true;}
 let base=floor(axis.at-fract(pixel))+fract(pixel);
 for(var i=0u;i<4u;i++){
  let primitive=ink_primitive(base+vec2<f32>(f32(i&1u),f32(i>>1u)),0u);
  if(primitive!=fringe&&finite_triangle_hides(primitive,axis)){return true;}
 }
 return false;
}
