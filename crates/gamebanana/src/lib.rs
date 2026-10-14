//! The only code that knows GameBanana's URLs, query parameters and JSON fields. It maps API responses into
//! deadloader's own types and reports an incompatible API as a distinct error. It talks to the network through the
//! [`Fetch`] trait, never directly: the app plugs in the `net` crate (allowlist, logging, rate limits), and tests plug
//! in recorded fixtures.

pub mod error;
pub mod parse;
pub mod request;
pub mod types;

pub use error::{ApiChange, ApiChangeReason, Error, FetchError};
pub use request::{ListQuery, SearchQuery, SearchTextError, Sort};
pub use types::*;

/// A plain HTTP GET. Success means a 2xx response and its body; anything else is a [`FetchError`].
pub trait Fetch {
    fn get(&self, url: &str) -> Result<Vec<u8>, FetchError>;
}

/// The catalog's view of GameBanana: builds the request, fetches it, maps the answer.
#[derive(Debug)]
pub struct Client<F> {
    fetch: F,
}

impl<F: Fetch> Client<F> {
    pub fn new(fetch: F) -> Self {
        Self { fetch }
    }

    pub fn list(&self, query: &ListQuery) -> Result<CatalogPage, Error> {
        let body = self.get(
            parse::list_endpoint(query.section),
            &request::list_url(query),
        )?;
        parse::parse_list(query.section, &body)
    }

    pub fn search(&self, query: &SearchQuery) -> Result<CatalogPage, Error> {
        let body = self.get(parse::SEARCH_ENDPOINT, &request::search_url(query))?;
        parse::parse_search(&body)
    }

    /// Root categories of a section, or the children of `parent` (the heroes under Skins).
    pub fn categories(
        &self,
        section: Section,
        parent: Option<u64>,
    ) -> Result<Vec<Category>, Error> {
        let body = self.get(
            parse::categories_endpoint(section),
            &request::categories_url(section, parent),
        )?;
        parse::parse_categories(section, &body)
    }

    pub fn details(&self, section: Section, id: u64) -> Result<ModDetails, Error> {
        let body = self.get(
            parse::details_endpoint(section),
            &request::details_url(section, id),
        )?;
        parse::parse_details(section, &body)
    }

    fn get(&self, endpoint: &'static str, url: &str) -> Result<Vec<u8>, Error> {
        self.fetch
            .get(url)
            .map_err(|e| parse::fetch_error(endpoint, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// Answers every GET with one canned result and records the URLs asked for.
    struct Canned {
        answer: Result<Vec<u8>, FetchError>,
        urls: RefCell<Vec<String>>,
    }

    impl Canned {
        fn new(answer: Result<&[u8], FetchError>) -> Client<Self> {
            Client::new(Self {
                answer: answer.map(<[u8]>::to_vec),
                urls: RefCell::new(Vec::new()),
            })
        }
    }

    impl Fetch for Canned {
        fn get(&self, url: &str) -> Result<Vec<u8>, FetchError> {
            self.urls.borrow_mut().push(url.to_owned());
            self.answer.clone()
        }
    }

    fn urls(client: &Client<Canned>) -> Vec<String> {
        client.fetch.urls.borrow().clone()
    }

    const PAGE: &[u8] = br#"{"_aMetadata": {"_nRecordCount": 1, "_bIsComplete": true},
        "_aRecords": [{"_idRow": 1, "_sModelName": "Mod", "_sName": "One"}]}"#;

    #[test]
    fn list_requests_the_index_and_maps_the_page() {
        let client = Canned::new(Ok(PAGE));
        let query = ListQuery {
            section: Section::Mod,
            sort: Sort::MostLiked,
            page: 2,
            category: None,
        };
        let page = client.list(&query).expect("page");
        assert_eq!(page.items[0].name, "One");
        assert_eq!(urls(&client), vec![request::list_url(&query)]);
    }

    #[test]
    fn search_requests_the_search_endpoint() {
        let client = Canned::new(Ok(PAGE));
        let query = SearchQuery::new(Section::Mod, "one", 1).expect("valid");
        assert_eq!(client.search(&query).expect("page").total, 1);
        assert_eq!(urls(&client), vec![request::search_url(&query)]);
    }

    #[test]
    fn categories_and_details_request_their_endpoints() {
        let client = Canned::new(Ok(b"[]"));
        assert!(
            client
                .categories(Section::Mod, Some(33_295))
                .expect("list")
                .is_empty()
        );
        assert_eq!(
            urls(&client),
            vec![request::categories_url(Section::Mod, Some(33_295))]
        );

        let client = Canned::new(Ok(br#"{"_idRow": 7, "_sName": "Seven"}"#));
        assert_eq!(
            client.details(Section::Sound, 7).expect("details").name,
            "Seven"
        );
        assert_eq!(urls(&client), vec![request::details_url(Section::Sound, 7)]);
    }

    #[test]
    fn transport_failure_is_a_fetch_error() {
        let failure = FetchError::Transport("connection refused".into());
        let client = Canned::new(Err(failure.clone()));
        assert_eq!(client.details(Section::Mod, 1), Err(Error::Fetch(failure)));
    }

    #[test]
    fn not_found_status_with_error_document() {
        let client = Canned::new(Err(FetchError::Status {
            status: 404,
            body: br#"{"_sErrorCode": "NO_SUCH_RECORD", "_sErrorMessage": "gone"}"#.to_vec(),
        }));
        assert_eq!(client.details(Section::Mod, 1), Err(Error::NotFound));
    }

    #[test]
    fn rejected_request_names_the_endpoint() {
        let client = Canned::new(Err(FetchError::Status {
            status: 400,
            body: br#"{"_sErrorCode": "INPUT_ERRORS", "_aErrorData": {"_sSort": {"_sErrorCode": "UNKNOWN_SORT"}}}"#
                .to_vec(),
        }));
        let query = ListQuery {
            section: Section::Sound,
            sort: Sort::Newest,
            page: 1,
            category: None,
        };
        match client.list(&query) {
            Err(Error::ApiChanged(change)) => {
                assert_eq!(change.endpoint, "Sound/Index");
                assert_eq!(
                    change.reason.to_string(),
                    "error_code:INPUT_ERRORS[_sSort=UNKNOWN_SORT]"
                );
            }
            other => panic!("expected ApiChanged, got {other:?}"),
        }
    }
}
