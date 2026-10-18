    }
    pub async fn from_probe(url:String,probe:&crate::range_fetch::Reply)->Result<Self,String> {
        if !crate::range_shape::single_cloud(&probe.bytes) {return Err("Ranged source must contain one original cloud".into());}
        if probe.etag.is_none()&&!crate::manifest::immutable_key(&url) {return Err("Streaming requires an ETag or immutable source URL".into());}
        let (guid,name)=crate::stream::cloud_identity(&probe.bytes).ok_or("Missing original cloud identity")?;
