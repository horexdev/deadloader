//! A per-read idle timeout for ureq connections.
//!
//! ureq 3 only knows deadlines for whole phases (connect, headers, body). A download has no body deadline, because it
//! may rightly take an hour, so a connection that goes silent without closing would block its reading thread until
//! Windows gives up on the socket, which can take hours. This connector wraps every connection so that no single wait
//! for data lasts longer than the idle limit; a phase deadline that comes sooner still wins.
//!
//! It uses ureq's `unversioned` transport API, which may change in any ureq release: a ureq update that breaks the
//! build here needs this file adapted, not the feature dropped.

use std::time::Duration;
use ureq::Error;
use ureq::unversioned::transport::time::Duration as UreqDuration;
use ureq::unversioned::transport::{Buffers, ConnectionDetails, Connector, NextTimeout, Transport};

/// The last connector of the chain: wraps the finished connection (TCP, proxy, TLS) in [`Idle`].
#[derive(Debug, Clone, Copy)]
pub(crate) struct IdleTimeout(pub Duration);

impl Connector<Box<dyn Transport>> for IdleTimeout {
    type Out = Idle<Box<dyn Transport>>;

    fn connect(
        &self,
        _details: &ConnectionDetails,
        chained: Option<Box<dyn Transport>>,
    ) -> Result<Option<Self::Out>, Error> {
        Ok(chained.map(|inner| Idle {
            inner,
            limit: self.0,
        }))
    }
}

/// A connection whose reads and writes never wait longer than `limit` at a time.
#[derive(Debug)]
pub(crate) struct Idle<T> {
    inner: T,
    limit: Duration,
}

impl<T> Idle<T> {
    /// The sooner of the phase deadline and the idle limit. Reported as the phase's own timeout, so a stalled body
    /// reads as a body timeout.
    fn clamp(&self, timeout: NextTimeout) -> NextTimeout {
        let after = match timeout.after {
            UreqDuration::Exact(after) if after <= self.limit => timeout.after,
            _ => UreqDuration::Exact(self.limit),
        };
        NextTimeout {
            after,
            reason: timeout.reason,
        }
    }
}

impl<T: Transport> Transport for Idle<T> {
    fn buffers(&mut self) -> &mut dyn Buffers {
        self.inner.buffers()
    }

    fn transmit_output(&mut self, amount: usize, timeout: NextTimeout) -> Result<(), Error> {
        let timeout = self.clamp(timeout);
        self.inner.transmit_output(amount, timeout)
    }

    fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, Error> {
        let timeout = self.clamp(timeout);
        self.inner.await_input(timeout)
    }

    fn is_open(&mut self) -> bool {
        self.inner.is_open()
    }

    fn is_tls(&self) -> bool {
        self.inner.is_tls()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use ureq::Timeout;
    use ureq::unversioned::transport::LazyBuffers;

    /// Records the timeouts it is given.
    #[derive(Debug)]
    struct Recorder {
        buffers: LazyBuffers,
        seen: Arc<Mutex<Vec<NextTimeout>>>,
    }

    impl Transport for Recorder {
        fn buffers(&mut self) -> &mut dyn Buffers {
            &mut self.buffers
        }

        fn transmit_output(&mut self, _amount: usize, timeout: NextTimeout) -> Result<(), Error> {
            self.seen.lock().expect("lock").push(timeout);
            Ok(())
        }

        fn await_input(&mut self, timeout: NextTimeout) -> Result<bool, Error> {
            self.seen.lock().expect("lock").push(timeout);
            Ok(false)
        }

        fn is_open(&mut self) -> bool {
            true
        }
    }

    fn idle(limit: Duration) -> (Idle<Recorder>, Arc<Mutex<Vec<NextTimeout>>>) {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let inner = Recorder {
            buffers: LazyBuffers::new(1024, 1024),
            seen: Arc::clone(&seen),
        };
        (Idle { inner, limit }, seen)
    }

    fn next(after: UreqDuration, reason: Timeout) -> NextTimeout {
        NextTimeout { after, reason }
    }

    #[test]
    fn a_wait_without_a_deadline_ends_after_the_idle_limit() {
        let (mut conn, seen) = idle(Duration::from_secs(60));
        conn.await_input(next(UreqDuration::NotHappening, Timeout::RecvBody))
            .expect("read");
        conn.transmit_output(0, next(UreqDuration::NotHappening, Timeout::SendRequest))
            .expect("write");
        let seen = seen.lock().expect("lock");
        assert_eq!(seen[0].after, UreqDuration::from_secs(60));
        assert_eq!(seen[0].reason, Timeout::RecvBody);
        assert_eq!(seen[1].after, UreqDuration::from_secs(60));
        assert_eq!(seen[1].reason, Timeout::SendRequest);
    }

    #[test]
    fn a_sooner_deadline_wins() {
        let (mut conn, seen) = idle(Duration::from_secs(60));
        conn.await_input(next(UreqDuration::from_secs(30), Timeout::RecvResponse))
            .expect("read");
        conn.await_input(next(UreqDuration::from_secs(90), Timeout::RecvBody))
            .expect("read");
        let seen = seen.lock().expect("lock");
        assert_eq!(seen[0].after, UreqDuration::from_secs(30));
        assert_eq!(seen[0].reason, Timeout::RecvResponse);
        assert_eq!(seen[1].after, UreqDuration::from_secs(60));
    }
}
