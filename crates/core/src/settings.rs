//! Settings (decision 99): what the user chose, in `deadloader-data\settings.json`.
//!
//! Pretty-printed JSON with a schema number, like the manifest, because users may read and edit it (Settings →
//! Advanced, decision 156). A newer schema is refused rather than misread.
//!
//! Loading is forgiving about values and strict about shape. A file that is not JSON, not an object, has no valid
//! schema number, or has a section that is not an object is refused: it is not a settings file we understand. Inside
//! a section, every field stands on its own: a missing field takes its default (forward and backward
//! compatibility); a value of the wrong type, an unknown choice or a number outside its range takes its default; an
//! unknown field is ignored. Each of these is reported as a [`Warning`] with the field's path, so the caller can log
//! it and tell the user — one typo in a hand edit must not throw away every other setting. Warnings never stop the
//! program from starting; the file itself is left as it is until the next save.
//!
//! Saving is strict: settings with anything a load would warn about are never written ([`SettingsError::Rejected`]),
//! and [`from_json_strict`] gives the Advanced editor the same check before it saves the user's text. Unknown fields
//! are refused there too, because saving would silently drop them.
//!
//! Rules between settings are enforced by methods, never by silently changing values behind the user's back:
//! deadloader's own update check is unavailable in local mode (157) and NSFW beyond "hidden" needs the age
//! confirmation (109). A file that breaks a rule is corrected on load with a warning.
//!
//! This module only models and stores; the caller logs loads, saves, warnings and the age confirmation.

use crate::library::profile_file_name;
use serde::de::{self, DeserializeOwned, Deserializer};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Map, Value};
use std::fmt;
use std::io::{self, Write};
use std::path::Path;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

pub const SCHEMA: u32 = 1;
pub const FILE_NAME: &str = "settings.json";

/// The allowed values of a numeric setting, both ends included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub min: u32,
    pub max: u32,
}

impl Range {
    pub fn contains(self, value: u32) -> bool {
        (self.min..=self.max).contains(&value)
    }
}

/// The ranges of numeric settings, for the UI's inputs and for validation.
pub mod ranges {
    use super::Range;

    pub const ZOOM_PERCENT: Range = Range { min: 50, max: 200 };
    pub const PAGE_TTL_MINUTES: Range = Range { min: 1, max: 1440 };
    pub const PARALLEL_DOWNLOADS: Range = Range { min: 1, max: 16 };
    /// KiB/s when a limit is set; no limit is `None`.
    pub const SPEED_LIMIT_KIB_PER_S: Range = Range {
        min: 1,
        max: 1_048_576,
    };
    pub const MAX_EXTRACTED_MIB: Range = Range {
        min: 1,
        max: 65_536,
    };
    pub const MAX_EXTRACTED_FACTOR: Range = Range { min: 1, max: 1000 };
    pub const MAX_VPK_FILES: Range = Range { min: 1, max: 4096 };
    pub const MAX_ENTRY_RATIO: Range = Range {
        min: 2,
        max: 10_000,
    };
    pub const PARALLEL_API_REQUESTS: Range = Range { min: 1, max: 4 };
    pub const CONNECT_TIMEOUT_S: Range = Range { min: 1, max: 120 };
    pub const READ_TIMEOUT_S: Range = Range { min: 1, max: 600 };
    pub const RETRIES: Range = Range { min: 0, max: 10 };
    pub const THUMBNAIL_LIMIT_MIB: Range = Range {
        min: 10,
        max: 10_240,
    };
    pub const LOG_RETENTION_DAYS: Range = Range { min: 1, max: 365 };
    pub const LOG_SIZE_LIMIT_MIB: Range = Range {
        min: 1,
        max: 10_240,
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub general: General,
    pub game: Game,
    pub catalog: Catalog,
    pub library: Library,
    pub downloads: Downloads,
    pub network: Network,
    pub cache: Cache,
    pub logs: Logs,
    pub advanced: Advanced,
    pub ui: Ui,
    pub acknowledgements: Acknowledgements,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct General {
    pub language: UiLanguage,
    pub startup_profile: StartupProfile,
    /// deadloader's own update check on GitHub (54/70). Asked in the first-run notice, off unless enabled there;
    /// always off in local mode (157) — read it through [`Settings::app_update_check`].
    pub check_app_updates: bool,
    /// On top of Windows DPI scaling (96/101).
    pub zoom_percent: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Game {
    /// Repair a game-updated gameinfo.gi automatically (18).
    pub auto_fix_gameinfo: bool,
    /// Game folders added by hand; detected installations are found again at every start and not stored (21/69).
    pub manual_installations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Catalog {
    /// Off = local mode: manual import only, zero network requests (108). Off until the first-run choice, so
    /// nothing is requested before the user has agreed.
    pub enabled: bool,
    /// Set through [`Settings::set_nsfw`]: anything but hidden needs the age confirmation (109).
    pub nsfw: NsfwMode,
    pub description_images: DescriptionImages,
    /// Whether "always allow images from this host" starts checked in the image dialog (122).
    pub always_allow_host_prechecked: bool,
    /// Hosts the user allowed for description images only — not for API calls or downloads (122).
    pub allowed_image_hosts: Vec<String>,
    pub gif_autoplay: bool,
    pub external_links: ExternalLinks,
    /// How long a catalog page is shown from cache without asking the network (58).
    pub page_ttl_minutes: u32,
    pub prefetch_next_page: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Library {
    pub check_updates_at_start: bool,
    /// Installing still waits for confirmation (39); this only fetches the files ahead.
    pub download_updates_automatically: bool,
    pub deletion: Deletion,
    pub old_versions: OldVersions,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Downloads {
    pub parallel: u32,
    /// KiB/s; `None` = no limit.
    pub speed_limit_kib_per_s: Option<u32>,
    pub keep_archives: bool,
    pub rar_method: RarMethod,
    /// Archive limits (127): at most min(this, factor × archive size) unpacked from one archive.
    pub max_extracted_mib: u32,
    pub max_extracted_factor: u32,
    pub max_vpk_files: u32,
    /// An entry must unpack to less than this many times its compressed size.
    pub max_entry_ratio: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Network {
    pub parallel_api_requests: u32,
    pub connect_timeout_s: u32,
    pub read_timeout_s: u32,
    /// Retries on 429/5xx, with exponential backoff.
    pub retries: u32,
    pub proxy: Proxy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Cache {
    /// The oldest thumbnails are evicted above this (58).
    pub thumbnail_limit_mib: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Logs {
    pub retention_days: u32,
    pub size_limit_mib: u32,
    /// Write `C:\Users\<name>` as `%USERPROFILE%` (94).
    pub mask_user_path: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Advanced {
    /// Added to the built-in GameBanana hosts of the network allowlist and marked in the log (53).
    pub extra_allowed_domains: Vec<String>,
}

/// Window state that is remembered, not chosen on the Settings screen.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Ui {
    /// The user's choice; below ~1100 px the sidebar collapses anyway without changing this (139/141).
    pub sidebar_collapsed: bool,
    pub last_selected_profile: Option<String>,
}

/// What the user confirmed. Not preferences: kept by [`Settings::reset`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Acknowledgements {
    /// The first-run notice (107/111); `None` until it is acknowledged.
    pub notice: Option<NoticeAcknowledgement>,
    /// When the user confirmed legal age for NSFW content (109); asked once.
    pub age_confirmed_at: Option<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeAcknowledgement {
    /// The notice version; a newer notice re-shows its changed screens (111).
    pub version: u32,
    pub at: Timestamp,
    /// The language it was read in (English prevails on discrepancies).
    pub language: Lang,
    pub catalog: CatalogChoice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogChoice {
    Catalog,
    LocalMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NsfwMode {
    /// Not listed, and thumbnails of NSFW mods are not downloaded (27).
    #[default]
    Hidden,
    Blurred,
    Shown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptionImages {
    #[default]
    OnClick,
    Always,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalLinks {
    /// Show the full URL and wait for confirmation (31).
    #[default]
    Ask,
    OpenImmediately,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Deletion {
    #[default]
    Permanent,
    RecycleBin,
}

/// What happens to old mod versions. Only "keep all" is decided (22); old versions are removed by hand with
/// "delete old versions". Other policies would be added here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OldVersions {
    #[default]
    KeepAll,
}

/// How RAR archives are opened. Not decided yet: the implementation spike of 123 picks between compiled-in readers
/// and external programs, so `Auto` (try them in 123's order) is the default and the explicit choices may change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RarMethod {
    #[default]
    Auto,
    Builtin,
    /// Windows' `tar.exe`.
    Tar,
    SevenZip,
    #[serde(rename = "winrar")]
    WinRar,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum Proxy {
    /// Direct connections.
    #[default]
    None,
    System,
    /// `http://host:port`. No credentials: deadloader stores none (32).
    Manual {
        address: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupProfile {
    #[default]
    LastSelected,
    Named(String),
}

/// The 16 UI languages (106).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    En,
    PtBr,
    Cs,
    Fr,
    De,
    It,
    Id,
    Ja,
    Ko,
    Es419,
    Pl,
    Ru,
    ZhHans,
    EsEs,
    Tr,
    Uk,
}

/// The system language if it is one of ours, otherwise English (106) — or a fixed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiLanguage {
    #[default]
    System,
    Fixed(Lang),
}

/// A moment in UTC, to the second, written as `2026-10-15T09:30:05Z`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(OffsetDateTime);

/// Whether deadloader checks GitHub for its own updates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppUpdateCheck {
    On,
    Off,
    /// Local mode promises zero requests (157); the UI shows the option disabled with the reason.
    UnavailableInLocalMode,
}

/// A change the rules between settings don't allow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleError {
    /// NSFW beyond "hidden" needs the age confirmation first (109).
    AgeNotConfirmed,
    UnavailableInLocalMode,
}

/// Settings read from a file, with what had to be corrected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loaded {
    pub settings: Settings,
    pub warnings: Vec<Warning>,
}

/// One field that could not be used as written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warning {
    /// Dotted path in the file, e.g. `downloads.parallel`.
    pub path: String,
    pub problem: Problem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    /// Not a setting of this version; ignored.
    UnknownField,
    /// Wrong type or unknown choice; the default is used. The reason is English, for the log.
    InvalidValue { reason: String },
    /// The default is used.
    OutOfRange { found: u64, min: u32, max: u32 },
    /// A list entry that is not valid; removed.
    InvalidEntry { entry: String },
    /// A list entry given twice; the repeat is removed.
    DuplicateEntry { entry: String },
    /// NSFW was set beyond "hidden" without the age confirmation; set back to hidden.
    NeedsAgeConfirmation,
    /// deadloader's update check was on in local mode; turned off.
    UnavailableInLocalMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsError {
    NewerSchema {
        found: u32,
        supported: u32,
    },
    Json {
        line: usize,
        column: usize,
        message: String,
    },
    /// The file (empty path) or a section is not a JSON object.
    NotAnObject {
        path: String,
    },
    MissingSchema,
    InvalidSchema,
    /// Not saved, because of these problems.
    Rejected(Vec<Warning>),
    Io(String),
}

impl Default for General {
    fn default() -> Self {
        Self {
            language: UiLanguage::System,
            startup_profile: StartupProfile::LastSelected,
            check_app_updates: false,
            zoom_percent: 100,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            auto_fix_gameinfo: true,
            manual_installations: Vec::new(),
        }
    }
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            enabled: false,
            nsfw: NsfwMode::Hidden,
            description_images: DescriptionImages::OnClick,
            always_allow_host_prechecked: true,
            allowed_image_hosts: Vec::new(),
            gif_autoplay: false,
            external_links: ExternalLinks::Ask,
            page_ttl_minutes: 10,
            prefetch_next_page: true,
        }
    }
}

impl Default for Library {
    fn default() -> Self {
        Self {
            check_updates_at_start: true,
            download_updates_automatically: false,
            deletion: Deletion::Permanent,
            old_versions: OldVersions::KeepAll,
        }
    }
}

impl Default for Downloads {
    fn default() -> Self {
        Self {
            parallel: 4,
            speed_limit_kib_per_s: None,
            keep_archives: false,
            rar_method: RarMethod::Auto,
            max_extracted_mib: 2048,
            max_extracted_factor: 20,
            max_vpk_files: 256,
            max_entry_ratio: 100,
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self {
            parallel_api_requests: 2,
            connect_timeout_s: 10,
            read_timeout_s: 30,
            retries: 5,
            proxy: Proxy::None,
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            thumbnail_limit_mib: 100,
        }
    }
}

impl Default for Logs {
    fn default() -> Self {
        Self {
            retention_days: 30,
            size_limit_mib: 100,
            mask_user_path: true,
        }
    }
}

impl Lang {
    /// In the order of decision 106.
    pub const ALL: [Lang; 16] = [
        Lang::En,
        Lang::PtBr,
        Lang::Cs,
        Lang::Fr,
        Lang::De,
        Lang::It,
        Lang::Id,
        Lang::Ja,
        Lang::Ko,
        Lang::Es419,
        Lang::Pl,
        Lang::Ru,
        Lang::ZhHans,
        Lang::EsEs,
        Lang::Tr,
        Lang::Uk,
    ];

    /// The BCP 47 tag, as written in the file.
    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::PtBr => "pt-BR",
            Lang::Cs => "cs",
            Lang::Fr => "fr",
            Lang::De => "de",
            Lang::It => "it",
            Lang::Id => "id",
            Lang::Ja => "ja",
            Lang::Ko => "ko",
            Lang::Es419 => "es-419",
            Lang::Pl => "pl",
            Lang::Ru => "ru",
            Lang::ZhHans => "zh-Hans",
            Lang::EsEs => "es-ES",
            Lang::Tr => "tr",
            Lang::Uk => "uk",
        }
    }

    pub fn from_code(code: &str) -> Option<Lang> {
        Self::ALL.into_iter().find(|l| l.code() == code)
    }
}

const SYSTEM_LANGUAGE: &str = "system";

fn unknown_language<E: de::Error>(code: &str) -> E {
    let known: Vec<&str> = Lang::ALL.iter().map(|l| l.code()).collect();
    E::custom(format!(
        "unknown language \"{code}\", expected \"{SYSTEM_LANGUAGE}\" or one of {}",
        known.join(", ")
    ))
}

impl Serialize for Lang {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.code())
    }
}

impl<'de> Deserialize<'de> for Lang {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = String::deserialize(deserializer)?;
        Lang::from_code(&code).ok_or_else(|| unknown_language(&code))
    }
}

impl Serialize for UiLanguage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UiLanguage::System => serializer.serialize_str(SYSTEM_LANGUAGE),
            UiLanguage::Fixed(lang) => lang.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for UiLanguage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let code = String::deserialize(deserializer)?;
        if code == SYSTEM_LANGUAGE {
            return Ok(UiLanguage::System);
        }
        Lang::from_code(&code)
            .map(UiLanguage::Fixed)
            .ok_or_else(|| unknown_language(&code))
    }
}

impl Timestamp {
    /// Converted to UTC; fractions of a second are dropped.
    pub fn new(at: OffsetDateTime) -> Self {
        let utc = at.to_offset(UtcOffset::UTC);
        Self(utc.replace_nanosecond(0).unwrap_or(utc))
    }

    pub fn get(self) -> OffsetDateTime {
        self.0
    }

    fn parse(text: &str) -> Option<Self> {
        let b = text.as_bytes();
        let shape = b.len() == 20
            && b[4] == b'-'
            && b[7] == b'-'
            && b[10] == b'T'
            && b[13] == b':'
            && b[16] == b':'
            && b[19] == b'Z';
        if !shape {
            return None;
        }
        let number = |from: usize, to: usize| -> Option<u16> {
            let digits = text.get(from..to)?;
            if !digits.bytes().all(|c| c.is_ascii_digit()) {
                return None;
            }
            digits.parse().ok()
        };
        let small = |from: usize, to: usize| number(from, to).and_then(|n| u8::try_from(n).ok());
        let month = Month::try_from(small(5, 7)?).ok()?;
        let date = Date::from_calendar_date(i32::from(number(0, 4)?), month, small(8, 10)?).ok()?;
        let time = Time::from_hms(small(11, 13)?, small(14, 16)?, small(17, 19)?).ok()?;
        Some(Self(PrimitiveDateTime::new(date, time).assume_utc()))
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (date, time) = (self.0.date(), self.0.time());
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            date.year(),
            u8::from(date.month()),
            date.day(),
            time.hour(),
            time.minute(),
            time.second()
        )
    }
}

impl Serialize for Timestamp {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Timestamp::parse(&text).ok_or_else(|| {
            de::Error::custom(format!(
                "\"{text}\" is not a UTC time like 2026-10-15T09:30:05Z"
            ))
        })
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = &self.path;
        match &self.problem {
            Problem::UnknownField => write!(f, "{path}: not a setting of this version, ignored"),
            Problem::InvalidValue { reason } => {
                write!(f, "{path}: {reason}, the default is used")
            }
            Problem::OutOfRange { found, min, max } => {
                write!(
                    f,
                    "{path}: {found} is outside {min}-{max}, the default is used"
                )
            }
            Problem::InvalidEntry { entry } => {
                write!(f, "{path}: \"{entry}\" is not valid here and was removed")
            }
            Problem::DuplicateEntry { entry } => {
                write!(
                    f,
                    "{path}: \"{entry}\" is listed twice, the repeat was removed"
                )
            }
            Problem::NeedsAgeConfirmation => write!(
                f,
                "{path}: showing NSFW content needs the age confirmation, set back to hidden"
            ),
            Problem::UnavailableInLocalMode => write!(
                f,
                "{path}: unavailable in local mode (no network requests), turned off"
            ),
        }
    }
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewerSchema { found, supported } => write!(
                f,
                "made by a newer deadloader (format {found}; this version reads up to {supported})"
            ),
            Self::Json {
                line,
                column,
                message,
            } => write!(
                f,
                "not valid JSON at line {line}, column {column}: {message}"
            ),
            Self::NotAnObject { path } if path.is_empty() => {
                write!(f, "the settings file must be a JSON object {{ … }}")
            }
            Self::NotAnObject { path } => {
                write!(f, "\"{path}\" must be a JSON object {{ … }}")
            }
            Self::MissingSchema => write!(
                f,
                "the \"schema\" number is missing, so this is not a deadloader settings file"
            ),
            Self::InvalidSchema => write!(f, "\"schema\" must be a whole number from 1"),
            Self::Rejected(warnings) => {
                write!(f, "not saved:")?;
                for w in warnings {
                    write!(f, "\n- {w}")?;
                }
                Ok(())
            }
            Self::Io(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SettingsError {}

impl From<serde_json::Error> for SettingsError {
    fn from(e: serde_json::Error) -> Self {
        Self::Json {
            line: e.line(),
            column: e.column(),
            message: e.to_string(),
        }
    }
}

impl From<io::Error> for SettingsError {
    fn from(e: io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

/// Collects warnings with their paths.
struct Checker<'a> {
    warnings: &'a mut Vec<Warning>,
}

impl Checker<'_> {
    fn warn(&mut self, path: &str, problem: Problem) {
        self.warnings.push(Warning {
            path: path.to_owned(),
            problem,
        });
    }

    fn range(&mut self, path: &str, value: &mut u32, range: Range, default: u32) {
        if !range.contains(*value) {
            self.warn(
                path,
                Problem::OutOfRange {
                    found: u64::from(*value),
                    min: range.min,
                    max: range.max,
                },
            );
            *value = default;
        }
    }

    /// Keeps the entries `normalise` accepts, in order and without repeats (`same` decides what a repeat is).
    fn list(
        &mut self,
        path: &str,
        list: &mut Vec<String>,
        normalise: impl Fn(&str) -> Option<String>,
        same: impl Fn(&str, &str) -> bool,
    ) {
        let mut kept: Vec<String> = Vec::new();
        for entry in std::mem::take(list) {
            match normalise(&entry) {
                None => self.warn(path, Problem::InvalidEntry { entry }),
                Some(value) if kept.iter().any(|k| same(k, &value)) => {
                    self.warn(path, Problem::DuplicateEntry { entry });
                }
                Some(value) => kept.push(value),
            }
        }
        *list = kept;
    }

    fn profile_name(&mut self, path: &str, name: &str) -> bool {
        let ok = profile_file_name(name).is_ok();
        if !ok {
            self.warn(
                path,
                Problem::InvalidValue {
                    reason: format!("\"{name}\" can't be a profile name"),
                },
            );
        }
        ok
    }
}

/// A host name as written in allowlists: lowercase letters, digits and hyphens in dot-separated labels.
fn normalise_host(entry: &str) -> Option<String> {
    let host = entry.trim().to_ascii_lowercase();
    let valid_label = |label: &str| {
        !label.is_empty()
            && label.len() <= 63
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    };
    (host.len() <= 253 && host.split('.').all(valid_label)).then_some(host)
}

/// `http://host:port` with an optional trailing slash; IPv6 hosts in brackets; no credentials, no path.
fn is_valid_proxy_address(address: &str) -> bool {
    let Some(rest) = address.strip_prefix("http://") else {
        return false;
    };
    let rest = rest.strip_suffix('/').unwrap_or(rest);
    let (host_ok, port) = if let Some(bracketed) = rest.strip_prefix('[') {
        match bracketed.split_once("]:") {
            Some((ip, port)) => (
                !ip.is_empty() && ip.bytes().all(|b| b.is_ascii_hexdigit() || b == b':'),
                port,
            ),
            None => return false,
        }
    } else {
        match rest.rsplit_once(':') {
            Some((host, port)) => (
                !host.contains(char::is_whitespace) && normalise_host(host).is_some(),
                port,
            ),
            None => return false,
        }
    };
    host_ok
        && !port.is_empty()
        && port.bytes().all(|b| b.is_ascii_digit())
        && port.parse::<u16>().is_ok_and(|p| p != 0)
}

impl Settings {
    /// Brings every value into its range and the settings in line with their rules, replacing what is wrong with
    /// its default. Returns what was changed; nothing means the settings are valid as they are.
    pub fn normalize(&mut self) -> Vec<Warning> {
        let mut warnings = Vec::new();
        let mut c = Checker {
            warnings: &mut warnings,
        };
        let d = Settings::default();

        c.range(
            "general.zoom_percent",
            &mut self.general.zoom_percent,
            ranges::ZOOM_PERCENT,
            d.general.zoom_percent,
        );
        if let StartupProfile::Named(name) = &self.general.startup_profile
            && !c.profile_name("general.startup_profile", name)
        {
            self.general.startup_profile = d.general.startup_profile.clone();
        }

        c.list(
            "game.manual_installations",
            &mut self.game.manual_installations,
            |p| (!p.trim().is_empty()).then(|| p.to_owned()),
            |a, b| a.eq_ignore_ascii_case(b),
        );

        c.list(
            "catalog.allowed_image_hosts",
            &mut self.catalog.allowed_image_hosts,
            normalise_host,
            |a, b| a == b,
        );
        c.range(
            "catalog.page_ttl_minutes",
            &mut self.catalog.page_ttl_minutes,
            ranges::PAGE_TTL_MINUTES,
            d.catalog.page_ttl_minutes,
        );

        let dl = &mut self.downloads;
        c.range(
            "downloads.parallel",
            &mut dl.parallel,
            ranges::PARALLEL_DOWNLOADS,
            d.downloads.parallel,
        );
        if let Some(limit) = dl.speed_limit_kib_per_s
            && !ranges::SPEED_LIMIT_KIB_PER_S.contains(limit)
        {
            c.warn(
                "downloads.speed_limit_kib_per_s",
                Problem::OutOfRange {
                    found: u64::from(limit),
                    min: ranges::SPEED_LIMIT_KIB_PER_S.min,
                    max: ranges::SPEED_LIMIT_KIB_PER_S.max,
                },
            );
            dl.speed_limit_kib_per_s = d.downloads.speed_limit_kib_per_s;
        }
        c.range(
            "downloads.max_extracted_mib",
            &mut dl.max_extracted_mib,
            ranges::MAX_EXTRACTED_MIB,
            d.downloads.max_extracted_mib,
        );
        c.range(
            "downloads.max_extracted_factor",
            &mut dl.max_extracted_factor,
            ranges::MAX_EXTRACTED_FACTOR,
            d.downloads.max_extracted_factor,
        );
        c.range(
            "downloads.max_vpk_files",
            &mut dl.max_vpk_files,
            ranges::MAX_VPK_FILES,
            d.downloads.max_vpk_files,
        );
        c.range(
            "downloads.max_entry_ratio",
            &mut dl.max_entry_ratio,
            ranges::MAX_ENTRY_RATIO,
            d.downloads.max_entry_ratio,
        );

        let net = &mut self.network;
        c.range(
            "network.parallel_api_requests",
            &mut net.parallel_api_requests,
            ranges::PARALLEL_API_REQUESTS,
            d.network.parallel_api_requests,
        );
        c.range(
            "network.connect_timeout_s",
            &mut net.connect_timeout_s,
            ranges::CONNECT_TIMEOUT_S,
            d.network.connect_timeout_s,
        );
        c.range(
            "network.read_timeout_s",
            &mut net.read_timeout_s,
            ranges::READ_TIMEOUT_S,
            d.network.read_timeout_s,
        );
        c.range(
            "network.retries",
            &mut net.retries,
            ranges::RETRIES,
            d.network.retries,
        );
        if let Proxy::Manual { address } = &net.proxy
            && !is_valid_proxy_address(address)
        {
            c.warn(
                "network.proxy",
                Problem::InvalidValue {
                    reason: format!(
                        "the proxy address \"{address}\" is not like http://host:port (no user name or password)"
                    ),
                },
            );
            net.proxy = d.network.proxy.clone();
        }

        c.range(
            "cache.thumbnail_limit_mib",
            &mut self.cache.thumbnail_limit_mib,
            ranges::THUMBNAIL_LIMIT_MIB,
            d.cache.thumbnail_limit_mib,
        );
        c.range(
            "logs.retention_days",
            &mut self.logs.retention_days,
            ranges::LOG_RETENTION_DAYS,
            d.logs.retention_days,
        );
        c.range(
            "logs.size_limit_mib",
            &mut self.logs.size_limit_mib,
            ranges::LOG_SIZE_LIMIT_MIB,
            d.logs.size_limit_mib,
        );

        c.list(
            "advanced.extra_allowed_domains",
            &mut self.advanced.extra_allowed_domains,
            normalise_host,
            |a, b| a == b,
        );

        if let Some(name) = &self.ui.last_selected_profile
            && !c.profile_name("ui.last_selected_profile", name)
        {
            self.ui.last_selected_profile = None;
        }

        if self
            .acknowledgements
            .notice
            .as_ref()
            .is_some_and(|n| n.version == 0)
        {
            c.warn(
                "acknowledgements.notice",
                Problem::InvalidValue {
                    reason: "the notice version must be 1 or higher".to_owned(),
                },
            );
            self.acknowledgements.notice = None;
        }

        // Rules between settings, after every value is usable on its own.
        if !self.can_set_nsfw(self.catalog.nsfw) {
            c.warn("catalog.nsfw", Problem::NeedsAgeConfirmation);
            self.catalog.nsfw = NsfwMode::Hidden;
        }
        if self.is_local_mode() && self.general.check_app_updates {
            c.warn("general.check_app_updates", Problem::UnavailableInLocalMode);
            self.general.check_app_updates = false;
        }
        warnings
    }

    /// Local mode: the catalog is off and deadloader makes no network requests at all (108).
    pub fn is_local_mode(&self) -> bool {
        !self.catalog.enabled
    }

    /// Turning the catalog off (local mode) also turns deadloader's update check off (157); turning it back on
    /// does not turn the check back on — that is the user's separate choice.
    pub fn set_catalog_enabled(&mut self, enabled: bool) {
        self.catalog.enabled = enabled;
        if !enabled {
            self.general.check_app_updates = false;
        }
    }

    pub fn app_update_check(&self) -> AppUpdateCheck {
        if self.is_local_mode() {
            AppUpdateCheck::UnavailableInLocalMode
        } else if self.general.check_app_updates {
            AppUpdateCheck::On
        } else {
            AppUpdateCheck::Off
        }
    }

    pub fn set_app_update_check(&mut self, on: bool) -> Result<(), RuleError> {
        if on && self.is_local_mode() {
            return Err(RuleError::UnavailableInLocalMode);
        }
        self.general.check_app_updates = on;
        Ok(())
    }

    /// Whether `mode` can be chosen now; if not, the UI asks for the age confirmation first (dialog D3).
    pub fn can_set_nsfw(&self, mode: NsfwMode) -> bool {
        mode == NsfwMode::Hidden || self.acknowledgements.age_confirmed_at.is_some()
    }

    pub fn set_nsfw(&mut self, mode: NsfwMode) -> Result<(), RuleError> {
        if !self.can_set_nsfw(mode) {
            return Err(RuleError::AgeNotConfirmed);
        }
        self.catalog.nsfw = mode;
        Ok(())
    }

    /// Records the age confirmation (109). The caller logs it.
    pub fn confirm_age(&mut self, at: OffsetDateTime) {
        self.acknowledgements.age_confirmed_at = Some(Timestamp::new(at));
    }

    /// Records the acknowledged first-run notice (111) and applies its catalog choice (108).
    pub fn acknowledge_notice(
        &mut self,
        version: u32,
        at: OffsetDateTime,
        language: Lang,
        catalog: CatalogChoice,
    ) {
        self.acknowledgements.notice = Some(NoticeAcknowledgement {
            version,
            at: Timestamp::new(at),
            language,
            catalog,
        });
        self.set_catalog_enabled(catalog == CatalogChoice::Catalog);
    }

    /// Whether the notice of `current_version` still has to be shown (never acknowledged, or an older version).
    pub fn needs_notice(&self, current_version: u32) -> bool {
        self.acknowledgements
            .notice
            .as_ref()
            .is_none_or(|n| n.version < current_version)
    }

    /// "Reset all settings to defaults" (100). Every preference goes back to its default; kept are things that are
    /// not preferences: what the user acknowledged or confirmed (the notice, the age confirmation), the catalog
    /// choice made in the notice (resetting it would either cut the catalog off or turn the network on without
    /// asking), game folders added by hand (profiles point at them) and the selected profile.
    pub fn reset(&self) -> Settings {
        let mut fresh = Settings {
            acknowledgements: self.acknowledgements.clone(),
            ..Settings::default()
        };
        fresh.set_catalog_enabled(self.catalog.enabled);
        fresh.game.manual_installations = self.game.manual_installations.clone();
        fresh.ui.last_selected_profile = self.ui.last_selected_profile.clone();
        fresh
    }
}

/// One section: every field on its own (see the module docs).
fn read_section<S>(
    top: &Map<String, Value>,
    name: &str,
    warnings: &mut Vec<Warning>,
) -> Result<S, SettingsError>
where
    S: Default + Serialize + DeserializeOwned,
{
    let input = match top.get(name) {
        None => return Ok(S::default()),
        Some(Value::Object(input)) => input,
        Some(_) => {
            return Err(SettingsError::NotAnObject {
                path: name.to_owned(),
            });
        }
    };
    let Value::Object(mut merged) = serde_json::to_value(S::default())? else {
        return Err(SettingsError::NotAnObject {
            path: name.to_owned(),
        });
    };
    // The defaults are valid, so a field that fails when tried on top of them is wrong by itself.
    for (key, value) in input {
        let mut trial = merged.clone();
        trial.insert(key.clone(), value.clone());
        match serde_json::from_value::<S>(Value::Object(trial)) {
            Ok(_) => {
                merged.insert(key.clone(), value.clone());
            }
            Err(e) => warnings.push(Warning {
                path: format!("{name}.{key}"),
                problem: Problem::InvalidValue {
                    reason: e.to_string(),
                },
            }),
        }
    }
    let merged = Value::Object(merged);
    let section: S = serde_json::from_value(merged.clone())?;
    unknown_fields(&merged, &serde_json::to_value(&section)?, name, warnings);
    Ok(section)
}

/// Keys of `input` that did not survive into `known`, at any depth.
fn unknown_fields(input: &Value, known: &Value, path: &str, warnings: &mut Vec<Warning>) {
    if let (Value::Object(input), Value::Object(known)) = (input, known) {
        for (key, value) in input {
            let path = format!("{path}.{key}");
            match known.get(key) {
                Some(known) => unknown_fields(value, known, &path, warnings),
                None => warnings.push(Warning {
                    path,
                    problem: Problem::UnknownField,
                }),
            }
        }
    }
}

const SECTIONS: [&str; 11] = [
    "general",
    "game",
    "catalog",
    "library",
    "downloads",
    "network",
    "cache",
    "logs",
    "advanced",
    "ui",
    "acknowledgements",
];

/// Reads settings, correcting what it can (see the module docs).
pub fn from_json(text: &str) -> Result<Loaded, SettingsError> {
    let Value::Object(top) = serde_json::from_str::<Value>(text)? else {
        return Err(SettingsError::NotAnObject {
            path: String::new(),
        });
    };
    let schema = top
        .get("schema")
        .ok_or(SettingsError::MissingSchema)?
        .as_u64()
        .filter(|s| *s >= 1)
        .ok_or(SettingsError::InvalidSchema)?;
    if schema > u64::from(SCHEMA) {
        return Err(SettingsError::NewerSchema {
            found: u32::try_from(schema).unwrap_or(u32::MAX),
            supported: SCHEMA,
        });
    }
    let mut warnings: Vec<Warning> = top
        .keys()
        .filter(|k| *k != "schema" && !SECTIONS.contains(&k.as_str()))
        .map(|k| Warning {
            path: k.clone(),
            problem: Problem::UnknownField,
        })
        .collect();
    let w = &mut warnings;
    let mut settings = Settings {
        general: read_section(&top, "general", w)?,
        game: read_section(&top, "game", w)?,
        catalog: read_section(&top, "catalog", w)?,
        library: read_section(&top, "library", w)?,
        downloads: read_section(&top, "downloads", w)?,
        network: read_section(&top, "network", w)?,
        cache: read_section(&top, "cache", w)?,
        logs: read_section(&top, "logs", w)?,
        advanced: read_section(&top, "advanced", w)?,
        ui: read_section(&top, "ui", w)?,
        acknowledgements: read_section(&top, "acknowledgements", w)?,
    };
    warnings.extend(settings.normalize());
    Ok(Loaded { settings, warnings })
}

/// For the Advanced editor (156): the user's text is accepted only if loading it would change nothing.
pub fn from_json_strict(text: &str) -> Result<Settings, SettingsError> {
    let loaded = from_json(text)?;
    if loaded.warnings.is_empty() {
        Ok(loaded.settings)
    } else {
        Err(SettingsError::Rejected(loaded.warnings))
    }
}

#[derive(Serialize)]
struct SettingsFile<'a> {
    schema: u32,
    #[serde(flatten)]
    settings: &'a Settings,
}

/// The file's text; refused if the settings are not valid as they are.
pub fn to_json(settings: &Settings) -> Result<String, SettingsError> {
    let mut checked = settings.clone();
    let problems = checked.normalize();
    if !problems.is_empty() {
        return Err(SettingsError::Rejected(problems));
    }
    Ok(serde_json::to_string_pretty(&SettingsFile {
        schema: SCHEMA,
        settings: &checked,
    })?)
}

/// Writes atomically: temp file, flushed, previous version kept as `.bak`, then renamed into place.
pub fn save(path: &Path, settings: &Settings) -> Result<(), SettingsError> {
    let text = to_json(settings)?;
    let tmp = path.with_extension("json.tmp");
    {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(text.as_bytes())?;
        f.sync_all()?;
    }
    if path.exists() {
        std::fs::copy(path, path.with_extension("json.bak"))?;
    }
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// A missing file is a first start: the defaults, without warnings.
pub fn load(path: &Path) -> Result<Loaded, SettingsError> {
    match std::fs::read_to_string(path) {
        Ok(text) => from_json(&text),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Loaded {
            settings: Settings::default(),
            warnings: Vec::new(),
        }),
        Err(e) => Err(e.into()),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use time::{Date, Month, PrimitiveDateTime, Time};

    fn at(hour: u8, offset_hours: i8) -> OffsetDateTime {
        PrimitiveDateTime::new(
            Date::from_calendar_date(2026, Month::October, 15).expect("date"),
            Time::from_hms(hour, 30, 5).expect("time"),
        )
        .assume_offset(UtcOffset::from_hms(offset_hours, 0, 0).expect("offset"))
    }

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("deadloader-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        dir
    }

    fn paths(warnings: &[Warning]) -> Vec<&str> {
        warnings.iter().map(|w| w.path.as_str()).collect()
    }

    fn load_text(text: &str) -> Loaded {
        from_json(text).expect("loads")
    }

    /// Every default, as the decision log states it.
    #[test]
    fn defaults_match_the_decision_log() {
        let expected = json!({
            "schema": 1,
            "general": {
                "language": "system",                 // 106: system language if supported, else English
                "startup_profile": "last_selected",   // 99
                "check_app_updates": false,           // 54/70: asked at first run, off unless enabled
                "zoom_percent": 100                   // 96/101
            },
            "game": {
                "auto_fix_gameinfo": true,            // 18
                "manual_installations": []            // 21/69: detected ones are not stored
            },
            "catalog": {
                "enabled": false,                     // 108: no network before the first-run choice
                "nsfw": "hidden",                     // 27/78
                "description_images": "on_click",     // 29
                "always_allow_host_prechecked": true, // 122
                "allowed_image_hosts": [],            // 122
                "gif_autoplay": false,                // 12
                "external_links": "ask",              // 31
                "page_ttl_minutes": 10,               // 58
                "prefetch_next_page": true            // 28
            },
            "library": {
                "check_updates_at_start": true,           // 38
                "download_updates_automatically": false,  // 39
                "deletion": "permanent",                  // 42
                "old_versions": "keep_all"                // 22
            },
            "downloads": {
                "parallel": 4,                        // 44
                "speed_limit_kib_per_s": null,        // 99: none
                "keep_archives": false,               // 45
                "rar_method": "auto",                 // 43/123: undecided until the spike
                "max_extracted_mib": 2048,            // 127: 2 GB
                "max_extracted_factor": 20,           // 127: 20 x archive size
                "max_vpk_files": 256,                 // 127
                "max_entry_ratio": 100                // 127: < 100:1
            },
            "network": {
                "parallel_api_requests": 2,           // 99
                "connect_timeout_s": 10,              // 99
                "read_timeout_s": 30,                 // 99
                "retries": 5,                         // 99
                "proxy": { "mode": "none" }           // 99
            },
            "cache": {
                "thumbnail_limit_mib": 100            // 58
            },
            "logs": {
                "retention_days": 30,                 // 51
                "size_limit_mib": 100,                // 51
                "mask_user_path": true                // 94
            },
            "advanced": {
                "extra_allowed_domains": []           // 53
            },
            "ui": {
                "sidebar_collapsed": false,           // 139/141
                "last_selected_profile": null         // 99
            },
            "acknowledgements": {
                "notice": null,                       // 107/111
                "age_confirmed_at": null              // 109
            }
        });
        let text = to_json(&Settings::default()).expect("serialise");
        let actual: Value = serde_json::from_str(&text).expect("valid JSON");
        assert_eq!(actual, expected);
    }

    #[test]
    fn the_file_is_readable_json_that_starts_with_the_schema() {
        let text = to_json(&Settings::default()).expect("serialise");
        assert!(
            text.starts_with("{\n  \"schema\": 1,\n  \"general\": {"),
            "{text}"
        );
    }

    fn customised() -> Settings {
        let mut s = Settings::default();
        s.general.language = UiLanguage::Fixed(Lang::PtBr);
        s.general.startup_profile = StartupProfile::Named("Evening".to_owned());
        s.general.zoom_percent = 125;
        s.game.manual_installations = vec![r"E:\Games\Deadlock".to_owned()];
        s.set_catalog_enabled(true);
        s.set_app_update_check(true).expect("catalog on");
        s.confirm_age(at(12, 3));
        s.set_nsfw(NsfwMode::Blurred).expect("age confirmed");
        s.catalog.description_images = DescriptionImages::Always;
        s.catalog.allowed_image_hosts = vec!["i.imgur.com".to_owned(), "cdn.7tv.app".to_owned()];
        s.catalog.external_links = ExternalLinks::OpenImmediately;
        s.catalog.page_ttl_minutes = 60;
        s.library.deletion = Deletion::RecycleBin;
        s.downloads.parallel = 8;
        s.downloads.speed_limit_kib_per_s = Some(2048);
        s.downloads.rar_method = RarMethod::SevenZip;
        s.network.proxy = Proxy::Manual {
            address: "http://127.0.0.1:8080".to_owned(),
        };
        s.logs.mask_user_path = false;
        s.advanced.extra_allowed_domains = vec!["example.org".to_owned()];
        s.ui.sidebar_collapsed = true;
        s.ui.last_selected_profile = Some("Evening".to_owned());
        s.acknowledge_notice(1, at(11, 3), Lang::Ru, CatalogChoice::Catalog);
        s
    }

    #[test]
    fn settings_round_trip_through_json_without_warnings() {
        let s = customised();
        let text = to_json(&s).expect("serialise");
        let loaded = load_text(&text);
        assert_eq!(loaded.warnings, vec![]);
        assert_eq!(loaded.settings, s);
    }

    #[test]
    fn missing_fields_and_sections_take_their_defaults() {
        let loaded = load_text(r#"{ "schema": 1, "downloads": { "parallel": 8 } }"#);
        assert_eq!(loaded.warnings, vec![]);
        let mut expected = Settings::default();
        expected.downloads.parallel = 8;
        assert_eq!(loaded.settings, expected);
    }

    #[test]
    fn an_out_of_range_value_takes_its_default_and_is_reported() {
        let loaded = load_text(
            r#"{ "schema": 1,
                 "general": { "zoom_percent": 20, "language": "de" },
                 "downloads": { "parallel": 99, "keep_archives": true },
                 "network": { "parallel_api_requests": 0, "retries": 11 } }"#,
        );
        let s = &loaded.settings;
        assert_eq!(s.general.zoom_percent, 100);
        assert_eq!(
            s.general.language,
            UiLanguage::Fixed(Lang::De),
            "neighbours are kept"
        );
        assert_eq!(s.downloads.parallel, 4);
        assert!(s.downloads.keep_archives, "neighbours are kept");
        assert_eq!(s.network.parallel_api_requests, 2);
        assert_eq!(s.network.retries, 5);
        assert_eq!(
            loaded.warnings,
            vec![
                Warning {
                    path: "general.zoom_percent".to_owned(),
                    problem: Problem::OutOfRange {
                        found: 20,
                        min: 50,
                        max: 200
                    }
                },
                Warning {
                    path: "downloads.parallel".to_owned(),
                    problem: Problem::OutOfRange {
                        found: 99,
                        min: 1,
                        max: 16
                    }
                },
                Warning {
                    path: "network.parallel_api_requests".to_owned(),
                    problem: Problem::OutOfRange {
                        found: 0,
                        min: 1,
                        max: 4
                    }
                },
                Warning {
                    path: "network.retries".to_owned(),
                    problem: Problem::OutOfRange {
                        found: 11,
                        min: 0,
                        max: 10
                    }
                },
            ]
        );
    }

    #[test]
    fn every_numeric_setting_is_range_checked() {
        let loaded = load_text(
            r#"{ "schema": 1,
                 "catalog": { "page_ttl_minutes": 0 },
                 "downloads": { "speed_limit_kib_per_s": 0, "max_extracted_mib": 0, "max_extracted_factor": 0,
                                "max_vpk_files": 0, "max_entry_ratio": 1 },
                 "network": { "connect_timeout_s": 0, "read_timeout_s": 601 },
                 "cache": { "thumbnail_limit_mib": 5 },
                 "logs": { "retention_days": 0, "size_limit_mib": 0 } }"#,
        );
        assert_eq!(
            paths(&loaded.warnings),
            vec![
                "catalog.page_ttl_minutes",
                "downloads.speed_limit_kib_per_s",
                "downloads.max_extracted_mib",
                "downloads.max_extracted_factor",
                "downloads.max_vpk_files",
                "downloads.max_entry_ratio",
                "network.connect_timeout_s",
                "network.read_timeout_s",
                "cache.thumbnail_limit_mib",
                "logs.retention_days",
                "logs.size_limit_mib",
            ]
        );
        assert_eq!(loaded.settings, Settings::default());
    }

    #[test]
    fn a_wrong_type_or_an_unknown_choice_takes_the_default_and_is_reported() {
        let loaded = load_text(
            r#"{ "schema": 1,
                 "catalog": { "nsfw": "blured", "gif_autoplay": "yes", "prefetch_next_page": false },
                 "network": { "proxy": { "mode": "manul", "address": "http://h:1" }, "retries": 1.5 },
                 "general": { "language": "xx" } }"#,
        );
        let s = &loaded.settings;
        assert_eq!(s.catalog.nsfw, NsfwMode::Hidden);
        assert!(!s.catalog.gif_autoplay);
        assert!(!s.catalog.prefetch_next_page, "neighbours are kept");
        assert_eq!(s.network.proxy, Proxy::None);
        assert_eq!(s.network.retries, 5);
        assert_eq!(s.general.language, UiLanguage::System);
        let mut got = paths(&loaded.warnings);
        got.sort_unstable();
        assert_eq!(
            got,
            vec![
                "catalog.gif_autoplay",
                "catalog.nsfw",
                "general.language",
                "network.proxy",
                "network.retries",
            ]
        );
        assert!(
            loaded
                .warnings
                .iter()
                .all(|w| matches!(w.problem, Problem::InvalidValue { .. })),
            "{:?}",
            loaded.warnings
        );
    }

    #[test]
    fn unknown_fields_are_ignored_with_a_warning() {
        let loaded = load_text(
            r#"{ "schema": 1, "theme": "light",
                 "catalog": { "colour": "pink", "gif_autoplay": true },
                 "network": { "proxy": { "mode": "system", "password": "x" } } }"#,
        );
        assert!(loaded.settings.catalog.gif_autoplay);
        assert_eq!(loaded.settings.network.proxy, Proxy::System);
        let mut got = paths(&loaded.warnings);
        got.sort_unstable();
        assert_eq!(
            got,
            vec!["catalog.colour", "network.proxy.password", "theme"]
        );
        assert!(
            loaded
                .warnings
                .iter()
                .all(|w| w.problem == Problem::UnknownField)
        );
        let saved = to_json(&loaded.settings).expect("serialise");
        assert!(!saved.contains("colour") && !saved.contains("theme"));
    }

    #[test]
    fn broken_json_is_an_error_with_line_and_column() {
        // A missing comma after the schema line.
        let err = from_json("{\n  \"schema\": 1\n  \"general\": {}\n}").expect_err("broken");
        assert!(
            matches!(
                err,
                SettingsError::Json {
                    line: 3,
                    column: 3,
                    ..
                }
            ),
            "{err:?}"
        );
        assert!(err.to_string().contains("line 3, column 3"), "{err}");
    }

    #[test]
    fn a_newer_schema_is_refused_instead_of_misread() {
        let err = from_json(r#"{ "schema": 2 }"#).expect_err("newer");
        assert_eq!(
            err,
            SettingsError::NewerSchema {
                found: 2,
                supported: SCHEMA
            }
        );
    }

    #[test]
    fn a_file_that_is_not_shaped_like_settings_is_refused() {
        assert_eq!(
            from_json("[]").expect_err("array"),
            SettingsError::NotAnObject {
                path: String::new()
            }
        );
        assert_eq!(
            from_json("{}").expect_err("no schema"),
            SettingsError::MissingSchema
        );
        assert_eq!(
            from_json(r#"{ "schema": "one" }"#).expect_err("text schema"),
            SettingsError::InvalidSchema
        );
        assert_eq!(
            from_json(r#"{ "schema": 0 }"#).expect_err("zero schema"),
            SettingsError::InvalidSchema
        );
        assert_eq!(
            from_json(r#"{ "schema": 1, "catalog": [] }"#).expect_err("section array"),
            SettingsError::NotAnObject {
                path: "catalog".to_owned()
            }
        );
    }

    #[test]
    fn strict_parsing_for_the_editor_refuses_what_loading_would_repair() {
        let text = r#"{ "schema": 1, "downloads": { "parallel": 99 }, "extra": 1 }"#;
        let err = from_json_strict(text).expect_err("strict");
        let SettingsError::Rejected(warnings) = err else {
            panic!("expected Rejected, got {err:?}");
        };
        assert_eq!(paths(&warnings), vec!["extra", "downloads.parallel"]);
        let fine = from_json_strict(r#"{ "schema": 1, "downloads": { "parallel": 9 } }"#)
            .expect("valid edit");
        assert_eq!(fine.downloads.parallel, 9);
    }

    #[test]
    fn saving_is_atomic_and_keeps_the_previous_version() {
        let dir = temp_dir("settings-save");
        let path = dir.join(FILE_NAME);
        let mut s = Settings::default();
        save(&path, &s).expect("first save");
        s.downloads.parallel = 6;
        save(&path, &s).expect("second save");
        assert_eq!(load(&path).expect("load").settings.downloads.parallel, 6);
        let backup = std::fs::read_to_string(dir.join("settings.json.bak")).expect("backup");
        assert_eq!(load_text(&backup).settings.downloads.parallel, 4);
        assert!(
            !dir.join("settings.json.tmp").exists(),
            "no temp file left behind"
        );
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn invalid_settings_are_never_saved() {
        let dir = temp_dir("settings-invalid");
        let path = dir.join(FILE_NAME);
        let mut s = Settings::default();
        s.downloads.parallel = 0;
        s.catalog.nsfw = NsfwMode::Shown;
        let err = save(&path, &s).expect_err("invalid");
        let SettingsError::Rejected(warnings) = err else {
            panic!("expected Rejected, got {err:?}");
        };
        assert_eq!(paths(&warnings), vec!["downloads.parallel", "catalog.nsfw"]);
        assert!(!path.exists());
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn a_missing_file_loads_as_defaults() {
        let dir = temp_dir("settings-missing");
        let loaded = load(&dir.join(FILE_NAME)).expect("load");
        assert_eq!(loaded.settings, Settings::default());
        assert_eq!(loaded.warnings, vec![]);
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    #[test]
    fn the_update_check_is_unavailable_in_local_mode() {
        let mut s = Settings::default();
        assert!(s.is_local_mode(), "no network before the first-run choice");
        assert_eq!(s.app_update_check(), AppUpdateCheck::UnavailableInLocalMode);
        assert_eq!(
            s.set_app_update_check(true),
            Err(RuleError::UnavailableInLocalMode)
        );
        assert_eq!(s.set_app_update_check(false), Ok(()));

        s.set_catalog_enabled(true);
        assert_eq!(s.app_update_check(), AppUpdateCheck::Off);
        s.set_app_update_check(true).expect("catalog on");
        assert_eq!(s.app_update_check(), AppUpdateCheck::On);

        s.set_catalog_enabled(false);
        assert_eq!(s.app_update_check(), AppUpdateCheck::UnavailableInLocalMode);
        assert!(!s.general.check_app_updates, "forced off, not just hidden");
        s.set_catalog_enabled(true);
        assert_eq!(
            s.app_update_check(),
            AppUpdateCheck::Off,
            "leaving local mode does not turn requests back on by itself"
        );
    }

    #[test]
    fn a_file_asking_for_update_checks_in_local_mode_is_corrected() {
        let loaded = load_text(
            r#"{ "schema": 1, "general": { "check_app_updates": true }, "catalog": { "enabled": false } }"#,
        );
        assert!(!loaded.settings.general.check_app_updates);
        assert_eq!(
            loaded.warnings,
            vec![Warning {
                path: "general.check_app_updates".to_owned(),
                problem: Problem::UnavailableInLocalMode
            }]
        );
    }

    #[test]
    fn nsfw_beyond_hidden_needs_the_age_confirmation() {
        let mut s = Settings::default();
        assert!(s.can_set_nsfw(NsfwMode::Hidden));
        assert!(!s.can_set_nsfw(NsfwMode::Blurred));
        assert!(!s.can_set_nsfw(NsfwMode::Shown));
        assert_eq!(s.set_nsfw(NsfwMode::Shown), Err(RuleError::AgeNotConfirmed));
        assert_eq!(s.catalog.nsfw, NsfwMode::Hidden, "nothing changed");

        s.confirm_age(at(12, 3));
        assert!(s.can_set_nsfw(NsfwMode::Shown));
        s.set_nsfw(NsfwMode::Shown).expect("confirmed");
        assert_eq!(s.catalog.nsfw, NsfwMode::Shown);
        s.set_nsfw(NsfwMode::Hidden).expect("always allowed");
        assert!(
            s.acknowledgements.age_confirmed_at.is_some(),
            "asked once (109): hiding again keeps the confirmation"
        );
    }

    #[test]
    fn a_file_showing_nsfw_without_the_age_confirmation_is_corrected() {
        let loaded = load_text(r#"{ "schema": 1, "catalog": { "nsfw": "shown" } }"#);
        assert_eq!(loaded.settings.catalog.nsfw, NsfwMode::Hidden);
        assert_eq!(
            loaded.warnings,
            vec![Warning {
                path: "catalog.nsfw".to_owned(),
                problem: Problem::NeedsAgeConfirmation
            }]
        );
        let ok = load_text(
            r#"{ "schema": 1, "catalog": { "nsfw": "blurred" },
                 "acknowledgements": { "age_confirmed_at": "2026-10-15T09:30:05Z" } }"#,
        );
        assert_eq!(ok.warnings, vec![]);
        assert_eq!(ok.settings.catalog.nsfw, NsfwMode::Blurred);
    }

    #[test]
    fn times_are_stored_in_utc_to_the_second() {
        let mut s = Settings::default();
        s.confirm_age(at(12, 3).replace_millisecond(250).expect("ms"));
        let text = to_json(&s).expect("serialise");
        assert!(
            text.contains("\"age_confirmed_at\": \"2026-10-15T09:30:05Z\""),
            "{text}"
        );
        let stamp = s.acknowledgements.age_confirmed_at.expect("set");
        assert_eq!(stamp.get(), at(9, 0));
    }

    #[test]
    fn a_damaged_record_is_dropped_so_it_is_asked_again() {
        let loaded = load_text(
            r#"{ "schema": 1,
                 "acknowledgements": {
                    "age_confirmed_at": "yesterday",
                    "notice": { "version": 0, "at": "2026-10-15T09:30:05Z", "language": "en", "catalog": "catalog" } } }"#,
        );
        assert_eq!(
            loaded.settings.acknowledgements,
            Acknowledgements::default()
        );
        let mut got = paths(&loaded.warnings);
        got.sort_unstable();
        assert_eq!(
            got,
            vec![
                "acknowledgements.age_confirmed_at",
                "acknowledgements.notice"
            ]
        );
    }

    #[test]
    fn acknowledging_the_notice_records_it_and_applies_the_catalog_choice() {
        let mut s = Settings::default();
        assert!(s.needs_notice(1));
        s.acknowledge_notice(1, at(12, 3), Lang::Uk, CatalogChoice::Catalog);
        assert!(!s.needs_notice(1));
        assert!(s.needs_notice(2), "a newer notice is shown again (111)");
        assert!(!s.is_local_mode());
        assert_eq!(
            s.acknowledgements.notice,
            Some(NoticeAcknowledgement {
                version: 1,
                at: Timestamp::new(at(12, 3)),
                language: Lang::Uk,
                catalog: CatalogChoice::Catalog,
            })
        );

        s.set_app_update_check(true).expect("catalog on");
        s.acknowledge_notice(2, at(13, 3), Lang::Uk, CatalogChoice::LocalMode);
        assert!(s.is_local_mode());
        assert!(!s.general.check_app_updates);
    }

    #[test]
    fn reset_restores_preferences_and_keeps_records_and_choices() {
        let s = customised();
        let reset = s.reset();
        let mut expected = Settings {
            acknowledgements: s.acknowledgements.clone(),
            ..Settings::default()
        };
        expected.catalog.enabled = true;
        expected.ui.last_selected_profile = Some("Evening".to_owned());
        expected.game.manual_installations = vec![r"E:\Games\Deadlock".to_owned()];
        assert_eq!(reset, expected);
        assert_eq!(reset.catalog.nsfw, NsfwMode::Hidden);
        assert_eq!(reset.app_update_check(), AppUpdateCheck::Off);
    }

    #[test]
    fn languages_are_the_sixteen_of_decision_106() {
        let codes: Vec<&str> = Lang::ALL.iter().map(|l| l.code()).collect();
        assert_eq!(
            codes,
            vec![
                "en", "pt-BR", "cs", "fr", "de", "it", "id", "ja", "ko", "es-419", "pl", "ru",
                "zh-Hans", "es-ES", "tr", "uk"
            ]
        );
        for lang in Lang::ALL {
            let text = format!(
                r#"{{ "schema": 1, "general": {{ "language": "{}" }} }}"#,
                lang.code()
            );
            let loaded = load_text(&text);
            assert_eq!(loaded.warnings, vec![], "{}", lang.code());
            assert_eq!(loaded.settings.general.language, UiLanguage::Fixed(lang));
        }
        assert_eq!(Lang::from_code("th"), None, "Thai is dropped (106)");
    }

    #[test]
    fn host_lists_are_normalised_and_bad_entries_dropped() {
        let loaded = load_text(
            r#"{ "schema": 1,
                 "catalog": { "allowed_image_hosts":
                    ["I.Imgur.com", " cdn.7tv.app ", "i.imgur.com", "https://x.com/a", "", "a..b", "*.x.com", "-a.com"] },
                 "advanced": { "extra_allowed_domains": ["example.org", "exa mple.org"] } }"#,
        );
        assert_eq!(
            loaded.settings.catalog.allowed_image_hosts,
            vec!["i.imgur.com", "cdn.7tv.app"]
        );
        assert_eq!(
            loaded.settings.advanced.extra_allowed_domains,
            vec!["example.org"]
        );
        let problems: Vec<(&str, &Problem)> = loaded
            .warnings
            .iter()
            .map(|w| (w.path.as_str(), &w.problem))
            .collect();
        let entry = |e: &str| e.to_owned();
        assert_eq!(
            problems,
            vec![
                (
                    "catalog.allowed_image_hosts",
                    &Problem::DuplicateEntry {
                        entry: entry("i.imgur.com")
                    }
                ),
                (
                    "catalog.allowed_image_hosts",
                    &Problem::InvalidEntry {
                        entry: entry("https://x.com/a")
                    }
                ),
                (
                    "catalog.allowed_image_hosts",
                    &Problem::InvalidEntry { entry: entry("") }
                ),
                (
                    "catalog.allowed_image_hosts",
                    &Problem::InvalidEntry {
                        entry: entry("a..b")
                    }
                ),
                (
                    "catalog.allowed_image_hosts",
                    &Problem::InvalidEntry {
                        entry: entry("*.x.com")
                    }
                ),
                (
                    "catalog.allowed_image_hosts",
                    &Problem::InvalidEntry {
                        entry: entry("-a.com")
                    }
                ),
                (
                    "advanced.extra_allowed_domains",
                    &Problem::InvalidEntry {
                        entry: entry("exa mple.org")
                    }
                ),
            ]
        );
    }

    #[test]
    fn a_manual_proxy_needs_an_http_host_and_port_without_credentials() {
        for good in [
            "http://127.0.0.1:8080",
            "http://proxy.lan:3128/",
            "http://[::1]:8080",
        ] {
            let mut s = Settings::default();
            s.network.proxy = Proxy::Manual {
                address: good.to_owned(),
            };
            assert_eq!(s.clone().normalize(), vec![], "{good}");
        }
        for bad in [
            "",
            "proxy.lan:3128",
            "socks5://127.0.0.1:1080",
            "http://user:secret@proxy.lan:3128",
            "http://proxy.lan",
            "http://proxy.lan:0",
            "http://proxy.lan:70000",
            "http://proxy.lan:80/path",
            "http://pro xy:80",
        ] {
            let mut s = Settings::default();
            s.network.proxy = Proxy::Manual {
                address: bad.to_owned(),
            };
            let warnings = s.normalize();
            assert_eq!(paths(&warnings), vec!["network.proxy"], "{bad}");
            assert_eq!(s.network.proxy, Proxy::None, "{bad}");
        }
    }

    #[test]
    fn profile_names_and_installations_are_checked() {
        let loaded = load_text(
            r#"{ "schema": 1,
                 "general": { "startup_profile": { "named": "a/b" } },
                 "game": { "manual_installations": ["D:\\Deadlock", " ", "d:\\deadlock"] },
                 "ui": { "last_selected_profile": "CON" } }"#,
        );
        let s = &loaded.settings;
        assert_eq!(s.general.startup_profile, StartupProfile::LastSelected);
        assert_eq!(s.ui.last_selected_profile, None);
        assert_eq!(s.game.manual_installations, vec![r"D:\Deadlock"]);
        assert_eq!(
            paths(&loaded.warnings),
            vec![
                "general.startup_profile",
                "game.manual_installations",
                "game.manual_installations",
                "ui.last_selected_profile",
            ]
        );
    }

    #[test]
    fn warnings_read_as_plain_sentences_for_the_log() {
        let w = Warning {
            path: "downloads.parallel".to_owned(),
            problem: Problem::OutOfRange {
                found: 99,
                min: 1,
                max: 16,
            },
        };
        assert_eq!(
            w.to_string(),
            "downloads.parallel: 99 is outside 1-16, the default is used"
        );
    }
}
