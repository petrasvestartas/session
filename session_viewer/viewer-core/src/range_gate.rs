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
            None => self.running.set(self.running.get().saturating_sub(1)),
        }
    }
}

/// A read waiting at the gate.
pub struct Enter {
    gate: Rc<RangeGate>,    // the gate
    turn: Option<Rc<Turn>>, // its place in the queue, once queued
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
            None if gate.running.get() < gate.limit && gate.queue.borrow().is_empty() => {
                gate.running.set(gate.running.get() + 1);
                Poll::Ready(Some(Permit { gate }))
            }
            None => {
                let turn = Rc::new(Turn::default());
                *turn.waker.borrow_mut() = Some(cx.waker().clone());
                gate.queue.borrow_mut().push_back(turn.clone());
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
        assert!(matches!(poll(&mut fourth), Poll::Ready(Some(_))));
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
        assert!(matches!(poll(&mut second), Poll::Ready(Some(_))));
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
        assert!(matches!(poll(&mut second), Poll::Ready(None)));
        assert!(matches!(poll(&mut gate.enter()), Poll::Ready(None)));
        drop(one);
        assert_eq!(gate.load(), (0, 0));
    }
}
