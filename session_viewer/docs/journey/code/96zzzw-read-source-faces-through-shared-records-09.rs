        let source=if shared {crate::object_rows::source(source)}else{source.into()};let slots=[crate::draw_slots::layout()];
        let shader = device.create_shader_module