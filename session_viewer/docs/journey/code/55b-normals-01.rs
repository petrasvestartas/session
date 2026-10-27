    }

    pub fn normal(&self,index:usize)->Result<[f32;3],&'static str> {
        let sample=self.samples.get(index).ok_or("Missing surface sample")?;
        if sample.normal.iter().map(|v|v*v).sum::<f64>()>1e-24 {return crate::normals::unit(sample.normal.map(|v|v as f32));}
        let [nu,nv]=self.steps;let grid_count=(nu+1)*(nv+1);
        let index=if index<grid_count {index}else{(0..grid_count).min_by(|&a,&b|{let distance=|i:usize|(0..2).map(|axis|(self.samples[i].uv[axis]-sample.uv[axis]).powi(2)).sum::<f64>();distance(a).total_cmp(&distance(b))}).unwrap()};
        let i=index/(nv+1);let j=index%(nv+1);
        for a in [i.saturating_sub(1),i.min(nu-1)] {for b in [j.saturating_sub(1),j.min(nv-1)] {
            let p=self.samples[a*(nv+1)+b].point;let u=self.samples[(a+1)*(nv+1)+b].point;let v=self.samples[a*(nv+1)+b+1].point;
            let u=session_rust::Vector::new(u[0]-p[0],u[1]-p[1],u[2]-p[2]);let v=session_rust::Vector::new(v[0]-p[0],v[1]-p[1],v[2]-p[2]);let n=u.cross(&v);
            if n.magnitude()>1e-12 {return crate::normals::unit([n[0] as f32,n[1] as f32,n[2] as f32]);}
        }}
        Ok([0.,0.,1.])
    }
}

