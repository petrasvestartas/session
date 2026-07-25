# face_diff chair0_cut_chair1 cut

A: `session_cpp/serialization/boolean_steps/chairs/chair0.stp`  
B: `session_cpp/serialization/boolean_steps/chairs/chair1.stp`  
ours: `session_cpp/serialization/boolean_steps/chairs/chair0_cut_chair1.step`  
eps 0.08165, 9 samples/face, grid 11

| side | solids | shells | faces | volume | OCCT valid |
|---|---|---|---|---|---|
| TRUE (OCCT cut) | 1 | - | 35 | 46.794114 | 1 |
| OURS | 3 | 3 | 75 | 207.386807 | 1 |

NOTE: result file has 3 shape units; diffing 1 picked (--pick green); whole-file volume/solid/face-count tickets suppressed

## Defect tickets (0)

No defects: every our-face verifies as true-result boundary, all true regions covered, counts/volume match.

## Our-face verdicts (35 OK / 35 faces)

| face | verdict | srf | area | bnd/int/ext/amb | bnd-frac | face/bbox | src |
|---|---|---|---|---|---|---|---|
| 40 | OK | BSplineSurface | 15.4305 | 7/0/2/0 | 0.78 | 0.69 | A |
| 41 | OK | BSplineSurface | 0.3901 | 6/0/3/0 | 0.67 | 0.60 | A |
| 42 | OK | BSplineSurface | 17.8624 | 9/0/0/0 | 1.00 | 1.00 | A |
| 43 | OK | BSplineSurface | 2.7486 | 9/0/0/0 | 1.00 | 1.00 | A |
| 44 | OK | BSplineSurface | 2.9989 | 9/0/0/0 | 1.00 | 1.00 | A |
| 45 | OK | BSplineSurface | 0.8978 | 9/0/0/0 | 1.00 | 1.00 | A |
| 46 | OK | BSplineSurface | 0.0545 | 6/0/3/0 | 0.67 | 0.54 | A |
| 47 | OK | BSplineSurface | 0.5714 | 6/0/3/0 | 0.67 | 0.61 | A |
| 48 | OK | BSplineSurface | 1.8738 | 9/0/0/0 | 1.00 | 1.00 | A |
| 49 | OK | BSplineSurface | 3.4792 | 9/0/0/0 | 1.00 | 1.00 | A |
| 50 | OK | BSplineSurface | 0.4117 | 9/0/0/0 | 1.00 | 1.00 | A |
| 51 | OK | BSplineSurface | 1.4904 | 5/0/4/0 | 0.56 | 0.54 | A |
| 52 | OK | BSplineSurface | 2.6540 | 9/0/0/0 | 1.00 | 1.00 | A |
| 53 | OK | BSplineSurface | 0.9782 | 9/0/0/0 | 1.00 | 1.00 | A |
| 54 | OK | BSplineSurface | 11.5616 | 5/1/3/0 | 0.56 | 0.51 | A |
| 55 | OK | BSplineSurface | 0.9790 | 6/0/3/0 | 0.67 | 0.51 | A |
| 56 | OK | BSplineSurface | 11.4057 | 8/0/1/0 | 0.89 | 0.78 | A |
| 57 | OK | BSplineSurface | 0.2402 | 6/0/3/0 | 0.67 | 0.51 | A |
| 58 | OK | BSplineSurface | 17.6516 | 5/0/4/0 | 0.56 | 0.47 | A |
| 59 | OK | BSplineSurface | 0.9190 | 7/0/2/0 | 0.78 | 0.73 | A |
| 60 | OK | BSplineSurface | 16.4399 | 5/0/4/0 | 0.56 | 0.47 | A |
| 61 | OK | BSplineSurface | 1.5030 | 4/0/5/0 | 0.44 | 0.41 | A |
| 62 | OK | BSplineSurface | 10.4259 | 4/0/5/0 | 0.44 | 0.43 | A |
| 63 | OK | BSplineSurface | 9.6412 | 2/0/7/0 | 0.22 | 0.38 | A |
| 64 | OK | BSplineSurface | 3.9261 | 3/0/6/0 | 0.33 | 0.36 | B |
| 65 | OK | BSplineSurface | 5.8668 | 7/0/2/0 | 0.78 | 0.67 | B |
| 66 | OK | BSplineSurface | 0.1309 | 9/0/0/0 | 1.00 | 0.86 | B |
| 67 | OK | BSplineSurface | 0.0608 | 6/0/3/0 | 0.67 | 0.52 | B |
| 68 | OK | BSplineSurface | 0.5069 | 9/0/0/0 | 1.00 | 0.90 | B |
| 69 | OK | BSplineSurface | 1.9259 | 8/0/1/0 | 0.89 | 0.85 | B |
| 70 | OK | BSplineSurface | 7.5797 | 4/0/5/0 | 0.44 | 0.41 | B |
| 71 | OK | BSplineSurface | 3.9228 | 3/0/6/0 | 0.33 | 0.33 | B |
| 72 | OK | BSplineSurface | 7.1495 | 5/0/4/0 | 0.56 | 0.73 | B |
| 73 | OK | BSplineSurface | 9.4731 | 8/0/1/0 | 0.89 | 0.72 | B |
| 74 | OK | BSplineSurface | 1.4740 | 7/0/2/0 | 0.78 | 0.75 | B |
