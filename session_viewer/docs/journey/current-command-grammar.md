# Current command grammar

Registered names, static option counts and aliases were checked against the maintained [registry source](../../src/app/command/verbs/mod.rs) at `66ff54224`: 80 registered names, 117 static options and 15 aliases. The bare-form messages retain the historical parser observations at `2f96b35d225015cfcb9bfe5e8ec66565ef99aa28`; this source audit does not claim a new parser execution.

[Historical parser observations](current-command-registry.json). That snapshot includes the retired Wood constructor commands. Parsing constructs an action; it does not prove that it succeeds on a particular selection. [Behavior and exercises](current-viewer.md) records the earlier October 9 checkpoint.

| Command | Bare form (historical snapshot) | Static options | Aliases |
| --- | --- | ---: | --- |
| View | Choose a View direction or display setting | 29 |  |
| Report | Parses | 0 |  |
| Point | Point needs one point | 0 |  |
| Line | Line needs 2 points | 0 |  |
| Arrow | Arrow needs 2 points | 0 |  |
| Polyline | Polyline needs at least 2 points | 3 |  |
| Curve | Curve needs at least 2 points | 0 |  |
| Close | Parses | 0 |  |
| Trim | Parses | 0 |  |
| Extend | Parses | 1 |  |
| Explode | Parses | 0 |  |
| Move | Parses | 0 | m |
| Rotate | Parses | 3 | rot |
| Scale | Parses | 0 | s |
| Copy | Parses | 0 |  |
| Orient 3 Points | Parses | 0 | orient3pt |
| Split | Parses | 0 |  |
| Save | Parses | 0 |  |
| Open | Parses | 0 |  |
| Delete | Parses | 0 | del |
| Undo | Parses | 0 |  |
| Redo | Parses | 0 |  |
| Hide | Parses | 0 |  |
| Show | Parses | 0 |  |
| Fit | Parses | 0 |  |
| Escape | Parses | 0 | esc |
| Layers | Parses | 3 |  |
| Opacity | missing a number; try Opacity 0.5 | 4 |  |
| Arrowhead | wrong number of arguments for Arrowhead | 4 |  |
| Element Attributes | Parses | 2 |  |
| Element Interactions | Parses | 2 |  |
| Snap | Parses | 7 |  |
| Grid Snap | Parses | 2 |  |
| Construction Plane | Parses | 5 |  |
| Arctic | Parses | 2 |  |
| Outline | Parses | 2 |  |
| Object | Parses | 0 |  |
| Edge | Parses | 0 |  |
| Face | Parses | 0 |  |
| Controls | Parses | 0 |  |
| Select Lasso | Parses | 0 |  |
| Select By Name | type a name after Select By Name; try Select By Name beam | 0 |  |
| Select All | Parses | 0 |  |
| Select Small | wrong number of arguments for Select Small | 0 |  |
| Clipping Plane | Parses | 11 |  |
| Box | Parses | 2 | bo |
| Sphere | Parses | 2 |  |
| Cylinder | Parses | 2 |  |
| Cone | Parses | 2 |  |
| Pyramid | Parses | 2 |  |
| Torus | Parses | 2 |  |
| Block With Hole | Parses | 2 |  |
| Tetrahedron | Parses | 2 |  |
| Octahedron | Parses | 2 |  |
| Dodecahedron | Parses | 2 |  |
| Icosahedron | Parses | 2 |  |
| Quad Sphere | Parses | 1 |  |
| Capsule | Parses | 1 |  |
| Nurbs Curve Circle | Parses | 0 |  |
| Nurbs Curve Ellipse | Parses | 0 |  |
| Nurbs Curve Arc | Parses | 2 |  |
| Nurbs Curve Parabola | Parses | 0 |  |
| Loft | Parses | 2 |  |
| Extrude | Parses | 2 |  |
| Nurbs Surface Loft | Parses | 2 | nurbssurface_loft |
| Nurbs Surface Network | Parses | 0 | nurbssurface_network, nurbsnurbs_network, Nurbs Nurbs Network |
| Nurbs Surface Revolve | Parses | 0 | nurbssurface_revolve |
| Nurbs Surface 4 Points | Parses | 0 | nurbssurface_4_points |
| Nurbs Surface Sweep1 | Parses | 0 | nurbssurface_sweep1 |
| Nurbs Surface Sweep2 | Parses | 0 | nurbssurface_sweep2 |
| Text | Text takes 1 to 80 characters · Example: Text Hello world | 0 |  |
| Project To Plane | Parses | 5 |  |
| Measure Distance | Parses | 0 |  |
| Length | Parses | 0 |  |
| BoundingBox | Parses | 0 |  |
| Length BoundingBox | Parses | 0 |  |
| Area | Parses | 0 |  |
| Volume | Parses | 0 |  |
| Add Group | Parses | 0 |  |
| Add Edge | Parses | 0 |  |

The maintained viewer no longer registers the Wood-specific Element construction commands from earlier teaching checkpoints. Those construction examples remain historical course content. Kernel Element viewing remains supported, including the registered `Element Attributes` and `Element Interactions` controls.

`Close` completes an active Polyline or Curve; it does not clear the document. `View Grid Off` is absent: use the supported `?nogrid=1` startup setting. `Element Attributes` starts Off and `Element Interactions` starts On; both accept bare toggle or On/Off. Historical teaching-only forms are listed in [rendering differences](rendering-differences.md).
