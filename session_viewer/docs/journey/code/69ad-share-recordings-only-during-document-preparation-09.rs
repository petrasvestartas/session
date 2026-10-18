}

pub fn templates(document:Rc<session_rust::Session>,shapes:&mut crate::cad_shape_cache::Cache)->Result<HashMap<String,Scene>,&'static str> {
    let mut templates=HashMap::new();
    for instance in &document.objects.instances {
        if !templates.contains_key(&instance.definition_guid) {
            let owner=Owner::new(document.clone(),instance.clone())?;templates.insert(instance.definition_guid.clone(),prepare(&owner,shapes)?);
        }
    }
