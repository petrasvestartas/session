use std::{cell::Cell,rc::Rc};
use wasm_bindgen::JsValue;

pub fn prepare(renderer:&mut crate::renderer::Renderer,editor:&crate::editor::Editor)->Result<crate::render_space::Rebase,JsValue>{
    renderer.prepare_origin(&editor.camera,editor.display_scene(),editor.selection(),&editor.highlighted_objects(),crate::browser_phase::now()).map_err(JsValue::from_str)
}

pub struct Wake {timer:Option<crate::timer::Timer>,pending:Rc<Cell<bool>>,fault:crate::gpu_fault::Fault}
impl Wake {
    pub fn new(fault:crate::gpu_fault::Fault)->Self{Self{timer:None,pending:Rc::new(Cell::new(false)),fault}}
    pub fn pending(&self)->bool{self.pending.get()}
    pub fn sync(&mut self,pending:bool,delay:u32)->Result<(),JsValue>{
        if !pending {self.pending.set(false);self.timer=None;return Ok(());}
        if self.pending.get(){return Ok(());}
        let flag=self.pending.clone();let fault=self.fault.clone();
        self.timer=Some(crate::timer::Timer::after(delay as i32,move||{
            flag.set(false);
            if crate::browser_runtime::owns(&fault)&&fault.message().is_none(){
                if let(Some(window),Ok(event))=(web_sys::window(),web_sys::Event::new("viewer-origin")){let _=window.dispatch_event(&event);}
            }
        })?);
        self.pending.set(true);Ok(())
    }
}
