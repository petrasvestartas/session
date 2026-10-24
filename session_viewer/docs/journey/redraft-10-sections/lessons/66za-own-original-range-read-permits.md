# 12.3 · Bounded scheduling, exact replies and cancellation

**Goal:** Cancel range work when its runtime closes. · **Steps:** 17 · **Source:** [app/manifest.rs](../../../../src/app/manifest.rs) · [app/session_io.rs](../../../../src/app/session_io.rs) · [app/stream.rs](../../../../src/app/stream.rs) · [app/range_gate.rs](../../../../src/app/range_gate.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Octree Cell Subdivision: 8 Child AABBs (L0 -&gt; L1 -&gt; L2)</text>
  <rect x="68" y="48" width="108" height="108" fill="#f8fafc" stroke="#111111" stroke-width="2"/>
  <line x1="122" y1="48" x2="122" y2="156" stroke="#111111" stroke-width="1.5"/>
  <line x1="68" y1="102" x2="176" y2="102" stroke="#111111" stroke-width="1.5"/>
  <rect x="122" y="48" width="54" height="54" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="149" y1="48" x2="149" y2="102" stroke="#2563eb" stroke-width="1.2"/>
  <line x1="122" y1="75" x2="176" y2="75" stroke="#2563eb" stroke-width="1.2"/>
  <rect x="149" y="48" width="27" height="27" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="238" y="88" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">sse = d_cell / z</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="238" y="110" text-anchor="middle" font-family="monospace" font-size="10" fill="#16a34a">sse > τ_split</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Frustum Cull + HTTP Range: bytes=offset..offset+len</text>
  <circle cx="432" cy="104" r="5" fill="#111111"/>
  <polygon points="432,104 562,50 562,156" fill="#fef9c3" fill-opacity="0.4" stroke="#111111" stroke-width="1.5" stroke-dasharray="4 3"/>
  <rect x="504" y="68" width="38" height="34" fill="#bbf7d0" stroke="#16a34a" stroke-width="2"/>
  <rect x="504" y="110" width="38" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="2"/>
  <rect x="592" y="64" width="102" height="78" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <rect x="600" y="74" width="86" height="16" fill="#bbf7d0" stroke="#16a34a" stroke-width="1.2"/>
  <rect x="600" y="96" width="86" height="16" fill="#dbeafe" stroke="#2563eb" stroke-width="1.2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Progressive Multi-Resolution Point Cloud</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="832" y="58" width="96" height="90" fill="none" stroke="#94a3b8" stroke-width="1.2" stroke-dasharray="3 3"/>
  <circle cx="860" cy="84" r="6" fill="#94a3b8"/>
  <circle cx="898" cy="116" r="6" fill="#94a3b8"/>
  <rect x="936" y="58" width="96" height="90" fill="none" stroke="#16a34a" stroke-width="1.75"/>
  <circle cx="956" cy="76" r="3.5" fill="#16a34a"/><circle cx="972" cy="76" r="3.5" fill="#16a34a"/><circle cx="988" cy="76" r="3.5" fill="#16a34a"/><circle cx="1004" cy="76" r="3.5" fill="#16a34a"/>
  <circle cx="956" cy="96" r="3.5" fill="#2563eb"/><circle cx="972" cy="96" r="3.5" fill="#2563eb"/><circle cx="988" cy="96" r="3.5" fill="#2563eb"/><circle cx="1004" cy="96" r="3.5" fill="#2563eb"/>
  <circle cx="956" cy="116" r="3.5" fill="#dc2626"/><circle cx="972" cy="116" r="3.5" fill="#dc2626"/><circle cx="988" cy="116" r="3.5" fill="#dc2626"/><circle cx="1004" cy="116" r="3.5" fill="#dc2626"/>
</svg>
</div>

## <a id="checkpoint-66za-own-original-range-read-permits"></a>1. Own range-read permits

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Scheduler: Permit Pool + HTTP Range Request</text>
  <rect x="52" y="50" width="268" height="44" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">GET Range: bytes=off..off+len</text>
  <rect x="52" y="104" width="268" height="44" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Bounded In-Flight Permits &amp; Epoch</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Byte Budget Check -&gt; queue.write_buffer(page)</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">charged_bytes &lt;= max_residency</text>
  <line x1="452" y1="100" x2="668" y2="100" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="128" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Upload Decoded Page to GPU Slot</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Resident GPU Pages Rendered in Viewport</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="832" y="60" width="92" height="86" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="878" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Page #1 ✓</text>
  <rect x="938" y="60" width="92" height="86" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="984" y="106" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">Page #2 ✓</text>
</svg>
</div>

Write the per-scene gate, queued turn, waiting-read and permit types.

### 1. `src/range_gate.rs`

Type the whole original helpers introduced today.

Create the file and type:

```rust
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};

/// Range reads a scene keeps in flight; the rest wait their turn.
pub const RANGE_READS: usize = 3;

/// A waiting read's place in the queue.
#[derive(Default)]
struct Turn {
    given: Cell<bool>,             // a slot was handed to it
    waker: RefCell<Option<Waker>>, // wakes its read
}

impl Turn {
    /// Wake the read waiting on this turn.
    fn wake(&self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

/// Lets a few reads run at once, first come first served; a closed gate turns waiting reads away.
pub struct RangeGate {
    limit: usize,                       // slots
    running: Cell<usize>,               // slots taken
    queue: RefCell<VecDeque<Rc<Turn>>>, // waiting reads, oldest first
    closed: Cell<bool>,                 // its scene is gone
}

impl RangeGate {
    /// A gate with `limit` slots.
    pub fn new(limit: usize) -> Rc<Self> {
        Rc::new(Self {
            limit: limit.max(1),
            running: Cell::new(0),
            queue: RefCell::new(VecDeque::new()),
            closed: Cell::new(false),
        })
    }

    /// Wait for a slot; None once the gate closed.
    pub fn enter(self: &Rc<Self>) -> Enter {
        Enter {
            gate: self.clone(),
            turn: None,
        }
    }

    /// Turn every waiting read away.
    pub fn close(&self) {
        self.closed.set(true);

        for turn in self.queue.take() {
            turn.wake();
        }
    }

    /// The gate's scene is gone.
    pub fn is_closed(&self) -> bool {
        self.closed.get()
    }

    /// Reads running and waiting.
    pub fn load(&self) -> (usize, usize) {
        (self.running.get(), self.queue.borrow().len())
    }

    /// A slot came free: hand it to the oldest waiting read.
    fn release(&self) {
        let next = self.queue.borrow_mut().pop_front();

        match next {
            Some(turn) => {
                turn.given.set(true);
                turn.wake();
            }
            None => self
                .running
                .set(self.running.get().saturating_sub(1)),
        }
    }
}

/// A read waiting at the gate.
pub struct Enter {
    gate: Rc<RangeGate>,    // the gate
    turn: Option<Rc<Turn>>, // its place in the queue, once queued
}

/// A slot at the gate, freed when dropped.
pub struct Permit {
    gate: Rc<RangeGate>, // the gate it came from
}

impl Drop for Permit {
    /// Free the slot.
    fn drop(&mut self) {
        self.gate.release();
    }
}
```

### 2. `src/lib.rs`

Register the original browser range-read gate.

**Locate this existing block:**

```rust
pub mod stream;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod stream;

pub mod range_gate;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-67a-queue-range-reads-in-arrival-order"></a>2. Poll and cancel queued range reads

Complete the waiting-read Future and its Drop behavior.

### 1. `src/range_gate.rs`

Extend this original owner with today’s complete helper. Supply only the unchanged original validation fixtures.

**Locate this existing block:**

```rust
}

/// A slot at the gate, freed when dropped.
pub struct Permit {
    gate: Rc<RangeGate>, // the gate it came from
}

impl Drop for Permit {
    /// Free the slot.
    fn drop(&mut self) {
        self.gate.release();
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

impl Future for Enter {
    type Output = Option<Permit>;

    /// A slot when one is free and no older read waits.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let gate = this.gate.clone();

        if gate.closed.get() {
            this.turn = None;
            return Poll::Ready(None);
        }

        match &this.turn {
            None if gate.running.get() < gate.limit
                && gate.queue.borrow().is_empty() =>
            {
                gate.running.set(gate.running.get() + 1);
                Poll::Ready(Some(Permit { gate }))
            }
            None => {
                let turn = Rc::new(Turn::default());
                *turn.waker.borrow_mut() = Some(cx.waker().clone());
                gate.queue
                    .borrow_mut()
                    .push_back(turn.clone());
                this.turn = Some(turn);
                Poll::Pending
            }
            Some(turn) if turn.given.get() => {
                this.turn = None;
                Poll::Ready(Some(Permit { gate }))
            }
            Some(turn) => {
                *turn.waker.borrow_mut() = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

impl Drop for Enter {
    /// A read dropped while waiting leaves the queue, or passes on a slot it was given.
    fn drop(&mut self) {
        let Some(turn) = self.turn.take() else {
            return;
        };

        if turn.given.get() {
            self.gate.release();
        } else {
            self.gate
                .queue
                .borrow_mut()
                .retain(|queued| !Rc::ptr_eq(queued, &turn));
        }
    }
}

/// A slot at the gate, freed when dropped.
pub struct Permit {
    gate: Rc<RangeGate>, // the gate it came from
}

impl Drop for Permit {
    /// Free the slot.
    fn drop(&mut self) {
        self.gate.release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Poll once with a waker that does nothing.
    fn poll(enter: &mut Enter) -> Poll<Option<Permit>> {
        let mut cx = Context::from_waker(Waker::noop());
        Pin::new(enter).poll(&mut cx)
    }

    /// The first reads run, the rest wait and enter oldest first as slots free.
    #[test]
    fn gate_runs_a_few_reads_and_queues_the_rest_in_order() {
        let gate = RangeGate::new(2);
        let mut first = gate.enter();
        let mut second = gate.enter();
        let mut third = gate.enter();
        let mut fourth = gate.enter();
        let one = poll(&mut first);
        let two = poll(&mut second);
        assert!(matches!(one, Poll::Ready(Some(_))));
        assert!(matches!(two, Poll::Ready(Some(_))));
        assert!(poll(&mut third).is_pending());
        assert!(poll(&mut fourth).is_pending());
        assert_eq!(gate.load(), (2, 2));

        drop(one);
        assert!(poll(&mut fourth).is_pending());
        let three = poll(&mut third);
        assert!(matches!(three, Poll::Ready(Some(_))));
        assert_eq!(gate.load(), (2, 1));

        drop(two);
        drop(three);
        assert!(matches!(
            poll(&mut fourth),
            Poll::Ready(Some(_))
        ));
        assert_eq!(gate.load(), (0, 0));
    }

    /// A newcomer does not jump the queue while a freed slot waits for its reader.
    #[test]
    fn gate_is_first_come_first_served() {
        let gate = RangeGate::new(1);
        let mut first = gate.enter();
        let mut second = gate.enter();
        let one = poll(&mut first);
        assert!(poll(&mut second).is_pending());
        drop(one);

        let mut late = gate.enter();
        assert!(poll(&mut late).is_pending());
        assert!(matches!(
            poll(&mut second),
            Poll::Ready(Some(_))
        ));
    }

    /// A read dropped while waiting leaves the queue; one dropped after its slot came passes it on.
    #[test]
    fn dropped_waiters_leave_or_pass_their_slot_on() {
        let gate = RangeGate::new(1);
        let mut first = gate.enter();
        let mut second = gate.enter();
        let mut third = gate.enter();
        let one = poll(&mut first);
        assert!(poll(&mut second).is_pending());
        assert!(poll(&mut third).is_pending());

        drop(one);
        drop(second);
        let three = poll(&mut third);
        assert!(matches!(three, Poll::Ready(Some(_))));
        assert_eq!(gate.load(), (1, 0));

        let mut fourth = gate.enter();
        assert!(poll(&mut fourth).is_pending());
        drop(fourth);
        assert_eq!(gate.load(), (1, 0));
    }

    /// Closing turns waiting reads away; running ones finish.
    #[test]
    fn closed_gate_turns_waiting_reads_away() {
        let gate = RangeGate::new(1);
        let mut first = gate.enter();
        let mut second = gate.enter();
        let one = poll(&mut first);
        assert!(poll(&mut second).is_pending());

        gate.close();
        assert!(gate.is_closed());
        assert!(matches!(
            poll(&mut second),
            Poll::Ready(None)
        ));
        assert!(matches!(
            poll(&mut gate.enter()),
            Poll::Ready(None)
        ));
        drop(one);
        assert_eq!(gate.load(), (0, 0));
    }
```

## <a id="checkpoint-67b-validate-exact-http-range-replies"></a>3. Validate exact HTTP range replies

### 1. `src/range_rules.rs`

Validate the source interval before issuing or accepting a read.

Create the file and type:

```rust
pub const MAX_RANGE: u64 = 64 * 1024 * 1024;
pub const PROBE_BYTES: u64 = 8192;

pub fn header(start: u64, length: u64) -> Result<String, String> {
    if length == 0 || length > MAX_RANGE {
        return Err("Invalid range length".into());
    }
    let end = start
        .checked_add(length - 1)
        .ok_or("Range end overflow")?;
    Ok(format!("bytes={start}-{end}"))
}

pub fn content_range(value: &str) -> Option<(u64, u64, u64)> {
    let value = value.strip_prefix("bytes ")?;
    let (interval, total) = value.split_once('/')?;
    let (start, end) = interval.split_once('-')?;
    let (start, end, total) = (
        start.parse().ok()?,
        end.parse().ok()?,
        total.parse().ok()?,
    );
    if start > end || end >= total {
        return None;
    }
    Some((start, end, total))
}

pub fn accept(
    start: u64,
    length: u64,
    status: u16,
    range: Option<&str>,
    received: u64,
) -> Result<u64, String> {
    header(start, length)?;
    if status != 206 {
        return Err(format!(
            "Range response requires HTTP206, received {status}"
        ));
    }
    let (first, last, total) =
        content_range(range.ok_or("Range response lacks Content-Range")?)
            .ok_or("Invalid Content-Range")?;
    if first != start || last - first + 1 != received || received > length {
        return Err("Range response does not match its requested bytes".into());
    }
    let end = start
        .checked_add(length)
        .ok_or("Range end overflow")?;
    if received != length
        && !(start == 0 && length == PROBE_BYTES && last + 1 == total && end > total)
    {
        return Err("Range response is incomplete".into());
    }
    Ok(total)
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod range_gate;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod range_gate;

pub mod range_rules;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/range_rules.rs`

Copy exact-response, EOF-probe and overflow regression fixtures.

**Locate this existing block:**

```rust
    Ok(total)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wrong_offsets_short_bodies_status_and_overflow_are_refused() {
        assert_eq!(header(50, 24).unwrap(), "bytes=50-73");
        assert_eq!(
            accept(50, 24, 206, Some("bytes 50-73/100"), 24).unwrap(),
            100
        );
        for (status, range, len) in [
            (200, "bytes 50-73/100", 24),
            (206, "bytes 49-72/100", 24),
            (206, "bytes 50-73/100", 23),
            (206, "bytes 50-74/100", 25),
            (206, "bytes 50-73/73", 24),
            (206, "bytes 50-72/100", 23),
        ] {
            assert!(accept(50, 24, status, Some(range), len).is_err());
        }
        assert!(accept(50, 24, 206, None, 24).is_err());
        assert!(header(u64::MAX - 1, 3).is_err());
        assert!(header(0, MAX_RANGE + 1).is_err());
        assert!(header(0, 0).is_err());
    }
    #[test]
    fn only_the_initial_probe_may_finish_at_an_earlier_file_end() {
        assert_eq!(
            accept(
                0,
                PROBE_BYTES,
                206,
                Some("bytes 0-23/24"),
                24
            )
            .unwrap(),
            24
        );
        assert!(
            accept(
                0,
                PROBE_BYTES,
                206,
                Some("bytes 0-23/100"),
                24
            )
            .is_err()
        );
        assert!(
            accept(
                1,
                PROBE_BYTES,
                206,
                Some("bytes 1-23/24"),
                23
            )
            .is_err()
        );
        for raw in [
            "bytes */24",
            "bytes 0-23/*",
            "items 0-23/24",
            "bytes 24-23/24",
        ] {
            assert!(content_range(raw).is_none());
        }
    }
}
```

## <a id="checkpoint-67c-retain-original-point-page-values"></a>4. Retain point page values

Retain original point page values.

### 1. `src/range_page.rs`

Retain original f64 pages and stable IDs separately from derived display values.

Create the file and type:

```rust
use std::ops::Range;

pub struct Page {
    pub rows: Range<u32>,
    pub coordinates: Vec<f64>,
    pub display: Vec<f32>,
    pub ids: Vec<u32>,
}
impl Page {
    pub fn decode(
        rows: Range<u32>,
        total: u32,
        raw: &[u8],
        ids: Option<&[u8]>,
    ) -> Result<Self, &'static str> {
        if rows.start > rows.end || rows.end > total {
            return Err("Point rows outside source");
        }
        let count = (rows.end - rows.start) as usize;
        if raw.len()
            != count
                .checked_mul(24)
                .ok_or("Page size overflow")?
        {
            return Err("Incomplete original coordinate page");
        }
        let coordinates = raw
            .chunks_exact(8)
            .map(|v| f64::from_le_bytes(v.try_into().unwrap()))
            .collect::<Vec<_>>();
        if coordinates
            .iter()
            .any(|v| !v.is_finite() || !(*v as f32).is_finite())
        {
            return Err("Invalid source coordinates");
        }
        let ids = if let Some(ids) = ids {
            if ids.len()
                != count
                    .checked_mul(4)
                    .ok_or("ID page size overflow")?
            {
                return Err("Incomplete original point ID page");
            }
            ids.chunks_exact(4)
                .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
                .collect::<Vec<_>>()
        } else {
            rows.clone().collect()
        };
        if ids
            .iter()
            .copied()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != count
        {
            return Err("Repeated original point IDs");
        }
        let display = coordinates
            .iter()
            .map(|v| *v as f32)
            .collect();
        Ok(Self {
            rows,
            coordinates,
            display,
            ids,
        })
    }
    pub fn bytes(&self) -> usize {
        self.coordinates.capacity() * 8
            + self.display.capacity() * 4
            + self.ids.capacity() * 4
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod range_rules;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod range_rules;

pub mod range_page;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/range_page.rs`

Copy source precision, high-ID, incomplete page and invalid-double fixtures.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pages_keep_original_f64_coordinates_high_ids_and_fallback_rows() {
        let raw = [
            1_000_000_000.125f64,
            2.,
            3.,
            1_000_000_000.375,
            4.,
            5.,
        ]
        .into_iter()
        .flat_map(f64::to_le_bytes)
        .collect::<Vec<_>>();
        let ids = [u32::MAX, 7]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>();
        let page = Page::decode(8..10, 20, &raw, Some(&ids)).unwrap();
        assert_eq!(page.coordinates[0], 1_000_000_000.125);
        assert_eq!(page.coordinates[3], 1_000_000_000.375);
        assert_eq!(page.display[0], page.display[3]);
        assert_eq!(page.ids, [u32::MAX, 7]);
        assert_eq!(
            Page::decode(8..10, 20, &raw, None)
                .unwrap()
                .ids,
            [8, 9]
        );
        assert!(Page::decode(8..10, 20, &raw[..47], Some(&ids)).is_err());
        assert!(Page::decode(8..10, 20, &raw, Some(&ids[..7])).is_err());
        assert!(Page::decode(8..10, 9, &raw, None).is_err());
        assert!(Page::decode(10..8, 20, &raw, None).is_err());
        let duplicates = [7u32, 7]
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>();
        assert!(Page::decode(8..10, 20, &raw, Some(&duplicates)).is_err());
    }
    #[test]
    fn invalid_original_doubles_never_become_a_display_page() {
        for value in [f64::NAN, f64::INFINITY, f64::MAX] {
            let raw = [value, 0., 0.]
                .into_iter()
                .flat_map(f64::to_le_bytes)
                .collect::<Vec<_>>();
            assert!(Page::decode(0..1, 1, &raw, None).is_err());
        }
    }
}
```

## <a id="checkpoint-67d-own-cancellable-range-requests"></a>5. Own cancellable range requests

### 1. `Cargo.toml`

Enable browser CORS request configuration without changing package versions.

**Locate this existing block:**

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wasm-bindgen = "=0.2.128"
web-sys = { version = "=0.3.105", features = ["console", "Window", "Document", "Element", "HtmlCanvasElement", "EventTarget", "AddEventListenerOptions", "PageTransitionEvent", "ErrorEvent", "PromiseRejectionEvent", "Location", "History", "UrlSearchParams", "Navigator", "Storage", "Performance", "PerformanceResourceTiming", "PerformanceEntry", "Event", "MouseEvent", "PointerEvent", "DomRect", "KeyboardEvent", "WheelEvent", "AddEventListenerOptions", "HtmlElement", "HtmlInputElement", "File", "FileList", "Blob", "CustomEvent", "CustomEventInit", "FocusOptions", "Url", "HtmlAnchorElement", "AbortController", "AbortSignal", "RequestInit", "RequestMode", "RequestCache", "EventSource", "MessageEvent", "Response", "Headers", "ReadableStream", "ReadableStreamDefaultReader"] }
console_error_panic_hook = "=0.1.7"
```

### 2. `src/range_fetch.rs`

Own active browser request handles and explicit cancellation.

Create the file and type:

```rust
pub use crate::range_rules::PROBE_BYTES;
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;

thread_local! {
    static REQUESTS:RefCell<BTreeMap<u64,web_sys::AbortController>>=const {RefCell::new(BTreeMap::new())};
    static NEXT:Cell<u64>=const {Cell::new(0)};
}

pub fn cancel() {
    for controller in REQUESTS
        .with(|requests| std::mem::take(&mut *requests.borrow_mut()))
        .into_values()
    {
        controller.abort();
    }
}

struct Read {
    key: u64,
    controller: web_sys::AbortController,
    timer: i32,
    stalled: Rc<Cell<bool>>,
    callback: Closure<dyn FnMut()>,
}
```

### 3. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(target_arch = "wasm32")]
pub mod range_fetch;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
```

## <a id="checkpoint-67e-time-out-stalled-source-reads"></a>6. Time out stalled source reads

### 1. `src/range_fetch.rs`

Connect the owned stall timer, progress reset and request cleanup.

**Locate this existing block:**

```rust
    callback: Closure<dyn FnMut()>,
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    callback: Closure<dyn FnMut()>,
}
impl Read {
    fn new() -> Result<Self, String> {
        let controller =
            web_sys::AbortController::new().map_err(|error| describe(error.into()))?;
        let key = NEXT.with(|next| {
            let key = next
                .get()
                .checked_add(1)
                .ok_or("Range request IDs exhausted")?;
            next.set(key);
            Ok::<_, String>(key)
        })?;
        let stalled = Rc::new(Cell::new(false));
        let owned = controller.clone();
        let flag = stalled.clone();
        let callback = Closure::<dyn FnMut()>::new(move || {
            flag.set(true);
            owned.abort();
        });
        let mut read = Self {
            key,
            controller,
            timer: 0,
            stalled,
            callback,
        };
        read.touch()?;
        REQUESTS.with(|requests| {
            requests
                .borrow_mut()
                .insert(key, read.controller.clone())
        });
        Ok(read)
    }
    fn touch(&mut self) -> Result<(), String> {
        let window = web_sys::window().ok_or("No browser window")?;
        window.clear_timeout_with_handle(self.timer);
        self.timer = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                self.callback.as_ref().unchecked_ref(),
                30_000,
            )
            .map_err(|error| describe(error.into()))?;
        Ok(())
    }
    fn error(&self, error: JsValue) -> String {
        if self.stalled.get() {
            "network error: range read stalled for30s".into()
        } else if self.controller.signal().aborted() {
            "Range read cancelled".into()
        } else {
            format!("network error: {}", describe(error))
        }
    }
}
impl Drop for Read {
    fn drop(&mut self) {
        REQUESTS.with(|requests| requests.borrow_mut().remove(&self.key));
        self.controller.abort();
        if let Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(self.timer);
        }
    }
}
fn describe(error: JsValue) -> String {
    error
        .as_string()
        .unwrap_or_else(|| format!("{error:?}"))
}
pub fn retryable(error: &str) -> bool {
    error.starts_with("network error")
}
```

## <a id="checkpoint-67f-bound-incoming-range-body-chunks"></a>7. Bound incoming range body chunks

### 1. `src/range_fetch.rs`

Read bounded body chunks while retaining the checked source request owner.

**Locate this existing block:**

```rust
    error.starts_with("network error")
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    error.starts_with("network error")
}

#[derive(Default)]
pub struct GetOpts {
    pub range: Option<(u64, u64)>,
    pub revalidate: bool,
}
pub struct Reply {
    pub status: u16,
    pub etag: Option<String>,
    pub total: Option<u64>,
    pub bytes: Vec<u8>,
}
impl Reply {
    pub fn whole(self) -> Option<Vec<u8>> {
        (self.status == 206 && self.total == Some(self.bytes.len() as u64))
            .then_some(self.bytes)
    }
}

async fn body(
    reader: &web_sys::ReadableStreamDefaultReader,
    limit: u64,
    read: &mut Read,
) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    loop {
        let part = JsFuture::from(reader.read())
            .await
            .map_err(|error| read.error(error))?;
        if js_sys::Reflect::get(&part, &"done".into())
            .map_err(|error| describe(error.into()))?
            .as_bool()
            == Some(true)
        {
            break;
        }
        let chunk = js_sys::Reflect::get(&part, &"value".into())
            .map_err(|error| describe(error.into()))?
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|error| describe(error.into()))?;
        if u64::from(chunk.length()) > limit - bytes.len() as u64 {
            return Err("Range response exceeds its byte limit".into());
        }
        bytes.extend_from_slice(&chunk.to_vec());
        read.touch()?;
    }
    Ok(bytes)
}
```

## <a id="checkpoint-67g-request-and-accept-one-source-interval"></a>8. Request and accept one source interval

### 1. `src/range_fetch.rs`

Connect HTTP range headers and exact response acceptance.

**Locate this existing block:**

```rust
}

async fn body(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub async fn get(url: &str, opts: &GetOpts) -> Result<Reply, String> {
    let (start, length) = opts
        .range
        .ok_or("This reader requires a byte range")?;
    if length == 0 {
        return Ok(Reply {
            status: 206,
            etag: None,
            total: None,
            bytes: vec![],
        });
    }
    let range = crate::range_rules::header(start, length)?;
    let mut read = Read::new()?;
    let init = web_sys::RequestInit::new();
    init.set_signal(Some(&read.controller.signal()));
    init.set_mode(web_sys::RequestMode::Cors);
    if opts.revalidate {
        init.set_cache(web_sys::RequestCache::NoCache);
    }
    let headers = web_sys::Headers::new().map_err(|error| describe(error.into()))?;
    headers
        .set("Range", &range)
        .map_err(|error| describe(error.into()))?;
    init.set_headers(&headers);
    let window = web_sys::window().ok_or("No browser window")?;
    let started = crate::browser_phase::now();
    let response = JsFuture::from(window.fetch_with_str_and_init(url, &init))
        .await
        .map_err(|error| read.error(error))?
        .dyn_into::<web_sys::Response>()
        .map_err(|error| describe(error.into()))?;
    let status = response.status();
    let etag = response
        .headers()
        .get("ETag")
        .map_err(|error| describe(error.into()))?;
    let content = response
        .headers()
        .get("Content-Range")
        .map_err(|error| describe(error.into()))?;
    if status != 206 {
        return Ok(Reply {
            status,
            etag,
            total: None,
            bytes: vec![],
        });
    }
    if response
        .headers()
        .get("Content-Length")
        .map_err(|error| describe(error.into()))?
        .is_some_and(|size| {
            size.parse::<u64>()
                .map_or(true, |size| size > length)
        })
    {
        return Err("Range response exceeds its byte limit".into());
    }
    let reader = response
        .body()
        .ok_or("Range response has no body")?
        .get_reader()
        .dyn_into::<web_sys::ReadableStreamDefaultReader>()
        .map_err(|error| describe(error.into()))?;
    let result = body(&reader, length, &mut read).await;
    if result.is_err() {
        let _ = reader.cancel();
    }
    reader.release_lock();
    let bytes = result?;
    let total = crate::range_rules::accept(
        start,
        length,
        status,
        content.as_deref(),
        bytes.len() as u64,
    )?;
    crate::browser_phase::finish(
        "HTTP range",
        started,
        bytes.len() as u64,
        url,
    );
    Ok(Reply {
        status,
        etag,
        total: Some(total),
        bytes,
    })
}
async fn body(
```

## <a id="checkpoint-67h-keep-range-revisions-and-background-results"></a>9. Keep range revisions and background results

### 1. `src/range_fetch.rs`

Validate source revisions and provide background range reads.

**Locate this existing block:**

```rust
    Ok(bytes)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(bytes)
}
pub async fn fetch_range(
    url: &str,
    start: u64,
    len: u64,
    revision: &Option<String>,
) -> Result<(Vec<u8>, Option<String>), String> {
    let reply = get(
        url,
        &GetOpts {
            range: Some((start, len)),
            revalidate: !crate::manifest::immutable_key(url),
        },
    )
    .await?;
    if reply.status != 206 || reply.bytes.len() as u64 != len {
        return Err(format!(
            "Range read failed: HTTP{} with{} of{len}bytes",
            reply.status,
            reply.bytes.len()
        ));
    }
    if revision.is_some() && revision != &reply.etag {
        return Err("Source changed during the range read; reload it".into());
    }
    Ok((reply.bytes, reply.etag))
}

pub struct Task<T> {
    done: js_sys::Promise,
    out: Rc<RefCell<Option<T>>>,
}
impl<T: 'static> Task<T> {
    pub fn start(work: impl Future<Output = T> + 'static) -> Self {
        let out = Rc::new(RefCell::new(None));
        let slot = out.clone();
        let done = wasm_bindgen_futures::future_to_promise(async move {
            *slot.borrow_mut() = Some(work.await);
            Ok(JsValue::UNDEFINED)
        });
        Self { done, out }
    }
    pub async fn wait(self) -> Option<T> {
        let _ = JsFuture::from(self.done).await;
        self.out.take()
    }
}
```

## <a id="checkpoint-67hza-queue-original-source-range-reads"></a>10. Queue source range reads

Write the browser range-read owner and cached metadata-window read.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

**Locate this existing block:**

```rust
}

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

#[cfg(target_arch = "wasm32")]
pub use web::*;

/// The reads: find the arrays, then fetch slices by range.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::manifest::immutable_key;
    use crate::range_fetch::{
        GetOpts, PROBE_BYTES, Reply, Task, fetch_range, get, retryable,
    };
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
                crate::browser_phase::finish(
                    "Range retry",
                    crate::browser_phase::now(),
                    0,
                    &error,
                );
                Some(
                    fetch_range(url, at, length, revision)
                        .await
                        .ok()?
                        .0,
                )
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
```

## <a id="checkpoint-67hzb-probe-original-source-array-layouts"></a>11. Probe source array layouts

Start owned asynchronous range tasks and inspect the bounded file probe.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

**Locate this existing block:**

```rust
        }
    }
}

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
    }

    /// Start reading one range of the file.
    pub(super) fn start_range(
        url: &str,
        (at, length): (u64, u64),
        revision: &Option<String>,
    ) -> Task<Option<Vec<u8>>> {
        let (url, revision) = (url.to_string(), revision.clone());
        Task::start(async move { source_range(&url, at, length, &revision).await })
    }

    /// The first 8 KB of a file, read once for every layout check and the file's size.
    pub async fn probe(url: &str) -> Option<Reply> {
        let reply = get(
            url,
            &GetOpts {
                range: Some((0, PROBE_BYTES)),
                revalidate: !immutable_key(url),
                ..Default::default()
            },
        )
        .await
        .ok()?;
        (reply.bytes.len() as u64 <= PROBE_BYTES).then_some(reply)
    }

    /// Find where a cloud's arrays are in the file, from its `probe`.
    pub async fn cloud_fields(url: &str, probe: &Reply) -> Option<CloudFields> {
        if probe.status != 206 {
            return None;
        }

        let (coords_at, coords_len, end) = cloud_layout(&probe.bytes)?;
        if end > probe.total? {
            return None;
        }

        if coords_len == 0 || !coords_len.is_multiple_of(POINT_BYTES) {
            return None;
        }

        let after = body_end(coords_at, coords_len, end)?;
        let mut colors = (after, 0);

        if after < end {
            let header =
                source_range(url, after, 16.min(end - after), &probe.etag).await?;
            let (tag, used) = varint(&header, 0)?;

            if tag >> 3 == 4 && tag & 7 == 2 {
                let (length, extra) = varint(&header, used)?;
                let body = after.checked_add((used + extra) as u64)?;
                body_end(body, length, end)?;
                colors = (body, length);
            }
        }

        Some(CloudFields {
            end,
            coords_at,
            coords_len,
            colors_at: colors.0,
            colors_len: colors.1,
            normals_at: 0,
            normals_len: 0,
            count: u32::try_from(coords_len / POINT_BYTES).ok()?,
            ids_at: 0,
            ids_len: 0,
            revision: probe.etag.clone(),
        })
    }
}

#[cfg(test)]
```

## <a id="checkpoint-67hzc-read-original-cloud-node-tables"></a>12. Read cloud node tables

Read original node-table fields through the cached metadata window.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

**Locate this existing block:**

```rust
        })
    }
}

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        })
    }

    /// Read the cloud's node table.
    pub async fn cloud_lod(url: &str, fields: &mut CloudFields) -> Option<CloudLod> {
        let mut at = body_end(
            fields.colors_at,
            fields.colors_len,
            fields.end,
        )?;
        let mut lod = CloudLod::default();
        let mut seen = [false; 7];
        let mut table_bytes = 0u64;
        let mut ids = None;
        let mut normals = (0, 0);
        let mut window = MetadataWindow::default();

        while at < fields.end {
            let header = window
                .read(
                    url,
                    at,
                    64.min(fields.end - at),
                    fields.end,
                    &fields.revision,
                )
                .await?;
            let (tag, used) = varint(header, 0)?;
            let (field, wire) = (
                usize::try_from(tag >> 3).ok()?,
                (tag & 7) as u32,
            );

            if field == 0 {
                return None;
            }

            if wire != 2 {
                if (8..=15).contains(&field) {
                    return None;
                }

                let skip = skip_scalar(header, used, wire)?;
                at = body_end(at, (used + skip) as u64, fields.end)?;
                continue;
            }

            let (length, extra) = varint(header, used)?;
            let body = body_end(at, (used + extra) as u64, fields.end)?;
            let next = body_end(body, length, fields.end)?;

            if (8..=14).contains(&field) {
                if seen[field - 8] {
                    return None;
                }

                table_bytes = table_bytes.checked_add(length)?;

                if table_bytes > MAX_TABLE_BYTES {
                    return None;
                }

                let raw = window
                    .read(
                        url,
                        body,
                        length,
                        fields.end,
                        &fields.revision,
                    )
                    .await?;

                if !lod.set_field(field, raw) {
                    return None;
                }

                seen[field - 8] = true;
            }

            if field == 5 && length == fields.coords_len {
                normals = (body, length);
            }

            if field == 15 {
                if length != u64::from(fields.count) * 4 || ids.is_some() {
                    return None;
                }

                ids = Some((body, length));
            }

            at = next;
        }

        if seen.contains(&false) || !lod.valid(fields.count) {
            return None;
        }

        (fields.ids_at, fields.ids_len) = ids.unwrap_or((0, 0));
        (fields.normals_at, fields.normals_len) = normals;
        Some(lod)
    }
}

#[cfg(test)]
```

## <a id="checkpoint-67i-read-source-arrays-through-the-bounded-queue"></a>13. Fetch coordinates, normals, and colours

Fetch bounded source-array slices through the queue already written.

### 1. `src/stream.rs`

Type today’s complete browser source readers; reuse the decoding and queue helpers already written.

**Locate this existing block:**

```rust
        Some(lod)
    }
}

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Some(lod)
    }

    /// Points `[from, to)` of the cloud.
    pub async fn fetch_positions(
        url: &str,
        fields: &CloudFields,
        from: u32,
        to: u32,
    ) -> Option<Vec<f32>> {
        if from > to || to > fields.count {
            return None;
        }

        let at = fields
            .coords_at
            .checked_add(u64::from(from).checked_mul(POINT_BYTES)?)?;
        let length = u64::from(to - from).checked_mul(POINT_BYTES)?;
        body_end(
            at,
            length,
            body_end(
                fields.coords_at,
                fields.coords_len,
                fields.end,
            )?,
        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        checked_positions(&raw, to - from)
    }

    /// Packed normals of points `[from, to)`; None when the cloud has none.
    pub async fn fetch_normals(
        url: &str,
        fields: &CloudFields,
        from: u32,
        to: u32,
    ) -> Option<Vec<u32>> {
        if fields.normals_len == 0 || from > to || to > fields.count {
            return None;
        }

        let at = fields
            .normals_at
            .checked_add(u64::from(from).checked_mul(POINT_BYTES)?)?;
        let length = u64::from(to - from).checked_mul(POINT_BYTES)?;
        body_end(
            at,
            length,
            body_end(
                fields.normals_at,
                fields.normals_len,
                fields.end,
            )?,
        )?;
        let raw = source_range(url, at, length, &fields.revision).await?;
        (raw.len() as u64 == length).then(|| normals_from(&raw))
    }

    /// Colours of `count` points starting at byte `at`.
    pub async fn fetch_colors(
        url: &str,
        fields: &CloudFields,
        at: u64,
        count: u32,
    ) -> Option<(Vec<u32>, u64)> {
        let end = body_end(
            fields.colors_at,
            fields.colors_len,
            fields.end,
        )?;

        if at < fields.colors_at || at > end || count > fields.count {
            return None;
        }

        // a colour is at most 8 bytes of varints
        let length = (u64::from(count) * 8).min(end - at);

        if length == 0 {
            return None;
        }

        let raw = source_range(url, at, length, &fields.revision).await?;
        let (colors, used) = colors_from(&raw, count)?;
        Some((colors, body_end(at, used as u64, end)?))
    }
}

#[cfg(test)]
```

## <a id="checkpoint-67j-keep-original-ranged-source-identity"></a>14. Keep ranged source identity

Keep original ranged source identity.

### 1. `src/stream.rs`

Read original source identity independently of display pages.

**Locate this existing block:**

```rust
}

/// Start, length of `coords` and end of the cloud message.
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

/// Original GUID and name precede the large coordinate array in the ranged layout.
pub fn cloud_identity(head: &[u8]) -> Option<(String, String)> {
    let mut at = 0usize;
    let objects_end = descend_message(head, &mut at, None, 3)?;
    let end = descend_message(head, &mut at, Some(objects_end), 8)?;
    let mut guid = String::new();
    let mut name = String::new();
    while (at as u64) < end {
        let f = field_at(head.get(at..)?, at as u64, end)?;
        if f.field == 3 {
            return (!guid.is_empty()).then_some((guid, name));
        }
        if f.wire == 2 && (f.field == 1 || f.field == 2) && f.value <= NAME_BYTES {
            let value =
                std::str::from_utf8(head.get(f.body as usize..f.next as usize)?)
                    .ok()?
                    .to_owned();
            if f.field == 1 {
                if !guid.is_empty() {
                    return None;
                }
                guid = value;
            } else {
                name = value;
            }
        }
        at = usize::try_from(f.next).ok()?;
    }
    None
}

/// Start, length of `coords` and end of the cloud message.
```

## <a id="checkpoint-67k-prepare-a-revision-bound-remote-source"></a>15. Prepare a revision-bound remote source

### 1. `src/range_source.rs`

Prepare a revision-bound original remote source descriptor.

Create the file and type:

```rust
#[cfg(target_arch = "wasm32")]
pub struct Source {
    pub guid: String,
    pub name: String,
    pub url: String,
    pub fields: crate::stream::CloudFields,
    pub lod: crate::stream::CloudLod,
}

#[cfg(target_arch = "wasm32")]
impl Source {
    pub async fn open(url: String) -> Result<Self, String> {
        let probe = crate::stream::probe(&url)
            .await
            .ok_or("Source probe failed")?;
        if probe.etag.is_none() && !crate::manifest::immutable_key(&url) {
            return Err("Streaming requires an ETag or immutable source URL".into());
        }
        let (guid, name) = crate::stream::cloud_identity(&probe.bytes)
            .ok_or("Missing original cloud identity")?;
        let mut fields = crate::stream::cloud_fields(&url, &probe)
            .await
            .ok_or("Not a single range-readable cloud")?;
        let lod = crate::stream::cloud_lod(&url, &mut fields)
            .await
            .ok_or("Invalid original cloud array or octree layout")?;
        Ok(Self {
            guid,
            name,
            url,
            fields,
            lod,
        })
    }
}
```

### 2. `src/lib.rs`

Register the source component.

**Locate this existing block:**

```rust
pub mod range_fetch;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod range_fetch;

pub mod range_source;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-67l-fetch-precise-coordinates-and-original-ids"></a>16. Fetch precise coordinates and IDs

Fetch precise coordinates and original IDs.

### 1. `src/range_source.rs`

Fetch matching original coordinate and ID pages under the source revision. Copy actual kernel-protobuf identity, precision and page-layout regression fixtures.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub async fn page(
        &self,
        rows: std::ops::Range<u32>,
    ) -> Result<crate::range_page::Page, String> {
        if rows.start > rows.end || rows.end > self.fields.count {
            return Err("Point rows outside source".into());
        }
        let count = u64::from(rows.end - rows.start);
        let at = self
            .fields
            .coords_at
            .checked_add(u64::from(rows.start) * 24)
            .ok_or("Coordinate offset overflow")?;
        let raw = crate::stream::source_range(
            &self.url,
            at,
            count * 24,
            &self.fields.revision,
        )
        .await
        .ok_or("Coordinate range failed")?;
        let ids = if self.fields.ids_len == 0 {
            None
        } else {
            let at = self
                .fields
                .ids_at
                .checked_add(u64::from(rows.start) * 4)
                .ok_or("ID offset overflow")?;
            Some(
                crate::stream::source_range(
                    &self.url,
                    at,
                    count * 4,
                    &self.fields.revision,
                )
                .await
                .ok_or("Original ID range failed")?,
            )
        };
        crate::range_page::Page::decode(rows, self.fields.count, &raw, ids.as_deref())
            .map_err(str::to_owned)
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;
    #[test]
    fn range_layout_and_original_pages_match_the_kernel_protobuf() {
        let mut cloud = session_rust::PointCloud::from_coords(
            vec![
                1_000_000_000.125,
                2.,
                3.,
                1_000_000_000.375,
                4.,
                5.,
            ],
            vec![],
            vec![],
        );
        cloud.name = "Precise ranged source".into();
        let _ = cloud.guid();
        let mut proto = cloud.to_proto();
        proto.point_ids = vec![u32::MAX, 7];
        let mut objects = session_rust::proto::Objects::default();
        objects.pointclouds.push(proto.clone());
        let document = session_rust::proto::Session {
            objects: Some(objects),
            ..Default::default()
        };
        let raw = document.encode_to_vec();
        assert_eq!(
            crate::stream::cloud_identity(&raw).unwrap(),
            (proto.guid.clone(), proto.name.clone())
        );
        let (at, length) = crate::stream::walk_to_coords(&raw).unwrap();
        assert_eq!(length, 48);
        let page = crate::range_page::Page::decode(
            0..2,
            2,
            &raw[at as usize..(at + length) as usize],
            Some(
                &proto
                    .point_ids
                    .iter()
                    .flat_map(|id| id.to_le_bytes())
                    .collect::<Vec<_>>(),
            ),
        )
        .unwrap();
        assert_eq!(page.coordinates, proto.coords);
        assert_eq!(page.ids, proto.point_ids);
        assert!(crate::stream::walk_to_coords(&raw[..at as usize - 1]).is_none());
    }
}
```

## <a id="checkpoint-67m-cancel-range-work-when-its-runtime-closes"></a>17. Cancel range work when its runtime closes

### 1. `src/browser_runtime.rs`

Cancel queued and active range work with its browser runtime.

**Locate this existing block:**

```rust
    fn drop(&mut self) {
        self.request.borrow_mut().cancel();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    fn drop(&mut self) {
        crate::stream::reset_range_gate();
        self.request.borrow_mut().cancel();
```

### 2. `src/browser.rs`

Cancel queued and active range work with its browser runtime.

**Locate this existing block:**

```rust
                        if matches!(action, Action::Close) {
                            publication_control.cancel();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        if matches!(action, Action::Close) {
                            crate::stream::reset_range_gate();
                            publication_control.cancel();
```

## Build & Run

![Recorded visual output for Lesson 12.3 (67m-cancel-range-work-when-its-runtime-closes)](../../../screenshots/journey/67m-cancel-range-work-when-its-runtime-closes.png)

```sh
npm --prefix ../session_tests run course -- dependencies 12.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 12.3
npm --prefix ../session_tests run course -- save 12.3
```

Continue with [12.4 · Charged residency and current streamed pages](../../consolidation/68-reserve-page-bytes-before-reading.md).
