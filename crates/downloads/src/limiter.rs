//! The global speed limit: one token bucket shared by every download, changeable while downloads run.

use deadloader_net::clock::Clock;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

/// The largest single grant, and so the largest read a download makes at once.
pub const CHUNK: usize = 64 * 1024;

/// A token per byte. The bucket holds a tenth of a second of the rate and a grant is never larger, so data keeps
/// flowing in small steps even at a very low limit (the stall detection sees progress) and a pause in reading does
/// not build up a burst.
pub struct RateLimiter {
    clock: Arc<dyn Clock>,
    bucket: Mutex<Bucket>,
}

impl std::fmt::Debug for RateLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RateLimiter")
            .field("bytes_per_s", &self.bytes_per_s())
            .finish_non_exhaustive()
    }
}

struct Bucket {
    /// Bytes per second; `None` = unlimited.
    rate: Option<u64>,
    tokens: f64,
    last: Instant,
}

impl Bucket {
    fn capacity(rate: u64) -> f64 {
        (rate / 10).max(1) as f64
    }

    fn refill(&mut self, now: Instant, rate: u64) {
        let dt = now.saturating_duration_since(self.last).as_secs_f64();
        self.last = now;
        self.tokens = (self.tokens + dt * rate as f64).min(Self::capacity(rate));
    }
}

impl RateLimiter {
    pub fn new(clock: Arc<dyn Clock>, limit_kib_per_s: Option<u32>) -> Self {
        let now = clock.now();
        Self {
            clock,
            bucket: Mutex::new(Bucket {
                rate: to_bytes(limit_kib_per_s),
                tokens: 0.0,
                last: now,
            }),
        }
    }

    /// Takes effect at the next grant; a download waiting for tokens notices within a tenth of a second.
    pub fn set_limit(&self, limit_kib_per_s: Option<u32>) {
        let now = self.clock.now();
        let mut bucket = self.lock();
        bucket.rate = to_bytes(limit_kib_per_s);
        bucket.tokens = 0.0;
        bucket.last = now;
    }

    pub fn bytes_per_s(&self) -> Option<u64> {
        self.lock().rate
    }

    /// Blocks until some bytes may be read and returns how many: at least 1, at most `want`, [`CHUNK`] and a tenth of
    /// a second of the limit.
    pub fn acquire(&self, want: usize) -> usize {
        let want = want.clamp(1, CHUNK);
        loop {
            let wait = {
                let mut bucket = self.lock();
                let Some(rate) = bucket.rate else {
                    return want;
                };
                let now = self.clock.now();
                bucket.refill(now, rate);
                let grant = want.min(Bucket::capacity(rate) as usize);
                if bucket.tokens >= grant as f64 {
                    bucket.tokens -= grant as f64;
                    return grant;
                }
                Duration::from_secs_f64((grant as f64 - bucket.tokens) / rate as f64)
            };
            // Short waits so a changed limit is picked up quickly.
            self.clock.sleep(wait.min(Duration::from_millis(100)));
        }
    }

    /// Gives back tokens of a grant that was not used up (the read returned fewer bytes).
    pub fn refund(&self, bytes: usize) {
        let mut bucket = self.lock();
        if let Some(rate) = bucket.rate {
            bucket.tokens = (bucket.tokens + bytes as f64).min(Bucket::capacity(rate));
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Bucket> {
        self.bucket.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

fn to_bytes(limit_kib_per_s: Option<u32>) -> Option<u64> {
    limit_kib_per_s
        .filter(|&kib| kib > 0)
        .map(|kib| u64::from(kib) * 1024)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::FakeClock;

    fn drain(limiter: &RateLimiter, mut bytes: usize) {
        while bytes > 0 {
            bytes -= limiter.acquire(bytes);
        }
    }

    #[test]
    fn unlimited_grants_at_once() {
        let clock = FakeClock::new();
        let limiter = RateLimiter::new(clock.shared(), None);
        assert_eq!(limiter.acquire(10_000), 10_000);
        assert_eq!(limiter.acquire(10 * CHUNK), CHUNK);
        assert_eq!(clock.elapsed(), Duration::ZERO);
    }

    #[test]
    fn a_limit_spreads_the_bytes_over_time() {
        let clock = FakeClock::new();
        let limiter = RateLimiter::new(clock.shared(), Some(100));
        drain(&limiter, 1024 * 1024);
        let secs = clock.elapsed().as_secs_f64();
        // 1 MiB at 100 KiB/s = 10.24 s.
        assert!((secs - 10.24).abs() < 10.24 * 0.05, "{secs}");
    }

    #[test]
    fn the_limit_can_change_while_downloading() {
        let clock = FakeClock::new();
        let limiter = RateLimiter::new(clock.shared(), Some(100));
        drain(&limiter, 512 * 1024);
        let first = clock.elapsed();
        limiter.set_limit(Some(1000));
        drain(&limiter, 1024 * 1024);
        let second = clock.elapsed() - first;
        assert!((first.as_secs_f64() - 5.12).abs() < 0.3, "{first:?}");
        assert!((second.as_secs_f64() - 1.024).abs() < 0.1, "{second:?}");
        limiter.set_limit(None);
        let before = clock.elapsed();
        drain(&limiter, 1024 * 1024);
        assert_eq!(clock.elapsed(), before);
    }

    #[test]
    fn zero_means_no_limit() {
        let clock = FakeClock::new();
        let limiter = RateLimiter::new(clock.shared(), Some(0));
        assert_eq!(limiter.bytes_per_s(), None);
    }

    #[test]
    fn refunds_do_not_exceed_the_bucket() {
        let clock = FakeClock::new();
        let limiter = RateLimiter::new(clock.shared(), Some(64));
        limiter.refund(10 * CHUNK);
        // A tenth of a second of 64 KiB/s.
        assert_eq!(limiter.acquire(CHUNK), 6553);
        assert_eq!(clock.elapsed(), Duration::ZERO);
        limiter.acquire(CHUNK);
        assert!(
            clock.elapsed() > Duration::ZERO,
            "the bucket held a tenth of a second at most"
        );
    }

    #[test]
    fn a_low_limit_still_delivers_bytes_every_tenth_of_a_second() {
        let clock = FakeClock::new();
        let limiter = RateLimiter::new(clock.shared(), Some(1));
        assert_eq!(limiter.acquire(CHUNK), 102);
        assert!(
            clock.elapsed() <= Duration::from_millis(101),
            "{:?}",
            clock.elapsed()
        );
        assert!(
            clock
                .sleeps()
                .iter()
                .all(|d| *d <= Duration::from_millis(100))
        );
    }
}
