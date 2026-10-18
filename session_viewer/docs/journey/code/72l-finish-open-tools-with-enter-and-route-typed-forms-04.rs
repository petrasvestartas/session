                self.tool_status=draft.prompt();return Ok(Change::View);
            },
            Action::ToolText(text)=>return self.tool_text(&text),
            Action::FinishTool=>{crate::drawing_coords::set_plane(self.construction_plane);let built=self.draft.as_ref().ok_or("No point tool is running")?.finish()?;return self.commit_tool(built);},
            Action::LayerEdit {key,edit}=>{
                if matches!(edit,crate::layer_edit::Edit::Current) {crate::layer_edit::apply(&mut self.scene,&key,edit)?;return Ok(Change::View);}
