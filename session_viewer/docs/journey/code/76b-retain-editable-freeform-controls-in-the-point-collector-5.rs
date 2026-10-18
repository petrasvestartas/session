            for name in ["Points","Rectangle","Polygon"] {buttons.push((name.into(),format!("Polyline {name}")));}
        }
        if matches!(self,Self::Polyline|Self::Curve) {buttons.insert(0,("Close".into(),"Close".into()));}buttons
    }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
