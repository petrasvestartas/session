# STEP File Format Reference

## Reference Repos
| Repo | Language | What to study |
|---|---|---|
| https://github.com/AlexFemec/STEP-file-parser | C# | Simple hand-written parser architecture, entity tree |
| https://github.com/Formlabs/foxtrot | Rust/nom | Complex entity parsing, high-perf lexer, PCURVE handling |
| https://github.com/stepcode/stepcode | C++ | EXPRESS schemas, entity attribute definitions |
| https://github.com/Open-Cascade-SAS/OCCT | C++ | Full NURBS read/write, BRep assembly from STEP |
| https://www.steptools.com/stds/stp_aim/html/ | Docs | Authoritative entity attribute specs |

---

## File Structure

```
ISO-10303-21;
HEADER;
  FILE_DESCRIPTION(('description'),'2;1');
  FILE_NAME('name','timestamp',...);
  FILE_SCHEMA(('AP214_AUTO_DESIGN'));
ENDSEC;
DATA;
#1 = CARTESIAN_POINT('',(0.,0.,0.));
#2 = ...
ENDSEC;
END-ISO-10303-21;
```

## Simple Entity Format
```
#<id> = TYPE_NAME(param1, param2, ...);
```
- References: `#123`
- Numbers: `1.0` or `3`
- Strings: `'text'`
- Enums: `.UNSPECIFIED.` `.TRUE.` `.FALSE.` `.U.` `.T.` `.F.`
- Null/unset: `$`
- Wildcard: `*`
- Lists: `(item, item, ...)`

## Complex Entity Format (for Rational NURBS)
Multiple inheritance via single entity with multiple type blocks:
```
#10=(BOUNDED_CURVE()
     B_SPLINE_CURVE(3,(#11,#12,#13,#14),.UNSPECIFIED.,.F.,.F.)
     B_SPLINE_CURVE_WITH_KNOTS((4,4),(0.,1.),.UNSPECIFIED.)
     RATIONAL_B_SPLINE_CURVE((1.,0.707,1.,0.707)));
```

---

## Knot Format

STEP stores knots as (multiplicities, values):
```
knot_multiplicities: (4,3,4)
knots: (0.0, 0.5, 1.0)
```

Our NurbsCurve/NurbsSurface uses flat knot vector:
```
{0,0,0,0, 0.5,0.5,0.5, 1,1,1,1}
```

**expand_knots(vals, mults) → flat:**
```cpp
for each (v, m): repeat v m times
```

**compress_knots(flat) → (vals, mults):**
```cpp
run-length encode: group consecutive equal values
```

---

## Entity → Session Type Mapping

| STEP Entity | Session Class | Notes |
|---|---|---|
| `CARTESIAN_POINT('',( x,y,z))` | `Point` | coordinates list |
| `DIRECTION('',(x,y,z))` | `Vector` | direction_ratios |
| `LINE('',#pt,#dir)` | `Line` | point + direction |
| `POLYLINE('',(#pt,#pt,...))` | `Polyline` | list of point refs |
| `B_SPLINE_CURVE_WITH_KNOTS` | `NurbsCurve` (m_is_rat=0) | degree, cvs, mults, knots |
| `B_SPLINE_CURVE_WITH_KNOTS` + `RATIONAL_B_SPLINE_CURVE` | `NurbsCurve` (m_is_rat=1) | same + weights list |
| `B_SPLINE_SURFACE_WITH_KNOTS` | `NurbsSurface` (m_is_rat=0) | u/v degree, 2D cv grid |
| `B_SPLINE_SURFACE_WITH_KNOTS` + `RATIONAL_B_SPLINE_SURFACE` | `NurbsSurface` (m_is_rat=1) | same + 2D weight grid |
| `PLANE('',#axis2)` | `NurbsSurface` degree-1 | convert axis to NURBS plane |
| `CYLINDRICAL_SURFACE('',#axis2,r)` | `NurbsSurface` rational deg-2×1 | exact circle NURBS |
| `ADVANCED_FACE('',(#bounds),#surf,.T.)` | `NurbsSurfaceTrimmed` | surf + outer/inner loops |
| `MANIFOLD_SOLID_BREP('',#shell)` | `BRep` | full topology assembly |

---

## B_SPLINE_CURVE_WITH_KNOTS Attributes

```
B_SPLINE_CURVE_WITH_KNOTS(
  name,              -- STRING
  degree,            -- INTEGER (= m_order - 1)
  control_points,    -- LIST of #CARTESIAN_POINT
  curve_form,        -- .UNSPECIFIED. | .POLYLINE_FORM. | ...
  closed_curve,      -- .T. or .F.
  self_intersect,    -- .U. (unknown) or .T./.F.
  knot_multiplicities, -- LIST of INTEGER
  knots,             -- LIST of REAL
  knot_spec          -- .UNIFORM_KNOTS. | .PIECEWISE_BEZIER_KNOTS. | .UNSPECIFIED.
);
```

## B_SPLINE_SURFACE_WITH_KNOTS Attributes

```
B_SPLINE_SURFACE_WITH_KNOTS(
  name,
  u_degree,          -- INTEGER
  v_degree,          -- INTEGER
  control_points,    -- LIST of LIST of #CARTESIAN_POINT (outer=u, inner=v)
  surface_form,      -- .UNSPECIFIED. | .PLANE_SURF. | ...
  u_closed,          -- .T./.F.
  v_closed,          -- .T./.F.
  self_intersect,
  u_knot_multiplicities, -- LIST of INTEGER
  v_knot_multiplicities,
  u_knots,           -- LIST of REAL
  v_knots,
  knot_spec
);
```

## BRep Topology Chain

```
MANIFOLD_SOLID_BREP → CLOSED_SHELL → ADVANCED_FACE[]
ADVANCED_FACE → FACE_OUTER_BOUND/FACE_BOUND[] + surface_ref + sense
FACE_OUTER_BOUND → EDGE_LOOP
EDGE_LOOP → ORIENTED_EDGE[]
ORIENTED_EDGE → EDGE_CURVE + orientation
EDGE_CURVE → VERTEX_POINT(start) + VERTEX_POINT(end) + curve_3d
VERTEX_POINT → CARTESIAN_POINT
```

For 2D UV trim curves on each face:
```
EDGE_CURVE.curve → SURFACE_CURVE
SURFACE_CURVE → curve_3d + PCURVE[]
PCURVE → surface_ref + DEFINITIONAL_REPRESENTATION → B_SPLINE_CURVE (in UV space)
```

---

## NurbsCurve ↔ STEP Field Map

| NurbsCurve field | STEP field |
|---|---|
| `m_order - 1` | `degree` |
| `m_cv_count` | `len(control_points)` |
| `m_is_rat` | complex entity has `RATIONAL_B_SPLINE_CURVE` |
| `m_nurbsknot` (flat) | expand from `(knot_multiplicities, knots)` |
| `m_cv[i*stride .. +3]` (xyz) | `CARTESIAN_POINT` coords |
| `m_cv[i*stride + 3]` (w, if rational) | `RATIONAL_B_SPLINE_CURVE.weights_data[i]` |

## NurbsSurface ↔ STEP Field Map

| NurbsSurface field | STEP field |
|---|---|
| `m_order[0] - 1`, `m_order[1] - 1` | `u_degree`, `v_degree` |
| `m_cv_count[0]`, `m_cv_count[1]` | outer list len, inner list len |
| `m_is_rat` | complex entity has `RATIONAL_B_SPLINE_SURFACE` |
| `m_nurbsknot[0]` (flat) | expand from `(u_knot_multiplicities, u_knots)` |
| `m_nurbsknot[1]` (flat) | expand from `(v_knot_multiplicities, v_knots)` |
| `m_cv[u*stride[0] + v*stride[1] .. +dim]` | `control_points[u][v]` |
| weight grid (if rational) | `RATIONAL_B_SPLINE_SURFACE.weights_data[u][v]` |

---

## Sample .step Files Available

```
session_data/elements/schoring_foot_0.step    # 1329 entities, 38 ADVANCED_FACE, 3 MANIFOLD_SOLID_BREP
session_data/elements/schoring_foot_1.step
session_data/elements/schoring_head_0.step
session_data/elements/schoring_head_1.step
session_data/elements/schoring_body_start_0.step
session_data/elements/schoring_body_start_1.step
session_data/elements/schoring_body_start_2.step
session_data/elements/schoring_body_start_3.step
session_data/elements/schoring_body_end_1.step
```
