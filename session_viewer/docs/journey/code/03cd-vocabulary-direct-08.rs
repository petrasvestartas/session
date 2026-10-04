                    if let Some(line) = self.model.take_command() {
                        self.commands.reply(&mut self.model, &line);
                    }
