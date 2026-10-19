  })
}

pub fn large_example()->session_rust::PointCloud {
    let mut coords=Vec::with_capacity(2_000_064*3);let mut colors=Vec::with_capacity(2_000_064*4);
    for i in 0..64 {coords.extend([(i%8) as f64*2./7.-1.,(i/8) as f64*2./7.-1.,0.2]);colors.extend([40,150,230,255]);}
    for quadrant in 0..4 {for i in 0..500_000 {coords.extend([(quadrant%2) as f64-1.+(i%708) as f64/707.,(quadrant/2) as f64-1.+(i/708) as f64/706.,0.2]);colors.extend([40+quadrant*40,150,230,255]);}}
    let mut p=session_rust::proto::PointCloud {name:"Original large cloud".into(),coords,colors,point_size:3.,lod_min:vec![-1.,-1.,-1.],lod_size:vec![2.],lod_spacing:vec![0.1],lod_level:vec![0],lod_first:vec![0],lod_count:vec![64],lod_children:vec![-1;40],..Default::default()};
    for quadrant in 0..4 {p.lod_min.extend([(quadrant%2) as f64-1.,(quadrant/2) as f64-1.,0.]);p.lod_size.push(1.);p.lod_spacing.push(1./707.);p.lod_level.push(1);p.lod_first.push(64+quadrant*500_000);p.lod_count.push(500_000);p.lod_children[4+quadrant as usize]=quadrant+1;}
    session_rust::PointCloud::from_proto(p)
}
