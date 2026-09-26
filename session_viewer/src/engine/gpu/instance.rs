use session_rust::Xform;

/// One object row as the shaders read it, 96 bytes.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub model: [f32; 16], // rotation and scale; translation is stored separately
    pub color: [f32; 4],  // rgba tint
    pub flags: u32,       // FLAG_* bits below
    pub ao_radius: f32,   // SSAO contact radius, world units
    pub spacing: f32,     // vertex spacing, world units; 0 = unknown
    pub _pad: u32,        // padding
}

const _: () = assert!(std::mem::size_of::<Instance>() == 96);

impl Instance {
    /// Selected: drawn tinted.
    pub const FLAG_SELECTED: u32 = 1 << 0;

    /// Hidden: skipped by every draw.
    pub const FLAG_HIDDEN: u32 = 1 << 1;

    /// Camera is inside the object: no back-face culling.
    pub const FLAG_INSIDE: u32 = 1 << 2;

    /// Sheet fill: flat color, no edges.
    pub const FLAG_PRINT: u32 = 1 << 3;

    /// Open mesh: no back-face culling.
    pub const FLAG_OPEN: u32 = 1 << 4;

    /// Part of a drawing sheet.
    pub const FLAG_SHEET: u32 = 1 << 5;

    /// Sampled surface: vertices are samples, not corners.
    pub const FLAG_SMOOTH: u32 = 1 << 6;

    /// Single face: stays shaded in x-ray.
    pub const FLAG_SINGLE: u32 = 1 << 7;

    /// Use the layer color instead of the object's.
    pub const FLAG_COLOR: u32 = 1 << 8;

    /// Use the layer color for edges too.
    pub const FLAG_EDGE_COLOR: u32 = 1 << 9;

    /// The object has faces, not only lines or points.
    pub const FLAG_HAS_FACES: u32 = 1 << 10;

    /// A retired or sink row; CPU only, always with FLAG_HIDDEN.
    pub const FLAG_DEAD: u32 = 1 << 11;

    /// A clipping plane: it cuts the scene and is never cut.
    pub const FLAG_CLIPPING_PLANE: u32 = 1 << 12;

    /// A verified closed solid: a cut through it gets a section cap.
    pub const FLAG_CLOSED: u32 = 1 << 13;

    /// A closed solid whose faces wind inward.
    pub const FLAG_INWARD: u32 = 1 << 14;

    /// A curve with arrowheads: its ribbons look along the curve for a head to stop under.
    pub const FLAG_HEADS: u32 = 1 << 15;

    /// The one row an empty scene binds: identity, grey, no flags.
    pub fn placeholder() -> Self {
        Self {
            model: Xform::identity().to_f32(),
            color: [0.5, 0.5, 0.5, 1.0],
            flags: 0,
            ao_radius: 0.0,
            spacing: 0.0,
            _pad: 0,
        }
    }
}

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
