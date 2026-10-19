    let ndc = clip.xyz / clip.w;

    if (ndc.z < 0.0 || ndc.z > 1.0 || clip_cut_ndc(0u,ndc)) {
        return s;
    }
