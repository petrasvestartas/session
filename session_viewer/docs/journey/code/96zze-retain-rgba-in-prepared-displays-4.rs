    pub fn with_alphas(mut self, alphas:Vec<f32>)->Result<Self,&'static str>{
        if alphas.len()!=self.vertices.len()||alphas.iter().any(|v|!v.is_finite()){
            return Err("Each display vertex needs finite alpha");
        }
        self.alphas=alphas;Ok(self)
    }
    pub fn alphas(&self)->&[f32]{&self.alphas}
    pub fn normals(&self) -> &[[f32; 3]] { &self.normals }