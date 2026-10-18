        if matches!(&action, Action::Close | Action::Undo | Action::Redo | Action::Adopt {..} | Action::Live(_) | Action::Import(_) | Action::Replace(_) | Action::ImportAt(_, _) | Action::ReplaceAt(_, _)) { self.draft = None;self.plane_menu=false;self.plane_only=false; }
        match action {
            Action::ScaleFactor(factor)=>{
                if !factor.is_finite()||factor<=0.{return Err("Scale wants a finite factor above zero");}
                let origin=self.gizmo_origin().unwrap_or([0.;3]);let delta=session_rust::Xform::scale_uniform(&session_rust::Point::new(origin[0],origin[1],origin[2]),factor);let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Scale,&delta,&ids,target);
            },
            Action::RotateWorld{axis,degrees}=>{
                if !degrees.is_finite(){return Err("Rotate needs finite degrees");}let origin=self.gizmo_origin().unwrap_or([0.;3]);let(o,p)=(session_rust::Xform::translation(origin[0],origin[1],origin[2]),session_rust::Xform::translation(-origin[0],-origin[1],-origin[2]));let turn=match axis{crate::gizmo::Axis::X=>session_rust::Xform::rotation_x(degrees,true),crate::gizmo::Axis::Y=>session_rust::Xform::rotation_y(degrees,true),crate::gizmo::Axis::Z=>session_rust::Xform::rotation_z(degrees,true)};let delta=&(&o*&turn)*&p;let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(crate::transform_draft::Kind::Rotate,&delta,&ids,target);
            },
            Action::BeginTransformation(kind)=>return self.begin_transform(kind),
            Action::Transformation {kind,delta}=>{let ids=self.selected_objects();let target=self.selection().filter(|_|self.selected_part.is_some());return self.apply_transform(kind,&delta,&ids,target);},
