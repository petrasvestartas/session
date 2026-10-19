    let ortho = v.view_proj.ortho_half_height() > 0.0;
    let eye = v.view_proj.eye();
    let sign=if v.forward_depth{-1.0}else{1.0};
    let toward = [sign*m[2],sign*m[6],sign*m[10]];
    // scene units per pixel at the anchor; the hatch phase wraps far above that
    let rows = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt();
