from .pointcloud import PointCloud
from .point import Point
from .vector import Vector
from .color import Color


def test_pointcloud_new():
    points = [
        Point(0.0, 0.0, 0.0),
        Point(1.0, 0.0, 0.0),
        Point(0.0, 1.0, 0.0),
    ]
    normals = [
        Vector(0.0, 0.0, 1.0),
        Vector(0.0, 1.0, 0.0),
        Vector(1.0, 0.0, 0.0),
    ]
    colors = [
        Color(255, 0, 0, 255),
        Color(0, 255, 0, 255),
        Color(0, 0, 255, 255),
    ]
    cloud = PointCloud(points, normals, colors)
    assert len(cloud) == 3
    assert not cloud.is_empty()


def test_pointcloud_default():
    cloud = PointCloud()
    assert len(cloud) == 0
    assert cloud.is_empty()
    assert cloud.name == "my_pointcloud"


def test_pointcloud_iadd_vector():
    cloud = PointCloud(
        [Point(1.0, 2.0, 3.0)],
        [Vector(0.0, 0.0, 1.0)],
        [Color(255, 0, 0, 255)],
    )
    v = Vector(4.0, 5.0, 6.0)
    cloud += v
    assert cloud.points[0].x == 5.0
    assert cloud.points[0].y == 7.0
    assert cloud.points[0].z == 9.0


def test_pointcloud_add_vector():
    cloud = PointCloud(
        [Point(1.0, 2.0, 3.0)],
        [Vector(0.0, 0.0, 1.0)],
        [Color(255, 0, 0, 255)],
    )
    v = Vector(4.0, 5.0, 6.0)
    cloud2 = cloud + v
    assert cloud2.points[0].x == 5.0
    assert cloud2.points[0].y == 7.0
    assert cloud2.points[0].z == 9.0


def test_pointcloud_isub_vector():
    cloud = PointCloud(
        [Point(1.0, 2.0, 3.0)],
        [Vector(0.0, 0.0, 1.0)],
        [Color(255, 0, 0, 255)],
    )
    v = Vector(4.0, 5.0, 6.0)
    cloud -= v
    assert cloud.points[0].x == -3.0
    assert cloud.points[0].y == -3.0
    assert cloud.points[0].z == -3.0


def test_pointcloud_sub_vector():
    cloud = PointCloud(
        [Point(1.0, 2.0, 3.0)],
        [Vector(0.0, 0.0, 1.0)],
        [Color(255, 0, 0, 255)],
    )
    v = Vector(4.0, 5.0, 6.0)
    cloud2 = cloud - v
    assert cloud2.points[0].x == -3.0
    assert cloud2.points[0].y == -3.0
    assert cloud2.points[0].z == -3.0


def test_pointcloud_str():
    cloud = PointCloud(
        [Point(0.0, 0.0, 0.0)],
        [Vector(0.0, 0.0, 1.0)],
        [Color(255, 0, 0, 255)],
    )
    s = str(cloud)
    assert "PointCloud" in s
    assert "points=1" in s


def test_pointcloud_json_serialization():
    cloud = PointCloud(
        [Point(1.0, 2.0, 3.0)],
        [Vector(0.0, 0.0, 1.0)],
        [Color(255, 0, 0, 255)],
    )
    json_data = cloud.to_json_data()
    cloud2 = PointCloud.from_json_data(json_data)
    assert cloud2.points[0].x == 1.0
    assert cloud2.points[0].y == 2.0
    assert cloud2.points[0].z == 3.0


def test_pointcloud_json_file():
    cloud = PointCloud(
        [
            Point(1.0, 2.0, 3.0),
            Point(4.0, 5.0, 6.0),
            Point(7.0, 8.0, 9.0),
        ],
        [
            Vector(0.0, 0.0, 1.0),
            Vector(0.0, 1.0, 0.0),
            Vector(1.0, 0.0, 0.0),
        ],
        [
            Color(255, 0, 0, 255),
            Color(0, 255, 0, 255),
            Color(0, 0, 255, 255),
        ],
    )
    cloud.to_json("test_pointcloud.json")
    cloud2 = PointCloud.from_json("test_pointcloud.json")

    assert len(cloud2) == 3
    assert cloud2.points[0].x == 1.0
    assert cloud2.points[1].y == 5.0
    assert cloud2.points[2].z == 9.0


def test_pointcloud_json_multiple_points():
    cloud = PointCloud(
        [
            Point(1.0, 2.0, 3.0),
            Point(4.0, 5.0, 6.0),
            Point(7.0, 8.0, 9.0),
        ],
        [
            Vector(0.0, 0.0, 1.0),
            Vector(0.0, 1.0, 0.0),
            Vector(1.0, 0.0, 0.0),
        ],
        [
            Color(255, 0, 0, 255),
            Color(0, 255, 0, 255),
            Color(0, 0, 255, 255),
        ],
    )
    json_data = cloud.to_json_data()
    cloud2 = PointCloud.from_json_data(json_data)

    assert len(cloud2) == 3
    assert cloud2.points[0].x == 1.0
    assert cloud2.points[1].y == 5.0
    assert cloud2.points[2].z == 9.0
    assert cloud2.normals[0].z == 1.0
    assert cloud2.colors[1].g == 255
    # Verify alpha is always 255 after deserialization
    assert cloud2.colors[0].a == 255
    assert cloud2.colors[1].a == 255
    assert cloud2.colors[2].a == 255
