use session_rust::Xform;

/// One row per object, read by the shader, so one pipeline draws many objects; the GPU reads it by offset, so the layout must match WGSL exactly.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub model: [f32; 16], // rotation and scale; translation is stored separately
    pub color: [f32; 4], // rgba tint
    pub flags: u32, // FLAG_* bits below
    pub _pad0: f32, // Padding exists only to satisfy GPU alignment rules; without it the shader reads the wrong fields.
    pub spacing: f32, // vertex spacing, world units; 0 = unknown
    pub _pad: u32, // padding
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

    /// The one row an empty scene binds: identity, grey, no flags.
    pub fn placeholder() -> Self {
        Self {
            model: Xform::identity().to_f32(),
            color: [0.5, 0.5, 0.5, 1.0],
            flags: 0,
            _pad0: 0.0,
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

    /// Every shader compiles and its struct offsets match Rust.
    #[test]
    fn shader_validation_and_layouts() {
        use crate::engine::gpu::glyphs::GlyphPoint;
        use crate::engine::gpu::segments::CylinderSegment;
        use std::mem::{offset_of, size_of};

        for (name, source) in lane_shaders() {
            let source = if source.contains("-> InkColor") {
                format!(
                    "{source}\n{}",
                    include_str!("../../shaders/ink_visibility.wgsl")
                )
            } else {
                source.to_string()
            };
            // --8<-- [start:step-13]
            let source = format!(
                "{source}\n{}\n{}",
                include_str!("../../shaders/normals.wgsl"),
                include_str!("../../shaders/physical.wgsl")
            );
            // --8<-- [end:step-13]
            let module = naga::front::wgsl::parse_str(&source)
                .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::default(),
            )
            .validate(&module)
            .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));

            for (_, ty) in module.types.iter() {
                let Some(structure) = ty.name.as_deref() else {
                    continue;
                };
                let (offsets, size) = match structure {
                    "CylinderSegment" => (
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
                        ],
                        size_of::<CylinderSegment>(),
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

    use crate::engine::pipelines::SCENE;

    /// The shared scene code declares Instance with the Rust fields.
    #[test]
    fn instance_mirror() {
        let rust = ["model", "color", "flags", "_pad0", "spacing"];
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
        ];

        for (name, src) in lane_shaders() {
            if src.contains("struct LineUniform") {
                assert_eq!(
                    wgsl_fields(src, "LineUniform"),
                    rust,
                    "{name}: LineUniform fields"
                );
            }
        }

        assert_eq!(std::mem::size_of::<LineUniform>(), 64);
    }

    /// Translations live in the shared code; no shader redeclares the structs.
    #[test]
    fn translations_mirror() {
        let binding = "@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;";

        for (name, src) in lane_shaders() {
            if src.contains("struct Instance") {
                assert!(src.contains(binding), "{name}: translations binding");
                assert!(src.contains("fn place("), "{name}: the place() helper");
            }
        }

        assert_eq!(&Instance::placeholder().model[12..15], &[0.0; 3]);
    }
}
