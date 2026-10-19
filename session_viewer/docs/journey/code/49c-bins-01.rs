use std::collections::HashMap;

// Small screen cells bound nearby queries; segments spanning too many cells stay in one conservative fallback list.
#[derive(Default)]
pub struct Bins { cells: HashMap<(i32,i32),Vec<usize>>, large: Vec<usize> }

impl Bins {
    const CELL: f64 = 64.;
    pub fn insert(&mut self, id: usize, ends: Option<[[f64; 2]; 2]>) {
        let Some([a,b])=ends else { self.large.push(id); return; };
        let low=std::array::from_fn::<_,2,_>(|i| (a[i].min(b[i])/Self::CELL).floor() as i32);
        let high=std::array::from_fn::<_,2,_>(|i| (a[i].max(b[i])/Self::CELL).floor() as i32);
        let width=high[0] as i64-low[0] as i64+1; let height=high[1] as i64-low[1] as i64+1;
        if width>64 || height>64 || width*height>64 { self.large.push(id); return; }
        for y in low[1]..=high[1] { for x in low[0]..=high[0] { self.cells.entry((x,y)).or_default().push(id); } }
    }
    pub fn near(&self, cursor: [f64; 2], radius: f64) -> Vec<usize> {
        let mut out=self.large.clone();
        let low=cursor.map(|v| ((v-radius)/Self::CELL).floor() as i32); let high=cursor.map(|v| ((v+radius)/Self::CELL).floor() as i32);
        for y in low[1]..=high[1] { for x in low[0]..=high[0] { if let Some(ids)=self.cells.get(&(x,y)) { out.extend(ids); } } }
        out.sort_unstable(); out.dedup(); out
    }
}
