# face_diff z15 fuse

A: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\chair0.stp`  
B: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\rot\B_z15.step`  
ours: `C:\pc\3_code\code_rust\session\session_cpp\serialization\boolean_steps\chairs\rot\res_z15_fuse.step`  
eps 0.08197, 9 samples/face, grid 11

| side | solids | shells | faces | volume | OCCT valid |
|---|---|---|---|---|---|
| TRUE (OCCT fuse) | 1 | - | 43 | 80.952185 | 1 |
| OURS | 0 | 1 | 54 | 125.177723 | 1 |

## Defect tickets (8)

| # | kind | face | srf | area | centroid | detail |
|---|---|---|---|---|---|---|
| 0 | COUNT | - | - | - | - | our faces 54 vs OCCT 43 (+11) |
| 1 | SOLIDITY | - | - | - | - | our solids 0 vs OCCT 1 (shells 1, valid 1) |
| 2 | VOLUME | - | - | - | - | our vol 125.1777 vs OCCT 80.9522 (rel 5.46e-01) |
| 3 | AREA | A FACE 3 | BSplineSurface | 2.9990 | (5.803,4.235,2.961) | ours 2.9464 vs operand 2.9990 (rel 1.8%) |
| 4 | AREA | A FACE 15 | BSplineSurface | 14.5682 | (8.380,1.287,2.429) | ours 12.4908 vs operand 14.5682 (rel 14.3%) |
| 5 | AREA | B FACE 2 | BSplineSurface | 2.7486 | (7.383,4.446,-2.592) | ours 2.5709 vs operand 2.7486 (rel 6.5%) |
| 6 | AREA | B FACE 3 | BSplineSurface | 2.9989 | (7.040,4.091,-2.964) | ours 2.9346 vs operand 2.9989 (rel 2.1%) |
| 7 | AGG_AREA | - | - | - | - | our total face area 261.854 vs expected true boundary 289.394 (rel -9.5%) |

## Our-face verdicts (54 OK / 54 faces)

| face | verdict | srf | area | bnd/int/ext/amb | bnd-frac | face/bbox | src |
|---|---|---|---|---|---|---|---|
| 0 | OK | BSplineSurface | 18.4413 | 8/1/0/0 | 0.89 | 0.83 | A |
| 1 | OK | BSplineSurface | 0.2737 | 6/3/0/0 | 0.67 | 0.60 | A |
| 2 | OK | BSplineSurface | 17.8624 | 9/0/0/0 | 1.00 | 1.00 | A |
| 3 | OK | BSplineSurface | 2.5139 | 8/1/0/0 | 0.89 | 0.91 | A |
| 4 | OK | BSplineSurface | 2.9464 | 9/0/0/0 | 1.00 | 0.98 | A |
| 5 | OK | BSplineSurface | 0.8978 | 9/0/0/0 | 1.00 | 1.00 | A |
| 6 | OK | BSplineSurface | 0.1626 | 6/3/0/0 | 0.67 | 0.60 | A |
| 7 | OK | BSplineSurface | 0.7985 | 6/3/0/0 | 0.67 | 0.64 | A |
| 8 | OK | BSplineSurface | 0.0355 | 6/3/0/0 | 0.67 | 0.48 | A |
| 9 | OK | BSplineSurface | 1.8738 | 9/0/0/0 | 1.00 | 1.00 | A |
| 10 | OK | BSplineSurface | 3.4792 | 9/0/0/0 | 1.00 | 1.00 | A |
| 11 | OK | BSplineSurface | 0.4117 | 9/0/0/0 | 1.00 | 1.00 | A |
| 12 | OK | BSplineSurface | 2.0517 | 7/2/0/0 | 0.78 | 0.74 | A |
| 13 | OK | BSplineSurface | 2.6540 | 9/0/0/0 | 1.00 | 1.00 | A |
| 14 | OK | BSplineSurface | 0.9782 | 9/0/0/0 | 1.00 | 1.00 | A |
| 15 | OK | BSplineSurface | 0.4137 | 3/6/0/0 | 0.33 | 0.45 | A |
| 16 | OK | BSplineSurface | 0.8436 | 6/3/0/0 | 0.67 | 0.51 | A |
| 17 | OK | BSplineSurface | 11.3499 | 7/2/0/0 | 0.78 | 0.60 | A |
| 18 | OK | BSplineSurface | 12.3746 | 9/0/0/0 | 1.00 | 0.85 | A |
| 19 | OK | BSplineSurface | 0.1163 | 6/3/0/0 | 0.67 | 0.50 | A |
| 20 | OK | BSplineSurface | 14.9771 | 4/4/1/0 | 0.44 | 0.40 | A |
| 21 | OK | BSplineSurface | 0.9636 | 8/1/0/0 | 0.89 | 0.73 | A |
| 22 | OK | BSplineSurface | 0.4921 | 8/1/0/0 | 0.89 | 0.60 | A |
| 23 | OK | BSplineSurface | 16.4399 | 5/0/4/0 | 0.56 | 0.47 | A |
| 24 | OK | BSplineSurface | 2.1293 | 6/3/0/0 | 0.67 | 0.59 | A |
| 25 | OK | BSplineSurface | 9.6412 | 2/0/7/0 | 0.22 | 0.38 | A |
| 26 | OK | BSplineSurface | 13.0652 | 9/0/0/0 | 1.00 | 0.91 | B |
| 27 | OK | BSplineSurface | 2.2111 | 7/2/0/0 | 0.78 | 0.59 | B |
| 28 | OK | BSplineSurface | 10.5340 | 9/0/0/0 | 1.00 | 0.92 | B |
| 29 | OK | BSplineSurface | 1.0908 | 6/3/0/0 | 0.67 | 0.64 | B |
| 30 | OK | BSplineSurface | 0.0335 | 3/6/0/0 | 0.33 | 0.49 | B |
| 31 | OK | BSplineSurface | 2.5709 | 9/0/0/0 | 1.00 | 0.94 | B |
| 32 | OK | BSplineSurface | 2.9346 | 9/0/0/0 | 1.00 | 0.98 | B |
| 33 | OK | BSplineSurface | 0.8978 | 9/0/0/0 | 1.00 | 1.00 | B |
| 34 | OK | BSplineSurface | 1.2653 | 8/1/0/0 | 0.89 | 0.79 | B |
| 35 | OK | BSplineSurface | 3.8146 | 7/2/0/0 | 0.78 | 0.69 | B |
| 36 | OK | BSplineSurface | 2.2750 | 9/0/0/0 | 1.00 | 1.00 | B |
| 37 | OK | BSplineSurface | 1.8738 | 9/0/0/0 | 1.00 | 1.00 | B |
| 38 | OK | BSplineSurface | 3.4792 | 9/0/0/0 | 1.00 | 1.00 | B |
| 39 | OK | BSplineSurface | 0.4117 | 9/0/0/0 | 1.00 | 1.00 | B |
| 40 | OK | BSplineSurface | 2.7651 | 9/0/0/0 | 1.00 | 1.00 | B |
| 41 | OK | BSplineSurface | 2.6540 | 9/0/0/0 | 1.00 | 1.00 | B |
| 42 | OK | BSplineSurface | 0.9782 | 9/0/0/0 | 1.00 | 1.00 | B |
| 43 | OK | BSplineSurface | 0.5358 | 9/0/0/0 | 1.00 | 0.92 | B |
| 44 | OK | BSplineSurface | 0.1111 | 5/4/0/0 | 0.56 | 0.49 | B |
| 45 | OK | BSplineSurface | 12.1111 | 6/3/0/0 | 0.67 | 0.57 | B |
| 46 | OK | BSplineSurface | 6.2012 | 7/2/0/0 | 0.78 | 0.83 | B |
| 47 | OK | BSplineSurface | 3.5706 | 8/1/0/0 | 0.89 | 0.85 | B |
| 48 | OK | BSplineSurface | 27.2033 | 7/0/2/0 | 0.78 | 0.73 | B |
| 49 | OK | BSplineSurface | 16.4399 | 5/0/4/0 | 0.56 | 0.47 | B |
| 50 | OK | BSplineSurface | 3.9294 | 6/1/2/0 | 0.67 | 0.61 | B |
| 51 | OK | BSplineSurface | 4.6777 | 6/2/1/0 | 0.67 | 0.67 | B |
| 52 | OK | BSplineSurface | 0.0997 | 6/3/0/0 | 0.67 | 0.49 | B |
| 53 | OK | BSplineSurface | 8.9975 | 2/0/7/0 | 0.22 | 0.36 | B |
