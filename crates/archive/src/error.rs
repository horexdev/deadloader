//! Errors a user can act on: each variant says what happened, and `Display` adds why and what to do.
//!
//! The UI maps variants to translated text; `Display` is the English wording used in logs.

use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ArchiveError {
    /// The file starts like none of zip, 7z and RAR.
    NotAnArchive,
    /// A RAR archive in a build without the RAR readers (targets other than MSVC).
    Rar,
    /// Broken structure or data: a truncated download, a bad checksum, a malformed header.
    Damaged {
        entry: Option<String>,
        detail: String,
    },
    /// Password protected: one entry, or the whole archive when even the file list is encrypted.
    Encrypted {
        entry: Option<String>,
    },
    /// A compression method or archive feature this build cannot decode.
    UnsupportedMethod {
        entry: Option<String>,
        method: String,
    },
    Limit(LimitExceeded),
    /// The free-space check before extraction failed (declared size + 10 %).
    InsufficientSpace {
        needed: u64,
        available: u64,
    },
    /// The disk filled up while writing; everything written so far has been removed.
    DiskFull,
    /// Creating, writing or renaming an output file failed.
    Io(io::Error),
}

/// Which safety limit stopped the extraction (decision 127). Nothing extracted is kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimitExceeded {
    /// More bytes were unpacked than `limit`; `entry` is the one being unpacked at that moment.
    TotalSize { limit: u64, entry: String },
    /// The archive holds `found` `.vpk` files, more than `limit`.
    TooManyVpks { limit: usize, found: usize },
    /// `entry` unpacked to at least `limit` times its compressed size.
    Ratio { limit: u64, entry: String },
    /// A 7z block needs a decoding dictionary (memory) of `needed` bytes, more than `limit`.
    Dictionary { limit: u64, needed: u64 },
}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnArchive => f.write_str(
                "the file is not a zip, 7z or RAR archive; the download may be incomplete or the file \
                 is not a mod — download it again",
            ),
            Self::Rar => f.write_str(
                "the file is a RAR archive, which this build of deadloader cannot read; unpack it with \
                 another program and add the .vpk as a local mod",
            ),
            Self::Damaged { entry, detail } => {
                f.write_str("the archive is damaged")?;
                if let Some(entry) = entry {
                    write!(f, " at \"{entry}\"")?;
                }
                write!(
                    f,
                    " ({detail}); the download was probably interrupted or corrupted — download it again"
                )
            }
            Self::Encrypted { entry } => {
                f.write_str("the archive is password protected")?;
                if let Some(entry) = entry {
                    write!(f, " (\"{entry}\")")?;
                }
                f.write_str(
                    "; deadloader does not unpack protected archives — unpack it yourself and add the \
                     .vpk as a local mod",
                )
            }
            Self::UnsupportedMethod { entry, method } => {
                write!(f, "the archive uses a compression method deadloader cannot unpack ({method}")?;
                if let Some(entry) = entry {
                    write!(f, ", \"{entry}\"")?;
                }
                f.write_str(
                    "); unpack it with another program and add the .vpk as a local mod",
                )
            }
            Self::Limit(limit) => write!(f, "{limit}"),
            Self::InsufficientSpace { needed, available } => write!(
                f,
                "not enough free disk space: the mod needs about {}, {} is free — free up space and try again",
                human_size(*needed),
                human_size(*available)
            ),
            Self::DiskFull => f.write_str(
                "the disk filled up while unpacking; the partial files were removed — free up space and \
                 try again",
            ),
            Self::Io(e) => write!(f, "could not write the unpacked files: {e}"),
        }
    }
}

impl fmt::Display for LimitExceeded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const ADVICE: &str = "it is probably damaged or not a mod; if you trust this file, raise the limit \
                              in Settings → Downloads";
        match self {
            Self::TotalSize { limit, entry } => write!(
                f,
                "the archive unpacks to more than {} (stopped at \"{entry}\"); {ADVICE}",
                human_size(*limit)
            ),
            Self::TooManyVpks { limit, found } => write!(
                f,
                "the archive holds {found} .vpk files, more than the limit of {limit}; {ADVICE}"
            ),
            Self::Ratio { limit, entry } => write!(
                f,
                "\"{entry}\" unpacks to more than {limit} times its packed size, which mod files never do; \
                 {ADVICE}"
            ),
            Self::Dictionary { limit, needed } => write!(
                f,
                "the archive needs {} of memory to unpack, more than the limit of {}; {ADVICE}",
                human_size(*needed),
                human_size(*limit)
            ),
        }
    }
}

impl std::error::Error for ArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<LimitExceeded> for ArchiveError {
    fn from(limit: LimitExceeded) -> Self {
        Self::Limit(limit)
    }
}

/// `1536` → "1.5 KB"; binary units with the labels Windows Explorer uses.
pub(crate) fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["KB", "MB", "GB", "TB"];
    if bytes < 1024 {
        return format!("{bytes} bytes");
    }
    // Precision loss above 2^53 bytes is irrelevant for a one-decimal display.
    #[allow(clippy::cast_precision_loss)]
    let mut value = bytes as f64 / 1024.0;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.1} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_are_shown_in_explorer_units() {
        assert_eq!(human_size(0), "0 bytes");
        assert_eq!(human_size(1023), "1023 bytes");
        assert_eq!(human_size(1536), "1.5 KB");
        assert_eq!(human_size(2 << 30), "2.0 GB");
        assert_eq!(human_size(5 << 40), "5.0 TB");
    }

    #[test]
    fn limit_errors_name_the_limit_the_entry_and_the_way_out() {
        let text = ArchiveError::from(LimitExceeded::TotalSize {
            limit: 2 << 30,
            entry: "big.vpk".into(),
        })
        .to_string();
        assert!(text.contains("2.0 GB"), "{text}");
        assert!(text.contains("big.vpk"), "{text}");
        assert!(text.contains("Settings → Downloads"), "{text}");

        let text = LimitExceeded::TooManyVpks {
            limit: 256,
            found: 300,
        }
        .to_string();
        assert!(text.contains("300") && text.contains("256"), "{text}");
    }

    #[test]
    fn every_error_says_what_to_do() {
        let errors = [
            ArchiveError::NotAnArchive,
            ArchiveError::Damaged {
                entry: Some("a.vpk".into()),
                detail: "bad checksum".into(),
            },
            ArchiveError::Encrypted { entry: None },
            ArchiveError::UnsupportedMethod {
                entry: None,
                method: "PPMD".into(),
            },
            ArchiveError::InsufficientSpace {
                needed: 10 << 20,
                available: 1 << 20,
            },
            ArchiveError::DiskFull,
        ];
        for e in errors {
            let text = e.to_string();
            assert!(text.contains(" — ") || text.contains("; "), "{text}");
        }
    }
}
