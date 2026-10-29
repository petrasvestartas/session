use crate::app::walk::{WalkCx, cloud::walk_cloud};
use crate::engine::gpu::{cloud::{Cloud, CloudRows}, lod::{LodWalk, Projection}};
use serde_json::{Value, json};
use session_rust::PointCloud;

pub struct Prepared {
    rows: CloudRows,
    cloud: Cloud,
    bounds: [[f64; 3]; 2],
}

impl Prepared {
    pub fn new(source: &PointCloud) -> Self {
        let mut rows = CloudRows::default();
        let row = walk_cloud(&mut rows, source, &WalkCx {
                quality: Default::default(),
            vert_base: 0, cloud_px: 0.0, row: 0, interactions: false, attributes: false,
        });
        let draw = &rows.draws[0];
        let cloud = Cloud {
            instance: 0, spacing: draw.spacing, node_first: draw.node_first,
            node_count: draw.node_count, resident: draw.count, chunks: Vec::new(),
        };
        let min = row.bounds.min_point();
        let max = row.bounds.max_point();
        Self {rows, cloud, bounds: [[min[0],min[1],min[2]], [max[0],max[1],max[2]]]}
    }

    pub fn metadata(&self) -> Value {
        json!({"evidence":"Actual production CPU walk_cloud and LodWalk; no GPU upload or frame acceptance",
            "source_points":self.cloud.resident,"copied_positions":self.rows.pos.len()/3,
            "nodes":self.cloud.node_count,"spacing":self.cloud.spacing,"bounds":self.bounds})
    }

    pub fn select(&self, resident: u32, ortho_h: f32, lod_px: f32, model: [f32; 16]) -> Vec<(u32, u32, f32, bool)> {
        assert!(resident <= self.cloud.resident);
        let cloud = Cloud {
            instance: self.cloud.instance, spacing: self.cloud.spacing,
            node_first: self.cloud.node_first, node_count: self.cloud.node_count,
            resident, chunks: Vec::new(),
        };
        let view = Projection {eye:[0.0,0.0,3.0], ortho_h, height_px:240, lod_px, nodes:&self.rows.nodes};
        let mut walk = LodWalk::default();
        walk.select(&view, &cloud, &model);
        walk.ranges.iter().map(|range| (range.first, range.count, range.spacing, range.tile)).collect()
    }
}
