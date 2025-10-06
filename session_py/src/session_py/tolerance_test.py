from session_py.tolerance import TOL, Tolerance
from session_py.point import Point


def test_tolerance_default_tolerance():
    assert TOL.precision == Tolerance.PRECISION
    assert TOL.precision == 3


def test_tolerance_format_number():
    assert TOL.format_number(0, precision=3) == "0.000"
    assert TOL.format_number(0.5, precision=3) == "0.500"
    assert TOL.format_number(float(0), precision=3) == "0.000"


def test_tolerance_format_number_with_default_precision():
    assert TOL.format_number(0) == "0.000"
    assert TOL.format_number(0.5) == "0.500"
    assert TOL.format_number(float(0)) == "0.000"


def test_tolerance_format_point():
    point = Point(0, 0, 0)
    assert str(point) == "Point(x=0.000, y=0.000, z=0.000)"


def test_tolerance_change_values():
    # Create a mutable tolerance instance
    tol = Tolerance("M")

    # Test default values
    assert tol.precision == Tolerance.PRECISION
    assert tol.absolute == Tolerance.ABSOLUTE

    # Change precision and test formatting
    tol.precision = 2
    assert tol.precision == 2
    assert tol.format_number(1.23456) == "1.23"

    # Change absolute tolerance and test zero checking
    tol.absolute = 1e-5
    assert tol.absolute == 1e-5
    assert tol.is_zero(1e-6)  # Should be true with new tolerance
    assert not tol.is_zero(1e-4)  # Should be false

    # Reset to defaults and verify
    tol.reset()
    assert tol.precision == Tolerance.PRECISION
    assert tol.absolute == Tolerance.ABSOLUTE
    assert tol.format_number(1.23456) == "1.235"  # Back to 3 decimal places

    # Verify absolute tolerance is back to default
    assert not tol.is_zero(1e-6)  # Should be false with default tolerance


def test_tolerance_is_zero():
    tol = Tolerance()
    assert tol.is_zero(1e-10)
    assert not tol.is_zero(1e-5)


def test_tolerance_is_close():
    tol = Tolerance()
    assert not tol.is_close(1.0, 1.0 + 1e-5)
    assert tol.is_close(1.0, 1.0 + 1e-6)
    assert tol.is_close(0.0, 0.0 + 1e-9)


def test_tolerance_geometric_key():
    tol = Tolerance()
    assert tol.geometric_key([1.0, 2.0, 3.0]) == "1.000,2.000,3.000"
    assert (
        tol.geometric_key([1.05725, 2.0195, 3.001], precision=3) == "1.057,2.019,3.001"
    )
    assert tol.geometric_key([1.0, 2.0, 3.0], precision=-1) == "1,2,3"


def test_tolerance_is_positive():
    tol = Tolerance()
    assert tol.is_positive(1e-7)
    assert not tol.is_positive(1e-10)


def test_tolerance_is_negative():
    tol = Tolerance()
    assert tol.is_negative(-1e-7)
    assert not tol.is_negative(-1e-10)


def test_tolerance_is_between():
    tol = Tolerance()
    assert tol.is_between(0.5, 0.0, 1.0)
    assert not tol.is_between(1.5, 0.0, 1.0)
