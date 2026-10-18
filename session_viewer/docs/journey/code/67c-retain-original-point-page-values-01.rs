use std::ops::Range;

pub struct Page {
    pub rows:Range<u32>,
    pub coordinates:Vec<f64>,
    pub display:Vec<f32>,
    pub ids:Vec<u32>,
}
impl Page {
    pub fn decode(rows:Range<u32>,total:u32,raw:&[u8],ids:Option<&[u8]>)->Result<Self,&'static str> {
        if rows.start>rows.end||rows.end>total {return Err("Point rows outside source");}
        let count=(rows.end-rows.start) as usize;
        if raw.len()!=count.checked_mul(24).ok_or("Page size overflow")? {return Err("Incomplete original coordinate page");}
        let coordinates=raw.chunks_exact(8).map(|v|f64::from_le_bytes(v.try_into().unwrap())).collect::<Vec<_>>();
        if coordinates.iter().any(|v|!v.is_finite()||!(*v as f32).is_finite()) {return Err("Invalid source coordinates");}
        let ids=if let Some(ids)=ids {
            if ids.len()!=count.checked_mul(4).ok_or("ID page size overflow")? {return Err("Incomplete original point ID page");}
            ids.chunks_exact(4).map(|v|u32::from_le_bytes(v.try_into().unwrap())).collect::<Vec<_>>()
        } else {rows.clone().collect()};
        if ids.iter().copied().collect::<std::collections::HashSet<_>>().len()!=count {return Err("Repeated original point IDs");}
        let display=coordinates.iter().map(|v|*v as f32).collect();Ok(Self {rows,coordinates,display,ids})
    }
    pub fn bytes(&self)->usize {self.coordinates.capacity()*8+self.display.capacity()*4+self.ids.capacity()*4}
}
