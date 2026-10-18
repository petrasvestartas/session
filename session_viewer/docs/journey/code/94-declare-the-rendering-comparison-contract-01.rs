use prost::Message;
use session_rust::{BRep,Color,Session};

pub struct Difference {
    pub subject:&'static str,
    pub teaching:&'static str,
    pub maintained:&'static str,
    pub owner:&'static str,
}

pub const DIFFERENCES:&[Difference]=&[
    Difference{subject:"camera",teaching:"forward depth, scene units",maintained:"reversed depth, f64 camera, mm to metres and anchor rebasing",owner:"src/camera.rs"},
    Difference{subject:"surfaces",teaching:"per-owner glass drawing, separate normal prepass",maintained:"packed arenas, primitive IDs, MRT and adaptive 1x/4x MSAA",owner:"src/engine/gpu/render.rs"},
    Difference{subject:"ink",teaching:"finite tile lists and centreline depth predicate",maintained:"plane-fit visibility, sag profiles and ribbon convolution",owner:"src/shaders/ink_visibility.wgsl"},
    Difference{subject:"ambient",teaching:"current GTAO with an explicit forward-depth adapter",maintained:"current reversed-depth inputs and production pass order",owner:"src/engine/gpu/ssao.rs"},
    Difference{subject:"defaults",teaching:"white background, opacity 1, no ground grid",maintained:"linear 0.9 background, desktop opacity 0.95, grid on",owner:"src/engine/gpu/view.rs"},
];

pub fn specimen()->Vec<u8>{
    let mut session=Session::new("viewer comparison");
    for(mut brep,z)in[(BRep::create_box(1500.,1500.,100.),-500.),(BRep::create_box(350.,350.,700.),-100.),(BRep::create_cylinder(120.,450.),250.),(BRep::create_cylinder(650.,120.),700.)]{
        brep.surfacecolor=Color::new(0.35,0.65,0.85,1.);
        brep.transform(&session_rust::Xform::translation(0.,0.,z));
        session.add_brep(brep,None);
    }
    session.to_proto().encode_to_vec()
}

pub fn original_scene(bytes:&[u8])->Result<crate::scene::Scene,String>{
    let mut scene=crate::scene::Scene::empty();
    let source=crate::document_source::prepare(bytes,&session_rust::Xform::identity(),0.)?;
    crate::document_geometry::append(&mut scene,&source,&session_rust::Xform::identity(),0.)?;
    Ok(scene)
}
