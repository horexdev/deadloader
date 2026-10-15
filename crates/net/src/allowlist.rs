//! Which hosts deadloader may talk to, and for what.
//!
//! Four sources, each scoped to request purposes (ADR 0007, decisions 53, 54, 122):
//! - built-in GameBanana hosts — everything except the update check;
//! - the GitHub API host — only the update check, and only while the user has it enabled;
//! - hosts the user added in Advanced (`user`) — same scope as the built-in ones;
//! - description-image hosts the user consented to (`user-image`) — description images only, never API calls or
//!   downloads, so consenting to show a picture from imgur cannot turn imgur into a download source.
//!
//! Matching is exact (no subdomain wildcards); the only pattern is GameBanana's numbered file caches.

use crate::url::Host;
use std::collections::BTreeSet;
use std::fmt;

pub const GITHUB_API_HOST: &str = "api.github.com";

const BUILT_IN: [&str; 3] = [
    "gamebanana.com",
    "images.gamebanana.com",
    "files.gamebanana.com",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Purpose {
    Catalog,
    Thumbnail,
    DescriptionImage,
    Download,
    UpdateCheck,
}

impl Purpose {
    /// Stable identifier for logs.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Catalog => "catalog",
            Self::Thumbnail => "thumbnail",
            Self::DescriptionImage => "description-image",
            Self::Download => "download",
            Self::UpdateCheck => "update-check",
        }
    }

    fn describe(self) -> &'static str {
        match self {
            Self::Catalog => "catalog request",
            Self::Thumbnail => "thumbnail",
            Self::DescriptionImage => "description image",
            Self::Download => "download",
            Self::UpdateCheck => "update check",
        }
    }
}

impl fmt::Display for Purpose {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.describe())
    }
}

/// Whether the user asked for this request (clicked something) or deadloader made it on its own (paging, thumbnails).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Initiator {
    Auto,
    User,
}

impl Initiator {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::User => "user",
        }
    }
}

/// Why a host was allowed; written to the network log as `host_source`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostSource {
    BuiltIn,
    GitHub,
    User,
    UserImage,
}

impl HostSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BuiltIn => "built-in",
            Self::GitHub => "github",
            Self::User => "user",
            Self::UserImage => "user-image",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Denied {
    NotListed,
    /// The host is listed, but its source does not cover this purpose (e.g. an image host used for a download).
    WrongPurpose(HostSource),
}

impl fmt::Display for Denied {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotListed => f.write_str("is not on the allowlist"),
            Self::WrongPurpose(HostSource::UserImage) => {
                f.write_str("is allowed only for description images")
            }
            Self::WrongPurpose(HostSource::GitHub) => {
                f.write_str("is allowed only for the update check")
            }
            Self::WrongPurpose(HostSource::BuiltIn | HostSource::User) => {
                f.write_str("is not used for the update check")
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Allowlist {
    update_check: bool,
    user: BTreeSet<Host>,
    images: BTreeSet<Host>,
}

impl Allowlist {
    /// Built-in GameBanana hosts only; the update check is off.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_update_check(&mut self, enabled: bool) {
        self.update_check = enabled;
    }

    pub fn update_check(&self) -> bool {
        self.update_check
    }

    /// Returns `false` if the host was already listed.
    pub fn add_user_host(&mut self, host: Host) -> bool {
        self.user.insert(host)
    }

    pub fn remove_user_host(&mut self, host: &Host) -> bool {
        self.user.remove(host)
    }

    pub fn user_hosts(&self) -> impl Iterator<Item = &Host> {
        self.user.iter()
    }

    /// Returns `false` if the host was already listed.
    pub fn add_image_host(&mut self, host: Host) -> bool {
        self.images.insert(host)
    }

    pub fn remove_image_host(&mut self, host: &Host) -> bool {
        self.images.remove(host)
    }

    pub fn image_hosts(&self) -> impl Iterator<Item = &Host> {
        self.images.iter()
    }

    /// Whether `host` may be contacted for `purpose`, and which list allowed it.
    ///
    /// Sources are consulted from the broadest to the narrowest scope, so a host present in both user lists is
    /// reported as `user`.
    pub fn check(&self, host: &Host, purpose: Purpose) -> Result<HostSource, Denied> {
        let source = if is_built_in(host.as_str()) {
            HostSource::BuiltIn
        } else if self.update_check && host.as_str() == GITHUB_API_HOST {
            HostSource::GitHub
        } else if self.user.contains(host) {
            HostSource::User
        } else if self.images.contains(host) {
            HostSource::UserImage
        } else {
            return Err(Denied::NotListed);
        };
        if covers(source, purpose) {
            Ok(source)
        } else {
            Err(Denied::WrongPurpose(source))
        }
    }
}

fn covers(source: HostSource, purpose: Purpose) -> bool {
    match source {
        HostSource::BuiltIn | HostSource::User => purpose != Purpose::UpdateCheck,
        HostSource::GitHub => purpose == Purpose::UpdateCheck,
        HostSource::UserImage => purpose == Purpose::DescriptionImage,
    }
}

/// `host` is already ASCII lowercase without a trailing dot (see [`Host`]).
fn is_built_in(host: &str) -> bool {
    BUILT_IN.contains(&host) || is_file_cache(host)
}

/// `filecache<1–3 digits>.gamebanana.com`: download mirrors seen as two-digit numbers (`filecache07`); the digit
/// count is not a documented contract, so up to three are accepted.
fn is_file_cache(host: &str) -> bool {
    host.strip_prefix("filecache")
        .and_then(|rest| rest.strip_suffix(".gamebanana.com"))
        .is_some_and(|digits| {
            (1..=3).contains(&digits.len()) && digits.bytes().all(|b| b.is_ascii_digit())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Purpose; 5] = [
        Purpose::Catalog,
        Purpose::Thumbnail,
        Purpose::DescriptionImage,
        Purpose::Download,
        Purpose::UpdateCheck,
    ];

    fn host(s: &str) -> Host {
        Host::parse(s).unwrap_or_else(|e| panic!("{s}: {e}"))
    }

    fn check(list: &Allowlist, h: &str, purpose: Purpose) -> Result<HostSource, Denied> {
        list.check(&host(h), purpose)
    }

    #[test]
    fn built_in_hosts_serve_gamebanana_purposes() {
        let list = Allowlist::new();
        for h in [
            "gamebanana.com",
            "images.gamebanana.com",
            "files.gamebanana.com",
            "filecache1.gamebanana.com",
            "filecache07.gamebanana.com",
            "filecache123.gamebanana.com",
            "FileCache07.GameBanana.com.",
        ] {
            for purpose in [
                Purpose::Catalog,
                Purpose::Thumbnail,
                Purpose::DescriptionImage,
                Purpose::Download,
            ] {
                assert_eq!(
                    check(&list, h, purpose),
                    Ok(HostSource::BuiltIn),
                    "{h} {purpose:?}"
                );
            }
        }
    }

    #[test]
    fn look_alikes_are_not_listed() {
        let list = Allowlist::new();
        for h in [
            "gamebanana.com.evil.net",
            "evilgamebanana.com",
            "evil-gamebanana.com",
            "www.gamebanana.com",
            "api.gamebanana.com",
            "filecache.gamebanana.com",
            "filecache1234.gamebanana.com",
            "filecachex.gamebanana.com",
            "filecache07x.gamebanana.com",
            "xfilecache07.gamebanana.com",
            "filecache07.gamebanana.com.evil.net",
            "filecache07.evil.gamebanana.com",
            "files.gamebanana.co",
            "gamebanana.net",
        ] {
            for purpose in ALL {
                assert_eq!(
                    check(&list, h, purpose),
                    Err(Denied::NotListed),
                    "{h} {purpose:?}"
                );
            }
        }
    }

    #[test]
    fn github_is_listed_only_while_the_update_check_is_enabled() {
        let mut list = Allowlist::new();
        assert_eq!(
            check(&list, GITHUB_API_HOST, Purpose::UpdateCheck),
            Err(Denied::NotListed)
        );
        list.set_update_check(true);
        assert_eq!(
            check(&list, "API.GitHub.com", Purpose::UpdateCheck),
            Ok(HostSource::GitHub)
        );
        assert_eq!(
            check(&list, "github.com", Purpose::UpdateCheck),
            Err(Denied::NotListed)
        );
        for purpose in [
            Purpose::Catalog,
            Purpose::Download,
            Purpose::DescriptionImage,
        ] {
            assert_eq!(
                check(&list, GITHUB_API_HOST, purpose),
                Err(Denied::WrongPurpose(HostSource::GitHub)),
                "{purpose:?}"
            );
        }
    }

    #[test]
    fn the_update_check_does_not_go_to_gamebanana() {
        let list = Allowlist::new();
        assert_eq!(
            check(&list, "gamebanana.com", Purpose::UpdateCheck),
            Err(Denied::WrongPurpose(HostSource::BuiltIn))
        );
    }

    #[test]
    fn user_hosts_are_exact_and_marked() {
        let mut list = Allowlist::new();
        assert!(list.add_user_host(host("Mirror.Example.org")));
        assert!(
            !list.add_user_host(host("mirror.example.org.")),
            "duplicate"
        );
        for purpose in [
            Purpose::Catalog,
            Purpose::Thumbnail,
            Purpose::DescriptionImage,
            Purpose::Download,
        ] {
            assert_eq!(
                check(&list, "mirror.example.org", purpose),
                Ok(HostSource::User)
            );
        }
        assert_eq!(
            check(&list, "mirror.example.org", Purpose::UpdateCheck),
            Err(Denied::WrongPurpose(HostSource::User))
        );
        assert_eq!(
            check(&list, "cdn.mirror.example.org", Purpose::Download),
            Err(Denied::NotListed),
            "no subdomain wildcards"
        );
        assert!(list.remove_user_host(&host("mirror.example.org")));
        assert_eq!(
            check(&list, "mirror.example.org", Purpose::Download),
            Err(Denied::NotListed)
        );
    }

    #[test]
    fn image_hosts_serve_only_description_images() {
        let mut list = Allowlist::new();
        assert!(list.add_image_host(host("i.imgur.com")));
        assert_eq!(
            check(&list, "i.imgur.com", Purpose::DescriptionImage),
            Ok(HostSource::UserImage)
        );
        for purpose in [
            Purpose::Catalog,
            Purpose::Thumbnail,
            Purpose::Download,
            Purpose::UpdateCheck,
        ] {
            assert_eq!(
                check(&list, "i.imgur.com", purpose),
                Err(Denied::WrongPurpose(HostSource::UserImage)),
                "{purpose:?}"
            );
        }
        assert_eq!(
            check(&list, "imgur.com", Purpose::DescriptionImage),
            Err(Denied::NotListed)
        );
        assert!(list.remove_image_host(&host("i.imgur.com")));
        assert_eq!(
            check(&list, "i.imgur.com", Purpose::DescriptionImage),
            Err(Denied::NotListed)
        );
    }

    #[test]
    fn a_host_in_both_user_lists_reports_the_broader_source() {
        let mut list = Allowlist::new();
        list.add_user_host(host("cdn.example.org"));
        list.add_image_host(host("cdn.example.org"));
        assert_eq!(
            check(&list, "cdn.example.org", Purpose::DescriptionImage),
            Ok(HostSource::User)
        );
        assert_eq!(
            check(&list, "cdn.example.org", Purpose::Download),
            Ok(HostSource::User)
        );
    }

    #[test]
    fn denials_read_as_sentences() {
        assert_eq!(Denied::NotListed.to_string(), "is not on the allowlist");
        assert_eq!(
            Denied::WrongPurpose(HostSource::UserImage).to_string(),
            "is allowed only for description images"
        );
    }
}
