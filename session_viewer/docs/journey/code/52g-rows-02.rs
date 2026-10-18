pub struct PreparedChain { pub source: Source, pub points: Rc<Vec<[f32; 3]>>, pub colour: [f32; 4], pub width: f32, pub heads: u32 }

impl Source {
    pub fn name(&self)->&str {match self {Self::Line(source)=>&source.name,Self::Polyline(source)=>&source.name,Self::Curve(source,_)=>&source.name}}
    pub fn locked(&self) -> bool {
        match self { Self::Line(r) => r.is_locked, Self::Polyline(r) => r.is_locked, Self::Curve(r, _) => r.is_locked }
    }
