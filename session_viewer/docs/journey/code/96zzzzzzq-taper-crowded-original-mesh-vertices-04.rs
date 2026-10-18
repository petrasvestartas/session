impl Marker {
    // The signed GPU size is a CSS diameter, or negative mesh spacing in mm.
    pub fn mesh_dot(center:[f32;3],colour:[f32;4],spacing:f32)->Self{
        Self{center,colour,diameter:if spacing>0.{-spacing}else{1.}}
    }
