use anyhow::{Result, ensure};
use prost::Message;
use serde_json::json;
use session_rust::Xform;
use sha2::{Digest, Sha256};
use std::{path::Path, rc::Rc};

fn main() -> Result<()> {
    let output = std::env::args_os().nth(1).ok_or_else(|| anyhow::anyhow!("supply evidence directory"))?;
    let output = Path::new(&output);
    std::fs::create_dir_all(output)?;
    let source = Rc::new(viewer_journey::cloud::large_example());
    let _ = source.guid();
    let source_sha = format!("{:x}", Sha256::digest(source.to_proto().encode_to_vec()));
    let course = viewer_journey::cloud::Prepared::new(source.clone()).map_err(anyhow::Error::msg)?;
    let production = current_viewer::selftest::cloud_witness::lod::Prepared::new(&source);
    let metadata = production.metadata();
    ensure!(metadata["source_points"] == 2_000_064 && metadata["copied_positions"] == 2_000_064);
    ensure!(metadata["nodes"] == course.nodes.len());
    // Production preprocessing rounds positions to f32; this is expected and reported.
    for (side, bounds) in [course.bounds.min, course.bounds.max].iter().enumerate() {
        for axis in 0..3 {
            ensure!((metadata["bounds"][side][axis].as_f64().unwrap() - bounds[axis]).abs() < 1e-7);
        }
    }
    let mut cases = Vec::new();
    for resident in [1_999_999, 2_000_000, 2_000_001] {
        for ortho in [100.0_f32, 0.7, 0.0] {
            for (placement, model) in [("identity", Xform::identity()), ("mirrored_nonuniform", Xform::scale_xyz(-2.0, 0.5, 4.0))] {
                let mut descriptor = course.descriptor();
                descriptor.resident = resident;
                let view = viewer_journey::cloud_lod::Projection {eye:[0.0,0.0,3.0], ortho_h:ortho, height_px:240, lod_px:2.0, nodes:&course.nodes};
                let mut walk = viewer_journey::cloud_lod::LodWalk::default();
                walk.select(&view, &descriptor, &model.to_f32());
                let actual: Vec<_> = walk.ranges.iter().map(|r| (r.first,r.count,r.spacing,r.tile)).collect();
                let expected = production.select(resident, ortho, 2.0, model.to_f32());
                ensure!(actual == expected, "Actual production/course LOD mismatch: {resident} {ortho} {placement}");
                let count = actual.iter().map(|r| r.1).sum::<u32>();
                ensure!(count == if resident < 2_000_000 || ortho < 1.0 {resident} else {64});
                ensure!(actual.iter().all(|r| r.0+r.1 <= resident));
                cases.push(json!({"resident":resident,"ortho_half_height":ortho,"placement":placement,"model":model.m,"production_ranges":expected,"course_ranges":actual}));
            }
        }
    }
    ensure!(source_sha == format!("{:x}", Sha256::digest(source.to_proto().encode_to_vec())), "Original source changed");
    std::fs::write(output.join("manifest.json"), serde_json::to_vec_pretty(&json!({
        "source_sha256":source_sha,"source_guid":source.guid(),"production":metadata,"cases":cases,
        "pixel_acceptance":false,"required_gpu_checks":["actual_large_cloud_changes_detail_with_the_view_and_reuses_its_original_upload","actual_resident_cloud_controls_query_original_points_outside_the_coarse_display_subset"]
    }))?)?;
    println!("PASS: 18 actual production/course CPU LOD pairs at the 2M boundary; actual 2,000,064-point source retained");
    Ok(())
}
