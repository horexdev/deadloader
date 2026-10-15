//! The catalog: GameBanana's listings and mod pages through the cache, and the pictures the webview shows.
//!
//! It connects the GameBanana module (URLs and JSON), the `net` client (allowlist, retries, the network log) and the
//! core cache, and decides what each call may cost:
//! - **pages** (listings, searches, categories): a fresh saved copy is served without a request; a stale one is
//!   served at once and refreshed in the background (stale-while-revalidate, decision 58); a missing one is fetched.
//!   The next page can be fetched ahead, per the setting (28).
//! - **mod pages** are kept as cards by update date: current while the listing shows no newer update.
//! - **local mode** (108): zero requests. Saved mod pages may still be shown; listings are not.
//! - **offline**: saved copies are served and the status says since when (N7); **rate limited** (429/5xx after the
//!   client's retries): nothing is asked until the wait is over (E5); an **API change** is its own status (E4).
//! - **NSFW** (27/78/109): hidden mods are left out of listings and their pictures never get an id; otherwise cards
//!   are flagged for the UI to blur.
//! - **pictures** get opaque [`ImageId`]s instead of URLs. [`Catalog::image`] serves them from the cache, fetching a
//!   missing one with the right [`deadloader_net::Purpose`]. Description images also need a click, and images from a
//!   host not allowed yet need the user's consent (29, 122): once, or always (saved to the settings).
//!
//! Everything is blocking: call it from worker threads, never from the UI thread. Background refreshes go through
//! [`Spawn`], changes the UI should hear about through [`Events`]. Requests are logged by `net`, cache files by the
//! core's files log, and the catalog's own decisions (status changes, consents, errors) by [`AppLog`].

mod error;
mod fetch;
mod gate;
mod id;
mod registry;
mod service;
mod view;

pub use deadloader_core::cache::{Blob, ImageType};
pub use deadloader_gamebanana::{Section, Sort};
pub use deadloader_net::Initiator;
pub use error::{CatalogError, ConsentError, ImageError, SearchProblem, Status};
pub use id::ImageId;
pub use service::{AppLog, Catalog, Event, Events, Parts, SettingsSink, Spawn, Threads};
pub use view::{
    Access, Card, Categories, ConsentPrompt, Description, DescriptionImage, FileView, Filter,
    GalleryImage, Granted, LinkView, Listing, ModPage, Origin, PendingHost, Query, Scope,
    Thumbnail,
};
