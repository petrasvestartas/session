        "source_cpu_known_payload_bytes": source_memory.known_bytes(), // register:release
        "source_cpu_known_payload": source_memory, // register:release
        "source_cpu_scope": "retained Session arrays/strings/values; Rc objects deduplicated; not RSS or total heap", // register:release
        "source_cpu_exclusions": "allocator/Rc/map overhead and spare map slots, private cloud LOD/capacity, GUID allocations, private nested metadata/element/BVH caches, tree/graph/component-extra payloads, streamed descriptors, upload staging and loader buffers", // register:release
