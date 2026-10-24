const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
module.exports=id=>{
 const folder=path.resolve('target','course-'+id,'src'),read=name=>fs.readFileSync(path.join(folder,name),'utf8').replaceAll('\r\n','\n');
 const input=read('file_input.rs'),document=read('document.rs'),browser=read('browser.rs');
 const legacy=input.includes('pub fn action(event: &web_sys::Event, delivery: &Delivery) -> Result<Option<crate::editor::Action>, JsValue>')
  && input.includes('crate::editor::Action::ReplaceAt(array.to_vec(), location)')
  && document.includes('proto::Session::decode(bytes).map_err(|_| "Invalid session protobuf")')
  && browser.includes('report(&error);\n                    if event.type_() == "viewer-file" { panel.answer("Open", &error); }'.replaceAll('\\n','\n'));
 const ownedInput=input.includes('delivery: &Delivery, scene: &crate::scene::Scene) -> Result<Option<crate::editor::Action>, JsValue>')
  && input.includes('crate::publication::prepare(scene,&bytes,replace,Some(location)).map_err(|error|JsValue::from_str(&error))?')
  && browser.includes('format!("Cannot open file: {error:?}")');
 const owned=ownedInput
  && read('publication.rs').includes('crate::scene_document::import(&mut candidate,bytes,')
  && read('scene_document.rs').includes('crate::document_source::prepare(bytes,place,point_px)?')
  && read('document_source.rs').includes('proto::Session::decode(bytes).map_err(|e|e.to_string())?');
 const measuredOwned=ownedInput
  && read('publication.rs').includes('crate::scene_document::import_at(&mut candidate,bytes,"",&session_rust::Xform::identity(),0.,location)?')
  && read('scene_document.rs').includes('crate::document_source::prepare_at(bytes,place,point_px,location)?')
  && read('document_source.rs').includes('prepare_observed(bytes,place,point_px,location,&mut crate::load_measure::now,&mut observe)')
  && read('document_source.rs').includes('run_result("decode",size,clock,observe,||proto::Session::decode(bytes).map_err(|_|"Invalid session protobuf".to_owned()))?');
 assert.equal(Number(legacy)+Number(owned)+Number(measuredOwned),1,'Unsupported or ambiguous actual frozen browser decode path; refuse text fallback');
 return measuredOwned?'owned-measured-static-document-decode':owned?'owned-browser-decode':'legacy-static-document-decode';
};
