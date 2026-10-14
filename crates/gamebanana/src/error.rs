//! Errors of the GameBanana module. "GameBanana could not be reached" and "GameBanana answered in a way we do not
//! understand" are kept apart: the first is usually temporary, the second means this version of deadloader needs an
//! update (decision 98), and the UI says so differently.

use std::fmt;

/// What a [`crate::Fetch`] implementation reports when a GET does not produce a successful response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FetchError {
    /// The server answered with a non-success status. The body is kept because GameBanana explains its errors in
    /// JSON, which tells a missing record apart from a changed API.
    Status { status: u16, body: Vec<u8> },
    /// No usable answer: connection, TLS, timeout, a blocked host, … described by the transport.
    Transport(String),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status { status, .. } => {
                write!(f, "the server answered with HTTP status {status}")
            }
            Self::Transport(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for FetchError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// GameBanana could not be reached or failed without explaining why.
    Fetch(FetchError),
    /// GameBanana says the submission does not exist (deleted, or a wrong id).
    NotFound,
    /// The submission belongs to another game on GameBanana.
    WrongGame { game_id: u64 },
    /// The response does not match what this module understands: the unofficial API has changed.
    ApiChanged(ApiChange),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fetch(e) => write!(f, "could not get an answer from GameBanana: {e}"),
            Self::NotFound => write!(
                f,
                "GameBanana has no such submission; it may have been deleted"
            ),
            Self::WrongGame { game_id } => write!(
                f,
                "the submission belongs to another game (GameBanana game {game_id}), not Deadlock"
            ),
            Self::ApiChanged(change) => write!(f, "{change}"),
        }
    }
}

impl std::error::Error for Error {}

/// Where and how a response stopped matching the API this module was written against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiChange {
    /// The endpoint, e.g. `Mod/Index`.
    pub endpoint: &'static str,
    pub reason: ApiChangeReason,
}

impl fmt::Display for ApiChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GameBanana's API answered {} in a way this version of deadloader does not understand ({})",
            self.endpoint, self.reason
        )
    }
}

/// A short, machine-readable reason; its `Display` form is stable enough to log and to group reports by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiChangeReason {
    /// GameBanana rejected the request with an error code, e.g. `INPUT_ERRORS` for a renamed parameter or sort.
    /// `details` lists `parameter=CODE` pairs when GameBanana names them.
    ErrorCode { code: String, details: Vec<String> },
    /// The body is not JSON at all.
    NotJson,
    /// A field this module cannot do without is absent.
    MissingField(String),
    /// A field has another type or the document another structure; the parser's description is kept.
    UnexpectedShape(String),
}

impl fmt::Display for ApiChangeReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorCode { code, details } if details.is_empty() => {
                write!(f, "error_code:{code}")
            }
            Self::ErrorCode { code, details } => {
                write!(f, "error_code:{code}[{}]", details.join(","))
            }
            Self::NotJson => write!(f, "not_json"),
            Self::MissingField(field) => write!(f, "missing_field:{field}"),
            Self::UnexpectedShape(detail) => write!(f, "unexpected_shape:{detail}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reasons_have_short_machine_readable_forms() {
        let cases = [
            (
                ApiChangeReason::ErrorCode {
                    code: "INPUT_ERRORS".into(),
                    details: vec!["_sSort=UNKNOWN_SORT".into()],
                },
                "error_code:INPUT_ERRORS[_sSort=UNKNOWN_SORT]",
            ),
            (
                ApiChangeReason::ErrorCode {
                    code: "SOMETHING".into(),
                    details: vec![],
                },
                "error_code:SOMETHING",
            ),
            (ApiChangeReason::NotJson, "not_json"),
            (
                ApiChangeReason::MissingField("_idRow".into()),
                "missing_field:_idRow",
            ),
        ];
        for (reason, expected) in cases {
            assert_eq!(reason.to_string(), expected);
        }
    }

    #[test]
    fn api_change_names_the_endpoint() {
        let error = Error::ApiChanged(ApiChange {
            endpoint: "Mod/Index",
            reason: ApiChangeReason::NotJson,
        });
        let text = error.to_string();
        assert!(text.contains("Mod/Index"), "{text}");
        assert!(text.contains("not_json"), "{text}");
    }
}
