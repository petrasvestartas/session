# Combine the extensions into the current viewer

Sixteen lessons take checkpoint 21 to the maintained viewer.

![Running viewer: Combine the extensions into the current viewer.](screenshots/extensions-color-channels.png)

## Starting point

Copy checkpoint 21 and work in the copy:

```bash
cp -r docs/lessons/21 docs/lessons/my-integrated
cd docs/lessons/my-integrated
cargo check -j4 --lib
```

Each lesson's finished crate is `docs/lessons/current-N/`; diff against it whenever a check fails.

## Follow these checkpoints in order

1. [Refresh diagnostics and resource checks](current-1.md)
2. [Create, trim, extend and explode](current-2.md)
3. [Make control dragging respect object placement](current-3.md)
4. [Draw a solid, readable gumball](current-4.md)
5. [Build the nested session and graph panel](current-5.md)
6. [Build the egui panel and command interface](current-6.md)
7. [Finish the shared editing wiring](current-7.md)
8. [Dock the workspace, edit source geometry and save](current-8.md)
9. [Keep source dragging live and build one layer tree](current-9.md)
10. [Split curves and faces while keeping the shell joined](current-10.md)
11. [Separate face and edge colors and keep large-object dragging live](current-11.md)
12. [Contact shadows that follow object size](current-12.md)
13. [Polyline options, ordered input and a remembered view](current-13.md)
14. [Attributes On|Off and a phone keyboard](current-14.md)
15. [Translucent faces and the Opacity command](current-15.md)
16. [Attribute features in red and a one-row command dock](current-16.md)

`docs/lessons/current-16/src` is the maintained `src`.

## Expected viewer result

A command dock across the bottom, a layer tree on the right, a toolbar on the left, and a gumball on the selection. See the [phone layout](screenshots/extensions-workspace-current-phone.png).

[![Full viewer result for extend integrated tutorial](screenshots/extensions-workspace-current-desktop.png)](screenshots/extensions-workspace-current-desktop.png)
