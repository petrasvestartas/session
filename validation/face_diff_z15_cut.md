# face_diff z15 cut

A: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\chair0.stp`  
B: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\rot\B_z15.step`  
ours: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\rot\res_z15_cut.step`  
eps 0.0819, 9 samples/face, grid 11

| side | solids | shells | faces | volume | OCCT valid |
|---|---|---|---|---|---|
| TRUE (OCCT cut) | 2 | - | 44 | 80.297281 | 1 |
| OURS | 0 | 2 | 38 | 40.812740 | 1 |

**WARNING: OCCT self-inconsistent on this config** -- vol identities imply common=-0.0004 (from cut), 79.6415 (from fuse), but OCCT common=0.0000. The oracle truth itself is unreliable here; treat VOLUME/COUNT/AGG tickets as indicative only.

## Defect tickets (6)

| # | kind | face | srf | area | centroid | detail |
|---|---|---|---|---|---|---|
| 0 | COUNT | - | - | - | - | our faces 38 vs OCCT 44 (-6) |
| 1 | SOLIDITY | - | - | - | - | our solids 0 vs OCCT 2 (shells 2, valid 1) |
| 2 | VOLUME | - | - | - | - | our vol 40.8127 vs OCCT 80.2973 (rel 4.92e-01) |
| 3 | AREA | A FACE 2 | BSplineSurface | 2.7486 | (6.176,4.490,2.537) | ours 2.5139 vs operand 2.7486 (rel 8.5%) |
| 4 | AREA | A FACE 3 | BSplineSurface | 2.9990 | (5.803,4.235,2.961) | ours 2.9464 vs operand 2.9990 (rel 1.8%) |
| 5 | AREA | A FACE 15 | BSplineSurface | 14.5682 | (8.380,1.287,2.429) | ours 12.4908 vs operand 14.5682 (rel 14.3%) |

## Our-face verdicts (38 OK / 38 faces)

| face | verdict | srf | area | bnd/int/ext/amb | bnd-frac | face/bbox | src |
|---|---|---|---|---|---|---|---|
| 0 | OK | BSplineSurface | 0.1626 | 6/0/3/0 | 0.67 | 0.60 | A |
| 1 | OK | BSplineSurface | 0.0355 | 3/0/6/0 | 0.33 | 0.48 | A |
| 2 | OK | BSplineSurface | 0.4921 | 6/0/3/0 | 0.67 | 0.60 | A |
| 3 | OK | BSplineSurface | 0.5095 | 5/0/4/0 | 0.56 | 0.47 | B |
| 4 | OK | BSplineSurface | 18.4413 | 8/0/1/0 | 0.89 | 0.83 | A |
| 5 | OK | BSplineSurface | 0.2737 | 6/0/3/0 | 0.67 | 0.60 | A |
| 6 | OK | BSplineSurface | 17.8624 | 9/0/0/0 | 1.00 | 1.00 | A |
| 7 | OK | BSplineSurface | 2.5139 | 9/0/0/0 | 1.00 | 0.91 | A |
| 8 | OK | BSplineSurface | 2.9464 | 9/0/0/0 | 1.00 | 0.98 | A |
| 9 | OK | BSplineSurface | 0.8978 | 9/0/0/0 | 1.00 | 1.00 | A |
| 10 | OK | BSplineSurface | 0.7985 | 6/0/3/0 | 0.67 | 0.64 | A |
| 11 | OK | BSplineSurface | 1.8738 | 9/0/0/0 | 1.00 | 1.00 | A |
| 12 | OK | BSplineSurface | 3.4792 | 9/0/0/0 | 1.00 | 1.00 | A |
| 13 | OK | BSplineSurface | 0.4117 | 9/0/0/0 | 1.00 | 1.00 | A |
| 14 | OK | BSplineSurface | 2.0517 | 7/0/2/0 | 0.78 | 0.74 | A |
| 15 | OK | BSplineSurface | 2.6540 | 9/0/0/0 | 1.00 | 1.00 | A |
| 16 | OK | BSplineSurface | 0.9782 | 9/0/0/0 | 1.00 | 1.00 | A |
| 17 | OK | BSplineSurface | 0.4137 | 3/0/6/0 | 0.33 | 0.45 | A |
| 18 | OK | BSplineSurface | 0.8435 | 6/0/3/0 | 0.67 | 0.51 | A |
| 19 | OK | BSplineSurface | 11.3499 | 7/0/2/0 | 0.78 | 0.60 | A |
| 20 | OK | BSplineSurface | 12.3746 | 9/0/0/0 | 1.00 | 0.85 | A |
| 21 | OK | BSplineSurface | 0.1163 | 6/0/3/0 | 0.67 | 0.50 | A |
| 22 | OK | BSplineSurface | 14.9771 | 3/0/6/0 | 0.33 | 0.40 | A |
| 23 | OK | BSplineSurface | 0.9636 | 7/0/2/0 | 0.78 | 0.73 | A |
| 24 | OK | BSplineSurface | 16.4399 | 5/0/4/0 | 0.56 | 0.47 | A |
| 25 | OK | BSplineSurface | 2.1293 | 5/0/4/0 | 0.56 | 0.59 | A |
| 26 | OK | BSplineSurface | 9.6412 | 2/0/7/0 | 0.22 | 0.38 | A |
| 27 | OK | BSplineSurface | 7.0811 | 6/0/3/0 | 0.67 | 0.65 | B |
| 28 | OK | BSplineSurface | 6.2043 | 5/0/4/0 | 0.56 | 0.60 | B |
| 29 | OK | BSplineSurface | 0.1777 | 8/0/1/0 | 0.89 | 0.83 | B |
| 30 | OK | BSplineSurface | 0.0644 | 6/0/3/0 | 0.67 | 0.52 | B |
| 31 | OK | BSplineSurface | 0.3394 | 9/0/0/0 | 1.00 | 0.87 | B |
| 32 | OK | BSplineSurface | 1.7502 | 9/0/0/0 | 1.00 | 0.90 | B |
| 33 | OK | BSplineSurface | 8.5417 | 7/0/2/0 | 0.78 | 0.51 | B |
| 34 | OK | BSplineSurface | 4.7962 | 5/0/4/0 | 0.56 | 0.40 | B |
| 35 | OK | BSplineSurface | 2.0475 | 3/0/6/0 | 0.33 | 0.40 | B |
| 36 | OK | BSplineSurface | 8.1045 | 5/1/3/0 | 0.56 | 0.51 | B |
| 37 | OK | BSplineSurface | 0.6437 | 7/0/2/0 | 0.78 | 0.71 | B |
