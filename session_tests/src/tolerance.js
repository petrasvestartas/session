/**
 * Tolerance settings for geometric operations.
 * Mirrors the Tolerance class from Python, C++, and Rust implementations.
 */

// Mathematical constants
export const PI = Math.PI;
export const TO_DEGREES = 180.0 / Math.PI;
export const TO_RADIANS = Math.PI / 180.0;

// Scale factor
export const SCALE = 1e6;

export class Tolerance {
  // Default tolerance values
  static ABSOLUTE = 1e-9;
  static RELATIVE = 1e-6;
  static ANGULAR = 1e-6;
  static APPROXIMATION = 1e-3;
  static PRECISION = 3;
  static LINEARDEFLECTION = 1e-3;
  static ANGULARDEFLECTION = 1e-1;
  static ANGLE_TOLERANCE_DEGREES = 0.11;
  static ZERO_TOLERANCE = 1e-12;
  static ROUNDING = 6;

  constructor(unit = "M") {
    this._unit = unit;
    this._absolute = null;
    this._relative = null;
    this._angular = null;
    this._approximation = null;
    this._precision = null;
    this._lineardeflection = null;
    this._angulardeflection = null;
  }

  reset() {
    this._absolute = null;
    this._relative = null;
    this._angular = null;
    this._approximation = null;
    this._precision = null;
    this._lineardeflection = null;
    this._angulardeflection = null;
  }

  // Getters
  get unit() { return this._unit || "M"; }
  get absolute() { return this._absolute !== null ? this._absolute : Tolerance.ABSOLUTE; }
  get relative() { return this._relative !== null ? this._relative : Tolerance.RELATIVE; }
  get angular() { return this._angular !== null ? this._angular : Tolerance.ANGULAR; }
  get approximation() { return this._approximation !== null ? this._approximation : Tolerance.APPROXIMATION; }
  get precision() { return this._precision !== null ? this._precision : Tolerance.PRECISION; }
  get lineardeflection() { return this._lineardeflection !== null ? this._lineardeflection : Tolerance.LINEARDEFLECTION; }
  get angulardeflection() { return this._angulardeflection !== null ? this._angulardeflection : Tolerance.ANGULARDEFLECTION; }

  // Setters
  set unit(value) {
    if (value !== "M" && value !== "MM") {
      throw new Error(`Invalid unit: ${value}`);
    }
    this._unit = value;
  }
  set absolute(value) { this._absolute = value; }
  set relative(value) { this._relative = value; }
  set angular(value) { this._angular = value; }
  set approximation(value) { this._approximation = value; }
  set precision(value) {
    if (value === 0) {
      throw new Error("Precision cannot be zero.");
    }
    this._precision = value;
  }
  set lineardeflection(value) { this._lineardeflection = value; }
  set angulardeflection(value) { this._angulardeflection = value; }

  // Tolerance operations
  tolerance(truevalue, rtol, atol) {
    return rtol * Math.abs(truevalue) + atol;
  }

  compare(a, b, rtol, atol) {
    return Math.abs(a - b) <= this.tolerance(b, rtol, atol);
  }

  isZero(a) {
    return Math.abs(a) <= this.absolute;
  }

  isPositive(a) {
    return a > this.absolute;
  }

  isNegative(a) {
    return a < -this.absolute;
  }

  isBetween(value, minval, maxval) {
    const atol = this.absolute;
    return minval - atol <= value && value <= maxval + atol;
  }

  isClose(a, b) {
    return this.compare(a, b, this.relative, this.absolute);
  }

  isAllClose(A, B) {
    if (A.length !== B.length) return false;
    const rtol = this.relative;
    const atol = this.absolute;
    return A.every((a, i) => {
      const b = B[i];
      if (Array.isArray(a) && Array.isArray(b)) {
        return this.isAllClose(a, b);
      }
      return this.compare(a, b, rtol, atol);
    });
  }

  isAngleZero(a) {
    return Math.abs(a) <= this.angular;
  }

  isAnglesClose(a, b) {
    return Math.abs(a - b) <= this.angular;
  }

  // Formatting
  formatNumber(number, precision = null) {
    const prec = precision !== null ? precision : this.precision;

    if (prec === 0) {
      throw new Error("Precision cannot be zero.");
    }

    if (prec === -1) {
      return String(Math.round(number));
    }

    if (prec < -1) {
      const p = -prec - 1;
      const factor = Math.pow(10, p);
      return String(Math.round(number / factor) * factor);
    }

    return number.toFixed(prec);
  }

  geometricKey(x, y, z, precision = null) {
    const prec = precision !== null ? precision : this.precision;

    if (prec === 0) {
      throw new Error("Precision cannot be zero.");
    }

    if (prec === -1) {
      return `${Math.floor(x)},${Math.floor(y)},${Math.floor(z)}`;
    }

    if (prec < -1) {
      const p = -prec - 1;
      const factor = Math.pow(10, p);
      return `${Math.round(x / factor) * factor},${Math.round(y / factor) * factor},${Math.round(z / factor) * factor}`;
    }

    // Handle -0.000 case
    const fx = x.toFixed(prec);
    const fy = y.toFixed(prec);
    const fz = z.toFixed(prec);
    const minzero = (-0.0).toFixed(prec);

    const rx = fx === minzero ? (0.0).toFixed(prec) : fx;
    const ry = fy === minzero ? (0.0).toFixed(prec) : fy;
    const rz = fz === minzero ? (0.0).toFixed(prec) : fz;

    return `${rx},${ry},${rz}`;
  }

  geometricKeyXY(x, y, precision = null) {
    const prec = precision !== null ? precision : this.precision;

    if (prec === 0) {
      throw new Error("Precision cannot be zero.");
    }

    if (prec === -1) {
      return `${Math.floor(x)},${Math.floor(y)}`;
    }

    if (prec < -1) {
      const p = -prec - 1;
      const factor = Math.pow(10, p);
      return `${Math.round(x / factor) * factor},${Math.round(y / factor) * factor}`;
    }

    const fx = x.toFixed(prec);
    const fy = y.toFixed(prec);
    const minzero = (-0.0).toFixed(prec);

    const rx = fx === minzero ? (0.0).toFixed(prec) : fx;
    const ry = fy === minzero ? (0.0).toFixed(prec) : fy;

    return `${rx},${ry}`;
  }

  static roundTo(value, ndigits) {
    const factor = Math.pow(10, ndigits);
    return Math.round(value * factor) / factor;
  }
}

// Utility function
export function isFinite(x) {
  return Number.isFinite(x);
}

// Global tolerance instance
export const TOLERANCE = new Tolerance();

// Default export for convenience
export default TOLERANCE;
