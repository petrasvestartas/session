        crate::range_page::Page::decode(rows,self.fields.count,&raw,ids.as_deref()).map_err(str::to_owned)
    }

}

