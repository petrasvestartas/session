    pub fn scene_mut(&mut self)->&mut Scene {match self {Self::Whole(p)=>&mut p.scene,Self::Part(p)=>&mut p.scene}}
    pub fn scenes(&self)->[&Scene;2] {