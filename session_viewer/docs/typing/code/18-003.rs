
#[cfg(test)]
mod tiles_tests {
    use crate::engine::gpu::frame::LineUniform;
    use crate::engine::gpu::lane_shaders;

    /// Every shader compiles and its struct offsets match Rust.
    #[test]
    fn shader_validation_and_layouts() {
        use crate::engine::gpu::glyphs::GlyphPoint;
        use crate::engine::gpu::segments::{CylinderSegment, StrokeSegment};
        use std::mem::{offset_of, size_of};

        for (name, source) in lane_shaders() {
            use crate::engine::pipelines::{ink_source, scene_source, shared};
            // shaders that use the camera get the shared scene code
            let scene = source.contains("mvp") || source.contains("line.");
            let source = if source.contains("-> InkColor") {
                ink_source(source)
            } else if scene {
                scene_source(source)
            } else {
                source.to_string()
            };

            let source = shared(&source);
            let module = naga::front::wgsl::parse_str(&source)
                .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                // allows pack2x16float, as wgpu does
                naga::valid::Capabilities::default()
                    | naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32,
            )
            .validate(&module)
            .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));

            for (_, ty) in module.types.iter() {
                let Some(structure) = ty.name.as_deref() else {
                    continue;
                };
                let (offsets, size) = match structure {
                    "StrokeSegment" => (
                        vec![
                            0,
                            4,
                            8,
                            offset_of!(CylinderSegment, radius),
                            16,
                            20,
                            24,
                            offset_of!(CylinderSegment, instance_id),
                            offset_of!(CylinderSegment, color),
                            offset_of!(CylinderSegment, facing),
                            offset_of!(StrokeSegment, previous),
                            offset_of!(StrokeSegment, next),
                        ],
                        size_of::<StrokeSegment>(),
                    ),
                    "ProjectedTriangle" => (
                        vec![0, 16, 32, 48, 64, 80],
                        super::super::triangle_tiles::PROJECTED_BYTES as usize,
                    ),
                    "GlyphPoint" => (
                        vec![
                            offset_of!(GlyphPoint, center),
                            offset_of!(GlyphPoint, radius),
                            offset_of!(GlyphPoint, color),
                            offset_of!(GlyphPoint, instance_id),
                            offset_of!(GlyphPoint, facing),
                            offset_of!(GlyphPoint, facing_ext),
                        ],
                        size_of::<GlyphPoint>(),
                    ),
                    "LineUniform" => (
                        vec![
                            0,
                            4,
                            8,
                            12,
                            16,
                            20,
                            24,
                            28,
                            32,
                            44,
                            offset_of!(LineUniform, lit),
                            offset_of!(LineUniform, backface),
                            offset_of!(LineUniform, origin),
                            offset_of!(LineUniform, frame),
                            offset_of!(LineUniform, opacity),
                        ],
                        size_of::<LineUniform>(),
                    ),
                    _ => continue,
                };
                let naga::TypeInner::Struct { members, span } = &ty.inner else {
                    panic!("{name}: {structure} is not a struct")
                };
                assert_eq!(*span as usize, size, "{name}: {structure} stride");
                assert_eq!(members.len(), offsets.len(), "{name}: {structure} members");

                for (member, expected) in members.iter().zip(offsets) {
                    assert_eq!(
                        member.offset as usize, expected,
                        "{name}: {structure}.{:?}",
                        member.name
                    );
                }
            }
        }
    }
}
