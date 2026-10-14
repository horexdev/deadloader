//! URLs of the GameBanana API v11 endpoints the catalog uses.
//!
//! Only this module knows endpoint paths and query parameter names. Every query value goes through
//! [`encode_component`], so user input (the search text) can never add parameters or change the path. Parameter order
//! is fixed, which keeps URLs stable in the network log and lets tests compare them as plain strings.

use crate::types::Section;
use std::fmt;

/// GameBanana's id for Deadlock.
pub const GAME_ID: u64 = 20_948;
/// Records per `Index` page. GameBanana honours this value for `Index` (but not for search).
pub const LIST_PAGE_SIZE: u32 = 50;
/// Records per search page; fixed by GameBanana, the API ignores any other value.
pub const SEARCH_PAGE_SIZE: u32 = 15;
/// GameBanana trims the search text and rejects it below this many bytes (not characters).
pub const SEARCH_MIN_BYTES: usize = 2;
/// GameBanana rejects a trimmed search text above this many bytes (not characters).
pub const SEARCH_MAX_BYTES: usize = 100;

const BASE: &str = "https://gamebanana.com/apiv11";

/// Catalog orderings GameBanana supports. There is no alphabetical order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sort {
    Newest,
    LatestUpdated,
    MostDownloaded,
    MostLiked,
    MostViewed,
    MostCommented,
    Oldest,
}

/// One page of a section's catalog, optionally narrowed to a category (e.g. a hero under Skins).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListQuery {
    pub section: Section,
    pub sort: Sort,
    /// 1-based; 0 is treated as 1.
    pub page: u32,
    pub category: Option<u64>,
}

/// A validated text search in one section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchQuery {
    section: Section,
    text: String,
    page: u32,
}

/// Why a search text cannot be sent: GameBanana would refuse it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchTextError {
    TooShort,
    TooLong,
}

impl fmt::Display for SearchTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShort => write!(
                f,
                "the search text is shorter than {SEARCH_MIN_BYTES} bytes"
            ),
            Self::TooLong => write!(f, "the search text is longer than {SEARCH_MAX_BYTES} bytes"),
        }
    }
}

impl std::error::Error for SearchTextError {}

impl SearchQuery {
    /// Trims `text` the way GameBanana does and checks its length in UTF-8 bytes; `page` is 1-based (0 means 1).
    pub fn new(section: Section, text: &str, page: u32) -> Result<Self, SearchTextError> {
        let text = text.trim();
        if text.len() < SEARCH_MIN_BYTES {
            return Err(SearchTextError::TooShort);
        }
        if text.len() > SEARCH_MAX_BYTES {
            return Err(SearchTextError::TooLong);
        }
        Ok(Self {
            section,
            text: text.to_owned(),
            page: page.max(1),
        })
    }

    pub fn section(&self) -> Section {
        self.section
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn page(&self) -> u32 {
        self.page
    }
}

fn model(section: Section) -> &'static str {
    match section {
        Section::Mod => "Mod",
        Section::Sound => "Sound",
    }
}

fn sort_alias(sort: Sort) -> &'static str {
    match sort {
        Sort::Newest => "Generic_Newest",
        Sort::LatestUpdated => "Generic_LatestUpdated",
        Sort::MostDownloaded => "Generic_MostDownloaded",
        Sort::MostLiked => "Generic_MostLiked",
        Sort::MostViewed => "Generic_MostViewed",
        Sort::MostCommented => "Generic_MostCommented",
        Sort::Oldest => "Generic_Oldest",
    }
}

/// Builds `path?key=value&…` with every key and value percent-encoded.
fn url(path: &str, params: &[(&str, &str)]) -> String {
    let mut out = format!("{BASE}/{path}");
    for (i, (key, value)) in params.iter().enumerate() {
        out.push(if i == 0 { '?' } else { '&' });
        out.push_str(&encode_component(key));
        out.push('=');
        out.push_str(&encode_component(value));
    }
    out
}

/// `section/Index`: one catalog page of Deadlock submissions.
pub fn list_url(query: &ListQuery) -> String {
    let page = query.page.max(1).to_string();
    let per_page = LIST_PAGE_SIZE.to_string();
    let game = GAME_ID.to_string();
    let category = query.category.map(|c| c.to_string());
    let mut params = vec![
        ("_nPage", page.as_str()),
        ("_nPerpage", per_page.as_str()),
        ("_aFilters[Generic_Game]", game.as_str()),
    ];
    if let Some(category) = &category {
        params.push(("_aFilters[Generic_Category]", category.as_str()));
    }
    params.push(("_sSort", sort_alias(query.sort)));
    url(&format!("{}/Index", model(query.section)), &params)
}

/// `Util/Search/Results`: text search within one section of Deadlock.
pub fn search_url(query: &SearchQuery) -> String {
    let game = GAME_ID.to_string();
    let page = query.page.to_string();
    url(
        "Util/Search/Results",
        &[
            ("_sModelName", model(query.section)),
            ("_sOrder", "best_match"),
            ("_idGameRow", game.as_str()),
            ("_sSearchString", query.text.as_str()),
            ("_nPage", page.as_str()),
        ],
    )
}

/// `section/Categories`: Deadlock's root categories, or the children of `parent` (Skins → heroes).
pub fn categories_url(section: Section, parent: Option<u64>) -> String {
    let path = format!("{}/Categories", model(section));
    match parent {
        None => {
            let game = GAME_ID.to_string();
            url(
                &path,
                &[
                    ("_idGameRow", game.as_str()),
                    ("_sSort", "a_to_z"),
                    ("_bShowEmpty", "true"),
                ],
            )
        }
        Some(parent) => {
            let parent = parent.to_string();
            url(
                &path,
                &[
                    ("_idCategoryRow", parent.as_str()),
                    ("_sSort", "a_to_z"),
                    ("_bShowEmpty", "true"),
                ],
            )
        }
    }
}

/// `section/{id}/ProfilePage`: everything about one submission, including its files.
pub fn details_url(section: Section, id: u64) -> String {
    url(&format!("{}/{id}/ProfilePage", model(section)), &[])
}

/// Percent-encodes a query key or value: RFC 3986 unreserved characters stay, every other UTF-8 byte becomes `%XX`.
/// Small enough that a URL crate is not worth its size here.
pub fn encode_component(text: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut out = String::with_capacity(text.len());
    for &byte in text.as_bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(HEX[usize::from(byte >> 4)]));
            out.push(char::from(HEX[usize::from(byte & 0x0F)]));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list(section: Section, sort: Sort, page: u32, category: Option<u64>) -> String {
        list_url(&ListQuery {
            section,
            sort,
            page,
            category,
        })
    }

    #[test]
    fn mod_index_url() {
        assert_eq!(
            list(Section::Mod, Sort::Newest, 1, None),
            "https://gamebanana.com/apiv11/Mod/Index?_nPage=1&_nPerpage=50\
             &_aFilters%5BGeneric_Game%5D=20948&_sSort=Generic_Newest"
        );
    }

    #[test]
    fn index_url_with_category_filter() {
        assert_eq!(
            list(Section::Mod, Sort::MostDownloaded, 3, Some(33_295)),
            "https://gamebanana.com/apiv11/Mod/Index?_nPage=3&_nPerpage=50\
             &_aFilters%5BGeneric_Game%5D=20948&_aFilters%5BGeneric_Category%5D=33295\
             &_sSort=Generic_MostDownloaded"
        );
    }

    #[test]
    fn sound_index_url() {
        assert_eq!(
            list(Section::Sound, Sort::LatestUpdated, 2, None),
            "https://gamebanana.com/apiv11/Sound/Index?_nPage=2&_nPerpage=50\
             &_aFilters%5BGeneric_Game%5D=20948&_sSort=Generic_LatestUpdated"
        );
    }

    #[test]
    fn every_sort_has_its_alias() {
        let cases = [
            (Sort::Newest, "Generic_Newest"),
            (Sort::LatestUpdated, "Generic_LatestUpdated"),
            (Sort::MostDownloaded, "Generic_MostDownloaded"),
            (Sort::MostLiked, "Generic_MostLiked"),
            (Sort::MostViewed, "Generic_MostViewed"),
            (Sort::MostCommented, "Generic_MostCommented"),
            (Sort::Oldest, "Generic_Oldest"),
        ];
        for (sort, alias) in cases {
            let url = list(Section::Mod, sort, 1, None);
            assert!(url.ends_with(&format!("&_sSort={alias}")), "{url}");
        }
    }

    #[test]
    fn page_zero_means_first_page() {
        assert!(list(Section::Mod, Sort::Newest, 0, None).contains("?_nPage=1&"));
        let search = SearchQuery::new(Section::Mod, "haze", 0).expect("valid");
        assert!(search_url(&search).ends_with("&_nPage=1"));
    }

    #[test]
    fn search_url_encodes_the_text() {
        let query = SearchQuery::new(Section::Mod, "haze skin", 1).expect("valid");
        assert_eq!(
            search_url(&query),
            "https://gamebanana.com/apiv11/Util/Search/Results?_sModelName=Mod&_sOrder=best_match\
             &_idGameRow=20948&_sSearchString=haze%20skin&_nPage=1"
        );
    }

    #[test]
    fn search_text_cannot_inject_parameters() {
        let query = SearchQuery::new(Section::Sound, "a&_nPage=9#x/y?z+", 2).expect("valid");
        assert_eq!(
            search_url(&query),
            "https://gamebanana.com/apiv11/Util/Search/Results?_sModelName=Sound&_sOrder=best_match\
             &_idGameRow=20948&_sSearchString=a%26_nPage%3D9%23x%2Fy%3Fz%2B&_nPage=2"
        );
    }

    #[test]
    fn search_text_is_utf8_percent_encoded() {
        let query = SearchQuery::new(Section::Mod, "меч", 1).expect("valid");
        assert!(search_url(&query).contains("&_sSearchString=%D0%BC%D0%B5%D1%87&"));
    }

    #[test]
    fn search_text_is_trimmed_and_measured_in_bytes() {
        let query = SearchQuery::new(Section::Mod, "  ab  ", 1).expect("valid");
        assert_eq!(query.text(), "ab");
        assert_eq!(
            SearchQuery::new(Section::Mod, "  a  ", 1),
            Err(SearchTextError::TooShort)
        );
        assert_eq!(
            SearchQuery::new(Section::Mod, "", 1),
            Err(SearchTextError::TooShort)
        );
        // One Cyrillic letter is two bytes: enough for GameBanana's minimum.
        assert!(SearchQuery::new(Section::Mod, "м", 1).is_ok());
        assert!(SearchQuery::new(Section::Mod, &"a".repeat(100), 1).is_ok());
        assert_eq!(
            SearchQuery::new(Section::Mod, &"a".repeat(101), 1),
            Err(SearchTextError::TooLong)
        );
        // 60 Cyrillic letters are 120 bytes: GameBanana refuses them.
        assert_eq!(
            SearchQuery::new(Section::Mod, &"м".repeat(60), 1),
            Err(SearchTextError::TooLong)
        );
    }

    #[test]
    fn root_categories_url() {
        assert_eq!(
            categories_url(Section::Mod, None),
            "https://gamebanana.com/apiv11/Mod/Categories?_idGameRow=20948&_sSort=a_to_z&_bShowEmpty=true"
        );
        assert_eq!(
            categories_url(Section::Sound, None),
            "https://gamebanana.com/apiv11/Sound/Categories?_idGameRow=20948&_sSort=a_to_z&_bShowEmpty=true"
        );
    }

    #[test]
    fn child_categories_url() {
        assert_eq!(
            categories_url(Section::Mod, Some(33_295)),
            "https://gamebanana.com/apiv11/Mod/Categories?_idCategoryRow=33295&_sSort=a_to_z&_bShowEmpty=true"
        );
    }

    #[test]
    fn details_urls() {
        assert_eq!(
            details_url(Section::Mod, 655_209),
            "https://gamebanana.com/apiv11/Mod/655209/ProfilePage"
        );
        assert_eq!(
            details_url(Section::Sound, 94_152),
            "https://gamebanana.com/apiv11/Sound/94152/ProfilePage"
        );
    }

    #[test]
    fn encoder_keeps_only_unreserved_characters() {
        assert_eq!(encode_component("Az09-._~"), "Az09-._~");
        assert_eq!(encode_component(" !\"%'()*"), "%20%21%22%25%27%28%29%2A");
        assert_eq!(encode_component("[]"), "%5B%5D");
        assert_eq!(encode_component("é"), "%C3%A9");
        assert_eq!(encode_component(""), "");
    }
}
