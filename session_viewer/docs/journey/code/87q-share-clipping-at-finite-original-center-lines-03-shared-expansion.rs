 let base=crate::finite_project_source::expand(base).replace("#include \"finite_visibility.wgsl\"",include_str!("finite_visibility.wgsl"));
 let base=if grid{base.replace("const FINITE_GRID:bool=false;","const FINITE_GRID:bool=true;")}else{base};
 let source=if pick{format!("{}\n{}\n{ids}",base.replace("@group(1)","@group(2)").replace("return vec2<i32>(pixel);","return vec2<i32>(pixel)-vec2<i32>(i32(crop._a),i32(crop._b));"),include_str!("pick_crop.wgsl"))}else{base};crate::clipping_gpu::source(&source,if pick{3}else{2},0)
}
