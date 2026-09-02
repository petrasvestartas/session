//! `Instance` - the one object row every instance-reading shader indexes by `instance_id`,
//! its flag bits, and the WGSL field parser the mirror tests use to prove the five shaders
//! declare the same row and the same translation table. No buffer and no bind group here:
//! `objects.rs` owns both tables.

use session_rust::Xform;

/// One object row as the five instance-reading shaders see it: the model's rotation/scale
/// with a ZERO translation column (the anchored translation is the 16 B row at group 2
/// binding 1 - `InstanceTable::translations`), the tint, the flag bits and two scalars the
/// ink lanes read. 96 B, the storage stride.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub(crate) model: [f32; 16], // 64 B - column-major, from Xform::to_f32(), [12..15] = 0
    pub(crate) color: [f32; 4], // 16 B
    pub(crate) flags: u32, // 4 B - bit 0 reserved for FLAG_SELECTED
    /// This object's world AABB diagonal, in world units. The ink lanes CLAMP their lift to a
    /// fraction of it - see `LIFT_MAX_EXTENT` in ribbon.wgsl. 0.0 = unknown, no clamp.
    ///
    /// Without it the lift is a fraction of EYE DEPTH, so its world size grows with camera
    /// distance while an object's front-to-back size does not: past some distance the back
    /// wireframe is lifted in front of the front faces and the object goes see-through. Measured
    /// on a 1000 mm box at a 2px pen, that distance is 242 m for a band and 91 m for a marker -
    /// ordinary zoom-out in a scene spanning tens of metres.
    pub(crate) extent: f32, // 4 B
    /// Vertex spacing in world units (see `ObjectRows::spacing`). The ink lanes drop
    /// markers once this projects below a few pixels; 0 = unknown, never culled.
    pub(crate) spacing: f32, // 4 B
    pub(crate) _pad: u32, // 4 B - pad the row to 96 B (storage array stride)
}

impl Instance {
    /// The row is skipped by every draw. Bit 1; bit 0 is reserved for FLAG_SELECTED.
    pub const FLAG_HIDDEN: u32 = 1 << 1;
    /// The eye is inside this object's bounds (per-frame CPU test, see `update_inside_flags`).
    /// Both edge lanes then skip the facing cull - from inside a solid every face points away -
    /// and the flat lane hugs BOTH adjacent face planes, since the back-facing ones are the
    /// visible surface from in there. Bit 2, matching FLAG_INSIDE in ribbon.wgsl/cylinder.wgsl.
    pub const FLAG_INSIDE: u32 = 1 << 2;

    /// The mesh broadcast a zero edge width: it is PRINT, not surface - a PDF glyph, a poché
    /// region, any triangulated fill. triangle.wgsl lights such faces flat (lit = 1.0), so the
    /// authored colour reads the same from the back of the sheet as from the front. Bit 3.
    pub const FLAG_PRINT: u32 = 1 << 3;

    /// The mesh is NOT closed (boundary edges exist), so the facing cull's premise - both
    /// adjacent faces away = far side of a solid, hidden - is void: an interior surface can be
    /// genuinely visible through the hole, faces drawn but its wireframe culled (the bunny's
    /// open base). Set once at build time from Mesh::is_closed(); the edge lanes then skip the
    /// facing cull exactly as FLAG_INSIDE does and occlusion falls to the depth test, which
    /// both lanes already write honestly. Bit 4.
    pub const FLAG_OPEN: u32 = 1 << 4;

    /// This row belongs to a PLANAR file - a drawing sheet. Its fills write no depth (they are
    /// exactly coplanar and composite in document order instead), so the sheet's ink has nothing
    /// to fight and takes NO lift: ribbon.wgsl reads this bit and keeps the pen on the page. That
    /// is what lets the lettering pass, drawn last with a >= depth test, land on top of the
    /// linework the way the page draws it.
    pub const FLAG_SHEET: u32 = 1 << 5;

    /// The one-row placeholder an empty scene draws from: identity, mid grey, no flags.
    pub(crate) fn placeholder() -> Self {
        Self { model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0 }
    }
}

/// The field names of a WGSL `struct <name> { .. }`, in declaration order: `//` comments
/// stripped, fields split on `,` and newlines, the name taken before its `:`. Test-only.
#[cfg(test)]
pub(crate) fn wgsl_fields(src: &str, struct_name: &str) -> Vec<String> {
    let at = src.find(&format!("struct {struct_name}")).expect("struct declared in the shader");
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
    use crate::engine::gpu::glyphs::GlyphPoint;
    use crate::engine::gpu::segments::CylinderSegment;

    /// The Rust row's field names; `_pad` is layout-only and WGSL pads implicitly.
    const INSTANCE_FIELDS: [&str; 6] = ["model", "color", "flags", "extent", "spacing", "_pad"];

    /// Every instance-reading shader declares `Instance` with exactly the Rust fields, in order,
    /// and the Rust row is the 96 B stride the storage array uses.
    #[test]
    fn instance_mirror() {
        let shaders = [
            ("triangle.wgsl", include_str!("../../shaders/triangle.wgsl")),
            ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")),
            ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl")),
            ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
            ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
        ];
        let rust: Vec<&str> = INSTANCE_FIELDS.iter().copied().filter(|n| !n.starts_with('_')).collect();

        for (name, src) in shaders {
            assert_eq!(wgsl_fields(src, "Instance"), rust, "{name}: Instance fields");
        }
        assert_eq!(std::mem::size_of::<Instance>(), 96);
    }

    /// Five shaders declare `LineUniform`. The names cannot match 1:1: Rust's `eye: [f32; 3]`
    /// is `eye_x/eye_y/eye_z` in WGSL (three scalars fill the pad before `anchor`'s 16 B
    /// alignment) and `_pad1` is layout-only - so the comparison goes through that mapping,
    /// and the 48 B size is asserted on the Rust side.
    #[test]
    fn line_uniform_mirror() {
        let shaders = [
            ("grid.wgsl", include_str!("../../shaders/grid.wgsl")),
            ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")),
            ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl")),
            ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
            ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
        ];
        let rust = ["thickness", "proj_y", "ortho_h", "vp_h", "vp_w", "eye_x", "eye_y", "eye_z", "anchor"];

        for (name, src) in shaders {
            assert_eq!(wgsl_fields(src, "LineUniform"), rust, "{name}: LineUniform fields");
        }
        assert_eq!(std::mem::size_of::<LineUniform>(), 48);
    }

    /// cylinder.wgsl and ribbon.wgsl read the same 40 B segment row. The ends are three scalars
    /// each in WGSL (a `vec3<f32>` would pad the row to 48), so Rust's `p0`/`p1` map to
    /// `p0x/p0y/p0z` and `p1x/p1y/p1z`.
    #[test]
    fn cylinder_segment_mirror() {
        let shaders = [
            ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")),
            ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl")),
        ];
        let rust = ["p0x", "p0y", "p0z", "radius", "p1x", "p1y", "p1z", "instance_id", "color", "facing"];

        for (name, src) in shaders {
            assert_eq!(wgsl_fields(src, "CylinderSegment"), rust, "{name}: CylinderSegment fields");
        }
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
    }

    /// Every instance-reading shader binds the 16 B translation table at group 2 binding 1
    /// and adds it to exactly its POINT transforms (`model * vec4(p, 1.0)`), never to a
    /// direction; the Rust row (`[f32; 4]`) is the 16 B stride, and the placeholder row's
    /// model carries no translation of its own.
    #[test]
    fn translations_mirror() {
        let shaders = [
            ("triangle.wgsl", include_str!("../../shaders/triangle.wgsl"), 1),
            ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl"), 2),
            ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl"), 2),
            ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl"), 1),
            ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl"), 1),
        ];
        let binding = "@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;";

        for (name, src, points) in shaders {
            assert!(src.contains(binding), "{name}: translations binding");
            let point_lines: Vec<&str> = src.lines().filter(|l| l.contains("model * vec4<f32>(") && l.contains(", 1.0)")).collect();
            assert_eq!(point_lines.len(), points, "{name}: point transforms");
            for line in &point_lines {
                assert!(line.contains("translations["), "{name}: a point transform without the translation: {line}");
            }
            assert_eq!(src.matches("translations[").count(), points, "{name}: the translation reaches no direction");
        }
        assert_eq!(std::mem::size_of::<[f32; 4]>(), 16);
        assert_eq!(&Instance::placeholder().model[12..15], &[0.0; 3]);
    }

    /// sphere.wgsl and glyph.wgsl read the same 48 B glyph row, field for field.
    #[test]
    fn glyph_point_mirror() {
        let shaders = [
            ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
            ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
        ];
        let rust = ["center", "radius", "color", "instance_id", "facing", "facing_ext"];

        for (name, src) in shaders {
            assert_eq!(wgsl_fields(src, "GlyphPoint"), rust, "{name}: GlyphPoint fields");
        }
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 48);
    }
}
