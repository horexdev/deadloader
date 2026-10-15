//! The image protocol: catalog thumbnails, gallery pictures and description images, served by the core (ADR 0001).
//!
//! The webview never fetches a picture from the internet. It asks for `https://dl.localhost/img/<id>`, where `<id>`
//! is an opaque id the catalog issued; WebView2 has no custom schemes, so Tauri intercepts that origin in-process and
//! hands the request to [`respond`], which never touches the network stack of the webview. The catalog serves the
//! picture from its cache or fetches it itself, through the allowlist and the network log. There is no form of the
//! URL that names a remote address, so the protocol can't be used as a proxy.
//!
//! The CSP allows this origin for `img-src` only (see `policy.rs`).

use deadloader_catalog::{Blob, Catalog, ImageError, ImageId};
use deadloader_core::cache::{CacheFs, FileLog};
use deadloader_core::log::Clock;
use std::sync::Arc;
use tauri::http::{Method, Request, Response, StatusCode, header};
use tauri::{Builder, Manager, Runtime};

/// The scheme registered with Tauri.
pub const SCHEME: &str = "dl";
/// What the scheme becomes in WebView2 with `use_https_scheme` (`https://<scheme>.localhost`); the CSP's `img-src`
/// names it, and the policy tests check that the two agree.
#[cfg(test)]
pub const ORIGIN: &str = "https://dl.localhost";
/// The only path: `/img/<id>`.
const IMAGE_PATH: &str = "/img/";

/// Where the protocol gets pictures from: the catalog, or a fake in tests.
pub trait ImageSource: Send + Sync {
    fn image(&self, id: &ImageId) -> Result<Blob, ImageError>;
}

impl<F, C, L> ImageSource for Catalog<F, C, L>
where
    F: CacheFs + Send + Sync + 'static,
    C: Clock + Send + Sync + 'static,
    L: FileLog + Send + Sync + 'static,
{
    fn image(&self, id: &ImageId) -> Result<Blob, ImageError> {
        Catalog::image(self, id)
    }
}

/// Managed state: the catalog the protocol serves from. Until the app manages one, pictures answer 503.
pub struct Images(pub Arc<dyn ImageSource>);

/// Registers the protocol. Requests are answered on Tauri's blocking pool, never on the UI thread: a cache miss
/// waits for the network.
pub fn register<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.register_asynchronous_uri_scheme_protocol(SCHEME, |ctx, request, responder| {
        let app = ctx.app_handle().clone();
        tauri::async_runtime::spawn_blocking(move || {
            let images = app.try_state::<Images>();
            let source = images.as_ref().map(|images| images.0.as_ref());
            responder.respond(respond(source, &request));
        });
    })
}

/// One request → one response. Only `GET`/`HEAD /img/<id>` on our own origin is served.
pub fn respond(source: Option<&dyn ImageSource>, request: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    let head = request.method() == Method::HEAD;
    if !head && request.method() != Method::GET {
        return status(StatusCode::METHOD_NOT_ALLOWED);
    }
    // Tauri hands the request over as `dl://localhost/…`; the WebView2 form is `https://dl.localhost/…`.
    if !matches!(request.uri().host(), Some("localhost" | "dl.localhost")) {
        return status(StatusCode::NOT_FOUND);
    }
    let Some(id) = request
        .uri()
        .path()
        .strip_prefix(IMAGE_PATH)
        .and_then(ImageId::parse)
    else {
        return status(StatusCode::NOT_FOUND);
    };
    let Some(source) = source else {
        return status(StatusCode::SERVICE_UNAVAILABLE);
    };
    match source.image(&id) {
        Ok(blob) => {
            let length = blob.bytes.len();
            let body = if head { Vec::new() } else { blob.bytes };
            secured(Response::builder().status(StatusCode::OK))
                .header(header::CONTENT_TYPE, blob.content_type.mime())
                .header(header::CONTENT_LENGTH, length)
                .body(body)
                .unwrap_or_else(|_| status(StatusCode::INTERNAL_SERVER_ERROR))
        }
        Err(error) => status(error_status(&error)),
    }
}

/// What each failure looks like to the webview; the UI shows E19 and keeps the placeholder.
fn error_status(error: &ImageError) -> StatusCode {
    match error {
        ImageError::UnknownId => StatusCode::NOT_FOUND,
        // Not clicked, not consented to, or refused by the allowlist.
        ImageError::NotActivated | ImageError::Blocked => StatusCode::FORBIDDEN,
        // Temporary: local mode, offline, GameBanana asked to wait.
        ImageError::LocalMode | ImageError::Offline | ImageError::RateLimited => {
            StatusCode::SERVICE_UNAVAILABLE
        }
        ImageError::Unreachable => StatusCode::GATEWAY_TIMEOUT,
        ImageError::Upstream { .. } | ImageError::NotAnImage | ImageError::TooLarge => {
            StatusCode::BAD_GATEWAY
        }
        ImageError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// Headers on every answer: the bytes are what the cache checked them to be, and the webview keeps no second copy
/// (our cache is the one the user sees and clears, decision 58).
fn secured(builder: tauri::http::response::Builder) -> tauri::http::response::Builder {
    builder
        .header(header::CACHE_CONTROL, "no-store")
        .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
}

fn status(code: StatusCode) -> Response<Vec<u8>> {
    let mut response = Response::new(Vec::new());
    *response.status_mut() = code;
    let headers = response.headers_mut();
    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store"),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        header::HeaderValue::from_static("nosniff"),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use deadloader_catalog::ImageType;
    use std::sync::Mutex;

    const ID: &str = "0123456789abcdef0123456789abcdef";
    const PNG: &[u8] = b"\x89PNG\r\n\x1a\nimage";

    /// Answers every id with one result and records the ids asked for.
    struct Fake {
        answer: Result<Blob, ImageError>,
        asked: Mutex<Vec<ImageId>>,
    }

    impl Fake {
        fn new(answer: Result<Blob, ImageError>) -> Self {
            Self {
                answer,
                asked: Mutex::new(Vec::new()),
            }
        }

        fn png() -> Self {
            Self::new(Ok(Blob {
                bytes: PNG.to_vec(),
                content_type: ImageType::Png,
            }))
        }

        fn asked(&self) -> Vec<ImageId> {
            self.asked.lock().expect("lock").clone()
        }
    }

    impl ImageSource for Fake {
        fn image(&self, id: &ImageId) -> Result<Blob, ImageError> {
            self.asked.lock().expect("lock").push(*id);
            self.answer.clone()
        }
    }

    fn get(uri: &str) -> Request<Vec<u8>> {
        request("GET", uri)
    }

    fn request(method: &str, uri: &str) -> Request<Vec<u8>> {
        Request::builder()
            .method(method)
            .uri(uri)
            .body(Vec::new())
            .expect("request")
    }

    fn header(response: &Response<Vec<u8>>, name: header::HeaderName) -> Option<&str> {
        response.headers().get(name).and_then(|v| v.to_str().ok())
    }

    #[test]
    fn the_origin_is_the_scheme_under_https_localhost() {
        assert_eq!(ORIGIN, format!("https://{SCHEME}.localhost"));
    }

    #[test]
    fn serves_an_issued_image_with_its_type_and_no_caching() {
        let fake = Fake::png();
        for uri in [
            format!("dl://localhost/img/{ID}"),
            format!("https://dl.localhost/img/{ID}"),
        ] {
            let response = respond(Some(&fake), &get(&uri));
            assert_eq!(response.status(), StatusCode::OK, "{uri}");
            assert_eq!(response.body(), PNG);
            assert_eq!(header(&response, header::CONTENT_TYPE), Some("image/png"));
            assert_eq!(header(&response, header::CONTENT_LENGTH), Some("13"));
            assert_eq!(header(&response, header::CACHE_CONTROL), Some("no-store"));
            assert_eq!(
                header(&response, header::X_CONTENT_TYPE_OPTIONS),
                Some("nosniff")
            );
        }
        assert_eq!(fake.asked(), [ImageId::parse(ID).expect("id"); 2]);
    }

    #[test]
    fn head_answers_without_a_body() {
        let response = respond(
            Some(&Fake::png()),
            &request("HEAD", &format!("dl://localhost/img/{ID}")),
        );
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.body().is_empty());
        assert_eq!(header(&response, header::CONTENT_LENGTH), Some("13"));
    }

    #[test]
    fn only_reads_are_allowed() {
        let fake = Fake::png();
        for method in ["POST", "PUT", "DELETE", "OPTIONS", "PATCH"] {
            let response = respond(
                Some(&fake),
                &request(method, &format!("dl://localhost/img/{ID}")),
            );
            assert_eq!(
                response.status(),
                StatusCode::METHOD_NOT_ALLOWED,
                "{method}"
            );
        }
        assert!(fake.asked().is_empty());
    }

    #[test]
    fn never_proxies_an_address_and_rejects_anything_but_an_id() {
        let fake = Fake::png();
        let encoded = "https%3A%2F%2Fevil.example%2Fa.png";
        for uri in [
            "dl://localhost/".to_owned(),
            "dl://localhost/img/".to_owned(),
            format!("dl://localhost/img/{}", ID.to_uppercase()),
            format!("dl://localhost/img/{ID}/x"),
            format!("dl://localhost/img/{ID}0"),
            format!("dl://localhost/thumb/{ID}"),
            format!("dl://localhost/img/{encoded}"),
            "dl://localhost/img/https://evil.example/a.png".to_owned(),
            "dl://localhost/img/..%2F..%2Fsettings.json".to_owned(),
            format!("dl://localhost/IMG/{ID}"),
            // Another host under the intercepted prefix (`https://dl.*`).
            format!("https://dl.evil.example/img/{ID}"),
            format!("dl://evil.example/img/{ID}"),
        ] {
            let response = respond(Some(&fake), &get(&uri));
            assert_eq!(response.status(), StatusCode::NOT_FOUND, "{uri}");
            assert!(response.body().is_empty(), "{uri}");
        }
        assert!(fake.asked().is_empty());
    }

    #[test]
    fn a_query_string_does_not_change_the_image() {
        let fake = Fake::png();
        let response = respond(
            Some(&fake),
            &get(&format!("dl://localhost/img/{ID}?url=https://evil.example")),
        );
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(fake.asked(), [ImageId::parse(ID).expect("id")]);
    }

    #[test]
    fn without_a_catalog_pictures_are_unavailable() {
        let response = respond(None, &get(&format!("dl://localhost/img/{ID}")));
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[test]
    fn failures_have_their_own_status_codes() {
        let cases = [
            (ImageError::UnknownId, 404),
            (ImageError::NotActivated, 403),
            (ImageError::Blocked, 403),
            (ImageError::LocalMode, 503),
            (ImageError::Offline, 503),
            (ImageError::RateLimited, 503),
            (ImageError::Unreachable, 504),
            (ImageError::Upstream { status: 404 }, 502),
            (ImageError::NotAnImage, 502),
            (ImageError::TooLarge, 502),
            (ImageError::Internal, 500),
        ];
        for (error, code) in cases {
            let response = respond(
                Some(&Fake::new(Err(error.clone()))),
                &get(&format!("dl://localhost/img/{ID}")),
            );
            assert_eq!(response.status().as_u16(), code, "{error:?}");
            assert!(response.body().is_empty(), "{error:?}");
            assert_eq!(header(&response, header::CONTENT_TYPE), None, "{error:?}");
            assert_eq!(header(&response, header::CACHE_CONTROL), Some("no-store"));
        }
    }
}
