#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Param{pub name:&'static str,pub value:f64}
pub struct Made{pub source:Rc<Element>,pub plane:Option<Plane>}
fn items<T:WoodElement>(items:Vec<T>)->Vec<Made>{items.into_iter().map(|v|{let mut plane=v.base_plane();if let Some(p)=&mut plane{p.name="base_plane".into();p.is_visible=false;p.linecolor=session_rust::Color::black();}Made{source:Rc::new(v.to_element()),plane}}).collect()}
pub fn build(kind:Kind,values:&[Param],sources:&[(Geometry,session_rust::Xform)])->Result<Vec<Made>,String>{
 let curves=sources.iter().filter_map(|(g,m)|match g{Geometry::Line(l)=>{let l=l.transformed(m);Some(Polyline::new(vec![l.start(),l.end()]))},Geometry::Polyline(p)=>Some(p.transformed(m)),_=>None}).collect::<Vec<_>>();
 let loops=curves.iter().filter(|p|p.is_closed()&&p.point_count()>=4).cloned().collect::<Vec<_>>();
 Ok(match kind{
  Kind::Beam=>items(crate::element_beam::build(&curves,values[0].value,values[1].value)?),
  Kind::Column=>items(crate::element_column::build(&curves,values[0].value,values[1].value)?),
  Kind::Plate=>items(crate::element_plate::build(&loops,values[0].value)?),
  Kind::Variable=>items(vec![crate::element_beam_variable::build(loops)?]),
  Kind::Block=>items(vec![crate::element_block::build(loops)?]),
  Kind::Support=>{let planes=sources.iter().filter_map(|(g,m)|match g{Geometry::Point(p)=>Some(Plane::new(p.transformed(m),session_rust::Vector::x_axis(),session_rust::Vector::y_axis())),Geometry::Plane(p)=>Some(p.transformed(m)),_=>None}).collect::<Vec<_>>();items(crate::element_support::build(&planes)?)},
 })
}
