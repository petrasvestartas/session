        let action = if event.type_()=="viewer-samples"{
            if let Some(event)=event.dyn_ref::<web_sys::CustomEvent>(){
                let value=event.detail();
                if value.is_null(){sample_forced=None;}
                else if let Some(count)=value.as_f64().filter(|v|*v==1.||*v==4.){sample_forced=Some(count as u32);}
            }
            None
        }else if event.type_() == "cancel"