import math
from .vector import Vector
from .tolerance import Tolerance, TO_DEGREES


def test_default_constructor():
    v = Vector()
    assert v[0] == 0 and v[1] == 0 and v[2] == 0


def test_constructor():
    v = Vector(0.57, -158.63, 180.890)
    assert v[0] == 0.57 and v[1] == -158.63 and v[2] == 180.890


def test_static_methods():
    v = Vector.x_axis()
    assert (v.x, v.y, v.z) == (1.0, 0.0, 0.0)
    v = Vector.y_axis()
    assert (v.x, v.y, v.z) == (0.0, 1.0, 0.0)
    v = Vector.z_axis()
    assert (v.x, v.y, v.z) == (0.0, 0.0, 1.0)


def test_from_start_and_end():
    start = Vector(8.7, 5.7, -1.87)
    end = Vector(1, 1.57, 2)
    v = Vector.from_start_and_end(start, end)
    assert abs(v[0] - (-7.7)) < Tolerance.ZERO_TOLERANCE
    assert abs(v[1] - (-4.13)) < Tolerance.ZERO_TOLERANCE
    assert abs(v[2] - 3.87) < Tolerance.ZERO_TOLERANCE


def test_operators():
    v1 = Vector(1, 2, 3)
    v2 = Vector(4, 5, 6)
    v3 = v1 + v2
    assert v3[0] == 5 and v3[1] == 7 and v3[2] == 9
    v3 = v1 - v2
    assert v3[0] == -3 and v3[1] == -3 and v3[2] == -3
    v3 = v1 * 2
    assert v3[0] == 2 and v3[1] == 4 and v3[2] == 6
    v3 = v1 / 2
    assert v3[0] == 0.5 and v3[1] == 1 and v3[2] == 1.5
    v3 = Vector(1, 2, 3)
    v3 += v2
    assert v3[0] == 5 and v3[1] == 7 and v3[2] == 9
    v3 -= v2
    assert v3[0] == 1 and v3[1] == 2 and v3[2] == 3
    v3 *= 2
    assert v3[0] == 2 and v3[1] == 4 and v3[2] == 6
    v3 /= 2
    assert v3[0] == 1 and v3[1] == 2 and v3[2] == 3


def test_reverse():
    v = Vector(1, 2, 3)
    v.reverse()
    assert v[0] == -1 and v[1] == -2 and v[2] == -3


def test_length():
    v = Vector(5.5697, -9.84, 1.587)
    magnitude = v.magnitude()
    assert magnitude == 11.4177811806848


def test_unitize():
    v = Vector(5.5697, -9.84, 1.587)
    normalized_vector = v.normalize()
    assert normalized_vector.magnitude() == 1
    v.normalize_self()
    assert v.magnitude() == 1


def test_projection():
    v = Vector(1, 1, 1)
    x_axis = Vector(1, 0, 0)
    y_axis = Vector(0, 1, 0)
    z_axis = Vector(0, 0, 1)

    projection, _, _, _ = v.projection(x_axis)
    assert projection[0] == 1 and projection[1] == 0 and projection[2] == 0
    projection, _, _, _ = v.projection(y_axis)
    assert projection[0] == 0 and projection[1] == 1 and projection[2] == 0
    projection, _, _, _ = v.projection(z_axis)
    assert projection[0] == 0 and projection[1] == 0 and projection[2] == 1


def test_is_parallel_to():
    v1 = Vector(0, 0, 1)
    v2 = Vector(0, 0, 2)
    v3 = Vector(0, 0, -1)
    v4 = Vector(0, 1, -1)
    assert v1.is_parallel_to(v2) == 1
    assert v1.is_parallel_to(v3) == -1
    assert v1.is_parallel_to(v4) == 0


def test_dot():
    v1 = Vector(1, 0, 0)
    v2 = Vector(0, 1, 0)
    v3 = Vector(-1, 0, 0)
    assert v1.dot(v2) == 0
    assert v1.dot(v3) == -1
    assert v1.dot(v1) == 1

    dot_product = v1.dot(v2)
    magnitudes = v1.magnitude() * v2.magnitude()
    if magnitudes > 0.0:
        cos_angle = dot_product / magnitudes
        angle = math.acos(cos_angle)
        angle_degrees = angle * TO_DEGREES
        assert angle_degrees == 90


def test_cross():
    v1 = Vector(1, 0, 0)
    v2 = Vector(0, 1, 0)
    v3 = v1.cross(v2)
    assert v3[0] == 0 and v3[1] == 0 and v3[2] == 1


def test_angle():
    v1 = Vector(1, 1, 0)
    v2 = Vector(0, 1, 0)
    angle = v1.angle(v2, False)
    assert abs(angle - 45) < Tolerance.ZERO_TOLERANCE
    v1 = Vector(-1, 1, 0)
    angle = v1.angle(v2, True)
    assert abs(angle - (-45)) < Tolerance.ZERO_TOLERANCE


def test_get_leveled_vector():
    v = Vector(1, 1, 1)
    scale = 1.0
    leveled_vector = v.get_leveled_vector(scale)
    assert (
        abs(leveled_vector.magnitude() - 4.1684325329666283) < Tolerance.ZERO_TOLERANCE
    )


def test_cosine_law():
    triangle_edge_length_a = 100
    triangle_edge_length_b = 150
    angle_in_degrees_between_edges = 115
    triangle_edge_length_c = Vector.cosine_law(
        triangle_edge_length_a,
        triangle_edge_length_b,
        angle_in_degrees_between_edges,
        True,
    )

    triangle_edge_length_c = round(triangle_edge_length_c * 100) / 100
    assert triangle_edge_length_c == 212.55


def test_sine_law_angle():
    triangle_edge_length_a = 212.55
    angle_in_degrees_in_front_of_a = 115
    triangle_edge_length_b = 150

    angle_in_degrees_in_front_of_b = Vector.sine_law_angle(
        triangle_edge_length_a, angle_in_degrees_in_front_of_a, triangle_edge_length_b
    )

    angle_in_degrees_in_front_of_b = round(angle_in_degrees_in_front_of_b * 100) / 100
    assert angle_in_degrees_in_front_of_b == 39.76


def test_sine_law_length():
    triangle_edge_length_a = 212.55
    angle_in_degrees_in_front_of_a = 115
    angle_in_degrees_in_front_of_b = 39.761714

    triangle_edge_length_b = Vector.sine_law_length(
        triangle_edge_length_a,
        angle_in_degrees_in_front_of_a,
        angle_in_degrees_in_front_of_b,
    )

    triangle_edge_length_b = round(triangle_edge_length_b * 100) / 100
    assert triangle_edge_length_b == 150


def test_angle_between_vector_xy_components():
    v = Vector(math.sqrt(3), 1, 0)
    angle = Vector.angle_between_vector_xy_components(v)
    assert round(angle * 100) / 100 == 30
    v = Vector(1, math.sqrt(3), 0)
    angle = Vector.angle_between_vector_xy_components(v)
    assert round(angle * 100) / 100 == 60


def test_sum_of_vectors():
    vectors = [Vector(1, 1, 1), Vector(2, 2, 2), Vector(3, 3, 3)]
    sum_vector = Vector.sum_of_vectors(vectors)
    assert sum_vector[0] == 6 and sum_vector[1] == 6 and sum_vector[2] == 6


def test_coordinate_direction_angles():
    v = Vector(35.4, 35.4, 86.6)
    alpha_beta_gamma = v.coordinate_direction_3angles(True)
    assert abs(alpha_beta_gamma[0] - 69.274204) < 1e-6
    assert abs(alpha_beta_gamma[1] - 69.274204) < 1e-6
    assert abs(alpha_beta_gamma[2] - 30.032058) < 1e-6

    v = Vector(1, 1, math.sqrt(2))
    phi_theta = v.coordinate_direction_2angles(True)
    assert abs(phi_theta[0] - 45) < 1e-6
    assert abs(phi_theta[1] - 45) < 1e-6


def test_vector_equality():
    v1 = Vector(1.0, 2.0, 3.0)
    v2 = Vector(1.0, 2.0, 3.0)
    v2.guid = v1.guid
    assert v1 == v2
    v3 = Vector(1.1, 2.0, 3.0)
    assert v1 != v3


def test_vector_constructor_values():
    v = Vector(0.57, -158.63, 180.890)
    assert (v[0], v[1], v[2]) == (0.57, -158.63, 180.890)


def test_vector_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    vec = Vector(1.5, 2.5, 3.5)
    vec.name = "test_vector"

    path = Path(__file__).resolve().parents[2] / "test_vector.json"
    json_dump(vec, path)
    loaded = json_load(path)

    assert isinstance(loaded, Vector)
    assert loaded.x == vec.x
    assert loaded.y == vec.y
    assert loaded.z == vec.z
    assert loaded.name == vec.name
