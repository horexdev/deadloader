//! Progress reporting: a smoothed speed and a throttle so the UI gets about ten updates a second per task.

use std::time::{Duration, Instant};

/// Bytes per second, smoothed exponentially with a time constant of a couple of seconds, so the shown speed and the
/// time left don't jump with every network hiccup.
#[derive(Debug, Clone)]
pub struct SpeedMeter {
    tau: Duration,
    last: Option<(Instant, u64)>,
    speed: Option<f64>,
}

impl SpeedMeter {
    pub fn new(tau: Duration) -> Self {
        Self {
            tau,
            last: None,
            speed: None,
        }
    }

    /// Records that `bytes` have been received in total by `now`; returns the smoothed speed.
    pub fn sample(&mut self, now: Instant, bytes: u64) -> u64 {
        if let Some((then, before)) = self.last {
            let dt = now.saturating_duration_since(then).as_secs_f64();
            if dt > 0.0 {
                let instant = bytes.saturating_sub(before) as f64 / dt;
                let alpha = 1.0 - (-dt / self.tau.as_secs_f64()).exp();
                self.speed = Some(match self.speed {
                    Some(speed) => speed + alpha * (instant - speed),
                    None => instant,
                });
                self.last = Some((now, bytes));
            }
        } else {
            self.last = Some((now, bytes));
        }
        self.current()
    }

    pub fn current(&self) -> u64 {
        self.speed.map_or(0, |s| s.round() as u64)
    }

    /// After a pause or a restart: the next sample starts from scratch.
    pub fn reset(&mut self) {
        self.last = None;
        self.speed = None;
    }
}

/// Lets an event through at most once per interval.
#[derive(Debug, Clone)]
pub struct Throttle {
    every: Duration,
    last: Option<Instant>,
}

impl Throttle {
    pub fn new(every: Duration) -> Self {
        Self { every, last: None }
    }

    pub fn ready(&mut self, now: Instant) -> bool {
        match self.last {
            Some(last) if now.saturating_duration_since(last) < self.every => false,
            _ => {
                self.last = Some(now);
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MS: Duration = Duration::from_millis(1);

    #[test]
    fn a_steady_rate_is_reported_as_is() {
        let t0 = Instant::now();
        let mut meter = SpeedMeter::new(Duration::from_secs(2));
        assert_eq!(meter.sample(t0, 0), 0);
        for i in 1..=50u32 {
            meter.sample(t0 + 100 * MS * i, u64::from(i) * 100_000);
        }
        assert_eq!(meter.current(), 1_000_000);
    }

    #[test]
    fn a_short_stall_does_not_drop_the_speed_to_zero() {
        let t0 = Instant::now();
        let mut meter = SpeedMeter::new(Duration::from_secs(2));
        for i in 0..=20u32 {
            meter.sample(t0 + 100 * MS * i, u64::from(i) * 100_000);
        }
        // Nothing for 300 ms.
        let speed = meter.sample(t0 + 2300 * MS, 2_000_000);
        assert!(speed > 800_000, "{speed}");
        assert!(speed < 1_000_000, "{speed}");
    }

    #[test]
    fn reset_starts_over() {
        let t0 = Instant::now();
        let mut meter = SpeedMeter::new(Duration::from_secs(2));
        meter.sample(t0, 0);
        meter.sample(t0 + 1000 * MS, 1000);
        meter.reset();
        assert_eq!(meter.current(), 0);
        assert_eq!(meter.sample(t0 + 2000 * MS, 5000), 0);
        assert_eq!(meter.sample(t0 + 3000 * MS, 7000), 2000);
    }

    #[test]
    fn the_throttle_lets_ten_a_second_through() {
        let t0 = Instant::now();
        let mut throttle = Throttle::new(100 * MS);
        let passed = (0..1000u32)
            .filter(|i| throttle.ready(t0 + MS * *i))
            .count();
        assert_eq!(passed, 10);
    }
}
