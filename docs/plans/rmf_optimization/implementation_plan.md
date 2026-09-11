# Implementation Plan: O(1) RMF Query via Precomputation + SLERP

## Executive Summary

**Problem:** Current `perpendicular_frame_at()` is O(n) - 36x slower than `frame_at()`
**Solution:** Precompute frames at sparse samples, use quaternion SLERP for O(1) queries
**Expected Speedup:** 30-50x for repeated queries

## Algorithm Design

### Mathematical Foundation

The rotation minimizing frame (RMF) satisfies the ODE:
```
dr/ds · t(s) = 0   (r perpendicular to tangent)
```

This has no closed-form solution for general NURBS. However, we can:
1. Precompute exact RMF at N sample points using Double Reflection
2. Store as quaternions (rotation from world frame)
3. Interpolate between samples using SLERP (spherical linear interpolation)

### Quaternion Representation

Frame [r, s, t] as rotation matrix R, converted to quaternion q:
```
q = mat3_to_quaternion(R)
```

SLERP between frames:
```
q(u) = slerp(q_i, q_{i+1}, u)
     = q_i * (q_i^-1 * q_{i+1})^u
```

For small angles (which is our case):
```
q(u) ≈ normalize((1-u)*q_i + u*q_{i+1})  // NLERP, faster
```

### Error Analysis

For a curve with curvature κ and sample spacing Δt:
- Maximum angular error: θ_max ≈ κ * Δt² / 2
- For 20 samples on unit domain: Δt = 0.05
- Typical curvature κ < 10: θ_max < 0.0125 rad ≈ 0.7°

This is acceptable for graphics/CAD applications.

## Data Structures

### New Members in NurbsCurve class

```cpp
// In nurbscurve.h - private section
mutable bool m_rmf_cached = false;
mutable std::vector<double> m_rmf_params;      // Sample parameters
mutable std::vector<std::array<double,4>> m_rmf_quaternions;  // Quaternions [w,x,y,z]
mutable std::vector<Point> m_rmf_origins;      // Origins at each sample

// Helper methods
void invalidate_rmf_cache();
void ensure_rmf_cache() const;
static std::array<double,4> frame_to_quaternion(const Vector& r, const Vector& s, const Vector& t);
static void quaternion_to_frame(const std::array<double,4>& q, Vector& r, Vector& s, Vector& t);
static std::array<double,4> slerp(const std::array<double,4>& q0, const std::array<double,4>& q1, double u);
```

### Memory Overhead

Per curve: ~20 samples × (4 doubles quaternion + 3 doubles origin + 1 double param) = 160 doubles ≈ 1.3 KB

## Algorithm: perpendicular_frame_at_fast()

```cpp
bool NurbsCurve::perpendicular_frame_at_fast(double t, bool normalized,
    Point& origin, Vector& xaxis, Vector& yaxis, Vector& zaxis) const
{
    // 1. Validate input
    if (!is_valid()) return false;
    auto [t0, t1] = domain();
    double param = normalized ? t0 + t * (t1 - t0) : t;
    if (param < t0 || param > t1) return false;

    // 2. Ensure cache is populated (lazy init)
    ensure_rmf_cache();

    // 3. Binary search for bracketing samples - O(log N)
    auto it = std::lower_bound(m_rmf_params.begin(), m_rmf_params.end(), param);
    int idx = std::max(0, (int)(it - m_rmf_params.begin()) - 1);
    idx = std::min(idx, (int)m_rmf_params.size() - 2);

    // 4. Compute interpolation factor
    double u = (param - m_rmf_params[idx]) / (m_rmf_params[idx+1] - m_rmf_params[idx]);
    u = std::clamp(u, 0.0, 1.0);

    // 5. SLERP quaternions - O(1)
    auto q = slerp(m_rmf_quaternions[idx], m_rmf_quaternions[idx+1], u);

    // 6. Convert quaternion to frame vectors
    quaternion_to_frame(q, xaxis, yaxis, zaxis);

    // 7. Interpolate origin (simple linear)
    const Point& p0 = m_rmf_origins[idx];
    const Point& p1 = m_rmf_origins[idx+1];
    origin = Point(
        p0[0] + u * (p1[0] - p0[0]),
        p0[1] + u * (p1[1] - p0[1]),
        p0[2] + u * (p1[2] - p0[2])
    );

    // 8. For better origin accuracy, evaluate curve directly
    origin = point_at(param);

    return true;
}
```

## Algorithm: ensure_rmf_cache()

```cpp
void NurbsCurve::ensure_rmf_cache() const {
    if (m_rmf_cached) return;

    // Number of samples: adaptive based on curve complexity
    int num_samples = std::max(20, span_count() * 4);

    auto [t0, t1] = domain();
    double dt = (t1 - t0) / (num_samples - 1);

    m_rmf_params.resize(num_samples);
    m_rmf_quaternions.resize(num_samples);
    m_rmf_origins.resize(num_samples);

    // Compute RMF at each sample using existing Double Reflection
    for (int i = 0; i < num_samples; i++) {
        double t = t0 + i * dt;
        m_rmf_params[i] = t;

        Point o;
        Vector r, s, T;
        // Use original method for accurate computation
        perpendicular_frame_at_internal(t, false, o, r, s, T);

        m_rmf_origins[i] = o;
        m_rmf_quaternions[i] = frame_to_quaternion(r, s, T);
    }

    m_rmf_cached = true;
}
```

## Quaternion Math Helpers

```cpp
// Rotation matrix to quaternion (frame columns are [r, s, t])
std::array<double,4> NurbsCurve::frame_to_quaternion(
    const Vector& r, const Vector& s, const Vector& t)
{
    // Shepperd's method for numerical stability
    double trace = r[0] + s[1] + t[2];
    double w, x, y, z;

    if (trace > 0) {
        double S = std::sqrt(trace + 1.0) * 2;
        w = 0.25 * S;
        x = (s[2] - t[1]) / S;
        y = (t[0] - r[2]) / S;
        z = (r[1] - s[0]) / S;
    } else if (r[0] > s[1] && r[0] > t[2]) {
        double S = std::sqrt(1.0 + r[0] - s[1] - t[2]) * 2;
        w = (s[2] - t[1]) / S;
        x = 0.25 * S;
        y = (s[0] + r[1]) / S;
        z = (t[0] + r[2]) / S;
    } else if (s[1] > t[2]) {
        double S = std::sqrt(1.0 + s[1] - r[0] - t[2]) * 2;
        w = (t[0] - r[2]) / S;
        x = (s[0] + r[1]) / S;
        y = 0.25 * S;
        z = (t[1] + s[2]) / S;
    } else {
        double S = std::sqrt(1.0 + t[2] - r[0] - s[1]) * 2;
        w = (r[1] - s[0]) / S;
        x = (t[0] + r[2]) / S;
        y = (t[1] + s[2]) / S;
        z = 0.25 * S;
    }
    return {w, x, y, z};
}

// Quaternion to frame (returns [r, s, t])
void NurbsCurve::quaternion_to_frame(const std::array<double,4>& q,
    Vector& r, Vector& s, Vector& t)
{
    double w = q[0], x = q[1], y = q[2], z = q[3];

    r = Vector(1 - 2*(y*y + z*z), 2*(x*y + w*z), 2*(x*z - w*y));
    s = Vector(2*(x*y - w*z), 1 - 2*(x*x + z*z), 2*(y*z + w*x));
    t = Vector(2*(x*z + w*y), 2*(y*z - w*x), 1 - 2*(x*x + y*y));
}

// SLERP interpolation
std::array<double,4> NurbsCurve::slerp(
    const std::array<double,4>& q0,
    const std::array<double,4>& q1,
    double u)
{
    double dot = q0[0]*q1[0] + q0[1]*q1[1] + q0[2]*q1[2] + q0[3]*q1[3];

    // Handle opposite quaternions (same rotation)
    std::array<double,4> q1_adj = q1;
    if (dot < 0) {
        q1_adj = {-q1[0], -q1[1], -q1[2], -q1[3]};
        dot = -dot;
    }

    // For very close quaternions, use linear interpolation
    if (dot > 0.9995) {
        std::array<double,4> result = {
            q0[0] + u * (q1_adj[0] - q0[0]),
            q0[1] + u * (q1_adj[1] - q0[1]),
            q0[2] + u * (q1_adj[2] - q0[2]),
            q0[3] + u * (q1_adj[3] - q0[3])
        };
        double norm = std::sqrt(result[0]*result[0] + result[1]*result[1] +
                                result[2]*result[2] + result[3]*result[3]);
        return {result[0]/norm, result[1]/norm, result[2]/norm, result[3]/norm};
    }

    double theta = std::acos(dot);
    double sin_theta = std::sin(theta);
    double w0 = std::sin((1-u) * theta) / sin_theta;
    double w1 = std::sin(u * theta) / sin_theta;

    return {
        w0*q0[0] + w1*q1_adj[0],
        w0*q0[1] + w1*q1_adj[1],
        w0*q0[2] + w1*q1_adj[2],
        w0*q0[3] + w1*q1_adj[3]
    };
}
```

## Cache Invalidation

Must invalidate cache when curve geometry changes:

```cpp
void NurbsCurve::invalidate_rmf_cache() {
    m_rmf_cached = false;
    m_rmf_params.clear();
    m_rmf_quaternions.clear();
    m_rmf_origins.clear();
}
```

Call `invalidate_rmf_cache()` in:
- `set_cv()`, `set_cv_4d()`
- `set_knot()`
- `set_weight()`
- `transform()`
- `reverse()`
- `trim()`
- `insert_knot()`
- Any method that modifies curve geometry

## API Strategy

**Option A: Replace existing method**
- Rename current method to `perpendicular_frame_at_exact()`
- New cached version becomes `perpendicular_frame_at()`
- Transparent to users, immediate speedup

**Option B: Add new method**
- Keep `perpendicular_frame_at()` as-is (exact)
- Add `perpendicular_frame_at_fast()` (cached)
- Users choose based on accuracy needs

**Recommendation: Option A** - most applications don't need exact RMF

## Performance Expectations

| Operation | Current | New (Cached) |
|-----------|---------|--------------|
| First query | 116 µs | ~200 µs (build cache) |
| Subsequent queries | 116 µs | **2-3 µs** |
| Memory per curve | 0 | ~1.3 KB |

**Speedup for repeated queries: 40-60x**

## Testing Strategy

1. **Accuracy tests:**
   - Compare cached vs exact at 1000 random parameters
   - Max angular error should be < 1°
   - Max position error should be < tolerance

2. **Performance tests:**
   - Benchmark 10000 queries: before vs after
   - Verify O(1) query time

3. **Consistency tests:**
   - Verify frame orthonormality
   - Verify tangent alignment with curve

4. **Edge cases:**
   - Query at exact sample points
   - Query near domain boundaries
   - High-curvature curves

## Implementation Phases

### Phase 1: Add quaternion helpers (1 hour)
- Add quaternion math functions
- Unit tests for quaternion conversions

### Phase 2: Add cache infrastructure (1 hour)
- Add mutable cache members
- Implement ensure_rmf_cache()
- Implement invalidate_rmf_cache()

### Phase 3: Implement fast query (1 hour)
- Binary search + SLERP interpolation
- Handle edge cases

### Phase 4: Integration (1 hour)
- Add invalidation calls to modifying methods
- Update perpendicular_frame_at() to use cache
- Update tests

### Phase 5: Port to Rust and Python (2 hours)
- Same algorithm in session_rust
- Same algorithm in session_py

## References

1. Wang et al. "Computation of Rotation Minimizing Frames" (2008)
   https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/Computation-of-rotation-minimizing-frames.pdf

2. Shoemake "Animating Rotation with Quaternion Curves" (1985)
   - SLERP algorithm

3. Farouki "Rational rotation-minimizing frames" (2010)
   https://faculty.engineering.ucdavis.edu/farouki/wp-content/uploads/sites/51/2021/07/Rational-rotation-minimizing-frames.pdf
