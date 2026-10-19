        assert!(previously_clipped > 0, "This fixture must exercise grid clipping under a small-scene range");
    }
}

// A deliberately narrow scene-only interval is the positive clipping control.
fn scene_only_depth_control(camera:&Camera)->session_rust::Xform {
    use session_rust::{Point,Vector,Xform};
    let forward=camera.orientation.rotate_vector(Vector::y_axis());let up=camera.orientation.rotate_vector(Vector::z_axis());
    let target=Point::new(camera.target[0],camera.target[1],camera.target[2]);let eye=&target-&(&forward*camera.distance);
    let view=Xform::look_at_right_handed(&eye,&target,&up);let range=camera.distance+2.*camera.radius;
    let projection=match camera.projection {
        Projection::Perspective=>Xform::perspective(std::f64::consts::PI/3.,camera.aspect,(camera.distance*10.).max(range),camera.distance*1e-4),
        Projection::Orthographic=>{let h=camera.distance*(std::f64::consts::PI/6.).tan();let w=h*camera.aspect;Xform::orthographic(-w,w,-h,h,range,-range)},
    };projection*view
}
