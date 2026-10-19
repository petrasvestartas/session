        let lod=crate::stream::cloud_lod(&url,&mut fields).await.ok_or("Invalid original cloud array or octree layout")?;
        let properties=crate::stream::cloud_properties(&url,&fields).await.ok_or("Invalid original cloud presentation fields")?;
        let total=probe.total.ok_or("Missing original file size")?;
        let length=total.checked_sub(fields.end).ok_or("Cloud exceeds its original file")?;
        if length>1024*1024 {return Err("Original source metadata exceeds its range limit".into());}
        if length>0 {
            let raw=crate::stream::source_range(&url,fields.end,length,&fields.revision).await.ok_or("Original source metadata range failed")?;
            crate::range_shape::metadata(&raw,&guid)?;
        }
        Ok(Self {properties,guid,name,url,fields,lod})
    }
