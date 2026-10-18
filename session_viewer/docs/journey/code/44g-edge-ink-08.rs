            let dy = i64::from(y) - i64::from(self.cursor[1]);
            let distance = (dx * dx + dy * dy) as u64;
            if row == 0 || distance > u64::from(self.radius).pow(2) { continue; }
            let key = (crate::sub_id::surface(sub), distance, row, sub);
            if best.is_none_or(|old| key < old) { best = Some(key); }
        } }
        Ok(best.map(|(_, _, row, sub)| [row, sub]))
