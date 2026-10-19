et_pointcolors(vec![red.clone(), green.clone(), blue.clone(), red, green, blue]),
        Mode::Faces => mesh.set_facecolors(vec![red, blue]),
    }
    mesh
}

pub fn selection()->[f32;4]{Color::yellow().to_unified_array()}
pub fn selection_shader()->String{
    let [r,g,b,_]=selection();
    format!("const SELECTION_COLOR: vec3<f32> = vec3<f32>({r:?}, {g:?}, {b:?});")
}
