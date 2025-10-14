from .color import Color


def test_color_constructor():
    red = Color(255, 0, 0, 255, "red")
    assert red.name == "red"
    assert red.guid != ""
    assert red.r == 255
    assert red.g == 0
    assert red.b == 0
    assert red.a == 255


def test_color_equality():
    c1 = Color(0, 100, 50, 200)
    c2 = Color(0, 100, 50, 200)
    assert c1 == c2
    assert not (c1 != c2)

    c3 = Color(0, 100, 50, 200)
    c4 = Color(1, 100, 50, 200)
    assert not (c3 == c4)
    assert c3 != c4


def test_color_white():
    white = Color.white()
    assert white.name == "white"
    assert white.r == 255
    assert white.g == 255
    assert white.b == 255
    assert white.a == 255


def test_color_black():
    black = Color.black()
    assert black.name == "black"
    assert black.r == 0
    assert black.g == 0
    assert black.b == 0
    assert black.a == 255


def test_color_to_float_array():
    color = Color(255, 128, 64, 255)
    float_array = color.to_float_array()
    assert float_array == [1.0, 0.5019607843137255, 0.25098039215686274, 1.0]


def test_color_from_float():
    color = Color.from_float(1.0, 0.5, 0.25, 1.0)
    assert color.r == 255
    assert color.g == 127  # 0.5 * 255 = 127.5, rounded to 127
    assert color.b == 63  # 0.25 * 255 = 63.75, rounded to 63
    assert color.a == 255


def test_color_red():
    red = Color.red()
    assert red.name == "red"
    assert red.r == 255
    assert red.g == 0
    assert red.b == 0
    assert red.a == 255


def test_color_green():
    green = Color.green()
    assert green.name == "green"
    assert green.r == 0
    assert green.g == 255
    assert green.b == 0
    assert green.a == 255


def test_color_blue():
    blue = Color.blue()
    assert blue.name == "blue"
    assert blue.r == 0
    assert blue.g == 0
    assert blue.b == 255
    assert blue.a == 255


def test_color_silver():
    silver = Color.silver()
    assert silver.name == "silver"
    assert silver.r == 192
    assert silver.g == 192
    assert silver.b == 192
    assert silver.a == 255


def test_color_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    color = Color(128, 64, 192, 255, "purple")

    path = Path(__file__).resolve().parents[2] / "test_color.json"
    json_dump(color, path)
    loaded = json_load(path)

    assert isinstance(loaded, Color)
    assert loaded.r == color.r
    assert loaded.g == color.g
    assert loaded.b == color.b
    assert loaded.a == color.a
    assert loaded.name == color.name


def test_color_grey():
    grey = Color.grey()
    assert grey.name == "grey"
    assert grey.r == 128
    assert grey.g == 128
    assert grey.b == 128
    assert grey.a == 255
