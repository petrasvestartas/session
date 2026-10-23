    for source in &loaded.document.objects.breps {add!(source,scene.insert_prepared_cad(shapes.prepare(source.clone()).map_err(str::to_owned)?));}
    for source in &loaded.document.objects.pointclouds {let id=add!(source,scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).map_err(str::to_owned)?));scene.cloud_point_size(id,point_px).map_err(str::to_owned)?;}
    for source in &loaded.sheets {let original=&source.original;let model=crate::nested_placement::model(&loaded.world,place,&original.guid);let id=scene.insert_sheet(source.clone()).map_err(str::to_owned)?;scene.place_following(id,model).map_err(str::to_owned)?;owners.push((original.guid.clone(),id,original.name.clone()));}
    for source in loaded.document.objects.components.iter().filter(|source|crate::production_matrix_walk::is_matrix(source)) {
        let model=crate::nested_placement::model(&loaded.world,place,&source.guid);
        let id=scene.insert_matrix(std::rc::Rc::new(source.clone())).map_err(str::to_owned)?;
        scene.place_matrix(id,model).map_err(str::to_owned)?;owners.push((source.guid.clone(),id,source.name.clone()));
    }
    let templates=crate::instance_definition::templates(loaded.document.clone(),&mut shapes).map_err(str::to_owned)?;let mut used=scene.used_guids();
    for instance in &loaded.document.objects.instances {
        let template=&templates[&instance.definition_guid];let mut guid=instance.guid().to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}
