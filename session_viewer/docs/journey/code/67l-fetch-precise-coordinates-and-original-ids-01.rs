        Ok(Self {guid,name,url,fields,lod})
    }
    pub async fn page(&self,rows:std::ops::Range<u32>)->Result<crate::range_page::Page,String> {
        if rows.start>rows.end||rows.end>self.fields.count {return Err("Point rows outside source".into());}
        let count=u64::from(rows.end-rows.start);
        let at=self.fields.coords_at.checked_add(u64::from(rows.start)*24).ok_or("Coordinate offset overflow")?;
        let raw=crate::stream::source_range(&self.url,at,count*24,&self.fields.revision).await.ok_or("Coordinate range failed")?;
        let ids=if self.fields.ids_len==0 {None} else {
            let at=self.fields.ids_at.checked_add(u64::from(rows.start)*4).ok_or("ID offset overflow")?;
            Some(crate::stream::source_range(&self.url,at,count*4,&self.fields.revision).await.ok_or("Original ID range failed")?)
        };
        crate::range_page::Page::decode(rows,self.fields.count,&raw,ids.as_deref()).map_err(str::to_owned)
    }
}
