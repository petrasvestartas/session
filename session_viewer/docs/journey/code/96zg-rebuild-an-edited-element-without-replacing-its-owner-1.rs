            Geometry::Element(source)=>{
                let mut staging=Self::empty();
                let prepared=crate::element_original::insert(&mut staging,source)?;
                let mut replacement=staging.objects().iter().find(|row|row.id==prepared).ok_or("Prepared Element is missing")?.clone();
                for vertex in replacement.mesh.vertices(){
                    let point=model.transform_point(&session_rust::Point::new(vertex[0]as f64,vertex[1]as f64,vertex[2]as f64));
                    if (0..3).any(|i|!(point[i]as f32).is_finite()){return Err("Edited Element exceeds display range");}
                }
                let row=self.objects.iter_mut().find(|row|row.id==id).ok_or("Element owner is missing")?;
                replacement.id=row.id;replacement.guid=row.guid.clone();
                replacement.metadata=row.metadata.clone();replacement.model=model;
                *row=replacement;
            },
            Geometry::Plane(source)=>{let prepared=crate::plane_original::prepared(&source)?;