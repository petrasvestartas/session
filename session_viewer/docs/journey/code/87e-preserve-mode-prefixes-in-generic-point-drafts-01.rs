pub trait Tool: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn command_prefix(&self)->String{self.name().into()}
    fn point_prompt(&self,_index:usize)->Option<String>{None}
    fn count(&self) -> usize;
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str>;
