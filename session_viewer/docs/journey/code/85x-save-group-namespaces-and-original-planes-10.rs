        }

        for item in &objects.planes {if !crate::plane_original::valid(&session_rust::Plane::from_proto(item.clone())) {return Err("Invalid original plane frame".into())}}
        for item in &objects.bboxes {if !crate::bounds_original::valid(&session_rust::OBB::from_proto(item.clone()).map_err(|e|e.to_string())?){return Err("Invalid original box frame".into())}}
        for item in &objects.points {
