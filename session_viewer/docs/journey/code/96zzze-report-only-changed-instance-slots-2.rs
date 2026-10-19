a + 1 == *b)
        .map(|run| run[0]..run[0] + run.len() as u32),
    );
    Change {
      all: false,
      slots: coalesce(dirty, self.end()),
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A definition of `triangles` triangles from face index `faces` with instance `rows`.
    fn definition(key: u32, rows: &[u32], faces: u32, triangles: u32) -> Definition<'_> {
        Definition {
            key,
            rows,
            faces: faces..faces + 3 * triangles,
            pipes: 0..0,
            ribbons: 0..0,
        }
    }

    /// Every used slot names its member and ids no other slot takes, past the arena's; the table
    /// is sorted by first id, as the shaders' binary search needs.
    fn check(space: &SlotSpace, members: &[(u32, Vec<u32>, u32)], arena: u32) {
        let mut ids = Vec::new();
        let texels: Vec<[u32; 4]> = (0..space.end()).map(|slot| space.texel(slot)).collect();
        assert_eq!(texels[0], [space.end() - 1, arena, 0, 0]);
        assert!(
            texels[1..].windows(2).all(|pair| pair[0][1] <= pair[1][1]),
            "sorted"
        );
        let draws = space.draws();

        for (key, rows, triangles) in members {
            let run = space
                .runs
                .iter()
                .find(|run| run.key == Some(*key))
                .expect("a run");
            let draw = draws
                .iter()
                .find(|d| d.slots.start == run.start)
                .expect("a draw");
            assert_eq!(draw.slots.len(), rows.len());

            for (position, row) in rows.iter().enumerate() {
                let slot = run.start + position as u32;
                let [texel_row, first, _, count] = texels[slot as usize];
                assert_eq!((texel_row, count), (*row, *triangles), "slot {slot}");
                assert_eq!(space.slot(slot)[0], *row);
                assert_eq!(space.slot(slot)[1].wrapping_add(run.first), first);
                ids.push(first..first + count);
            }
        }

        ids.sort_by_key(|range| range.start);
        assert!(
            ids.windows(2).all(|pair| pair[0].end <= pair[1].start),
            "ids overlap"
        );
        assert!(ids.first().is_none_or(|range| range.start >= arena));
        let count = space.triangle_count().unwrap_or(arena);
        assert!(ids.last().is_none_or(|range| range.end <= count));
    }

    /// Instances number their triangles one after another, past the arena's: each slot's base
    /// turns a definition triangle into its own id, and the table names the slot back.
    #[test]
    fn instance_ids_follow_the_arena() {
        let mut space = SlotSpace::default();
        let definitions = [definition(1, &[7, 8], 30, 2), definition(2, &[9], 60, 1)];
        assert!(space.reset(&definitions, 100).all);
        assert_eq!(space.triangle_count(), Some(105));
        let texels: Vec<[u32; 4]> = (0..4).map(|slot| space.texel(slot)).collect();
        let want = [
            [3, 100, 0, 0],
            [7, 100, 10, 2],
            [8, 102, 10, 2],
            [9, 104, 20, 1],
        ];
        assert_eq!(texels, want);
        // triangle 11 of the definition, drawn by the second slot, is id 103 zero-based
        assert_eq!(space.slot(2), [8, 92]);
        assert_eq!(space.slot(2)[1] + 11, 103);
        assert_eq!(space.slot(3)[1] + 20, 104);

        assert!(space.reset(&[], 100).all);
        assert_eq!(space.triangle_count(), None);
        assert_eq!(space.texel(0), [0, 100, 0, 0]);
    }

    /// Definitions of `members` (key, rows, triangles), the faces of key k at 30 * k.
    fn definitions(members: &[(u32, Vec<u32>, u32)]) -> Vec<Definition<'_>> {
        members
            .iter()
            .map(|(key, rows, triangles)| definition(*key, rows, 30 * *key, *triangles))
            .collect()
    }

    /// Members join and leave as the scene does, by swap removal: each edit writes its own slot
    /// and the head, a full run moves now and then, and the slots always match the members.
    #[test]
    fn edits_write_only_their_own_slots() {
        let mut members: Vec<(u32, Vec<u32>, u32)> = vec![
            (1, (0..40).collect(), 2),
            (2, (100..110).collect(), 5),
            (3, vec![200], 1),
        ];
        let mut space = SlotSpace::default();
        space.reset(&definitions(&members), 1000);
        check(&space, &members, 1000);
        let mut seed = 5u32;
        let mut next = |n: u32| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) % n
        };
        let mut row = 1000;
        let mut moves = 0;

        for step in 0..600 {
            let batch = next(3) as usize;
            let (key, rows, _) = &mut members[batch];
            let mut touched = Vec::new();

            // join, or leave by swap removal
            if next(2) == 0 || rows.len() < 2 {
                touched.push((*key, rows.len() as u32));
                rows.push(row);
                row += 1;
            } else {
                let at = next(rows.len() as u32) as usize;
                rows.swap_remove(at);

                if at < rows.len() {
                    touched.push((*key, at as u32));
                }
            }

            let change = space.update(&definitions(&members), &touched, 1000);
            check(&space, &members, 1000);

            match change.written() {
                // the head, the member's slot and the slot a leaver vacated
                Some(written) if written > 3 => moves += 1,
                Some(_) => {}
                None => panic!("step {step} numbered everything again"),
            }
        }

        // a full run moves to room a quarter bigger: rarely
        assert!(moves < 60, "{moves} moves");
    }

    /// A heavy definition joined one instance at a time never reserves much more than its
    /// instances use: every reserved id costs the tile tables 96 bytes.
    #[test]
    fn heavy_definitions_reserve_few_ids() {
        let triangles = 870_000;
        let mut rows: Vec<u32> = vec![0];
        let mut space = SlotSpace::default();
        space.reset(&[definition(1, &rows, 0, triangles)], 1000);

        for row in 1..40 {
            let touched = [(1, rows.len() as u32)];
            rows.push(row);
            space.update(&[definition(1, &rows, 0, triangles)], &touched, 1000);
            check(&space, &[(1, rows.clone(), triangles)], 1000);
            let used = rows.len() as u64 * u64::from(triangles);
            let reserved = u64::from(space.triangle_count().unwrap() - space.base);
            assert!(
                reserved <= used * 3 / 2 + u64::from(SPARE_IDS),
                "{row}: {reserved} ids for {used}"
            );
        }
    }

    /// The last instance of a definition frees its room for another; a new definition takes it.
    #[test]
    fn freed_room_is_taken_again() {
        let mut space = SlotSpace::default();
        let (a, b): (Vec<u32>, Vec<u32>) = ((0..8).collect(), (10..14).collect());
        space.reset(&[definition(1, &a, 0, 2), definition(2, &b, 30, 2)], 50);
        let change = space.update(&[definition(2, &b, 30, 2)], &[], 50);
        assert_eq!(change.written(), Some(9), "the head and the freed run");
        let c: Vec<u32> = (20..26).collect();
        let change = space.update(
            &[definition(2, &b, 30, 2), definition(3, &c, 60, 1)],
            &[],
            50,
        );
        assert_eq!(
            change.written(),
            Some(1 + 6),
            "the head and the room it took"
        );
        let run = space.runs.iter().find(|run| run.key == Some(3)).unwrap();
        assert_eq!(run.start, 1, "the freed room at the front");
        check(&space, &[(2, b.clone(), 2), (3, c.clone(), 1)], 50);
    }

    /// The arena grows: within the headroom only the head changes, past it everything is
    /// numbered again with room to spare.
    #[test]
    fn the_arena_grows_into_the_headroom() {
        let mut space = SlotSpace::default();
        let rows: Vec<u32> = (0..5).collect();
        space.reset(&[definition(1, &rows, 0, 3)], 100);
        let change = space.follow(200);
        assert!(change.all, "past the ids: numbered again");
        check(&space, &[(1, rows.clone(), 3)], 200);
        assert_eq!(space.base, 200 + headroom(200));

        for arena in [210, 500, 900] {
            let change = space.follow(arena);
            assert_eq!(change.slots, vec![0..1], "{arena}: the head alone");
            check(&space, &[(1, rows.clone(), 3)], arena);
        }

        assert!(space.follow(10_000).all);
        assert!(space.follow(10_000) == Change::default());
    }
}
