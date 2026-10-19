#[cfg(target_arch="wasm32")]
pub struct Source {pub guid:String,pub name:String,pub url:String,pub fields:crate::stream::CloudFields,pub lod:crate::stream::CloudLod}

#[cfg(target_arch="wasm32")]
impl Source {
    pub async fn open(url:String)->Result<Self,String> {
        let probe=crate::stream::probe(&url).await.ok_or("Source probe failed")?;
        if probe.etag.is_none()&&!crate::manifest::immutable_key(&url) {return Err("Streaming requires an ETag or immutable source URL".into());}
        let (guid,name)=crate::stream::cloud_identity(&probe.bytes).ok_or("Missing original cloud identity")?;
        let mut fields=crate::stream::cloud_fields(&url,&probe).await.ok_or("Not a single range-readable cloud")?;
        let lod=crate::stream::cloud_lod(&url,&mut fields).await.ok_or("Invalid original cloud array or octree layout")?;
        Ok(Self {guid,name,url,fields,lod})
    }
}
