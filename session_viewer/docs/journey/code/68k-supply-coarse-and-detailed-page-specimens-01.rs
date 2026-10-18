use std::rc::Rc;
use crate::{range_cloud::Prepared,range_source::Source,range_page::Page,page_cache::Budget};

pub fn prepared()->Rc<Prepared> {
    let source=Rc::new(Source {properties:crate::stream_properties::Properties::default(),guid:"remote-original".into(),name:"Streamed source".into(),url:"points.pb".into(),fields:crate::stream::CloudFields {end:200,coords_at:10,coords_len:96,colors_at:106,colors_len:0,normals_at:0,normals_len:0,count:4,ids_at:120,ids_len:16,revision:Some("v1".into())},lod:crate::stream::CloudLod {min:vec![-1.,-1.,-1.,-1.,-1.,-1.],size:vec![2.,2.],spacing:vec![0.1,0.05],level:vec![0,1],first:vec![0,2],count:vec![2,2],children:[vec![1],vec![-1;15]].concat()}});
    let budget=Budget::new(4096);let prepared=Prepared::new(source,budget.clone()).unwrap();
    let page=|rows,values:Vec<f64>,ids:Vec<u32>,colors:Vec<u32>| {let raw=values.into_iter().flat_map(f64::to_le_bytes).collect::<Vec<_>>();let ids=ids.into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>();Page::decode(rows,4,&raw,Some(&ids)).unwrap().attributes(colors,vec![]).unwrap()};
    let root=page(0..2,vec![-0.5,-0.5,0.3,0.5,-0.5,0.3],vec![10,11],vec![0xff00_00ff,0xff00_00ff]);let charge=budget.reserve(root.bytes()).unwrap();prepared.pin_root(root,charge).unwrap();
    let next=page(2..4,vec![-0.5,0.5,0.3,0.5,0.5,0.3],vec![u32::MAX,16_777_217],vec![0xffff_0000,0xffff_0000]);let charge=budget.reserve(next.bytes()).unwrap();drop(prepared.adopt(next,charge).unwrap());prepared
}
