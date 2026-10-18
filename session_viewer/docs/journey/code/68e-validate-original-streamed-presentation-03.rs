    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_size_visibility_and_locking_are_checked_separately_from_placement_overrides() {
        let mut properties=Properties::default();let field=|field,wire,value|crate::stream::Field {field,wire,value,body:0,next:0};assert!(properties.field(&field(6,1,0),&3.5f64.to_le_bytes()));assert!(properties.field(&field(16,0,0),&[]));assert!(properties.field(&field(17,0,1),&[]));assert_eq!((properties.point_px,properties.visible,properties.locked),(3.5,false,true));assert!(!properties.field(&field(6,1,0),&4f64.to_le_bytes()));
        for px in [0.,-1.,f64::NAN,f64::INFINITY,f64::MAX] {assert!(!Properties::default().field(&field(6,1,0),&px.to_le_bytes()));}
        assert!(!Properties::default().field(&field(16,0,2),&[]));assert!(!Properties::default().field(&field(6,2,8),&1f64.to_le_bytes()));
    }
}
