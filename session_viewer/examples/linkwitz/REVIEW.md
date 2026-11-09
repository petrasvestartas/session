# Force-density book review

One read-only reviewer was assigned to each of the 36 slides. The sources were Linkwitz's Chapter 6, “Force density method: design of a timber shell,” plus the matrix-formulation and reading notes. The local references are under `target/slide-review/book/`; do not commit the private study transcription into Teaching.

**All 36 slide reviews completed.** Each slide had a separate read-only reviewer, with the chapter, study notes, generator, screenshot, and protobuf examined independently. The separate prompts below are retained for reproducibility, including for the reviews that initially hit the model quota and later completed.

## Findings to carry forward

These are review findings, not claims that the whole deck has already been corrected.

- **Slide 1:** projected 3D equilibrium and closure are correct. Define the force-vector notation, state `p_z = -5 kN`, and distinguish the projected polygon from a true-to-magnitude planar construction.
- **Slide 2:** numerical data agree with the exercise. State the topology, fixed coordinates, prescribed force densities, applied load, unknown free coordinates, and units more explicitly.
- **Slide 4:** the wrong cable is drawn: the code selects `e3` while the box and equations describe `e1`. Single-source the selected edge and label its anchor.
- **Slide 3:** the connectivity matrix is correct. Distinguish tension arrows from incidence orientation, label row/column order, and state that the matrix encodes topology only; do not propagate the printed coordinate-difference sign slip.
- **Slides 5–8:** the scalar force-component signs are correct: `-3`, `+2`, `-3`, and `+4 kN` in cable order. Show evaluated substitutions, label the fixed neighbours, distinguish coordinate metres from force kilonewtons, and clarify stressed-length notation.
- **Slide 9:** physical equilibrium is correct. Use the proper neighbour index and make the natural `-3 + 2 - 3 + 4 + 0 = 0 kN` order explicit.
- **Slide 10:** prescribed `q`, not the quotient alone, makes the coordinate solve linear. State knowns, units, and force recovery after solving.
- **Slides 11–12:** settled values are correct. Qualify the midpoint/weighted-average rule by load and positive force densities; do not imply that a linear animation between different `q` values is an equilibrium history.
- **Slide 13:** the equation correctly requires `p_x = +4`, but the green load arrow points left. Reverse it to `+x` or change the entire numerical example consistently; do not flip the correct numerator sign.
- **Slide 14:** the trial point does not move the cables or displayed lengths, three original lengths are numerically wrong, and the length vector should be lowercase `l`, not diagonal `L`. Label trial positions as non-equilibrium states and derive both length states from the geometry.
- **Slide 15:** the differential needs a minus under the deck's direction-cosine convention: `dl_i = -dx_0 cos(alpha_i)`. The perpendicular-motion statement is first-order only.
- **Slide 16:** declare the row as the transposed Jacobian, explain that its entries are the negatives of the pull direction cosines, and correct the first cell to `0.706` by deriving lengths from the node geometry.
- **Slides 17–19:** make the Jacobian row/transpose convention consistent, retain the outer minus in the scalar expansion, use approximation for rounded decimal products, and show `Uq = Qu` from diagonal/vector definitions. Lengths disappear from the coordinate solve, not from subsequent force recovery.
- **Slides 20–21:** the matrix products and coordinate differences are correct. Negative-times-zero expansions lose the negative operand in the shared Python formatter; use explicit, braced vector notation so the rendered product title does not lose `x_N`, and distinguish tension arrows from incidence orientation.
- **Slide 22:** `MatrixEquation.product` drops the sign of `-1 × 0`; use operand signs rather than the sign of their product. Use an uppercase fixed-node subscript `F`.
- **Slide 23:** matrix values and signs are correct; unbraced TeX scripts can hide `x_N`. Prefer explicit braced vector/subscript notation and validate the parser separately.
- **Slides 24–26:** matrix signs and coordinates are correct. Define `D_N`, `D_F`, and the right-hand side explicitly; show that downward load gives `p_z < 0`.
- **Slides 27–31:** the general local equation has a latent applied-load sign error, hidden by zero `p_x`. Use `sum q(x_neighbour - x_node) + p = 0`, or its negated matrix-row form. Explain fixed-neighbour contributions, boundary nodes that remain free, and coupled versus independent solving.
- **Slides 32–34:** the matrix, coordinates, and exact equilibrium checks are correct. Define the load vector over all free nodes, avoid implying dynamic relaxation, and distinguish rounded from exact verification.
- **Slide 35:** the loaded vertical correction is `-0.03 m`, not `+0.03 m`. The general position is the neighbour average plus `p/(4q)`, not an unconditional average; use “unit force density,” not cable weight. This loaded example remains anticlastic—vertical load alone does not imply a synclastic shape.
- **Slide 36:** full-precision recovery is correct, but `8 × 1.08 = 8.61` is false displayed arithmetic. Show enough precision or an unrounded substitution and explicit approximation.
- **Cross-deck:** derive lengths from `NODES`; the current `LENGTHS` constants are `[4.254, 4.535, 5.704, 4.478]`, while the node geometry gives approximately `[4.250000, 4.534589, 5.706356, 4.479118]`.
- **Cross-deck:** later reveals can retain the topic title instead of the current step name, and highlight writes for preserved edges can silently miss their targets.
- **Book coverage:** a complete Chapter 6 treatment should add a materialization coda distinguishing stressed length `l` from unstressed cutting length `l_0` and axial stiffness `EA`. Do not overcrowd slide 36 with it.

The animation/color patch does **not** apply this list as a wholesale content rewrite. Its completed final-morph restore and math-only animation-startup fixes are separate from these conceptual recommendations.

## Separate review prompts

Run each prompt in a new read-only reviewer. The working directory is `C:/pc/3_code/code_rust/wood_research/session/session_viewer`. Each prompt intentionally refers to the real book, generator, screenshot, and protobuf rather than only a slide title.

### Slide 3 — Connectivity

```text
Review slide 3 of 36, “Connectivity,” read-only. Do not edit files, regenerate decks, or launch agents. Read target/slide-review/book/01-force-density-method.md, 02-study-matrix-formulation.md, and 03-reading-notes.md, using INDEX.md and GLOSSARY.md only as finding aids. Inspect the topic-2 branch of examples/linkwitz/force_density.py, target/presentation/step-02.png, and generated/steps/02_00.pb if present. Trace the branch-node matrix to Linkwitz (6.14)–(6.20). Independently check row/column meanings, cable orientation, indexing, signs, units, and the relationship u = Cx. Distinguish printed sign inconsistencies from the deck's convention. Return a verdict, precise source/book references, mathematical or visual corrections, slide-specific missing concepts, and at most three feasible improvements using only the session kernel, protobuf sessions, Rust renderer, and WGSL. No implementation.
```

### Slide 7 — Cable e2

```text
Review slide 7 of 36, “Cable e₂,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-6 branch of examples/linkwitz/force_density.py, target/presentation/step-06.png, and generated/steps/06_00.pb if present. Compare with Linkwitz (6.4)–(6.6) and the four-bar exercise. Independently check the selected edge, endpoints, direction cosine, force sign, arrow orientation, length, force magnitude, units, and labels. Distinguish a force component from a length derivative and a coordinate projection from a force vector. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 8 — Cable e3

```text
Review slide 8 of 36, “Cable e₃,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-7 branch of examples/linkwitz/force_density.py, target/presentation/step-07.png, and generated/steps/07_00.pb if present. Compare with Linkwitz (6.4)–(6.6) and the four-bar exercise. Independently check the selected edge, endpoints, direction cosine, positive/negative components, arrow orientation, length, force magnitude, units, and labels. Check consistency with the preceding cable slides and the equilibrium sum. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 13 — An external load

```text
Review slide 13 of 36, “An external load,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-12 branch of examples/linkwitz/force_density.py, target/presentation/step-12.png, and generated/steps/12_00.pb if present. Trace the formula to (6.11)–(6.13). Independently check the load sign, weighted-average-plus-load-offset equation, solved coordinate, force recovery, arrow orientation, and units. Check both the initial and settled animation states rather than assuming interpolation is an equilibrium history. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 14 — Length is a function of n0

```text
Review slide 14 of 36, “Length is a function of n₀,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-13 branch of examples/linkwitz/force_density.py, target/presentation/step-13.png, and generated/steps/13_00.pb if present. Compare with (6.2), (6.21), and the distinction between stressed and unstressed lengths. Independently recompute lengths and coordinate differences from the node data, check vector/matrix notation and units, and inspect the moved-node animation and labels. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 16 — The Jacobian row

```text
Review slide 16 of 36, “The Jacobian row,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-15 branch of examples/linkwitz/force_density.py, target/presentation/step-15.png, and generated/steps/15_00.pb if present. Compare with (6.22)–(6.24). Independently differentiate every cable length, check signs, numerical values, row/column orientation, transpose notation, units, and links to the direction-cosine slides. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 19 — Lengths cancel

```text
Review slide 19 of 36, “Lengths cancel,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-18 branch of examples/linkwitz/force_density.py, target/presentation/step-18.png, and generated/steps/18_00.pb if present. Trace the substitution from (6.10), (6.25), and (6.27), including the Jacobian convention and applied-load sign. Check that the length elimination and morph are mathematically justified, with consistent symbols and units. Explain what is prescribed and what remains nonlinear, if anything. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 20 — Coordinates from C

```text
Review slide 20 of 36, “Coordinates from C,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-19 branch of examples/linkwitz/force_density.py, target/presentation/step-19.png, and generated/steps/19_00.pb if present. Compare with (6.14)–(6.20). Independently multiply the displayed matrices, check the split into free/fixed coordinates, signs, dimensions, units, cable-row mapping, and TeX rendering. Distinguish coordinate differences from physical force components. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 21 — Free-node contribution

```text
Review slide 21 of 36, “Free-node contribution,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-20 branch of examples/linkwitz/force_density.py, target/presentation/step-20.png, and generated/steps/20_00.pb if present. Compare with (6.18)–(6.20). Independently verify C_N, x_N, their product, dimensions, scalar/vector notation, signs, and the relationship to the fixed contribution and physical cable arrows. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 28 — Node b

```text
Review slide 28 of 36, “Node b,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-27 branch of examples/linkwitz/force_density.py, target/presentation/step-27.png, and the incremental reveal generated/steps/25_02.pb if present. Compare the local row with (6.11)–(6.13) and the matrix system (6.31)–(6.32). Recompute node b's free/fixed neighbours, force-density weights, diagonal, off-diagonal entries, RHS, solved coordinate, and residual. Check the applied-load sign, units, current step title, preserved cable highlights, and labels. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 31 — Node e

```text
Review slide 31 of 36, “Node e,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-30 branch of examples/linkwitz/force_density.py, target/presentation/step-30.png, and the incremental reveal generated/steps/25_05.pb if present. Compare the local row with (6.11)–(6.13) and (6.31)–(6.32). Recompute node e's free/fixed neighbours, weights, diagonal, off-diagonal entries, RHS, coordinate, and residual. Check the applied-load sign, units, coupled-solve explanation, current step title, preserved cable highlights, and labels. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```

### Slide 35 — One local rule, repeated

```text
Review slide 35 of 36, “One local rule, repeated,” read-only. Do not edit files, regenerate decks, or launch agents. Read the full chapter and both study-note files under target/slide-review/book/. Inspect the topic-34 branch of examples/linkwitz/force_density.py, target/presentation/step-34.png, and the incremental reveal generated/steps/26_00.pb if present. Compare with (6.11)–(6.13) and (6.31)–(6.32). Independently verify the interior-node average and the load-offset sign using exact coordinates and a nonzero downward load. Check free/fixed roles, vector-versus-scalar notation, units, coupled versus sequential solving, current step title, highlights, and labels. Return a verdict, exact corrections with source/book references, missing concepts, and at most three session-kernel/protobuf/Rust/WGSL improvements. No implementation.
```
