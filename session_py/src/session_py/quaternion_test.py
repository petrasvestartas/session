import math
from .quaternion import Quaternion
from .vector import Vector

PI = math.pi


def approx_f32(a, b, tol=1e-5):
    return abs(a - b) < tol


def vectors_close(a, b, tol=1e-5):
    return (
        approx_f32(a.x, b.x, tol)
        and approx_f32(a.y, b.y, tol)
        and approx_f32(a.z, b.z, tol)
    )


def test_quaternion_identity():
    q = Quaternion.identity()
    assert q.s == 1.0
    assert q.v.x == 0.0
    assert q.v.y == 0.0
    assert q.v.z == 0.0


def test_quaternion_from_axis_angle_90deg_z():
    axis = Vector(0.0, 0.0, 1.0)
    angle = PI / 2.0
    q = Quaternion.from_axis_angle(axis, angle)

    assert approx_f32(q.s, math.cos(PI / 4.0))
    assert approx_f32(q.v.z, math.sin(PI / 4.0))


def test_quaternion_rotate_vector_90deg_z():
    axis = Vector(0.0, 0.0, 1.0)
    angle = PI / 2.0
    q = Quaternion.from_axis_angle(axis, angle)

    v = Vector(1.0, 0.0, 0.0)
    rotated = q.rotate_vector(v)

    expected = Vector(0.0, 1.0, 0.0)
    assert vectors_close(rotated, expected)


def test_quaternion_rotate_vector_180deg_z():
    axis = Vector(0.0, 0.0, 1.0)
    angle = PI
    q = Quaternion.from_axis_angle(axis, angle)

    v = Vector(1.0, 0.0, 0.0)
    rotated = q.rotate_vector(v)

    expected = Vector(-1.0, 0.0, 0.0)
    assert vectors_close(rotated, expected)


def test_quaternion_normalize():
    q = Quaternion.from_sv(2.0, 0.0, 0.0, 0.0)
    normalized = q.normalize()

    assert approx_f32(normalized.magnitude(), 1.0)
    assert approx_f32(normalized.s, 1.0)


def test_quaternion_multiplication():
    q1 = Quaternion.from_axis_angle(Vector(0.0, 0.0, 1.0), PI / 2.0)
    q2 = Quaternion.from_axis_angle(Vector(0.0, 0.0, 1.0), PI / 2.0)
    q_combined = q1 * q2

    v = Vector(1.0, 0.0, 0.0)
    rotated = q_combined.rotate_vector(v)

    expected = Vector(-1.0, 0.0, 0.0)
    assert vectors_close(rotated, expected)


def test_quaternion_identity_rotation():
    q = Quaternion.identity()
    v = Vector(1.0, 2.0, 3.0)
    rotated = q.rotate_vector(v)

    assert vectors_close(rotated, v)


def test_quaternion_conjugate():
    q = Quaternion.from_sv(0.5, 0.5, 0.5, 0.5)
    conj = q.conjugate()

    assert conj.s == 0.5
    assert conj.v.x == -0.5
    assert conj.v.y == -0.5
    assert conj.v.z == -0.5


def test_quaternion_magnitude():
    q = Quaternion.from_sv(1.0, 0.0, 0.0, 0.0)
    assert approx_f32(q.magnitude(), 1.0)

    q2 = Quaternion.from_sv(2.0, 0.0, 0.0, 0.0)
    assert approx_f32(q2.magnitude(), 2.0)


def test_quaternion_rotate_around_x():
    axis = Vector(1.0, 0.0, 0.0)
    angle = PI / 2.0
    q = Quaternion.from_axis_angle(axis, angle)

    v = Vector(0.0, 1.0, 0.0)
    rotated = q.rotate_vector(v)

    expected = Vector(0.0, 0.0, 1.0)
    assert vectors_close(rotated, expected)


def test_quaternion_rotate_around_y():
    axis = Vector(0.0, 1.0, 0.0)
    angle = PI / 2.0
    q = Quaternion.from_axis_angle(axis, angle)

    v = Vector(0.0, 0.0, 1.0)
    rotated = q.rotate_vector(v)

    expected = Vector(1.0, 0.0, 0.0)
    assert vectors_close(rotated, expected)


def test_quaternion_json_roundtrip():
    from pathlib import Path
    from session_py.encoders import json_dump, json_load

    q = Quaternion.from_axis_angle(Vector(0, 0, 1), 1.57)
    q.name = "test_quat"

    path = Path(__file__).resolve().parents[2] / "test_quaternion.json"
    json_dump(q, path)
    loaded = json_load(path)

    assert isinstance(loaded, Quaternion)
    assert abs(loaded.s - q.s) < 1e-6
    assert loaded.name == q.name
