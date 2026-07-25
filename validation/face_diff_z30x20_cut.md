# face_diff z30x20 cut

A: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\chair0.stp`  
B: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\rot\B_z30x20.step`  
ours: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\rot\res_z30x20_cut.step`  
eps 0.08454, 9 samples/face, grid 11

| side | solids | shells | faces | volume | OCCT valid |
|---|---|---|---|---|---|
| TRUE (OCCT cut) | 1 | - | 32 | 54.258044 | 1 |
| OURS | 1 | 1 | 34 | 54.229700 | 1 |

## Defect tickets (7)

| # | kind | face | srf | area | centroid | detail |
|---|---|---|---|---|---|---|
| 0 | COUNT | - | - | - | - | our faces 34 vs OCCT 32 (+2) |
| 1 | AGG_AREA | - | - | - | - | our total face area 181.974 vs expected true boundary 168.572 (rel +8.0%) |
| 2 | SUSPECT_KEPT | ours FACE 19 | BSplineSurface | 21.8921 | (8.257,4.711,1.654) | on-source non-boundary samples int/ext 0/3 (~12.45 area kept beyond truth?) |
| 3 | SUSPECT_KEPT | ours FACE 16 | BSplineSurface | 11.8238 | (7.873,2.479,1.792) | on-source non-boundary samples int/ext 0/3 (~8.88 area kept beyond truth?) |
| 4 | SUSPECT_KEPT | ours FACE 32 | BSplineSurface | 5.8622 | (8.243,4.296,2.085) | on-source non-boundary samples int/ext 0/5 (~7.92 area kept beyond truth?) |
| 5 | SUSPECT_KEPT | ours FACE 0 | BSplineSurface | 18.5702 | (9.948,2.329,3.139) | on-source non-boundary samples int/ext 0/2 (~4.97 area kept beyond truth?) |
| 6 | SUSPECT_KEPT | ours FACE 33 | BSplineSurface | 7.3135 | (10.819,3.394,1.696) | on-source non-boundary samples int/ext 0/2 (~3.82 area kept beyond truth?) |

## Our-face verdicts (34 OK / 34 faces)

| face | verdict | srf | area | bnd/int/ext/amb | bnd-frac | face/bbox | src |
|---|---|---|---|---|---|---|---|
| 0 | OK | BSplineSurface | 18.5702 | 7/0/2/0 | 0.78 | 0.83 | A |
| 1 | OK | BSplineSurface | 17.8624 | 9/0/0/0 | 1.00 | 1.00 | A |
| 2 | OK | BSplineSurface | 2.7486 | 9/0/0/0 | 1.00 | 1.00 | A |
| 3 | OK | BSplineSurface | 2.9989 | 9/0/0/0 | 1.00 | 1.00 | A |
| 4 | OK | BSplineSurface | 0.8978 | 9/0/0/0 | 1.00 | 1.00 | A |
| 5 | OK | BSplineSurface | 0.6946 | 6/0/3/0 | 0.67 | 0.57 | A |
| 6 | OK | BSplineSurface | 1.6262 | 9/0/0/0 | 1.00 | 0.92 | A |
| 7 | OK | BSplineSurface | 0.2591 | 3/0/6/0 | 0.33 | 0.47 | A |
| 8 | OK | BSplineSurface | 0.2720 | 8/0/1/0 | 0.89 | 0.47 | A |
| 9 | OK | BSplineSurface | 0.4006 | 8/0/1/0 | 0.89 | 0.65 | A |
| 10 | OK | BSplineSurface | 1.8738 | 9/0/0/0 | 1.00 | 1.00 | A |
| 11 | OK | BSplineSurface | 3.4792 | 9/0/0/0 | 1.00 | 1.00 | A |
| 12 | OK | BSplineSurface | 0.4117 | 9/0/0/0 | 1.00 | 1.00 | A |
| 13 | OK | BSplineSurface | 2.7651 | 9/0/0/0 | 1.00 | 1.00 | A |
| 14 | OK | BSplineSurface | 2.6540 | 9/0/0/0 | 1.00 | 1.00 | A |
| 15 | OK | BSplineSurface | 0.9782 | 9/0/0/0 | 1.00 | 1.00 | A |
| 16 | OK | BSplineSurface | 11.8238 | 3/2/4/0 | 0.33 | 0.44 | A |
| 17 | OK | BSplineSurface | 0.5422 | 6/0/3/0 | 0.67 | 0.50 | A |
| 18 | OK | BSplineSurface | 12.3721 | 7/0/2/0 | 0.78 | 0.85 | A |
| 19 | OK | BSplineSurface | 21.8921 | 5/0/4/0 | 0.56 | 0.59 | A |
| 20 | OK | BSplineSurface | 16.4399 | 5/0/4/0 | 0.56 | 0.47 | A |
| 21 | OK | BSplineSurface | 4.1312 | 7/0/2/0 | 0.78 | 0.68 | A |
| 22 | OK | BSplineSurface | 9.6823 | 6/0/3/0 | 0.67 | 0.61 | A |
| 23 | OK | BSplineSurface | 9.6412 | 2/0/7/0 | 0.22 | 0.38 | A |
| 24 | OK | BSplineSurface | 6.8517 | 6/0/3/0 | 0.67 | 0.63 | B |
| 25 | OK | BSplineSurface | 5.3585 | 6/0/3/0 | 0.67 | 0.60 | B |
| 26 | OK | BSplineSurface | 0.1068 | 9/0/0/0 | 1.00 | 0.62 | A+B |
| 27 | OK | BSplineSurface | 0.0376 | 6/0/3/0 | 0.67 | 0.56 | B |
| 28 | OK | BSplineSurface | 0.0878 | 6/0/3/0 | 0.67 | 0.49 | B |
| 29 | OK | BSplineSurface | 0.2506 | 6/0/3/0 | 0.67 | 0.52 | B |
| 30 | OK | BSplineSurface | 6.9408 | 5/0/4/0 | 0.56 | 0.52 | B |
| 31 | OK | BSplineSurface | 4.1469 | 8/0/1/0 | 0.89 | 0.79 | B |
| 32 | OK | BSplineSurface | 5.8622 | 4/0/5/0 | 0.44 | 0.41 | B |
| 33 | OK | BSplineSurface | 7.3135 | 5/1/3/0 | 0.56 | 0.43 | B |
