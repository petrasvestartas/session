        crate::range_page::Page::decode(rows,self.fields.count,&raw,ids.as_deref()).map_err(str::to_owned)
    }
    pub async fn page_attributes(&self,rows:std::ops::Range<u32>,color_at:u64)->Result<(crate::range_page::Page,u64),String> {
        let page=self.page(rows.clone()).await?;let count=rows.end-rows.start;
        let (colors,next)=if self.fields.colors_len==0 {(Vec::new(),color_at)} else {
            let (colors,used)=crate::stream::fetch_colors(&self.url,&self.fields,color_at,count).await.ok_or("Source colour range failed")?;(colors,used)
        };
        let normals=if self.fields.normals_len==0 {Vec::new()} else {crate::stream::fetch_normals(&self.url,&self.fields,rows.start,rows.end).await.ok_or("Source normal range failed")?};
        Ok((page.attributes(colors,normals).map_err(str::to_owned)?,next))
    }

}
