    }
    fn buttons(&self)->Vec<(String,String)> {
        if matches!(self,Self::Project){return vec![("Cancel".into(),"Escape".into())]}
        if matches!(self,Self::Extend){return vec![("Distance".into(),"Distance".into()),("Next / Done".into(),"".into()),("Cancel".into(),"Escape".into())]}
        if let Self::Gather(recipe)=self{return recipe.chips.iter().map(|(label,line)|((*label).into(),(*line).into())).collect()}
        if let Self::Transform(kind)=self {return crate::transform_draft::Draft::new(*kind,crate::cplane::CPlane::Xy).buttons();}
