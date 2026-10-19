                markers.push(crate::marker::Marker { diameter: marker.display.diameter + 4., colour: yellow, ..marker.display });
            }
            Part::CloudPoint(id)=>{
                let p=crate::cloud_pick::position(scene,target.owner,id).ok_or("Cloud point is missing")?;
                markers.push(crate::marker::Marker {center:p.to_f32(),diameter:11.,colour:yellow});
            }
            Part::Object => {}
        } }
