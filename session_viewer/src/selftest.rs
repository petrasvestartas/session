//! Native headless self-test: builds the demo scene on a headless wgpu device and drives the
//! real pick code programmatically — so trimmed-surface selection/visibility can be verified
//! without a browser. Not compiled for wasm.
#![cfg(not(target_arch = "wasm32"))]

use crate::gpu_session::GpuSession;
use crate::engine::pipelines::build_geom_bind_group_layout;

fn headless_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::default();
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
    })).expect("no wgpu adapter (headless)");
    pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
        .expect("no wgpu device")
}

/// Build the demo scene headlessly and report, per trimmed surface, whether a ray aimed at it
/// (body) and at its boundary curve (edge) is picked. Returns a human-readable report.
pub fn run() -> String {
    let mut out = String::new();
    let (device, queue) = headless_device();
    let bgl = build_geom_bind_group_layout(&device);
    let mut gpu = GpuSession::new(&device, &bgl);
    let (mut session, _) = crate::demo::active_scene();
    gpu.rebuild_from(&session, &device, &queue);

    out += &format!("trimmed surfaces in session: {}\n", session.objects.nurbssurfacetrimmeds.len());
    out += &format!("trimmed pick meshes uploaded: {}\n", gpu.nurbs_trimmed_pick_meshes.len());

    let guids: Vec<String> = session.objects.nurbssurfacetrimmeds.iter().map(|t| t.guid().to_string()).collect();
    for guid in &guids {
        let (mesh, _xf) = match gpu.nurbs_trimmed_pick_meshes.get(guid) {
            Some(m) => m, None => { out += &format!("  {guid}: NO pick mesh\n"); continue; }
        };
        // centroid + a representative vertex
        let mut c = [0.0f64; 3]; let mut n = 0.0f64; let mut p = [0.0f64; 3];
        for vk in mesh.vertex.keys() {
            let v = mesh.vertex_point(*vk).unwrap();
            c[0]+=v[0]; c[1]+=v[1]; c[2]+=v[2]; n+=1.0; p=[v[0],v[1],v[2]];
        }
        if n == 0.0 { out += &format!("  {guid}: empty pick mesh\n"); continue; }
        let c = [c[0]/n, c[1]/n, c[2]/n];
        // ray from outside, through the vertex p, toward the centroid (guarantees a front hit if picking works)
        let mut d = [p[0]-c[0], p[1]-c[1], p[2]-c[2]];
        let dl = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt().max(1e-9);
        d = [d[0]/dl, d[1]/dl, d[2]/dl];
        let span = {
            let mut mn=[1e30;3]; let mut mx=[-1e30;3];
            for vk in mesh.vertex.keys() { let v=mesh.vertex_point(*vk).unwrap(); for k in 0..3 { mn[k]=f64::min(mn[k],v[k]); mx[k]=f64::max(mx[k],v[k]); } }
            ((mx[0]-mn[0]).powi(2)+(mx[1]-mn[1]).powi(2)+(mx[2]-mn[2]).powi(2)).sqrt()
        };
        let origin = session_rust::Point::new(p[0]+d[0]*span, p[1]+d[1]*span, p[2]+d[2]*span);
        let dir = session_rust::Vector::new(-d[0], -d[1], -d[2]);
        let radius = (span * 0.05) as f32;

        let body = gpu.pick_nurbssurfacetrimmeds(&origin, &dir);
        let body_hit = body.iter().any(|(g, _)| g == guid);
        let edge = gpu.pick_trimmed_edges(&origin, &dir, radius.max(5.0));
        let edge_hit = edge.iter().any(|(g, _)| g == guid);
        let nseg = gpu.guid_to_seg.get(guid).map(|r| r.len()).unwrap_or(0);
        let has_iid = gpu.pick.instance_id(guid).is_some();
        out += &format!("  {guid}: body_pick={} edge_pick={} segs={} instance_id={}\n",
                        body_hit, edge_hit, nseg, has_iid);

        // Replicate the viewer's selection aggregation: closest of kernel ray_cast (lookup
        // geometry incl. the translucent cut plane) vs the GPU body/edge picks. Reveals what
        // actually wins when aiming at the torus (e.g. is the cut plane shadowing it?).
        let pd = |p: &session_rust::Point| -> f32 {
            (((p[0]-origin[0]).powi(2)+(p[1]-origin[1]).powi(2)+(p[2]-origin[2]).powi(2)).sqrt()) as f32
        };
        let ray = crate::pick::Ray { origin: [origin[0] as f32, origin[1] as f32, origin[2] as f32],
                                     direction: [dir[0] as f32, dir[1] as f32, dir[2] as f32] };
        let kernel_hits = crate::pick::pick_by_ray(&mut session, ray, radius.max(5.0));
        let mut best = (f32::MAX, String::from("<none>"));
        for h in &kernel_hits { if (h.distance as f32) < best.0 { best = (h.distance as f32, h.guid().to_string()); } }
        if let Some((g,p)) = body.iter().next() { let d=pd(p); if d<best.0 { best=(d,g.clone()); } }
        if let Some((g,p)) = edge.iter().next() { let d=pd(p); if d<best.0 { best=(d,g.clone()); } }
        out += &format!("      -> viewer would select: '{}'  (kernel_hits={})\n", best.1, kernel_hits.len());

        // Flag mechanism: set FLAG_SELECTED / FLAG_HIDDEN and verify they land on the instance
        // (proves highlight/hide CAN reach this object at the GPU layer).
        use crate::gpu_session::InstanceData;
        let iid = gpu.pick.instance_id(guid).unwrap() as usize;
        gpu.set_flag(guid, InstanceData::FLAG_SELECTED, true, &queue);
        let sel_on = gpu.instances_cpu[iid].flags & InstanceData::FLAG_SELECTED != 0;
        gpu.set_flag(guid, InstanceData::FLAG_SELECTED, false, &queue);
        let sel_off = gpu.instances_cpu[iid].flags & InstanceData::FLAG_SELECTED == 0;
        gpu.set_flag(guid, InstanceData::FLAG_HIDDEN, true, &queue);
        let hid_on = gpu.instances_cpu[iid].flags & InstanceData::FLAG_HIDDEN != 0;
        gpu.set_flag(guid, InstanceData::FLAG_HIDDEN, false, &queue);
        out += &format!("      -> FLAG_SELECTED set/clear ok={} | FLAG_HIDDEN set ok={}\n",
                        sel_on && sel_off, hid_on);
    }

    // vmap membership: the tree treats a node as a clickable leaf only if its guid is in the vmap.
    // This is the fix site — every trimmed guid must be present (else no highlight / no tree-hide).
    let vmap = crate::state_ui::build_vmap(&session);
    let mut all_in = true;
    for guid in &guids {
        let present = vmap.contains_key(guid);
        if !present { all_in = false; }
        out += &format!("  vmap contains '{}' = {}\n", guid, present);
    }
    out += &format!("RESULT: all trimmed guids are tree leaves (selectable+hideable) = {}\n", all_in);

    // Sub-object edit pipeline: a trimmed surface edits its base m_surface (seam/boundary CV
    // moves) then re-tessellates the trim. Mirror reupload_edit_surface's core: perturb one
    // base CV and confirm the trim still meshes to a non-empty result (proves edit -> re-tess).
    let mut edit_ok = true;
    for ts in &mut session.objects.nurbssurfacetrimmeds {
        let cv = ts.m_surface.cv(0, 0).unwrap().to_vec();
        ts.m_surface.set_cv(0, 0, &session_rust::Point::new(cv[0] + 1.0, cv[1], cv[2]));
        let m = ts.mesh_render(20.0, 0.01);
        let ok = m.number_of_vertices() > 0 && m.number_of_faces() > 0;
        if !ok { edit_ok = false; }
        out += &format!("  edit-retessellate '{}' = {} (v={} f={})\n",
                        ts.guid(), ok, m.number_of_vertices(), m.number_of_faces());
    }
    out += &format!("RESULT: trimmed edit -> re-tessellate produces valid mesh = {}\n", edit_ok);

    // STRESS the commit path: a gumball drag moves a CV by large amounts. Re-run set_cv +
    // mesh_render with growing deltas on a multi-plane split part and watch for hang / NaN /
    // empty mesh (any of which would manifest as a viewer freeze on commit).
    if let Some(ts) = session.objects.nurbssurfacetrimmeds.iter_mut()
        .find(|t| !t.cut_planes.is_empty()) {
        let base = ts.m_surface.cv(0, 0).unwrap().to_vec();
        let mut worst = (0usize, 0usize, true);
        for step in 0..40 {
            let d = step as f64 * 25.0; // up to 1000 units, far past the surface size
            ts.m_surface.set_cv(0, 0, &session_rust::Point::new(base[0] + d, base[1] + d * 0.5, base[2] - d));
            let m = ts.mesh_render(20.0, 0.01);
            let nv = m.number_of_vertices();
            let mut finite = true;
            for vk in m.vertex.keys() {
                let p = m.vertex_point(*vk).unwrap();
                if !(p[0].is_finite() && p[1].is_finite() && p[2].is_finite()) { finite = false; break; }
            }
            worst = (nv.max(worst.0), m.number_of_faces().max(worst.1), worst.2 && finite);
        }
        out += &format!("STRESS '{}': 40 large-delta re-tess ok, peak v={} f={} all_finite={}\n",
                        ts.guid(), worst.0, worst.1, worst.2);
    }
    out
}
