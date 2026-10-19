    }

    pub async fn cloud_properties(url:&str,fields:&CloudFields)->Option<crate::stream_properties::Properties> {
        let mut properties=crate::stream_properties::Properties::default();let mut window=MetadataWindow::default();
        let mut at=body_end(fields.coords_at,fields.coords_len,fields.end)?;
        while at<fields.end {
            let header=window.read(url,at,64.min(fields.end-at),fields.end,&fields.revision).await?;
            let f=field_at(header,at,fields.end)?;
            let bytes=if (f.field,f.wire)==(6,1) {window.read(url,f.body,8,fields.end,&fields.revision).await?} else {&[]};
            if !properties.field(&f,bytes) {return None;}at=f.next;
        }Some(properties)
    }

    /// Points `[from, to)` of the cloud.
    pub async fn fetch_positions(
