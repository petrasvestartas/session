    let object=object_at(instance);
    var output=geometry(input.position,input.colour,input.normal,object);
    output.primitive=(object.selection.x>>1u)+occurrence/3u+1u;
    return output;