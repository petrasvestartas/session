# 05 · Depth and visible ink

Ink is every edge, curve, marker, dot and arrow drawn over the faces, and a face in front must hide it pixel by pixel. Lesson 04b copied the two shader files that decide this; this lesson reads them part by part.

![Reversed depth, and why a thick stroke must transfer the surface depth to its axis before comparing.](illustrations/ink-visibility.svg)

## Step 1 · src/shaders/ink_visibility.wgsl

The depth and triangle textures the face pass leaves behind, the tolerances, and a stroke's center line as one fragment sees it.

`lessons/05/src/shaders/ink_visibility.wgsl` · read, copied in 04b

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:ink-inputs"
```

## Step 2 · src/shaders/ink_visibility.wgsl

Read the scene depth, fit a plane from two neighbours, and carry the face's depth to another point.

`lessons/05/src/shaders/ink_visibility.wgsl` · read, copied in 04b

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:ink-fit"
```

## Step 3 · src/shaders/ink_visibility.wgsl

A stroke fragment carries the face's depth to the stroke's center line, then compares.

`lessons/05/src/shaders/ink_visibility.wgsl` · read, copied in 04b

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:ink-axis"
```

## Step 4 · src/shaders/ink_visibility.wgsl

Markers and dots fit along x and y, carry to their centre, and test the centre pixel too.

`lessons/05/src/shaders/ink_visibility.wgsl` · read, copied in 04b

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:ink-discs"
```

## Step 5 · src/shaders/ink_visibility.wgsl

Where the face pass recorded a triangle, that triangle's exact depth slope replaces the fitted one.

`lessons/05/src/shaders/ink_visibility.wgsl` · read, copied in 04b

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:ink-plane"
```

## Step 6 · src/shaders/projected_triangle.wgsl

A triangle in screen space: three edge lines say whether it covers a point, a depth plane says how deep.

`lessons/05/src/shaders/projected_triangle.wgsl` · read, copied in 04b

```wgsl
--8<-- "lessons/05/src/shaders/projected_triangle.wgsl:projected-triangle"
```

## Step 7 · src/shaders/ink_visibility.wgsl

The whole test: plane fit first, then the triangles around the center line exactly; clipping calls stay false until lesson 18b.

`lessons/05/src/shaders/ink_visibility.wgsl` · read, copied in 04b

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:ink-exact"
```

## Step 8 · src/shaders/ink_visibility.wgsl

Hidden ink fades behind see-through faces instead of vanishing, and the projected triangle file is pasted in.

`lessons/05/src/shaders/ink_visibility.wgsl` · read, copied in 04b

```wgsl
--8<-- "lessons/05/src/shaders/ink_visibility.wgsl:ink-glass"
```

Run `cargo check` in `lessons/05/`.

## Check

Nothing new to type: `cargo check` in `lessons/05/` passes as in 04d. Once lesson 06 walks a solid, its far edges stay hidden behind its faces while the near ones keep their full width.
