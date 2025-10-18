import math


# Mathematical constants
PI = math.pi
TO_DEGREES = 180.0 / math.pi
TO_RADIANS = math.pi / 180.0

# Scale factor
SCALE = 1e6


class Tolerance:
    """Tolerance settings for geometric operations.

    Parameters
    ----------
    unit : {"M", "MM"}, optional
        The unit of the tolerance settings.
    name : str, optional
        The name of the tolerance settings.
    """

    _instance = None
    _is_inited = False

    SUPPORTED_UNITS = ["M", "MM"]

    # Default tolerance values (f32 only)
    ABSOLUTE = 1e-9
    RELATIVE = 1e-6
    ANGULAR = 1e-6
    APPROXIMATION = 1e-3
    PRECISION = 3
    LINEARDEFLECTION = 1e-3
    ANGULARDEFLECTION = 1e-1
    ANGLE_TOLERANCE_DEGREES = 0.11
    ZERO_TOLERANCE = 1e-12

    def __new__(cls, *args, **kwargs):
        if not cls._instance:
            cls._instance = object.__new__(cls)
            cls._is_inited = False
        return cls._instance

    def __init__(
        self,
        unit="M",
        absolute=None,
        relative=None,
        angular=None,
        approximation=None,
        precision=None,
        lineardeflection=None,
        angulardeflection=None,
        name=None,
    ):
        if not self._is_inited:
            self._unit = None
            self._absolute = None
            self._relative = None
            self._angular = None
            self._approximation = None
            self._precision = None
            self._lineardeflection = None
            self._angulardeflection = None

        self._is_inited = True

        if unit is not None:
            self.unit = unit
        if absolute is not None:
            self.absolute = absolute
        if relative is not None:
            self.relative = relative
        if angular is not None:
            self.angular = angular
        if approximation is not None:
            self.approximation = approximation
        if precision is not None:
            self.precision = precision
        if lineardeflection is not None:
            self.lineardeflection = lineardeflection
        if angulardeflection is not None:
            self.angulardeflection = angulardeflection

    def __repr__(self):
        return f"Tolerance(unit='{self.unit}', absolute={self.absolute}, relative={self.relative}, angular={self.angular}, approximation={self.approximation}, precision={self.precision}, lineardeflection={self.lineardeflection}, angulardeflection={self.angulardeflection})"

    def reset(self):
        """Reset all precision settings to their default values."""
        self._absolute = None
        self._relative = None
        self._angular = None
        self._approximation = None
        self._precision = None
        self._lineardeflection = None
        self._angulardeflection = None

    @property
    def unit(self):
        return self._unit or "M"

    @unit.setter
    def unit(self, value):
        if value not in ["M", "MM"]:
            raise ValueError(f"Invalid unit: {value}")
        self._unit = value

    @property
    def units(self):
        return self._unit or "M"

    @units.setter
    def units(self, value):
        if value not in ["M", "MM"]:
            raise ValueError(f"Invalid unit: {value}")
        self._unit = value

    @property
    def absolute(self):
        return self._absolute if self._absolute is not None else self.ABSOLUTE

    @absolute.setter
    def absolute(self, value):
        self._absolute = value

    @property
    def relative(self):
        return self._relative if self._relative is not None else self.RELATIVE

    @relative.setter
    def relative(self, value):
        self._relative = value

    @property
    def angular(self):
        return self._angular if self._angular is not None else self.ANGULAR

    @angular.setter
    def angular(self, value):
        self._angular = value

    @property
    def approximation(self):
        return (
            self._approximation
            if self._approximation is not None
            else self.APPROXIMATION
        )

    @approximation.setter
    def approximation(self, value):
        self._approximation = value

    @property
    def precision(self):
        return self._precision if self._precision is not None else self.PRECISION

    @precision.setter
    def precision(self, value):
        if value == 0:
            raise ValueError("Precision cannot be zero.")
        self._precision = value

    @property
    def lineardeflection(self):
        return (
            self._lineardeflection
            if self._lineardeflection is not None
            else self.LINEARDEFLECTION
        )

    @lineardeflection.setter
    def lineardeflection(self, value):
        self._lineardeflection = value

    @property
    def angulardeflection(self):
        return (
            self._angulardeflection
            if self._angulardeflection is not None
            else self.ANGULARDEFLECTION
        )

    @angulardeflection.setter
    def angulardeflection(self, value):
        self._angulardeflection = value

    def tolerance(self, truevalue, rtol, atol):
        """Compute the tolerance for a comparison."""
        return rtol * abs(truevalue) + atol

    def compare(self, a, b, rtol, atol):
        """Compare two values."""
        return abs(a - b) <= self.tolerance(b, rtol, atol)

    def is_zero(self, a, tol=None):
        """Check if a value is close enough to zero to be considered zero."""
        tol = tol if tol is not None else self.absolute
        return abs(a) <= tol

    def is_positive(self, a, tol=None):
        """Check if a value can be considered a strictly positive number."""
        tol = tol if tol is not None else self.absolute
        return a > tol

    def is_negative(self, a, tol=None):
        """Check if a value can be considered a strictly negative number."""
        tol = tol if tol is not None else self.absolute
        return a < -tol

    def is_between(self, value, minval, maxval, atol=None):
        """Check if a value is between two other values."""
        atol = atol if atol is not None else self.absolute
        return minval - atol <= value <= maxval + atol

    def is_close(self, a, b, rtol=None, atol=None):
        """Check if two values are close enough to be considered equal."""
        rtol = rtol if rtol is not None else self.relative
        atol = atol if atol is not None else self.absolute
        return self.compare(a, b, rtol, atol)

    def is_allclose(self, A, B, rtol=None, atol=None):
        """Check if two lists of values are element-wise close enough to be considered equal."""
        rtol = rtol if rtol is not None else self.relative
        atol = atol if atol is not None else self.absolute
        return all(
            (
                self.is_allclose(a, b, rtol, atol)
                if hasattr(a, "__iter__")
                else self.compare(a, b, rtol, atol)
            )
            for a, b in zip(A, B)
        )

    def is_angle_zero(self, a, tol=None):
        """Check if an angle is close enough to zero to be considered zero."""
        tol = tol if tol is not None else self.angular
        return abs(a) <= tol

    def is_angles_close(self, a, b, tol=None):
        """Check if two angles are close enough to be considered equal."""
        tol = tol if tol is not None else self.angular
        return abs(a - b) <= tol

    def geometric_key(self, xyz, precision=None, sanitize=True):
        """Compute the geometric key of a point."""
        x, y, z = xyz
        if not precision:
            precision = self.precision

        if precision == 0:
            raise ValueError("Precision cannot be zero.")

        if precision == -1:
            return f"{int(x)},{int(y)},{int(z)}"

        if precision < -1:
            precision = -precision - 1
            factor = 10**precision
            return f"{int(round(x / factor) * factor)},{int(round(y / factor) * factor)},{int(round(z / factor) * factor)}"

        if sanitize:
            minzero = f"-{0.0:.{precision}f}"
            if f"{x:.{precision}f}" == minzero:
                x = 0.0
            if f"{y:.{precision}f}" == minzero:
                y = 0.0
            if f"{z:.{precision}f}" == minzero:
                z = 0.0

        return f"{x:.{precision}f},{y:.{precision}f},{z:.{precision}f}"

    def geometric_key_xy(self, xy, precision=None, sanitize=True):
        """Compute the geometric key of a point in the XY plane."""
        x, y = xy
        if not precision:
            precision = self.precision

        if precision == 0:
            raise ValueError("Precision cannot be zero.")

        if precision == -1:
            return f"{int(x)},{int(y)}"

        if precision < -1:
            precision = -precision - 1
            factor = 10**precision
            return (
                f"{int(round(x / factor) * factor)},{int(round(y / factor) * factor)}"
            )

        if sanitize:
            minzero = f"-{0.0:.{precision}f}"
            if f"{x:.{precision}f}" == minzero:
                x = 0.0
            if f"{y:.{precision}f}" == minzero:
                y = 0.0

        return f"{x:.{precision}f},{y:.{precision}f}"

    def format_number(self, number, precision=None):
        """Format a number as a string."""
        if not precision:
            precision = self.precision

        if precision == 0:
            raise ValueError("Precision cannot be zero.")

        if precision == -1:
            return f"{int(round(number))}"

        if precision < -1:
            precision = -precision - 1
            factor = 10**precision
            return f"{int(round(number / factor) * factor)}"

        return f"{number:.{precision}f}"

    def precision_from_tolerance(self, tol=None):
        """Compute the precision from a given tolerance."""
        tol = tol or self.absolute
        if tol < 1:
            import decimal

            return abs(int(decimal.Decimal(str(tol)).as_tuple().exponent))
        raise NotImplementedError


def is_finite(x):
    """Test if a number is finite (equivalent to C++ IS_FINITE function)."""
    return math.isfinite(x)


# Global tolerance instance
TOL = Tolerance()
