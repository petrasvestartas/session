1.,0.]};renderer.set_grid_on(Some(crate::grid::Settings{spacing:0.1,half_cells:1,..Default::default()}),plane).unwrap();assert_eq!(red_pixels(&renderer,size),baseline,"coplanar red construction grid cannot paint over the solid");let first=renderer.tile_usage();renderer.resize(crate::viewport::Viewport{width:319,height:239});let _=red_pixels(&renderer,[319,239]);assert_eq!(renderer.tile_usage()[5],first[5]+1,"physical viewport change invalidates projection even with the same tile dimensions");
}

#[test]
fn a_point_on_a_sloping_original_face_keeps_its_disc_and_first_pick(){
 for density in [1u32,2]{
  let size=[320*density,240*density];let(front,_,_)=scene(0.3,2,false);let mut renderer=renderer(&front,size,density as f32);let flat=red_pixels(&renderer,size);
  let(sloped,_,point)=scene(0.5,2,true);renderer.set_scene(&sloped,None);
  assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],point.value(),"surface-fitted point owns the first pick before color");
  assert_eq!(red_pixels(&renderer,size),flat,"a coplanar point keeps the same full CSS disc on a sloping face");
  let(hidden,solid,_)=scene(0.7,2,true);renderer.set_scene(&hidden,None);assert_eq!(picked(&mut renderer,[0.,0.]).unwrap()[0],solid.value());assert_eq!(red_pixels(&renderer,size),0);
 }
}
