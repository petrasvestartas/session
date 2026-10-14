# Robot arm hierarchy

`robot_arm.cpp`, `robot_arm.py`, and `robot_arm.rs` build the same eight-link KUKA iiwa Session.
The OBJ meshes remain in link-local coordinates under `session_data/robot_arm`.

```text
link_0
└── link_1
    └── link_2
        └── link_3
            └── link_4
                └── link_5
                    └── link_6
                        └── link_7
```

Every node's entry in `Session.xforms` is its joint transform relative to its parent. Therefore:

```text
world(link_7) = local(link_0) * local(link_1) * ... * local(link_7)
```

Each example asserts that equation at every link, writes a protobuf scene, reloads it, and asserts
the tool transform again. Build and run instructions are at the end of each source file.
