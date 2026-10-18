    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrong_offsets_short_bodies_status_and_overflow_are_refused() {
        assert_eq!(header(50,24).unwrap(),"bytes=50-73");
        assert_eq!(accept(50,24,206,Some("bytes 50-73/100"),24).unwrap(),100);
        for (status,range,len) in [(200,"bytes 50-73/100",24),(206,"bytes 49-72/100",24),(206,"bytes 50-73/100",23),(206,"bytes 50-74/100",25),(206,"bytes 50-73/73",24),(206,"bytes 50-72/100",23)] {assert!(accept(50,24,status,Some(range),len).is_err());}
        assert!(accept(50,24,206,None,24).is_err());assert!(header(u64::MAX-1,3).is_err());assert!(header(0,MAX_RANGE+1).is_err());assert!(header(0,0).is_err());
    }
    #[test]
    fn only_the_initial_probe_may_finish_at_an_earlier_file_end() {
        assert_eq!(accept(0,PROBE_BYTES,206,Some("bytes 0-23/24"),24).unwrap(),24);
        assert!(accept(0,PROBE_BYTES,206,Some("bytes 0-23/100"),24).is_err());
        assert!(accept(1,PROBE_BYTES,206,Some("bytes 1-23/24"),23).is_err());
        for raw in ["bytes */24","bytes 0-23/*","items 0-23/24","bytes 24-23/24"] {assert!(content_range(raw).is_none());}
    }
}
