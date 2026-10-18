fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // rows of 32768 workgroups (PROJECT_ROW_GROUPS): a dispatch dimension holds at most 65535
    let local = id.x + id.y * 2097152u;
    let index = local + live_count.y;

    if (local>=live_count.x) {
        return;
    }

    let polygon = project_physical_triangle(local+1u, 0u);
    var out: ProjectedTriangle;

