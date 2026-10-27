        }
        let(dot_styles,glyph_flags)=if dots.is_empty(){(Vec::new(),0)}else{crate::production_glyph::prepare(source)};
        if dot_styles.len()!=dots.len(){return Err("Original marker metadata does not match source addresses");}
        Ok(Self { ink:crate::production_mesh_ink::mesh(source,count),dots,dot_styles,glyph_flags,dot_colours,pens,faces, triangles, edges, endpoints })
    }

    pub fn pen(&self,index:usize)->(f32,[f32;4]) {
