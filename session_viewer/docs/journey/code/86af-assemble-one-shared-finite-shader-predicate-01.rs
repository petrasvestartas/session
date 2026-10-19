pub fn source(stride:u64,pick:bool)->String{source_style(stride,pick,false)}
pub fn source_style(stride:u64,pick:bool,grid:bool)->String{
 let(base,ids)=match stride{44=>(include_str!("finite_stroke.wgsl"),include_str!("finite_stroke_pick.wgsl")),72=>(include_str!("finite_chain.wgsl"),include_str!("finite_chain_pick.wgsl")),32=>(include_str!("finite_marker.wgsl"),include_str!("finite_marker_pick.wgsl")),_=>panic!("Unsupported finite ink layout")};
 let helper=include_str!("projected_triangle.wgsl").split("// Pixels per tile side for this canvas.").next().unwrap();let base=base.replace("#include \"projected_triangle.wgsl\"",helper).replace("#include \"finite_visibility.wgsl\"",include_str!("finite_visibility.wgsl"));
 let base=if grid{base.replace("const FINITE_GRID:bool=false;","const FINITE_GRID:bool=true;")}else{base};
 if pick{format!("{}\n{}\n{ids}",base.replace("@group(1)","@group(2)").replace("return vec2<i32>(pixel);","return vec2<i32>(pixel)-vec2<i32>(i32(crop._a),i32(crop._b));"),include_str!("pick_crop.wgsl"))}else{base}
}
