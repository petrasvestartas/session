pub fn expand(shader: &str) -> String {
    let (geometry, viewport) = include_str!("projected_triangle.wgsl")
        .split_once("// Pixels per tile side for this canvas.").unwrap();
    shader.replace("#include \"near_clip.wgsl\"", include_str!("near_clip.wgsl"))
        .replace("#include \"projected_triangle.wgsl\"", geometry)
        .replace("#include \"projected_triangle.wgsl:viewport-span\"", viewport)
        .replace("#include \"stroke_coverage.wgsl\"", include_str!("stroke_coverage.wgsl"))
        .replace("#include \"chain_coverage.wgsl\"", include_str!("chain_coverage.wgsl"))
        .replace("#include \"marker_coverage.wgsl\"", include_str!("marker_coverage.wgsl"))
}

pub fn tiles() -> String { expand(include_str!("finite_triangle_tiles.wgsl")) }

pub fn scan() -> String { expand(include_str!("finite_scan_triangle_tiles.wgsl")) }

pub fn source() -> String { format!("{}\n{}\n{}",crate::object_record::shader(),include_str!("packed_vertex.wgsl"),expand(include_str!("finite_project.wgsl"))) }

pub fn records()->String {crate::object_rows::source(&source())}

pub fn lookup()->String {
 let source=records().replace("let index = local + live_count.y;","let index = local;").replace("let polygon = project_physical_triangle(local+1u, live_count.z, object_at(live_count.z).range.x);","let source=placed_triangle(index+1u,live_count.y);if(source.x==NO_TRIANGLE||source.y==SLOT_OWN_ROW){return;}let polygon=project_physical_triangle(source.x+1u,source.y,0u);");
 format!("@group(2) @binding(5) var slot_table:texture_2d<u32>;\n{}\n{}",include_str!("primitive_slot_lookup.wgsl"),source)
}

#[cfg(test)]
mod tests {
    #[test]
    fn assembled_projection_shader_has_a_valid_compute_interface() {
        use wgpu::naga;
        for source in [super::source(),super::records(),super::lookup()] {
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
        naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
            .validate(&module).unwrap();
        assert!(module.entry_points.iter().any(|entry| entry.name == "cs_main"
            && entry.stage == naga::ShaderStage::Compute));
        }
    }
}


#[cfg(test)]
mod near_tests {
 #[test]
 fn all_original_ink_sources_expand_one_valid_near_clip_owner(){
  let mut sources=vec![crate::stroke::shader(),crate::chain::shader(),super::expand(include_str!("guide.wgsl")),crate::clipping_gpu::source(&super::expand(include_str!("sheet.wgsl")),2,0)];
  for stride in [44,72,32]{for pick in [false,true]{for grid in [false,true]{sources.push(crate::finite_sources::source_style(stride,pick,grid));}}}
  for source in sources {
   let module=wgpu::naga::front::wgsl::parse_str(&source).unwrap_or_else(|error|panic!("{}",error.emit_to_string(&source)));
   wgpu::naga::valid::Validator::new(wgpu::naga::valid::ValidationFlags::all(),wgpu::naga::valid::Capabilities::all()).validate(&module).unwrap();
   assert!(!source.contains("#include"));
  }
 }
}
