//! The GameBanana module's [`Fetch`] over the `net` client, and what a failed request means for the catalog.

use deadloader_gamebanana::{Fetch, FetchError};
use deadloader_net::{Client, Initiator, NetError, Purpose, Request};
use std::sync::{Mutex, PoisonError};

/// API answers larger than this are refused; a mod page is ~25 KB, a page of 50 cards ~100 KB.
pub(crate) const MAX_API_BODY: u64 = 4 * 1024 * 1024;
/// The part of an error answer kept for GameBanana's error document.
const MAX_ERROR_BODY: u64 = 64 * 1024;

/// One catalog request's way to the network. `gamebanana` only sees "a transport failed"; the `net` error behind it
/// is kept so the catalog can tell offline from blocked from rate limited.
pub(crate) struct NetFetch<'a> {
    pub net: &'a Client,
    pub initiator: Initiator,
    pub failure: Mutex<Option<Failure>>,
}

/// What went wrong below the GameBanana module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Failure {
    Net(NetError),
    /// GameBanana answered with more than [`MAX_API_BODY`].
    TooLarge,
    /// The connection broke while reading the answer.
    Read(String),
}

impl<'a> NetFetch<'a> {
    pub(crate) fn new(net: &'a Client, initiator: Initiator) -> Self {
        Self {
            net,
            initiator,
            failure: Mutex::new(None),
        }
    }

    pub(crate) fn take_failure(&self) -> Option<Failure> {
        self.failure
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
    }

    fn fail(&self, failure: Failure, text: String) -> FetchError {
        *self.failure.lock().unwrap_or_else(PoisonError::into_inner) = Some(failure);
        FetchError::Transport(text)
    }
}

impl Fetch for &NetFetch<'_> {
    fn get(&self, url: &str) -> Result<Vec<u8>, FetchError> {
        let request = Request::new(Purpose::Catalog, self.initiator, url)
            .header("Accept", "application/json");
        let response = match self.net.get(&request) {
            Ok(response) => response,
            Err(e) => {
                let text = e.to_string();
                return Err(self.fail(Failure::Net(e), text));
            }
        };
        let status = response.status;
        let success = (200..300).contains(&status);
        let limit = if success {
            MAX_API_BODY
        } else {
            MAX_ERROR_BODY
        };
        match response.read_body(limit) {
            Ok(body) if success => Ok(body),
            Ok(body) => Err(FetchError::Status { status, body }),
            Err(deadloader_net::client::BodyError::TooLarge { .. }) if success => Err(self.fail(
                Failure::TooLarge,
                format!("the answer is over {limit} bytes"),
            )),
            // An error answer too large to hold GameBanana's error document: the status alone tells.
            Err(deadloader_net::client::BodyError::TooLarge { .. }) => Err(FetchError::Status {
                status,
                body: Vec::new(),
            }),
            Err(e) => {
                let text = e.to_string();
                Err(self.fail(Failure::Read(text.clone()), text))
            }
        }
    }
}
