}

#[cfg(target_arch = "wasm32")]
pub use web::*;

/// The reads: find the arrays, then fetch slices by range.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::range_fetch::{GetOpts, PROBE_BYTES, Reply, Task, fetch_range, get, retryable};
    use crate::manifest::immutable_key;
    use crate::range_gate::{RANGE_READS, RangeGate};
    use std::cell::RefCell;
    use std::rc::Rc;

    const POINT_BYTES: u64 = 24; // three doubles

    const MAX_TABLE_BYTES: u64 = 128 * 1024 * 1024; // largest node table read

    thread_local! {
        /// The scene's streaming range reads queue here.
        static GATE: RefCell<Rc<RangeGate>> = RefCell::new(RangeGate::new(RANGE_READS));
    }

    /// A new scene: turn the old scene's waiting reads away and start an empty queue.
    pub fn reset_range_gate() {
        GATE.with_borrow_mut(|gate| {
            gate.close();
            crate::range_fetch::cancel();
            *gate = RangeGate::new(RANGE_READS);
        });
    }

    impl MetadataWindow {
        /// The bytes at `at`, reading more when not cached.
        pub(super) async fn read(
            &mut self,
            url: &str,
            at: u64,
            length: u64,
            end: u64,
            revision: &Option<String>,
        ) -> Option<&[u8]> {
            body_end(at, length, end)?;

            if self.slice(at, length).is_none() {
                let read_length = Self::read_length(at, length, end)?;
                self.bytes = source_range(url, at, read_length, revision).await?;
                self.at = at;
            }

            self.slice(at, length)
        }
    }

    /// Read one range of the file.
    pub(crate) async fn source_range(
        url: &str,
        at: u64,
        length: u64,
        revision: &Option<String>,
    ) -> Option<Vec<u8>> {
        if !bounded_range(at, length) {
            return None;
        }

        if length == 0 {
            return Some(Vec::new());
        }

        // the stall timer starts once the read holds a slot, so queued reads never time out
        let gate = GATE.with_borrow(Rc::clone);
        let _permit = gate.enter().await?;

        match fetch_range(url, at, length, revision).await {
            Ok((bytes, _)) => Some(bytes),
            Err(error) if retryable(&error) && !gate.is_closed() => {
                crate::browser_phase::finish("Range retry",crate::browser_phase::now(),0,&error);
                Some(fetch_range(url, at, length, revision).await.ok()?.0)
            }
            Err(_) => None,
        }
    }

}

#[cfg(test)]
mod tests {
