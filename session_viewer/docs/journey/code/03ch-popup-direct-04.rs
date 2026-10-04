                    let mut line = None;
                    let choices = self.commands.completions(&self.model.completion_prefix);
                    let complete = if self.model.completion_visible && !choices.is_empty() { command_dock::popup(ui, &mut self.model, &mut None, &choices, &response, 0) } else { None };
                    let complete = complete.or_else(|| keys.tab.then(|| choices.first().copied()).flatten());
                    command_dock::finish(ui, &mut self.model, &response, &mut line, &self.commands, complete, &keys, true);
