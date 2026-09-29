
// A streamed cloud is drawn slice by slice as its bytes arrive; its shell document holds no kernel objects.
use crate::app::stream::{CloudFields, CloudLod, SheetFields};

use crate::app::walk::cloud::{StreamRows, StreamSlice, walk_stream_slice};

/// A streamed cloud's first slice.
pub struct StreamedInit {
    pub name: String,        // display name
    pub url: String,         // the cloud file
    pub place: Xform,        // world placement
    pub rows: StreamRows,    // the first points
    pub lod: CloudLod,       // the whole node table
    pub fields: CloudFields, // array positions in the file
    pub resident: u32,       // points in this slice
    pub point_px: f32,       // point size override
    pub col_at: u64,         // byte position of the next colour
    pub ceiling: u32,        // most streamed points on the page
}

/// A streamed cloud's slot in the scene.
pub struct StreamedCloud {
    pub name: String,        // display name
    pub url: String,         // the cloud file
    pub row: u32,            // its object row
    pub lod: CloudLod,       // the whole node table
    pub fields: CloudFields, // array positions in the file
    pub place: Xform,        // world placement
    pub done_to: u32,        // points loaded so far
    pub total: u32,          // points in the file
    pub point_px: f32,       // point size override
}

impl Scene {
    /// Streamed points and normals still to come after the rows walked so far, within the ceiling.
    fn stream_expect(&self) -> (u32, u32) {
        let mut done = 0u32;
        let mut points = 0u32;
        let mut normals = 0u32;

        for cloud in &self.streamed {
            let left = cloud.total.saturating_sub(cloud.done_to);
            done = done.saturating_add(cloud.done_to);
            points = points.saturating_add(left);

            if cloud.fields.normals_len > 0 {
                normals = normals.saturating_add(left);
            }
        }

        let room = self.stream_ceiling.saturating_sub(done); // register:stream
        (points.min(room), normals.min(room))
    }

    /// Add a streamed cloud from its first slice; returns its slot.
    pub fn add_streamed_cloud(&mut self, init: StreamedInit, gpu: &mut Gpu) -> usize {
        let slot = self.stream_cloud(init);
        self.upload_to(gpu);
        slot
    }

    /// The rows of a streamed cloud's first slice and its read-only shell document.
    pub(crate) fn stream_cloud(&mut self, init: StreamedInit) -> usize {
        let StreamedInit {
            name,
            url,
            place,
            rows,
            lod,
            fields,
            resident,
            point_px,
            col_at: _,
            ceiling,
        } = init;
        self.stream_ceiling = ceiling; // register:stream
        let total = fields.count;
        let row = self.push_row(self.docs.len(), &format!("stream:{url}"), place.clone(), 0);
        let slice = StreamSlice {
            rows,
            lod: &lod,
            from: 0,
            to: resident,
            row,
            point_px,
        };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        let o = self.tables.obj.rows.last_mut().unwrap();
        o.bounds = bounds;
        o.spacing = point_px;
        self.tables.bounds.union_with(&bounds.transformed(&place));
        let model = place.clone();
        self.push_doc(
            FileDoc {
                name: name.clone(),
                place,
                session: Rc::new(Session::new(&name)),
                point_px,
                display_only: true,
            },
            DocState::default(),
        );
        self.streamed.push(StreamedCloud {
            name,
            url,
            row,
            lod,
            fields,
            place: model,
            done_to: resident,
            total,
            point_px,
        });
        self.streamed.len() - 1 // register:stream
    }

    /// Add the next slice of streamed cloud `idx`.
    pub fn extend_streamed_cloud(&mut self, idx: usize, rows: StreamRows, to: u32, gpu: &mut Gpu) {
        let Some(sc) = self.streamed.get(idx) else {
            return;
        };

        if to <= sc.done_to {
            return;
        }

        let place = match self.document(sc.row) {
            Some(document) => document.place.clone(),
            None => Xform::identity(),
        };
        let row = sc.row;
        let slice = StreamSlice {
            rows,
            lod: &sc.lod,
            from: sc.done_to,
            to,
            row,
            point_px: sc.point_px,
        };
        let bounds = walk_stream_slice(&mut self.tables.cloud, &slice);
        self.tables.bounds.union_with(&bounds.transformed(&place));
        self.streamed[idx].done_to = to; // register:stream
        self.upload_to(gpu);
        gpu.objects
            .grow_local_bounds(&gpu.ctx, row, &bounds, &place);
    }
}

#[cfg(test)]
mod stream_tests {
    use super::*;

    /// A streamed cloud's first slice of `resident` of `count` points, normals when `normals`.
    fn streamed(count: u32, resident: u32, normals: bool, ceiling: u32) -> StreamedInit {
        StreamedInit {
            name: "scan".into(),
            url: "scan.pb".into(),
            place: Xform::identity(),
            rows: StreamRows {
                positions: vec![0.0; resident as usize * 3],
                colors: Vec::new(),
                normals: Vec::new(),
            },
            lod: CloudLod::default(),
            fields: CloudFields {
                end: 0,
                coords_at: 0,
                coords_len: 0,
                colors_at: 0,
                colors_len: 0,
                normals_at: 0,
                normals_len: u64::from(normals),
                count,
                ids_at: 0,
                ids_len: 0,
                revision: None,
            },
            resident,
            point_px: 1.0,
            col_at: 0,
            ceiling,
        }
    }

    /// The points still to stream are what the files hold, but never past the page's ceiling.
    #[test]
    fn stream_expect_is_the_rest_of_the_files_within_the_ceiling() {
        let mut scene = Scene::new();
        scene.stream_cloud(streamed(10, 4, false, 100));
        assert_eq!(scene.stream_expect(), (6, 0));

        scene.stream_cloud(streamed(50, 5, true, 100));
        assert_eq!(scene.stream_expect(), (51, 45));

        scene.stream_cloud(streamed(1000, 10, false, 20));
        assert_eq!(scene.stream_expect(), (1, 1));

        scene.stream_cloud(streamed(1000, 10, false, 20));
        assert_eq!(
            scene.stream_expect(),
            (0, 0),
            "a floor past the ceiling expects nothing more"
        );
    }
}
