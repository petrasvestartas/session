    let mut cpu = crate::memory::cpu(editor.presentation_scenes(), renderer.geometry_owners().map(|owner| &owner.source));
    cpu[4]+=renderer.arena_cpu_bytes();