//! `Upload` - the walked rows on their way to the GPU: every lane's table for one file, the
//! object rows included - ALL deltas. Built by `app::scene::Scene`, borrowed by
//! `Gpu::set_scene`, then emptied. No wgpu type and no kernel type here.

use crate::math::Aabb;
use super::arena::ArenaRows;
use super::cloud::CloudRows;
use super::glyphs::GlyphRows;
use super::objects::ObjectRows;
use super::segments::SegRows;

/// Everything `Gpu` needs to fill its buffers for one file. Deleting a lane = deleting its
/// field here, its file under `gpu/`, its producer under `walk/` and its line in `render.rs`.
pub struct Upload {
    pub obj: ObjectRows,
    pub arena: ArenaRows,
    pub seg: SegRows,
    pub glyph: GlyphRows,
    pub cloud: CloudRows,
    /// The world box of this upload's rows; `Gpu::set_scene` unions it into the scene box.
    pub bounds: Aabb,
}

impl Default for Upload {
    /// Every lane empty and the box inverted, ready for the first walk.
    fn default() -> Self {
        Self {
            obj: ObjectRows::default(),
            arena: ArenaRows::default(),
            seg: SegRows::default(),
            glyph: GlyphRows::default(),
            cloud: CloudRows::default(),
            bounds: Aabb::empty(),
        }
    }
}

impl Upload {
    /// Bake fixed face-plane rotation/scale once while these fresh delta rows still have placements.
    pub fn place_face_planes(&mut self, object_base: u32) {
        for plane in &mut self.arena.face_planes {
            let local = plane.instance_id.checked_sub(object_base).expect("face belongs to an earlier upload");
            let object = self.obj.rows.get(local as usize).expect("face instance absent from upload");
            super::plane_place::bake(plane, &object.place);
        }
    }

    /// Forget the uploaded rows and hand their allocations back: the GPU is their only holder now.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.obj.rows);
        self.arena.drop_rows();
        self.seg.drop_rows();
        self.glyph.drop_rows();
        self.cloud.drop_rows();
        self.bounds = Aabb::empty();
    }
}

/// Empty a table AND hand its allocation back; `clear()` alone keeps the capacity.
pub fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{arena::FacePlane, objects::ObjectRow};

    /// Incremental uploads resolve global face owners against only this delta's object rows.
    #[test]
    fn placed_planes_use_incremental_object_base() {
        let mut upload = Upload::default();
        let mut placement = session_rust::Xform::identity().m;
        placement[0] = 2.0;
        upload.obj.rows.push(ObjectRow::new(placement, 0));
        placement[0] = 3.0;
        upload.obj.rows.push(ObjectRow::new(placement, 0));
        for instance_id in [17, 18] {
            upload.arena.face_planes.push(FacePlane { point: [1.0, 0.0, 0.0], instance_id, normal: [0.0, 0.0, 1.0], _pad: 0 });
        }
        upload.place_face_planes(17);
        assert_eq!(upload.arena.face_planes[0].point, [2.0, 0.0, 0.0]);
        assert_eq!(upload.arena.face_planes[1].point, [3.0, 0.0, 0.0]);
    }
}
