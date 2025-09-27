from .color import Color


def test_color_constructor():
    """Test Color constructor."""
    red = Color(255, 0, 0, 255, "red")
    assert red.name == "red"
    assert red.guid != ""
    assert red.r == 255
    assert red.g == 0
    assert red.b == 0
    assert red.a == 255


def test_color_equality():
    """Test Color equality."""
    c1 = Color(0, 100, 50, 200)
    c2 = Color(0, 100, 50, 200)
    assert c1 == c2
    assert not (c1 != c2)

    c3 = Color(0, 100, 50, 200)
    c4 = Color(1, 100, 50, 200)
    assert not (c3 == c4)
    assert c3 != c4


def test_color_to_json_data():
    """Test Color to_json_data method."""
    color = Color(128, 64, 192, 255, "purple")
    data = color.to_json_data()
    assert data["type"] == "Color"
    assert data["name"] == "purple"
    assert data["r"] == 128
    assert data["g"] == 64
    assert data["b"] == 192
    assert data["a"] == 255
    assert "guid" in data


def test_color_from_json_data():
    """Test Color from_json_data method."""
    original_color = Color(200, 150, 100, 255, "bronze")
    data = original_color.to_json_data()
    restored_color = Color.from_json_data(data)
    assert restored_color.r == 200
    assert restored_color.g == 150
    assert restored_color.b == 100
    assert restored_color.a == 255
    assert restored_color.name == "bronze"
    assert restored_color.guid == original_color.guid


def test_color_to_json_from_json():
    """Test Color file I/O with to_json and from_json."""
    original = Color(255, 128, 64, 255, "sunset_orange")
    filename = "test_color.json"

    original.to_json(filename)
    loaded = Color.from_json(filename)

    assert loaded.r == original.r
    assert loaded.g == original.g
    assert loaded.b == original.b
    assert loaded.a == original.a
    assert loaded.name == original.name
    assert loaded.guid == original.guid


def test_color_white():
    """Test Color.white() class method."""
    white = Color.white()
    assert white.name == "white"
    assert white.r == 255
    assert white.g == 255
    assert white.b == 255
    assert white.a == 255


def test_color_black():
    """Test Color.black() class method."""
    black = Color.black()
    assert black.name == "black"
    assert black.r == 0
    assert black.g == 0
    assert black.b == 0
    assert black.a == 255


def test_color_to_float_array():
    """Test Color to_float_array method."""
    color = Color(255, 128, 64, 255)
    float_array = color.to_float_array()
    assert float_array == [1.0, 0.5019607843137255, 0.25098039215686274, 1.0]


def test_color_from_float():
    """Test Color.from_float() class method."""
    color = Color.from_float(1.0, 0.5, 0.25, 1.0)
    assert color.r == 255
    assert color.g == 127  # 0.5 * 255 = 127.5, rounded to 127
    assert color.b == 63  # 0.25 * 255 = 63.75, rounded to 63
    assert color.a == 255


def test_color_red():
    """Test Color.red() class method."""
    red = Color.red()
    assert red.name == "red"
    assert red.r == 255
    assert red.g == 0
    assert red.b == 0
    assert red.a == 255


def test_color_green():
    """Test Color.green() class method."""
    green = Color.green()
    assert green.name == "green"
    assert green.r == 0
    assert green.g == 255
    assert green.b == 0
    assert green.a == 255


def test_color_blue():
    """Test Color.blue() class method."""
    blue = Color.blue()
    assert blue.name == "blue"
    assert blue.r == 0
    assert blue.g == 0
    assert blue.b == 255
    assert blue.a == 255


def test_color_grey():
    """Test Color.grey() class method."""
    grey = Color.grey()
    assert grey.name == "grey"
    assert grey.r == 128
    assert grey.g == 128
    assert grey.b == 128
    assert grey.a == 255
