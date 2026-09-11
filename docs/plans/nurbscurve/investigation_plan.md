# Investigation Plan: Find Exact Rhino Interpolation Algorithm

## Problem Statement
The previous "sqrt(10)" solution only worked for the symmetric 4-point test case. For arbitrary points, it fails.

## New Test Case

**Input Points (5 points):**
```
P0 = (1.504692, 4.600241, 0)
P1 = (1.348081, 3.190749, 0)
P2 = (3.673744, 3.691902, 0)
P3 = (3.971304, 2.227595, 0)
P4 = (2.09198, 1.851731, 0)
```

**Rhino's Subdivision Output:**
```
{1.504692, 4.600241, 0}
{1.293495, 3.78711, 0}
{1.557715, 3.127399, 0}
{2.343046, 3.419411, 0}
{3.127223, 3.720142, 0}
{3.88168, 3.504475, 0}
{4.092978, 2.704819, 0}
{3.736666, 1.980136, 0}
{2.929593, 1.783941, 0}
{2.09198, 1.851731, 0}
```

**Our Current Output (with non-uniform Bessel + normalized params + arc-length subdivision):**
```
1.50469, 4.60024, 0
1.26958, 3.7904, 0
1.55072, 3.16648, 0
2.3403, 3.45879, 0
3.13415, 3.74104, 0
3.87209, 3.48454, 0
4.05063, 2.67057, 0
3.74379, 1.91518, 0
2.93043, 1.75738, 0
2.09198, 1.85173, 0
```

**Differences at Point 1:**
- Our: (1.26958, 3.7904)
- Rhino: (1.293495, 3.78711)
- Diff: (0.024, -0.003) - Improved but still ~2% off in x

**Our Control Points:**
```
CV[0] = (1.50469, 4.60024, 0)
CV[1] = (1.2604, 3.91775, 0)
CV[2] = (0.90837, 2.32357, 0)
CV[3] = (3.92076, 4.7354, 0)
CV[4] = (4.24231, 1.64187, 0)
CV[5] = (3.14191, 1.69564, 0)
CV[6] = (2.09198, 1.85173, 0)
```

**Our Knot Vector:**
```
[0, 0, 0, 0.787, 2.107, 2.936, 4, 4, 4]
```

## Chord Lengths Analysis

Need to compute:
- d01 = dist(P0, P1)
- d12 = dist(P1, P2)
- d23 = dist(P2, P3)
- d34 = dist(P3, P4)

## Phase 1: Understand Current Error Source

### Task 1.1: Get Rhino's control points
Ask user for Rhino's control points for this test case to compare directly.

### Task 1.2: Compute chord lengths
Calculate actual chord lengths to understand parameterization.

### Task 1.3: Compare our knot vector with Rhino's
If possible, get Rhino's knot vector.

## Phase 2: Research Alternatives

### Task 2.1: Test without sqrt(10) - use standard 2h divisor
Revert to standard Bessel formula.

### Task 2.2: Test centripetal parameterization
Use sqrt(chord) instead of chord for parameterization.

### Task 2.3: Test uniform parameterization
Use [0, 1, 2, 3, 4] instead of chord-based.

### Task 2.4: Test different knot averaging
Use de Boor knot averaging formula instead of direct params.

## Phase 3: OpenNURBS Deep Dive

### Task 3.1: Search for exact interpolation source
Look for ON_NurbsCurve::CreateInterpolatedCurve or similar.

### Task 3.2: Examine RhinoCommon source
If available, look at Curve.CreateInterpolatedCurve.

## Completed Tests

| Test | Point 1 X | Diff from Rhino | Result |
|------|-----------|-----------------|--------|
| Standard 2h Bessel | 1.1713 | 0.122 | WORSE |
| Centripetal param | 1.20577 | 0.088 | WORSE |
| Uniform param | 1.12133 | 0.172 | WORST |
| **Non-uniform Bessel** | **1.26958** | **0.024** | **CLOSEST** |

## Current Status

The non-uniform Bessel formula with chord-length parameterization normalized to [0, n-1]
gives the closest results but still differs by ~2% from Rhino's output.

**To proceed, we need Rhino's control points and knot vector for this test case.**
