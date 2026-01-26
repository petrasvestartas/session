# Notes: RMF Optimization Research

## Sources

### Microsoft Research - Double Reflection Method (Wang et al. 2008)
- URL: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/Computation-of-rotation-minimizing-frames.pdf
- Key points:
  - Double reflection has **4th order global approximation error**
  - Projection method (Klok) and rotation method (Bloomenthal) have 2nd order error
  - All methods have similar per-frame computational cost
  - This is the current industry standard

### UC Davis - Farouki (Pythagorean-Hodograph Curves)
- URL: https://faculty.engineering.ucdavis.edu/farouki/wp-content/uploads/sites/51/2021/07/Rational-rotation-minimizing-frames.pdf
- Key points:
  - **Exact RMF exists ONLY for PH curves**
  - PH curves: r'(t)·r'(t) is a perfect square polynomial
  - Exact solution involves **transcendental (logarithmic) functions**
  - General NURBS do NOT have closed-form RMF

### ACM - Computation of RMF
- URL: https://dl.acm.org/doi/10.1145/1330511.1330513
- Key points:
  - RMF requires solving ODE: dr/dt parallel to tangent
  - Three approximation categories: discrete, curve approximation, numerical integration
  - **No closed-form for general curves** - mathematically impossible

### Quaternion SLERP
- URL: https://en.wikipedia.org/wiki/Slerp
- Key points:
  - Closed-form: Slerp(q1,q2;u) = q1(q1^-1 q2)^u
  - Constant angular velocity interpolation
  - Can interpolate between precomputed frames

## Synthesized Findings

### Mathematical Reality
1. **O(1) closed-form RMF does NOT exist** for general polynomial/NURBS curves
2. RMF is defined by an ODE - inherently path-dependent
3. The "rotation minimizing" property requires integration along the curve
4. Only Pythagorean-Hodograph curves have exact solutions (with transcendental terms)

### Why Path-Dependence is Fundamental
The RMF condition is:
```
dr/ds · t(s) = 0  (r stays perpendicular to tangent)
ds/ds · t(s) = 0  (s stays perpendicular to tangent)
```
This is a first-order ODE system. The solution at parameter t depends on the entire path from t0 to t.

### Viable Optimization Strategies

#### Strategy 1: Precompute + Quaternion SLERP Interpolation
- Compute RMF at N points during curve construction (one-time O(N) cost)
- Store as quaternions
- Query: SLERP interpolate between nearest precomputed frames (O(1))
- Accuracy: depends on N, typically 10-20 points sufficient for smooth curves

#### Strategy 2: Lazy Caching with Subdivision
- First query at t: compute and cache
- Subsequent queries: interpolate from cached values
- Good for repeated queries on same curve

#### Strategy 3: Reduce Step Count (Current Implementation)
- Current: 100 steps per unit parameter
- For most applications: 20-30 steps sufficient
- Trade accuracy for speed

#### Strategy 4: Parallel Transport via Quaternion Exponential
- Compute tangent derivative (available analytically for NURBS)
- Use quaternion exponential map for rotation
- Still O(n) but faster per-step

## Recommended Approach

**Hybrid Precomputation + SLERP:**

1. During `NurbsCurve::create()` or on first frame query:
   - Compute RMF at knot values + midpoints (sparse sampling)
   - Store as quaternion array

2. For `perpendicular_frame_at(t)`:
   - Find bracketing precomputed frames
   - SLERP interpolate (O(1))
   - Transform to output vectors

**Expected Performance:**
- Setup: O(k) where k = number of knots (one-time)
- Query: O(1) constant time
- Memory: 4 doubles per sample point (quaternion)

## Error Analysis

For a curve with k knots, storing k*2 frames:
- Max angular error: ~curvature * (knot_span/2)^2
- For typical curves: <0.1 degree error with 20 samples
- Can add adaptive refinement for high-curvature regions
