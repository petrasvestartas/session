        self.completions(line)
    }
}

#[cfg(test)]
mod completion_tests {
    use super::*;
    use crate::command_dock::{CommandLine, theme};

    #[test]
    fn typed_exact_commands_survive_inline_completion_and_enter() {
        for line in ["View Isometric", "View Isometric Back", "Line", "Polyline", "Select Object"] {
            let context = egui::Context::default();
            context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
            context.set_fonts(theme::fonts([
                include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
                include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
                include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
            ]));
            let mut model = CommandLine { focus_command: true, command_open: true, ..Default::default() };
            let commands = Commands(crate::commands::names());
            let frame = |model: &mut CommandLine, events| {
                let mut submitted = None;
                let _ = context.run_ui(egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(900., 760.))),
                    focused: true, events, ..Default::default()
                }, |root| { command_dock::draw(root, model, &mut None, &mut submitted, &commands, true); });
                submitted
            };
            frame(&mut model, vec![]);
            for character in line.chars() {
                assert!(frame(&mut model, vec![egui::Event::Text(character.to_string())]).is_none());
            }
            assert_eq!(model.command, line);
            assert!(!model.inline_suffix);
            let submitted = frame(&mut model, vec![egui::Event::Key {
                key: egui::Key::Enter, physical_key: None, pressed: true,
                repeat: false, modifiers: Default::default(),
            }]);
            assert_eq!(submitted.as_deref(), Some(line));
            assert!(model.command.is_empty());
        }
    }
}
