use crate::browser_defaults::Settings;
#[test]
fn pointer_defaults_and_url_overrides_share_existing_editor_state(){
 let mut editor=crate::editor::Editor::default();
 for (coarse,raw,nogrid,opacity,grid) in [
 (false,None,None,0.95,true),(true,None,None,1.,true),
 (true,Some("0.5"),Some("1"),0.5,false),
 (false,Some("NaN"),None,0.95,true),(true,Some("inf"),None,1.,true),
 (false,Some("2"),None,1.,true),(false,Some("-1"),None,0.,true)]{
  Settings::new(coarse,raw,nogrid).apply(&mut editor);
  assert_eq!(editor.opacity,opacity);assert_eq!(editor.camera.grid.is_some(),grid);
 }
}
