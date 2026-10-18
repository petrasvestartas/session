use crate::{editor::Action,primitive_shape::Shape};

pub const ENTRIES:&[(&str,&Shape,usize)]=&[
    ("Box",&crate::primitive_box::SHAPE,0),
    ("Box Brep",&crate::primitive_box::SHAPE,0),
    ("Box Mesh",&crate::primitive_box::SHAPE,1),
    ("Pyramid",&crate::primitive_pyramid::SHAPE,0),
    ("Pyramid Brep",&crate::primitive_pyramid::SHAPE,0),
    ("Pyramid Mesh",&crate::primitive_pyramid::SHAPE,1),
    ("Block With Hole",&crate::primitive_block_with_hole::SHAPE,0),
    ("Block With Hole Brep",&crate::primitive_block_with_hole::SHAPE,0),
    ("Block With Hole Mesh",&crate::primitive_block_with_hole::SHAPE,1),
    ("Tetrahedron",&crate::primitive_tetrahedron::SHAPE,0),
    ("Tetrahedron Brep",&crate::primitive_tetrahedron::SHAPE,0),
    ("Tetrahedron Mesh",&crate::primitive_tetrahedron::SHAPE,1),
    ("Octahedron",&crate::primitive_octahedron::SHAPE,0),
    ("Octahedron Brep",&crate::primitive_octahedron::SHAPE,0),
    ("Octahedron Mesh",&crate::primitive_octahedron::SHAPE,1),
    ("Icosahedron",&crate::primitive_icosahedron::SHAPE,0),
    ("Icosahedron Brep",&crate::primitive_icosahedron::SHAPE,0),
    ("Icosahedron Mesh",&crate::primitive_icosahedron::SHAPE,1),
    ("Dodecahedron",&crate::primitive_dodecahedron::SHAPE,0),
    ("Dodecahedron Brep",&crate::primitive_dodecahedron::SHAPE,0),
    ("Dodecahedron Mesh",&crate::primitive_dodecahedron::SHAPE,1),
    ("bo",&crate::primitive_box::SHAPE,0),
];

pub fn command(name:&str,text:&str)->Option<Action> {ENTRIES.iter().find(|(key,_,_)|*key==name).map(|(_,shape,option)|Action::PrimitiveBegin {shape,option:*option,text:text.to_owned()})}
