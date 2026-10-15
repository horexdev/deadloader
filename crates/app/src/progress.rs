//! An apply's progress for the Library's applying banner, thinned out for the UI: nothing for an apply that ends
//! quickly (hard links take milliseconds), then a few reports a second, and a last one when it ends. Nothing runs while
//! no apply does: reports are made only when the engine calls.

use deadloader_core::engine::ApplyProgress;
use serde::Serialize;
use std::time::Duration;

/// An apply that ends sooner sends nothing: the banner would only flash.
pub const QUIET: Duration = Duration::from_millis(300);
/// At most one report this often.
pub const EVERY: Duration = Duration::from_millis(250);
/// Copying must have run this long before its rate says how long the rest takes.
const RATE_AFTER: Duration = Duration::from_secs(1);

/// What the UI hears about a running apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Applying {
    #[serde(flatten)]
    pub progress: ApplyProgress,
    /// The time the rest takes at the rate files were copied so far; `None` while nothing was copied, too early to
    /// tell, or only links are made (they take no time).
    pub left_ms: Option<u64>,
    /// The apply ended (done or failed): the banner goes.
    pub finished: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Throttle {
    quiet: Duration,
    every: Duration,
    sent: Option<Duration>,
    latest: ApplyProgress,
}

impl Default for Throttle {
    fn default() -> Self {
        Self::new(QUIET, EVERY)
    }
}

impl Throttle {
    pub fn new(quiet: Duration, every: Duration) -> Self {
        Self {
            quiet,
            every,
            sent: None,
            latest: ApplyProgress::default(),
        }
    }

    /// The engine's latest progress, `elapsed` since the apply started: what to send now, if anything.
    pub fn report(&mut self, progress: &ApplyProgress, elapsed: Duration) -> Option<Applying> {
        self.latest = *progress;
        if elapsed < self.quiet {
            return None;
        }
        if self
            .sent
            .is_some_and(|at| elapsed.saturating_sub(at) < self.every)
        {
            return None;
        }
        self.sent = Some(elapsed);
        Some(Applying {
            progress: *progress,
            left_ms: left_ms(progress, elapsed),
            finished: false,
        })
    }

    /// The apply ended: the last report, when the UI heard of it at all.
    pub fn finish(&mut self) -> Option<Applying> {
        self.sent.take().map(|_| Applying {
            progress: self.latest,
            left_ms: None,
            finished: true,
        })
    }
}

/// The rest of the bytes at the rate copies ran so far. Every file of one apply goes from the same library to the same
/// folder, so once one is copied the rest are too.
fn left_ms(progress: &ApplyProgress, elapsed: Duration) -> Option<u64> {
    if progress.bytes_copied == 0 || elapsed < RATE_AFTER {
        return None;
    }
    let rest = progress.bytes_total.saturating_sub(progress.bytes_done);
    let ms = u128::from(rest) * elapsed.as_millis() / u128::from(progress.bytes_copied);
    Some(u64::try_from(ms).unwrap_or(u64::MAX))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(ms: u64) -> Duration {
        Duration::from_millis(ms)
    }

    fn step(done: usize) -> ApplyProgress {
        ApplyProgress {
            done,
            total: 10,
            ..ApplyProgress::default()
        }
    }

    #[test]
    fn a_quick_apply_sends_nothing() {
        let mut t = Throttle::default();
        for i in 0..10 {
            assert_eq!(t.report(&step(i), at(i as u64 * 20)), None);
        }
        assert_eq!(t.finish(), None);
    }

    #[test]
    fn a_slow_apply_reports_a_few_times_a_second_and_once_more_at_the_end() {
        let mut t = Throttle::default();
        let sent: Vec<usize> = (0..=10)
            .filter_map(|i| t.report(&step(i), at(i as u64 * 100)))
            .map(|a| a.progress.done)
            .collect();
        // At 300, 600 and 900 ms.
        assert_eq!(sent, [3, 6, 9]);
        let last = t.finish().expect("the end is reported");
        assert!(last.finished);
        assert_eq!(last.progress.done, 10);
        assert_eq!(t.finish(), None, "only once");
    }

    #[test]
    fn the_time_left_follows_the_copy_rate() {
        let mut t = Throttle::default();
        let copying = ApplyProgress {
            done: 2,
            total: 6,
            linked: 0,
            copied: 1,
            bytes_done: 100,
            bytes_total: 400,
            bytes_copied: 100,
        };
        // 100 bytes in 2 s: the other 300 take 6 s.
        let a = t.report(&copying, at(2000)).expect("sent");
        assert_eq!(a.left_ms, Some(6000));
        assert!(!a.finished);
    }

    #[test]
    fn no_time_left_before_anything_was_copied_or_too_early() {
        let linking = ApplyProgress {
            linked: 3,
            bytes_done: 300,
            bytes_total: 400,
            ..step(3)
        };
        assert_eq!(left_ms(&linking, at(5000)), None);
        let early = ApplyProgress {
            copied: 1,
            bytes_done: 100,
            bytes_total: 400,
            bytes_copied: 100,
            ..step(1)
        };
        assert_eq!(left_ms(&early, at(400)), None);
    }

    /// The shape `ui/src/lib/api.ts` relies on.
    #[test]
    fn reports_are_flat_for_the_ui() {
        let a = Applying {
            progress: ApplyProgress {
                done: 1,
                total: 2,
                linked: 1,
                copied: 0,
                bytes_done: 5,
                bytes_total: 9,
                bytes_copied: 0,
            },
            left_ms: None,
            finished: false,
        };
        assert_eq!(
            serde_json::to_value(a).expect("json"),
            serde_json::json!({
                "done": 1, "total": 2, "linked": 1, "copied": 0, "bytes_done": 5, "bytes_total": 9,
                "bytes_copied": 0, "left_ms": null, "finished": false
            })
        );
    }
}
