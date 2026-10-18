    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
    fn build_on_plane(&self,points:&[[f64;3]],_plane:crate::cplane::CPlane)->Result<Built,&'static str> {self.build(points)}
    fn outline(&self,points:&[[f64;3]],_plane:crate::cplane::CPlane)->Vec<[f64;3]> {points.to_vec()}
    fn sides(&self)->Option<usize> {None}
    fn set_sides(&mut self,_sides:usize)->Result<(),&'static str> {Err("Sides works while drawing a polygon")}
    fn buttons(&self) -> Vec<(String, String)> { vec![("Cancel".into(), "Escape".into())] }
}
