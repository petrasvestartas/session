enum Tool {
    Idle,
    Drawing { start: [f32; 2] },
}

fn click(tool: Tool, point: [f32; 2]) -> (Tool, Option<f32>) {
    match tool {
        Tool::Idle => (Tool::Drawing { start: point }, None),
        Tool::Drawing { start } => {
            let dx = point[0] - start[0];
            let dy = point[1] - start[1];
            (Tool::Idle, Some((dx * dx + dy * dy).sqrt()))
        }
    }
}

fn main() {
    let (tool, result) = click(Tool::Idle, [0.0, 0.0]);
    assert_eq!(result, None);
    let (tool, result) = click(tool, [3.0, 4.0]);
    assert_eq!(result, Some(5.0));
    assert!(matches!(tool, Tool::Idle));
}
