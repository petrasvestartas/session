            let (v, n) = varint(raw, i)?;
            i += n;
            *k = u8::try_from(v).ok()?;
        }

