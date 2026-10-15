//! What the catalog hands to the UI. Pictures appear only as [`ImageId`]s, never as remote URLs: the webview loads
//! them through the core's image protocol (ADR 0001). Link URLs are text to show in the confirmation dialog (D1);
//! opening them is the core's job.

use crate::id::ImageId;
use deadloader_gamebanana::{
    AlternateSource, Antivirus, Category, CategoryRef, ContentRating, CreditGroup, Section, Sort,
};
use serde::{Deserialize, Serialize};

/// What a listing shows: a section's catalog in some order and category, or a text search. The UI sends it back as
/// it got it (`Event::PageUpdated`) or builds it itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Query {
    pub section: Section,
    pub filter: Filter,
    /// 1-based; 0 counts as 1.
    pub page: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Filter {
    /// `category` is a root category, or a hero under Skins (decision 131).
    Browse { sort: Sort, category: Option<u64> },
    /// Ranked by best match; GameBanana's page size is fixed at 15.
    Search { text: String },
}

impl Query {
    pub fn browse(section: Section, sort: Sort, category: Option<u64>, page: u32) -> Self {
        Self {
            section,
            filter: Filter::Browse { sort, category },
            page: page.max(1),
        }
    }

    pub fn search(section: Section, text: &str, page: u32) -> Self {
        Self {
            section,
            filter: Filter::Search {
                text: text.to_owned(),
            },
            page: page.max(1),
        }
    }

    /// The same listing, one page further.
    pub fn next(&self) -> Self {
        Self {
            page: self.page.saturating_add(1),
            ..self.clone()
        }
    }
}

/// Where an answer came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Origin {
    /// Fetched by this call.
    Network,
    /// Saved earlier. A stale copy is being refreshed in the background (or can't be while offline); its fetch time
    /// is what "showing saved results from 14:20" (N7) shows.
    Cache {
        fetched_at_unix_ms: i64,
        stale: bool,
    },
}

/// One page of cards.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Listing {
    pub items: Vec<Card>,
    /// Cards of this page left out because NSFW is hidden (decision 27).
    pub hidden_nsfw: u32,
    /// Matches across all pages, as GameBanana counts them.
    pub total: u64,
    pub page: u32,
    pub has_more: bool,
    pub origin: Origin,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Card {
    pub id: u64,
    pub section: Section,
    pub name: String,
    pub author: Option<String>,
    pub root_category: Option<CategoryRef>,
    /// For Skins: the hero.
    pub subcategory: Option<CategoryRef>,
    pub likes: u64,
    pub views: u64,
    /// Unix seconds.
    pub date_added: Option<i64>,
    pub date_updated: Option<i64>,
    pub thumbnail: Option<Thumbnail>,
    pub has_audio_preview: bool,
    /// Has content ratings or GameBanana suggests a warning: blur it when NSFW is "blurred".
    pub nsfw: bool,
    pub obsolete: bool,
    pub featured: bool,
    pub has_files: bool,
}

/// A card picture in two sizes (decision 152): `small` (220 px) while the card width times the display scale is at
/// most 220, otherwise `large` (530 px). They are the same id when GameBanana has only one size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Thumbnail {
    pub small: ImageId,
    pub large: ImageId,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Categories {
    pub items: Vec<Category>,
    pub origin: Origin,
}

/// Everything the mod page shows (spec S3).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModPage {
    pub id: u64,
    pub section: Section,
    pub name: String,
    pub version: Option<String>,
    pub author: Option<String>,
    pub credits: Vec<CreditGroup>,
    /// Root first: Skins → Celeste.
    pub category_path: Vec<CategoryRef>,
    /// Empty while NSFW is hidden and the mod is NSFW.
    pub gallery: Vec<GalleryImage>,
    pub has_audio_preview: bool,
    pub short_description: Option<String>,
    pub description: Description,
    pub licence: Option<Description>,
    pub likes: u64,
    pub views: u64,
    pub downloads: u64,
    pub date_added: Option<i64>,
    pub date_modified: Option<i64>,
    pub date_updated: Option<i64>,
    pub nsfw: bool,
    /// NSFW is hidden in the settings and this mod is NSFW: no pictures are offered.
    pub nsfw_hidden: bool,
    pub content_ratings: Vec<ContentRating>,
    pub obsolete: bool,
    /// Video links (YouTube) the author attached; opened through the link dialog (D1).
    pub embedded_media: Vec<String>,
    /// Links elsewhere (a repository, a chat); never downloaded from (decision 33).
    pub alternate_sources: Vec<AlternateSource>,
    pub files: Vec<FileView>,
    pub archived_files: Vec<FileView>,
    pub origin: Origin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GalleryImage {
    /// 530 px.
    pub preview: ImageId,
    /// Full size, on demand.
    pub original: ImageId,
    pub caption: Option<String>,
}

/// A downloadable file. Downloads ask the core by mod and file id; the URL stays in the core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FileView {
    pub id: u64,
    pub file_name: String,
    pub size_bytes: u64,
    pub date_added: Option<i64>,
    pub download_count: u64,
    pub description: Option<String>,
    pub version: Option<String>,
    pub md5: Option<String>,
    pub antivirus: Antivirus,
}

/// Sanitised HTML (crate `deadloader-html`) and what its placeholders point at.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Description {
    pub html: String,
    /// Targets of `data-dl-link="N"`.
    pub links: Vec<LinkView>,
    /// Targets of `data-dl-image="N"`.
    pub images: Vec<DescriptionImage>,
    /// Hosts waiting for consent (dialog D2), each with its images here, sorted by host.
    pub consent: Vec<PendingHost>,
    pub truncated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LinkView {
    pub url: String,
    pub host: String,
    /// A GameBanana mod link the UI can open inside deadloader.
    pub internal_mod_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DescriptionImage {
    pub id: ImageId,
    pub host: String,
    pub alt: Option<String>,
    pub access: Access,
}

/// What showing a description image takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Access {
    /// Show it now ("always load" for an allowed host, or loaded before).
    Load,
    /// Placeholder; a click loads it (`activate`).
    Click,
    /// Placeholder; a click opens the consent dialog first (`consent_prompt`, then `grant`).
    Consent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PendingHost {
    pub host: String,
    pub images: Vec<ImageId>,
}

/// Dialog D2: the host and whether "always allow images from this host" starts checked (a setting, 122).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConsentPrompt {
    pub host: String,
    pub always_allow_prechecked: bool,
}

/// The answer of dialog D2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// This image only; the host is not remembered.
    Once,
    /// This host from now on, for description images only; saved in the settings.
    Always,
}

/// Images that may be loaded now because of a consent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Granted {
    pub activated: Vec<ImageId>,
}
