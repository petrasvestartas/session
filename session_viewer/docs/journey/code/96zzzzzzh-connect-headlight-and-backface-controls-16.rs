use crate::{editor::{Editor,Action,Change},commands::{Command,parse}};
fn command(e:&mut Editor,line:&str)->Result<Change,&'static str>{
    let action=if let Some(result)=e.tool_line(line){result?}else{let Command::Editor(action)=parse(line).map_err(|_|"Invalid command")?else{return Err("Editor command")};action};e.apply(action)
}
#[test]
fn commands_preserve_active_draft_originals_annotations_and_redo(){
    let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddBox).unwrap();let id=e.scene.objects()[0].id;
    e.select_objects([id],false);let source=e.scene.objects()[0].geometry().unwrap().to_proto();e.apply(Action::Translate([1.,0.,0.])).unwrap();let moved=e.scene.placement(id).unwrap();e.apply(Action::Undo).unwrap();
    command(&mut e,"Length BoundingBox").unwrap();let mark=e.mark.clone();assert!(mark.is_some());let epoch=e.interaction_epoch;
    for line in ["View Lighting On","View Backfaces On","View Lighting Off","View Backfaces Off"]{assert!(matches!(command(&mut e,line),Ok(Change::View)));assert_eq!(e.mark,mark);assert_eq!(e.interaction_epoch,epoch);assert_eq!(e.scene.objects()[0].geometry().unwrap().to_proto(),source);}
    command(&mut e,"Line").unwrap();command(&mut e,"w0,0,0").unwrap();let points=e.draft.as_ref().unwrap().points.clone();let epoch=e.interaction_epoch;
    for line in ["vIeW lIgHtInG oN","View Backfaces On","View Lighting","View Backfaces"]{command(&mut e,line).unwrap();assert_eq!(e.draft.as_ref().unwrap().points,points);assert_eq!(e.interaction_epoch,epoch);}
    assert!(!e.lit&&!e.backfaces);command(&mut e,"Escape").unwrap();e.apply(Action::Redo).unwrap();assert_eq!(e.scene.placement(id).unwrap().m,moved.m);assert_eq!(e.scene.objects()[0].geometry().unwrap().to_proto(),source);
}
#[test]
fn strict_view_options_and_completion_share_existing_registry(){
    let mut e=Editor::default();for prefix in ["View Lighting","View Backfaces"]{
        assert!(crate::commands::ACTIONS.iter().any(|(name,_)|*name==prefix));
        command(&mut e,&format!("{prefix} On")).unwrap();let before=[e.lit,e.backfaces];
        for suffix in ["No","On Off","On extra"]{assert!(command(&mut e,&format!("{prefix} {suffix}")).is_err());assert_eq!([e.lit,e.backfaces],before);}
        command(&mut e,&format!("{prefix} Off")).unwrap();
    }assert!(!e.lit&&!e.backfaces);
}
