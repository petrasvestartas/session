        if matches!(target.part,Part::SourceControl(_)) {
            let original=crate::source_part::geometry(self,target.owner)?;
            let edited=crate::source_deform::transform(&original,crate::source_part::target(target.part)?,&local).map_err(|_|"Source control edit would invalidate geometry")?;
            return self.replace_original(target.owner,edited);
        }
        if self.objects.iter().any(|row|row.id==target.owner&&(row.surface.is_some()||row.cad.is_some())) {