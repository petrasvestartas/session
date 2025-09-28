import math
import session_py.globals as globals


def test_globals_initial_values():
    assert globals.ZERO_TOLERANCE == 1e-12
    assert globals.SCALE == 1e6
    assert globals.PI == 3.141592653589793
    assert globals.TO_DEGREES == 180.0 / math.pi
    assert globals.TO_RADIANS == math.pi / 180.0
    assert globals.ANGLE == 0.11
    assert globals.TOLERANCE == 1e-3
    assert globals.MILLI == 1e-3
    assert globals.MICRO == 1e-6
    assert globals.NANO == 1e-9
    assert globals.KILO == 1e3
    assert globals.MEGA == 1e6
    assert globals.GIGA == 1e9
    assert globals.FORCE == 1
    assert globals.MASS == 1
    assert globals.LENGTH == 1


def test_globals_double_precision_limits():
    """Test double precision limits and finite values (equivalent to C++ IS_FINITE)."""
    # Test that DOUBLE_MIN and DOUBLE_MAX are finite
    assert math.isfinite(globals.DOUBLE_MIN)
    assert math.isfinite(globals.DOUBLE_MAX)

    # Test that DOUBLE_MIN is positive and very small
    assert globals.DOUBLE_MIN > 0
    assert globals.DOUBLE_MIN < 1e-300

    # Test that DOUBLE_MAX is very large
    assert globals.DOUBLE_MAX > 1e300

    # Test epsilon values
    assert globals.EPSILON > 0
    assert globals.EPSILON < 1e-15
    assert globals.SQRT_EPSILON == math.sqrt(globals.EPSILON)

    # Test finite behavior (equivalent to C++ IS_FINITE function)
    assert math.isfinite(1.0)
    assert math.isfinite(globals.DOUBLE_MIN)
    assert math.isfinite(globals.DOUBLE_MAX)
    assert not math.isfinite(float("inf"))
    assert not math.isfinite(float("-inf"))
    assert not math.isfinite(float("nan"))


def test_globals_mathematical_constants():
    """Test mathematical constants precision."""
    # Test PI precision
    assert abs(globals.PI - math.pi) < globals.EPSILON

    # Test conversion consistency
    assert abs(globals.TO_DEGREES * globals.TO_RADIANS - 1.0) < globals.EPSILON
    assert abs(90.0 * globals.TO_RADIANS - globals.PI / 2.0) < globals.EPSILON
    assert abs(globals.PI * globals.TO_DEGREES - 180.0) < globals.EPSILON


def test_globals_tolerance_hierarchy():
    """Test that tolerance values are in correct order."""
    assert globals.ZERO_TOLERANCE < globals.MICRO
    assert globals.MICRO < globals.MILLI
    assert globals.MILLI <= globals.TOLERANCE  # They are equal (both 1e-3)
    assert globals.EPSILON < globals.SQRT_EPSILON
    assert globals.ZERO_TOLERANCE < globals.SQRT_EPSILON


def test_globals_scale_relationships():
    """Test scale factor relationships."""
    assert globals.SCALE == globals.MEGA
    assert globals.KILO * globals.KILO == globals.MEGA
    assert globals.MEGA * globals.KILO == globals.GIGA
    assert globals.MILLI * globals.KILO == 1.0
    assert globals.MICRO * globals.MEGA == 1.0


def test_globals_is_finite_function():
    """Test is_finite function (equivalent to C++ IS_FINITE)."""
    # Test finite values
    assert globals.is_finite(0.0)
    assert globals.is_finite(1.0)
    assert globals.is_finite(-1.0)
    assert globals.is_finite(globals.DOUBLE_MIN)
    assert globals.is_finite(globals.DOUBLE_MAX)
    assert globals.is_finite(globals.PI)
    assert globals.is_finite(globals.EPSILON)

    # Test infinite values
    assert not globals.is_finite(float("inf"))
    assert not globals.is_finite(float("-inf"))
    assert not globals.is_finite(float("nan"))

    # Test edge cases
    assert globals.is_finite(1e-308)  # Very small but finite
    assert globals.is_finite(1e308)  # Very large but finite


def test_globals_modification():
    globals.ZERO_TOLERANCE = 1e-15
    globals.SCALE = 2000.0
    assert globals.ZERO_TOLERANCE == 1e-15
    assert globals.SCALE == 2000.0


def test_globals_persistence():
    import session_py.globals as globals_again

    assert globals_again.ZERO_TOLERANCE == 1e-15
    assert globals_again.SCALE == 2000.0
    globals.ZERO_TOLERANCE = 1e-12  # reset to default for the other tests
    globals.SCALE = 1e6  # reset to default for the other tests
