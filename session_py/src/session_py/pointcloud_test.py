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


def test_pointcloud_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    points = [Point(0, 0, 0), Point(1, 1, 1)]
    normals = [Vector(0, 0, 1), Vector(0, 0, 1)]
    colors = [Color.red(), Color.blue()]
    cloud = PointCloud(points, normals, colors)
    cloud.name = "test_cloud"

    path = Path(__file__).resolve().parents[2] / "test_pointcloud.json"
    json_dump(cloud, path)
    loaded = json_load(path)

    assert isinstance(loaded, PointCloud)
    assert len(loaded.points) == 2
    assert loaded.points[0].x == 0.0
    assert loaded.points[1].z == 1.0
    assert loaded.name == cloud.name
