    pub fn dot_colour(&self,index:usize)->[f32;4]{
        self.dot_colours.get(if self.dot_colours.len()==1{0}else{index}).copied().unwrap_or([0.1,0.1,0.1,1.])
    }

    pub fn bytes(&self) -> usize {