
impl TileLayout {
    pub fn dimensions(self)->[u32;2]{[self.width,self.height]}
    /// Grid for a canvas size; tiles grow until the count fits.
    pub fn new(size: (u32, u32)) -> Self {
