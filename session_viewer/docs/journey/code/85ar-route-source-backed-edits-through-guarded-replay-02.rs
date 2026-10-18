
impl Intent {
    pub fn command(&self) -> &'static str {
        match self { Self::Command(_)=>"Command",Self::Move { .. } => "Move", Self::Delete { .. } => "Delete", Self::Save => "Save" }
    }

    pub fn capture_for(action:&Action,editor:&Editor)->Result<Option<Self>,&'static str>{
        if let Action::Translate(offset)=action{if offset.iter().any(|v|!v.is_finite()){return Err("Move needs finite coordinates")}if offset.iter().all(|v|*v==0.){return Ok(None)}}
        if let Some(pending)=crate::command_request::Pending::capture(action,editor){if !pending.keys(editor).is_empty(){return Ok(Some(Self::Command(pending)))}}
        if editor.selected_objects().len()>1{return Ok(None)}Self::capture(action,editor.whole_selection())
    }

    pub fn keys(&self, editor: &Editor) -> Result<Vec<ReloadKey>, &'static str> {
        match self {
            Self::Command(pending)=>{if !pending.valid(editor){return Err("Command target changed before loading sources")}Ok(pending.keys(editor))},
            Self::Save => Ok(editor.reload_keys()),
            Self::Move { id, .. } | Self::Delete { id } => {
