//! A counting gate: at most N callers inside at once, the rest wait (parallel API requests, decision 99).

use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

#[derive(Debug)]
pub(crate) struct Gate {
    state: Mutex<State>,
    freed: Condvar,
}

#[derive(Debug)]
struct State {
    limit: usize,
    inside: usize,
}

pub(crate) struct Pass<'a>(&'a Gate);

impl Drop for Pass<'_> {
    fn drop(&mut self) {
        self.0.lock().inside -= 1;
        self.0.freed.notify_one();
    }
}

impl Gate {
    pub(crate) fn new(limit: usize) -> Self {
        Self {
            state: Mutex::new(State {
                limit: limit.max(1),
                inside: 0,
            }),
            freed: Condvar::new(),
        }
    }

    /// Takes effect for the next callers; those inside finish normally.
    pub(crate) fn set_limit(&self, limit: usize) {
        self.lock().limit = limit.max(1);
        self.freed.notify_all();
    }

    /// Waits until fewer than the limit are inside.
    pub(crate) fn enter(&self) -> Pass<'_> {
        let mut state = self.lock();
        while state.inside >= state.limit {
            state = self
                .freed
                .wait(state)
                .unwrap_or_else(PoisonError::into_inner);
        }
        state.inside += 1;
        Pass(self)
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[test]
    fn never_lets_more_than_the_limit_in() {
        let gate = Arc::new(Gate::new(2));
        let inside = Arc::new(AtomicUsize::new(0));
        let most = Arc::new(AtomicUsize::new(0));
        let threads: Vec<_> = (0..8)
            .map(|_| {
                let (gate, inside, most) =
                    (Arc::clone(&gate), Arc::clone(&inside), Arc::clone(&most));
                std::thread::spawn(move || {
                    let _pass = gate.enter();
                    let now = inside.fetch_add(1, Ordering::SeqCst) + 1;
                    most.fetch_max(now, Ordering::SeqCst);
                    std::thread::sleep(Duration::from_millis(5));
                    inside.fetch_sub(1, Ordering::SeqCst);
                })
            })
            .collect();
        for t in threads {
            t.join().expect("thread");
        }
        assert!(most.load(Ordering::SeqCst) <= 2);
        assert_eq!(gate.lock().inside, 0);
    }

    #[test]
    fn a_zero_limit_still_lets_one_in() {
        let gate = Gate::new(0);
        let _pass = gate.enter();
        gate.set_limit(0);
        assert_eq!(gate.lock().limit, 1);
    }
}
