    fn depth_limits(&self)->[f64;2] {
        let scale=self.unit.to_meters();let distance=self.distance*scale;
        let half=crate::grid::Settings::default().half().unwrap()*scale;
        let target=self.target.map(|v|v*scale);
        let grid=match self.grid_plane {
            Some(plane)=>{let center=plane.origin();let d=std::array::from_fn::<_,3,_>(|i|target[i]-center[i]*scale);(d.iter().map(|v|v*v).sum::<f64>()).sqrt()+half*2f64.sqrt()},
            None=>{let x=target[0].abs()+half;let y=target[1].abs()+half;(x*x+y*y+target[2]*target[2]).sqrt()},
        };
        let scene=if self.radius>0. {self.radius*scale}else{distance};let reach=scene.max(grid);
        [(distance*10.).max(distance+2.*reach),(distance+2.*reach).max(1e-6)]
    }

    pub fn ray_unbounded(&self,screen:[f32;2])->Option<Ray> {
        if screen.iter().any(|v|!v.is_finite())||!self.distance.is_finite()||self.distance<=0. {return None;}
        let shift=self.shift();let half=self.distance*HALF_FOV.tan();
        let right=self.orientation.rotate_vector(Vector::x_axis());let up=self.orientation.rotate_vector(Vector::z_axis());let forward=self.orientation.rotate_vector(Vector::y_axis());
        let plane=&right*((screen[0]as f64-shift[0])*half*self.aspect)+&up*((screen[1]as f64-shift[1])*half);
        let [far,range]=self.depth_limits().map(|v|v/self.unit.to_meters());
        let (local,delta)=match self.projection {
            Projection::Perspective=>{let direction=&plane+&(&forward*self.distance);(&forward*(-self.distance)+&direction*NEAR_FRACTION,direction*(far/self.distance-NEAR_FRACTION))},
            Projection::Orthographic=>(plane-&forward*(self.distance+range),&forward*(2.*range)),
        };
        let max_distance=delta.magnitude();if !max_distance.is_finite()||max_distance<=0. {return None;}
        let origin=Point::new(self.target[0]+local[0],self.target[1]+local[1],self.target[2]+local[2]);
        if (0..3).any(|i|!origin[i].is_finite()) {return None;}
        Some(Ray{origin,direction:delta.normalized(),max_distance})
    }