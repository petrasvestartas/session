    for source in &loaded.document.objects.breps {add!(source,scene.insert_cad(source.clone()));}
    for source in &loaded.document.objects.pointclouds {let id=add!(source,scene.insert_cloud(crate::cloud::Prepared::new(source.clone()).map_err(str::to_owned)?));scene.cloud_point_size(id,point_px).map_err(str::to_owned)?;}
    let templates=crate::instance_definition::templates(loaded.document.clone()).map_err(str::to_owned)?;let mut used=scene.used_guids();
    for instance in &loaded.document.objects.instances {
        let template=&templates[&instance.definition_guid];let mut guid=instance.guid().to_owned();while !used.insert(guid.clone()) {guid=uuid::Uuid::new_v4().to_string();}
        // The kernel folds legacy instance placement into Session.xforms; combine that canonical placement with tree ancestry once, without another instance transform.
        let model=place*&crate::hierarchy::world(loaded.tree.as_ref(),&loaded.document.xforms,instance.guid());
        let owner=crate::instance_plan::Owner::new(loaded.document.clone(),instance.clone()).map_err(str::to_owned)?;
        let id=scene.append_instance(template,owner,model,guid,point_px).map_err(str::to_owned)?;owners.push((instance.guid().to_owned(),id,instance.name.clone()));
    }
    Ok(owners)
}
