use super::Instance;

/// Field names of a WGSL struct, in order.
#[cfg(test)]
pub(crate) fn wgsl_fields(src: &str, struct_name: &str) -> Vec<String> {
    let at = src
        .find(&format!("struct {struct_name}"))
        .expect("struct declared in the shader");
    let rest = &src[at..];
    let open = rest.find('{').expect("struct body opens");
    let close = rest.find('}').expect("struct body closes");

    rest[open + 1..close]
        .lines()
        .map(|l| l.split("//").next().unwrap_or(""))
        .flat_map(|l| l.split(','))
        .map(|f| f.split(':').next().unwrap_or("").trim())
        .filter(|n| !n.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::frame::LineUniform;
    use crate::engine::gpu::lane_shaders;

    use crate::engine::pipelines::SCENE;

    /// The shared scene code declares Instance with the Rust fields.
    #[test]
    fn instance_mirror() {
        let rust = [
            "model",
            "color",
            "flags",
            "ao_radius",
            "spacing",
            "edge_color",
            "effect",
        ];
        assert_eq!(wgsl_fields(SCENE, "Instance"), rust, "Instance fields");
    }

    /// The shared scene code declares LineUniform with the Rust fields.
    #[test]
    fn line_uniform_mirror() {
        let rust = [
            "thickness",
            "proj_y",
            "ortho_h",
            "vp_h",
            "vp_w",
            "eye_x",
            "eye_y",
            "eye_z",
            "anchor",
            "feather",
            "lit",
            "backface",
            "origin",
            "frame",
            "opacity",
            "plane_size",
        ];
        assert_eq!(
            wgsl_fields(SCENE, "LineUniform"),
            rust,
            "LineUniform fields"
        );
        assert_eq!(std::mem::size_of::<LineUniform>(), 80);
    }

    /// Translations live in the shared code; no shader redeclares the structs.
    #[test]
    fn translations_mirror() {
        let binding = "@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;";
        assert!(SCENE.contains(binding), "translations binding");
        assert!(SCENE.contains("fn place("), "the place() helper");

        for (name, src) in lane_shaders() {
            assert!(
                !src.contains("struct Instance"),
                "{name}: redeclares Instance"
            );
            assert!(
                !src.contains("struct LineUniform"),
                "{name}: redeclares LineUniform"
            );
        }

        assert_eq!(&Instance::placeholder().model[12..15], &[0.0; 3]);
    }
}

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
                            offset_of!(StrokeSegment, sag),
                            offset_of!(StrokeSegment, start_trim),
                            offset_of!(StrokeSegment, dash),
                            offset_of!(StrokeSegment, dash_offset),
                            offset_of!(StrokeSegment, end_trim),
                        ],
                        size_of::<StrokeSegment>(),
                    ),
                    "ProjectedTriangle" => (
                        vec![0, 16, 32, 48, 64, 80],
                        crate::engine::gpu::triangle_tiles::PROJECTED_BYTES as usize,
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
                            offset_of!(LineUniform, plane_size),
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
