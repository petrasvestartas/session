use super::*;
use crate::encoders::{json_dump, json_load};

#[test]
fn test_pointcloud_new() {
    let points = vec![
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
    ];
    let normals = vec![
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
    ];
    let colors = vec![
        Color::new(255, 0, 0, 255),
        Color::new(0, 255, 0, 255),
        Color::new(0, 0, 255, 255),
    ];
    let cloud = PointCloud::new(points, normals, colors);
    assert_eq!(cloud.len(), 3);
    assert!(!cloud.is_empty());
}

#[test]
fn test_pointcloud_default() {
    let cloud = PointCloud::default();
    assert_eq!(cloud.len(), 0);
    assert!(cloud.is_empty());
    assert_eq!(cloud.name, "my_pointcloud");
}

#[test]
fn test_pointcloud_add_assign_vector() {
    let mut cloud = PointCloud::new(
        vec![Point::new(1.0, 2.0, 3.0)],
        vec![Vector::new(0.0, 0.0, 1.0)],
        vec![Color::new(255, 0, 0, 255)],
    );
    let v = Vector::new(4.0, 5.0, 6.0);
    cloud += v;
    assert_eq!(cloud.points[0].x(), 5.0);
    assert_eq!(cloud.points[0].y(), 7.0);
    assert_eq!(cloud.points[0].z(), 9.0);
}

#[test]
fn test_pointcloud_add_vector() {
    let cloud = PointCloud::new(
        vec![Point::new(1.0, 2.0, 3.0)],
        vec![Vector::new(0.0, 0.0, 1.0)],
        vec![Color::new(255, 0, 0, 255)],
    );
    let v = Vector::new(4.0, 5.0, 6.0);
    let cloud2 = cloud + v;
    assert_eq!(cloud2.points[0].x(), 5.0);
    assert_eq!(cloud2.points[0].y(), 7.0);
    assert_eq!(cloud2.points[0].z(), 9.0);
}

#[test]
fn test_pointcloud_sub_assign_vector() {
    let mut cloud = PointCloud::new(
        vec![Point::new(1.0, 2.0, 3.0)],
        vec![Vector::new(0.0, 0.0, 1.0)],
        vec![Color::new(255, 0, 0, 255)],
    );
    let v = Vector::new(4.0, 5.0, 6.0);
    cloud -= v;
    assert_eq!(cloud.points[0].x(), -3.0);
    assert_eq!(cloud.points[0].y(), -3.0);
    assert_eq!(cloud.points[0].z(), -3.0);
}

#[test]
fn test_pointcloud_sub_vector() {
    let cloud = PointCloud::new(
        vec![Point::new(1.0, 2.0, 3.0)],
        vec![Vector::new(0.0, 0.0, 1.0)],
        vec![Color::new(255, 0, 0, 255)],
    );
    let v = Vector::new(4.0, 5.0, 6.0);
    let cloud2 = cloud - v;
    assert_eq!(cloud2.points[0].x(), -3.0);
    assert_eq!(cloud2.points[0].y(), -3.0);
    assert_eq!(cloud2.points[0].z(), -3.0);
}

#[test]
fn test_pointcloud_display() {
    let cloud = PointCloud::new(
        vec![Point::new(0.0, 0.0, 0.0)],
        vec![Vector::new(0.0, 0.0, 1.0)],
        vec![Color::new(255, 0, 0, 255)],
    );
    let display = format!("{cloud}");
    assert!(display.contains("PointCloud"));
    assert!(display.contains("points=1"));
}

#[test]
fn test_pointcloud_json_serialization() {
    let cloud = PointCloud::new(
        vec![Point::new(1.0, 2.0, 3.0)],
        vec![Vector::new(0.0, 0.0, 1.0)],
        vec![Color::new(255, 0, 0, 255)],
    );
    let json = cloud.jsondump().unwrap();
    let cloud2 = PointCloud::jsonload(&json).unwrap();
    assert_eq!(cloud2.points[0].x(), 1.0);
    assert_eq!(cloud2.points[0].y(), 2.0);
    assert_eq!(cloud2.points[0].z(), 3.0);
}

#[test]
fn test_pointcloud_json_file() {
    let cloud = PointCloud::new(
        vec![
            Point::new(1.0, 2.0, 3.0),
            Point::new(4.0, 5.0, 6.0),
            Point::new(7.0, 8.0, 9.0),
        ],
        vec![
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
        ],
        vec![
            Color::new(255, 0, 0, 255),
            Color::new(0, 255, 0, 255),
            Color::new(0, 0, 255, 255),
        ],
    );
    json_dump(&cloud, "test_pointcloud.json", true).unwrap();
    let cloud2 = json_load::<PointCloud>("test_pointcloud.json").unwrap();
    assert_eq!(cloud2.points[0].x(), 1.0);
    assert_eq!(cloud2.points[1].y(), 5.0);
    assert_eq!(cloud2.points[2].z(), 9.0);
    assert_eq!(cloud2.len(), 3);
}

#[test]
fn test_pointcloud_json_multiple_points() {
    let cloud = PointCloud::new(
        vec![
            Point::new(1.0, 2.0, 3.0),
            Point::new(4.0, 5.0, 6.0),
            Point::new(7.0, 8.0, 9.0),
        ],
        vec![
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
        ],
        vec![
            Color::new(255, 0, 0, 255),
            Color::new(0, 255, 0, 255),
            Color::new(0, 0, 255, 255),
        ],
    );
    let json = cloud.jsondump().unwrap();
    let cloud2 = PointCloud::jsonload(&json).unwrap();

    assert_eq!(cloud2.len(), 3);
    assert_eq!(cloud2.points[0].x(), 1.0);
    assert_eq!(cloud2.points[1].y(), 5.0);
    assert_eq!(cloud2.points[2].z(), 9.0);
    assert_eq!(cloud2.normals[0].z(), 1.0);
    assert_eq!(cloud2.colors[1].g, 255);
    // Verify alpha is always 255 after deserialization
    assert_eq!(cloud2.colors[0].a, 255);
    assert_eq!(cloud2.colors[1].a, 255);
    assert_eq!(cloud2.colors[2].a, 255);
}
