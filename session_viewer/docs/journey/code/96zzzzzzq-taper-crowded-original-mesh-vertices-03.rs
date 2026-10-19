        if !parts.dots.is_empty(){
            let mut b=session_rust::AABB::empty();for p in display.vertices(){b.union_with_point(p[0]as f64,p[1]as f64,p[2]as f64);}
            display.spacing=(b.diagonal().min(f32::MAX as f64)as f32)/(mesh.number_of_vertices()as f32).sqrt();
        }
        display.parts = parts; Ok(display)