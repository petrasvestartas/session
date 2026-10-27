        let result = if current {
            let mut mapped=bytes.to_vec();
            for row in mapped.chunks_exact_mut(image.window.row_bytes()as usize) {
                for pixel in row[..image.window.copy[2]as usize*8].chunks_exact_mut(8) {
                    let raw=[u32::from_le_bytes(pixel[..4].try_into().unwrap()),u32::from_le_bytes(pixel[4..].try_into().unwrap())];
                    let ids=remap(raw);pixel[..4].copy_from_slice(&ids[0].to_le_bytes());pixel[4..].copy_from_slice(&ids[1].to_le_bytes());
                }
            }
            image.window.closest(&mapped).map(Reply::Hit)
        } else { Ok(Reply::Stale) };