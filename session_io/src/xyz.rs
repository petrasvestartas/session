//! XYZ point cloud (`x y z [r g b [a]]`) -> PointCloud.
use session_rust::{Color, Point, PointCloud};
use std::io;

pub fn write_xyz_to_string(cloud: &PointCloud) -> String {
    let mut s = String::new();
    let has_color = cloud.color_count() == cloud.point_count();
    for (i, p) in cloud.get_points().iter().enumerate() {
        if has_color {
            let c = cloud.get_color(i);
            s.push_str(&format!("{} {} {} {} {} {}\n", p[0], p[1], p[2],
                (c.r * 255.0).round() as i32, (c.g * 255.0).round() as i32, (c.b * 255.0).round() as i32));
        } else {
            s.push_str(&format!("{} {} {}\n", p[0], p[1], p[2]));
        }
    }
    s
}

pub fn write_xyz(cloud: &PointCloud, filepath: &str) -> io::Result<()> {
    std::fs::write(filepath, write_xyz_to_string(cloud))
}

pub fn read_xyz(filepath: &str) -> io::Result<PointCloud> {
    let content = std::fs::read_to_string(filepath)?;
    Ok(read_xyz_from_str(&content))
}

pub fn read_xyz_from_str(content: &str) -> PointCloud {
    let mut cloud = PointCloud::default();
    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        let (x, y, z) = match (parts[0].parse::<f64>(), parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
            (Ok(x), Ok(y), Ok(z)) => (x, y, z),
            _ => continue,
        };
        cloud.add_point(&Point::new(x, y, z));
        // Optional per-point color: "x y z r g b [a]". Auto-detect 0-255 ints vs 0-1 floats.
        if parts.len() >= 6 {
            if let (Ok(r), Ok(g), Ok(b)) =
                (parts[3].parse::<f32>(), parts[4].parse::<f32>(), parts[5].parse::<f32>()) {
                let s = if r > 1.0 || g > 1.0 || b > 1.0 { 1.0 / 255.0 } else { 1.0 };
                let a = parts.get(6).and_then(|v| v.parse::<f32>().ok()).map(|a| a * s).unwrap_or(1.0);
                cloud.add_color(&Color::new(r * s, g * s, b * s, a));
            }
        }
    }
    cloud
}
