use std::{collections::HashSet, env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let bytes = fs::read(&args[2])?;
    let mut parts = bytes.splitn(4, |&byte| byte == b'\n');
    assert_eq!(parts.next(), Some(b"P6".as_slice()));
    let size: Vec<usize> = std::str::from_utf8(parts.next().ok_or("Missing image size")?)?
        .split_whitespace().map(str::parse).collect::<Result<_, _>>()?;
    assert_eq!(size.len(), 2);
    assert_eq!(parts.next(), Some(b"255".as_slice()));
    let pixels = parts.next().ok_or("Missing pixels")?;
    let (width, height) = (size[0], size[1]);
    assert_eq!(pixels.len(), width * height * 3);
    let mut ink = 0;
    let mut visible_objects = 0;

    if mode == "scene" {
        assert_eq!((width, height), (900, 700));
        ink = pixels.chunks_exact(3).filter(|pixel| *pixel.iter().min().unwrap() < 245).count();
        assert!(ink > 100 && ink * 10 < width * height * 9, "Scene is empty or fills the frame");
        let ids = fs::read(&args[4])?;
        assert_eq!(&ids[..4], b"HLI2");
        let number = |offset| u32::from_le_bytes(ids[offset..offset + 4].try_into().unwrap());
        assert_eq!((number(4), number(8)), (width as u32, height as u32));
        assert_eq!(ids.len(), 12 + width * height * 8);
        let valid: HashSet<u32> = args[5].split(',').map(str::parse).collect::<Result<_, _>>()?;
        let visible: HashSet<u32> = ids[12..].chunks_exact(8)
            .map(|pixel| u32::from_le_bytes(pixel[..4].try_into().unwrap()))
            .filter(|id| valid.contains(id)).collect();
        visible_objects = visible.len();
        assert!(visible_objects >= 2, "Grid ink alone does not prove scene objects rendered");
    } else {
        assert_eq!((width, height), (640, 480));
        let range = match mode.as_str() {
            "white" => 250..=255,
            "grey" => 185..=190,
            _ => panic!("Unknown expected frame: {mode}"),
        };
        assert!(pixels.iter().all(|value| range.contains(value)), "Unexpected background pixels");
    }

    fs::write(&args[3], format!(
        "{{\"width\":{width},\"height\":{height},\"ink\":{ink},\"visible_objects\":{visible_objects}}}\n"
    ))?;
    Ok(())
}
