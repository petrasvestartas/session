    pub fn clip(&self,point:[f64;3])->[f64;4] {
        let point=std::array::from_fn::<_,3,_>(|i|point[i]-self.origin[i]);
        let clip: [f64; 4] = std::array::from_fn(|row| point.iter().enumerate().fold(self.matrix[12+row], |sum,(i,v)| sum+self.matrix[i*4+row]*v));
        clip
    }
    pub fn point(&self,point:[f64;3])->Option<[f64;2]> {
        let clip=self.clip(point);
        if clip.iter().any(|v|!v.is_finite())||clip[3]<=1e-9||clip[2]<0.||clip[2]>clip[3] {return None;}
        self.pixel(clip)
    }
    pub fn unclipped(&self,point:[f64;3])->Option<[f64;2]> {
        let clip=self.clip(point);if !clip[3].is_finite()||clip[3]<=0. {return None;}
        self.pixel(clip)
    }
    fn pixel(&self,clip:[f64;4])->Option<[f64;2]> {
        if self.size.iter().any(|v|!v.is_finite()||*v<=0.) {return None;}
        let pixel=[(clip[0]/clip[3]+1.)*self.size[0]/2., (1.-clip[1]/clip[3])*self.size[1]/2.];
        pixel.iter().all(|v| v.is_finite()).then_some(pixel)
    }