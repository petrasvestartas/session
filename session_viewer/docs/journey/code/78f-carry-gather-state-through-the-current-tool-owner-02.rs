    }
    fn buttons(&self)->Vec<(String,String)> {
        if let Self::Gather(recipe)=self{return recipe.chips.iter().map(|(label,line)|((*label).into(),(*line).into())).collect()}
        if let Self::Transform(kind)=self {return crate::transform_draft::Draft::new(*kind,crate::cplane::CPlane::Xy).buttons();}
        if let Self::Primitive(shape)=self {return shape.options.iter().map(|(label,line)|((*label).into(),(*line).into())).collect();}
