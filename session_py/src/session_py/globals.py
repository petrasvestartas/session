import math
import sys

# Mathematical constants
PI = math.pi
TO_DEGREES = 180.0 / math.pi
TO_RADIANS = math.pi / 180.0

# Tolerance values
ZERO_TOLERANCE = 1e-12
ANGLE = 1.0
# General tolerance (C++)
TOLERANCE = 1e-3

# Double precision limits
DOUBLE_MIN = sys.float_info.min
DOUBLE_MAX = sys.float_info.max
EPSILON = sys.float_info.epsilon
SQRT_EPSILON = math.sqrt(EPSILON)

# Scale factor
SCALE = 1000.0

# Metric prefixes
GIGA = 1e9
MEGA = 1e6
KILO = 1e3
MILLI = 1e-3
MICRO = 1e-6
NANO = 1e-9

# Base units
FORCE = 1
MASS = 1
LENGTH = 1
