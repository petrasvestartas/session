use crate::scene::{Scene,ObjectId};

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Rectangle {pub bounds:[f64;4],pub crossing:bool}

impl Rectangle {
    pub fn new(from:[f64;2],to:[f64;2])->Option<Self> {
        if from.into_iter().chain(to).any(|value|!value.is_finite()) {return None;}
        let bounds=[from[0].min(to[0]),from[1].min(to[1]),from[0].max(to[0]),from[1].max(to[1])];
        (bounds[0]<bounds[2]&&bounds[1]<bounds[3]).then_some(Self {bounds,crossing:to[0]<from[0]})
    }

    pub fn contains(&self,point:[f64;2])->bool {
        point.iter().all(|value|value.is_finite())&&point[0]>=self.bounds[0]&&point[0]<=self.bounds[2]&&point[1]>=self.bounds[1]&&point[1]<=self.bounds[3]
    }

    pub fn crosses(&self,a:[f64;2],b:[f64;2])->bool {
        if a.into_iter().chain(b).any(|value|!value.is_finite()) {return false;}
        let [left,top,right,bottom]=self.bounds;let [dx,dy]=[b[0]-a[0],b[1]-a[1]];let (mut low,mut high)=(0_f64,1_f64);
        for (p,q) in [(-dx,a[0]-left),(dx,right-a[0]),(-dy,a[1]-top),(dy,bottom-a[1])] {
            if p==0. {if q<0. {return false;}continue;}
            let t=q/p;if p<0. {low=low.max(t);} else {high=high.min(t);}
        }
        low<=high
    }
}

pub fn screen(matrix:&[f64;16],point:[f64;3],size:[f64;2])->Option<[f64;2]> {
    let clip=|row:usize|point.iter().enumerate().map(|(i,value)|matrix[i*4+row]*value).sum::<f64>()+matrix[12+row];let w=clip(3);
    if !w.is_finite()||w<=0.||size.iter().any(|value|!value.is_finite()||*value<=0.) {return None;}
    let point=[(clip(0)/w+1.)*size[0]/2.,(1.-clip(1)/w)*size[1]/2.];point.iter().all(|value|value.is_finite()).then_some(point)
}

