        if object.geometry().is_none() { return Err("Reload editable sources before Move"); }
        object.model = model;
