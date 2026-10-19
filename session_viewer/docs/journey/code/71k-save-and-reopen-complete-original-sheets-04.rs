    for source in &loaded.document.objects.breps {add!(source,scene.insert_prepared_cad(shapes.prepare(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.pointclouds {let id=add!(source,scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).map_err(str::to_owned)?));scene.cloud_point_size(id,point_px).map_err(str::to_owned)?;}
    for source in &loaded.sheets {let original=&source.original;let model=crate::nested_placement::model(&loaded.world,place,&original.guid);let id=scene.insert_sheet(source.clone()).map_err(str::to_owned)?;scene.place_following(id,model).map_err(str::to_owned)?;owners.push((original.guid.clone(),id,original.name.clone()));}
    let templates=crate::instance_definition::templates(loaded.document.clone(),&mut shapes).map_err(str::to_owned)?;let mut used=scene.used_guids();
    for instance in &loaded.document.objects.instances {
