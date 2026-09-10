//! `Instance` - the one object row every instance-reading shader indexes by `instance_id`,
//! its flag bits, and the mirror tests that prove the shaders declare the same rows.
//! No buffer and no bind group here: `objects.rs` owns both tables.

use session_rust::Xform;

/// One object row as the shaders see it: rotation/scale with a ZERO translation column (the
/// anchored translation is the 16 B row at group 2 binding 1), tint, flag bits and two
/// scalars the ink lanes read. 96 B, the storage stride.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub model: [f32; 16],
    pub color: [f32; 4],
    pub flags: u32,
    /// Unused; keeps `spacing` at offset 88 and the row at 96 bytes.
    pub _pad0: f32,
    /// Vertex spacing, world units; markers thin once it projects small. 0 = unknown.
    pub spacing: f32,
    pub _pad: u32,
}

const _: () = assert!(std::mem::size_of::<Instance>() == 96);

impl Instance {
    /// The row is the current selection: the shaders tint it. Bit 0.
    pub const FLAG_SELECTED: u32 = 1 << 0;
    /// The row is skipped by every draw. Bit 1.
    pub const FLAG_HIDDEN: u32 = 1 << 1;
    /// The eye is inside this object's bounds (per-frame CPU test): the edge lanes skip the
    /// facing cull, since from inside a solid every face points away. Bit 2.
    pub const FLAG_INSIDE: u32 = 1 << 2;
    /// A print fill (zero edge width): lit flat, no wireframe. Bit 3.
    pub const FLAG_PRINT: u32 = 1 << 3;
    /// An open mesh (border edges): the facing cull's premise is void, skipped like INSIDE. Bit 4.
    pub const FLAG_OPEN: u32 = 1 << 4;
    /// A row of a planar drawing sheet: fills composite in document order. Bit 5.
    pub const FLAG_SHEET: u32 = 1 << 5;
    /// A TESSELLATION, not an authored mesh: its interior seams are an artifact of how finely
    /// the surface was sampled, not edges of the thing. The walk drops those seams before the
    /// GPU sees them; this flag is what tells the marker lane its vertices are samples, not
    /// corners. Bit 6.
    pub const FLAG_SMOOTH: u32 = 1 << 6;
    /// One face only (a NURBS surface, a one-face mesh or BRep): x-ray leaves it shaded, since
    /// it has no interior to look into.
    pub const FLAG_SINGLE: u32 = 1 << 7;

    /// The one-row placeholder an empty scene binds: identity, mid grey, no flags.
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

/// The field names of a WGSL `struct <name> { .. }`, in declaration order. Test-only.
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

    /// Validate the actual shader modules and every storage member offset, not merely field
    /// names: a valid Rust size alone does not prove WGSL's array stride.
    #[test]
    fn shader_validation_and_layouts() {
        use crate::engine::gpu::glyphs::GlyphPoint;
        use crate::engine::gpu::segments::{CylinderSegment, StrokeSegment};
        use std::mem::{offset_of, size_of};
        for (name, source) in lane_shaders() {
            // The backdrop declares no scene binding; every other lane is on the contract.
            let scene = source.contains("mvp") || source.contains("line.");
            let source =
                crate::engine::pipelines::assemble(source, scene, source.contains("-> InkColor"));
            let module = naga::front::wgsl::parse_str(&source)
                .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
            naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                // Core WGSL pack2x16float/unpack2x16float, also enabled unconditionally
                // by wgpu-naga-bridge; this does not enable shader-f16.
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

    const SCENE: &str = include_str!("../../shaders/scene.wgsl");

    /// The scene contract declares `Instance` with the Rust fields, in order.
    #[test]
    fn instance_mirror() {
        let rust = ["model", "color", "flags", "_pad0", "spacing"];
        assert_eq!(wgsl_fields(SCENE, "Instance"), rust, "Instance fields");
    }

    /// Every shader that declares `LineUniform` lists the Rust fields; `eye: [f32; 3]` is
    /// three scalars there.
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

    /// The translation table is bound at group 2 binding 1 and added through `place()`, never
    /// to a direction; no lane declares its own copy of the contract.
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
