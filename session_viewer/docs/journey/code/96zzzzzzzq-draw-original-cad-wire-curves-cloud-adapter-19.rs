    crate::production_cloud::SplatRecord {
      mvp_model: m,
      tint: [rgb[0], rgb[1], rgb[2], (px * 0.5).max(0.5)],
      first: range.first - row.rows.start,
      count: range.count,
      cum: cumulative,
      k: radius_factor(range, px, scale, view.ortho_h),
      rot,
      nrm_first: if draw.columns.has_normals { range.first - row.rows.start } else { u32::MAX },
      instance,
      flags,
      selected_point: 0,
    }.append(&mut bytes);