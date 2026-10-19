            for lane in ink { lane.pick(&mut pass, crop_group); }
            if let Some(clouds)=clouds {clouds.pick(ctx,&mut pass,window,mode);}
            if let Some(sheets)=sheets {sheets.pick(&mut pass,crop_group);}
            if let Some(text)=text {text.draw_ids(&mut pass,crop_group);}
        }
