from .point import Point
from .color import Color


def test_point_constructor():
    """Test Point constructor."""
    point = Point(1.0, 2.0, 3.0)
    assert point.name == "my_point"
    assert point.guid != ""
    assert point.x == 1.0
    assert point.y == 2.0
    assert point.z == 3.0
    assert point.width == 1.0
    assert point.pointcolor == Color.white()


def test_point_equality():
    """Test Point equality."""
    p1 = Point(1.0, 2.0, 3.0)
    p2 = Point(1.0, 2.0, 3.0)
    assert p1 == p2
    assert not (p1 != p2)

    p3 = Point(1.0, 2.0, 3.0)
    p4 = Point(1.1, 2.0, 3.0)
    assert not (p3 == p4)
    assert p3 != p4

###########################################################################################
# JSON
###########################################################################################

def test_point_to_json_data():
    """Test Point to_json_data method."""
    point = Point(15.5, 25.7, 35.9)
    point.name = "survey_point_A"
    point.width = 2.5
    point.pointcolor = Color(255, 128, 64, 255)
    data = point.to_json_data()
    assert data["type"] == "Point"
    assert data["name"] == "survey_point_A"
    assert data["x"] == 15.5
    assert data["y"] == 25.7
    assert data["z"] == 35.9
    assert data["width"] == 2.5
    assert data["pointcolor"]["r"] == 255
    assert data["pointcolor"]["g"] == 128
    assert data["pointcolor"]["b"] == 64
    assert data["pointcolor"]["a"] == 255


def test_point_from_json_data():
    """Test Point from_json_data method."""
    original_point = Point(42.1, 84.2, 126.3)
    original_point.name = "control_point_B"
    original_point.width = 3.0
    original_point.pointcolor = Color(200, 100, 50, 255)
    data = original_point.to_json_data()
    restored_point = Point.from_json_data(data)
    assert restored_point.x == 42.1
    assert restored_point.y == 84.2
    assert restored_point.z == 126.3
    assert restored_point.name == "control_point_B"
    assert restored_point.width == 3.0
    assert restored_point.pointcolor.r == 200
    assert restored_point.pointcolor.g == 100
    assert restored_point.pointcolor.b == 50
    assert restored_point.pointcolor.a == 255
    assert restored_point.guid == original_point.guid


def test_point_to_json_from_json():
    """Test Point file I/O with to_json and from_json."""
    original = Point(123.45, 678.90, 999.11)
    original.name = "file_test_point"
    original.width = 4.5
    original.pointcolor = Color(0, 255, 128, 255)
    filename = "test_point.json"

    original.to_json(filename)
    loaded = Point.from_json(filename)

    assert loaded.x == original.x
    assert loaded.y == original.y
    assert loaded.z == original.z
    assert loaded.name == original.name
    assert loaded.width == original.width
    assert loaded.pointcolor == original.pointcolor
    assert loaded.guid == original.guid

###########################################################################################
# No-copy Operators
###########################################################################################

def test_point_getitem():
    """Test Point getitem method."""
    point = Point(1.0, 2.0, 3.0)
    assert point[0] == 1.0
    assert point[1] == 2.0
    assert point[2] == 3.0

def test_point_setitem():
    """Test Point setitem method."""
    point = Point(1.0, 2.0, 3.0)
    point[0] = 4.0
    point[1] = 5.0
    point[2] = 6.0
    assert point.x == 4.0
    assert point.y == 5.0
    assert point.z == 6.0

def test_point_imul():
    """Test Point imul method."""
    point = Point(1.0, 2.0, 3.0)
    point *= 2.0
    assert point.x == 2.0
    assert point.y == 4.0
    assert point.z == 6.0

def test_point_itruediv():
    """Test Point itruediv method."""
    point = Point(2.0, 4.0, 6.0)
    point /= 2.0
    assert point.x == 1.0
    assert point.y == 2.0
    assert point.z == 3.0

def test_point_iadd():
    """Test Point iadd method."""
    point = Point(1.0, 2.0, 3.0)
    point += Point(4.0, 5.0, 6.0)
    assert point.x == 5.0
    assert point.y == 7.0
    assert point.z == 9.0

def test_point_isub():
    """Test Point isub method."""
    point = Point(5.0, 7.0, 9.0)
    point -= Point(4.0, 5.0, 6.0)
    assert point.x == 1.0
    assert point.y == 2.0
    assert point.z == 3.0

###########################################################################################
# Copy Operators
###########################################################################################

def test_point_mul():
    """Test Point mul method."""
    point = Point(1.0, 2.0, 3.0)
    result = point * 2.0
    assert result.x == 2.0
    assert result.y == 4.0
    assert result.z == 6.0

def test_point_truediv():
    """Test Point truediv method."""
    point = Point(2.0, 4.0, 6.0)
    result = point / 2.0
    assert result.x == 1.0
    assert result.y == 2.0
    assert result.z == 3.0

def test_point_add():
    """Test Point add method."""
    point = Point(1.0, 2.0, 3.0)
    result = point + Point(4.0, 5.0, 6.0)
    assert result.x == 5.0
    assert result.y == 7.0
    assert result.z == 9.0

def test_point_sub():
    """Test Point sub method."""
    point = Point(5.0, 7.0, 9.0)
    result = point - Point(4.0, 5.0, 6.0)
    assert result.x == 1.0
    assert result.y == 2.0
    assert result.z == 3.0

###########################################################################################
# Details
###########################################################################################

def test_point_ccw():
    """Test Point ccw method."""
    a = Point(0.0, 0.0, 0.0)
    b = Point(1.0, 0.0, 0.0)
    c = Point(0.0, 1.0, 0.0)
    assert Point.ccw(a, b, c)
    assert not Point.ccw(b, a, c)

def test_point_mid_point():
    """Test Point mid_point method."""
    p1 = Point(0.0, 0.0, 0.0)
    p2 = Point(1.0, 0.0, 0.0)
    mid = p1.mid_point(p2)
    assert round(mid.x, 6) == 0.5
    assert round(mid.y, 6) == 0.0
    assert round(mid.z, 6) == 0.0
    
def test_point_distance():
    """Test Point distance method."""
    p1 = Point(0.0, 0.0, 0.0)
    p2 = Point(1.0, 0.0, 0.0)
    assert round(p1.distance(p2), 6) == 1.0
    
def test_point_area():
    """Test Point area method."""
    points = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(0.0, 1.0, 0.0)]
    assert Point.area(points) == 0.5

def test_point_centroid_quad():
    """Test Point centroid_quad method."""
    vertices = [Point(0.0, 0.0, 0.0), Point(1.0, 0.0, 0.0), Point(1.0, 1.0, 0.0), Point(0.0, 1.0, 0.0)]
    centroid = Point.centroid_quad(vertices)
    assert round(centroid.x, 6) == 0.5
    assert round(centroid.y, 6) == 0.5
    assert round(centroid.z, 6) == 0.0
