            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len()+objects.components.len()+objects.sheets.len()+objects.instances.len();

        if count > MAX_OBJECTS {
