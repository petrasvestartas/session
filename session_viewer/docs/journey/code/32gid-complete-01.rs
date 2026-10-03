impl Reply {
    pub fn complete(self, editor: &mut crate::editor::Editor) -> Result<Option<crate::edit_replay::Reply>, String> {
        if !editor.hydrate(self.result?).map_err(str::to_owned)? { return Ok(None); }
        let reply = match self.intent {
            Some(intent) => intent.replay(editor).map_err(str::to_owned)?,
            None => crate::edit_replay::Reply::Changed(crate::editor::Change::Scene),
        };
        Ok(Some(reply))
    }

