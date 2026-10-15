//! Mod update detection (decisions 38, 39, 137, 164; spec S4, D5, F8, N3).
//!
//! Authors update a GameBanana mod by re-uploading every file under new file ids, so "an installed file's id is no
//! longer listed" is the signal that it was replaced. The replacement is proposed by the same archive name; after a
//! download, by the game files it changes (VPK overlap); otherwise the user maps it (D5). Files that appear while ours
//! are still listed are only "new files available", not an update.
//!
//! Pure logic: the app fetches GameBanana's answers, maps them into the types here and asks again with the update
//! posts once a mod turns out to have an update. No network, no files.

use crate::library::{
    Manifest, Mod, ModFile, ModelError, Profile, ProfileEntry, Section, Source, Successor,
};
use crate::overlap::FileIndex;
use crate::overlap::game_path;
use crate::selection;
use serde::Serialize;
use std::collections::HashMap;

/// What GameBanana says about one mod now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoteMod {
    Listed(ListedMod),
    Removed(RemovedReason),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListedMod {
    /// The mod's current version: free text (`v5.3`, `2.0`), shown as it is.
    pub version: Option<String>,
    pub date_updated: Option<i64>,
    pub files: Vec<RemoteFile>,
    /// Update posts, newest first; empty when they were not fetched. Only used for the changelog in D5.
    pub posts: Vec<UpdatePost>,
}

/// A file listed on the mod page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RemoteFile {
    pub id: u64,
    pub file_name: String,
    pub size_bytes: u64,
    pub date_added: Option<i64>,
    pub version: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdatePost {
    pub id: u64,
    pub title: String,
    pub version: Option<String>,
    pub date_added: Option<i64>,
    pub changelog: Vec<ChangeEntry>,
    /// The files uploaded with the post.
    pub file_ids: Vec<u64>,
    /// Untrusted HTML; sanitising is the renderer's job.
    pub text_html: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChangeEntry {
    pub category: Option<String>,
    pub text: String,
}

/// Why a mod can't be updated from GameBanana any more.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RemovedReason {
    /// GameBanana has no such mod.
    Deleted,
    Trashed,
    /// Hidden by GameBanana's moderators; it may come back.
    Withheld,
    /// Hidden by the author.
    Private,
    /// The mod page lists no files at all.
    NoFiles,
}

/// The result for one library mod.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModCheck {
    pub mod_id: String,
    pub status: Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Status {
    /// Imported from disk without a GameBanana link: never checked.
    Local,
    /// From GameBanana, but no answer was given for it (the request failed or was not made).
    NotChecked,
    /// None of the mod's current library files knows its GameBanana file id, so a replaced file can't be told.
    Untracked,
    UpToDate,
    /// Files were added while ours are still listed (decision 164): shown, but no N3 count and no D5.
    NewFiles {
        files: Vec<RemoteFile>,
    },
    UpdateAvailable(Update),
    Removed {
        reason: RemovedReason,
    },
}

/// What D5 shows for a mod with replaced files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Update {
    pub version: Option<String>,
    pub date_updated: Option<i64>,
    /// One entry per downloaded GameBanana file of ours that is no longer listed.
    pub replaced: Vec<Replaced>,
    /// GameBanana ids of our files that are still listed.
    pub kept: Vec<u64>,
    /// Listed files that are neither ours nor a proposed replacement: what the user can map a file to.
    pub others: Vec<RemoteFile>,
    /// The posts since our files were uploaded, newest first; empty when unknown.
    pub posts: Vec<UpdatePost>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Replaced {
    /// The GameBanana file our library files came from.
    pub remote_file_id: u64,
    pub archive_name: String,
    /// The library files extracted from it.
    pub files: Vec<String>,
    /// The listed file with the same archive name, if exactly one has it.
    pub candidate: Option<Candidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Candidate {
    pub file: RemoteFile,
    pub matched_by: NameMatch,
    /// Library files already downloaded from it (e.g. by "download updates automatically"), so no download is needed.
    pub in_library: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NameMatch {
    /// The same name, ignoring letter case.
    Same,
    /// The same name once GameBanana's `_xxxxx` suffix is ignored (it adds one when the name is taken, e.g.
    /// `cafevo_ea17d.zip` for `cafevo.zip`).
    WithoutSuffix,
}

/// Files uploaded within this many seconds of our newest listed file belong to the same upload, not "new files".
pub const SAME_UPLOAD_SECONDS: i64 = 3600;

/// Checks every mod of the library. `remote` answers for a GameBanana mod, or `None` when it wasn't asked.
pub fn check<'a>(
    manifest: &Manifest,
    remote: impl Fn(Section, u64) -> Option<&'a RemoteMod>,
) -> Vec<ModCheck> {
    manifest
        .mods
        .iter()
        .map(|m| {
            let answer = match m.source {
                Source::GameBanana { section, mod_id } => remote(section, mod_id),
                Source::Local { .. } => None,
            };
            let dismissed = manifest
                .dismissed_new_files
                .get(&m.id)
                .map_or(&[][..], Vec::as_slice);
            ModCheck {
                mod_id: m.id.clone(),
                status: check_mod_with(m, answer, dismissed),
            }
        })
        .collect()
}

/// One mod against GameBanana's answer for it.
pub fn check_mod(m: &Mod, remote: Option<&RemoteMod>) -> Status {
    check_mod_with(m, remote, &[])
}

/// [`check_mod`], leaving out new files the user hid (`dismissed`, GameBanana file ids, 168).
pub fn check_mod_with(m: &Mod, remote: Option<&RemoteMod>, dismissed: &[u64]) -> Status {
    if matches!(m.source, Source::Local { .. }) {
        return Status::Local;
    }
    let listed = match remote {
        None => return Status::NotChecked,
        Some(RemoteMod::Removed(reason)) => return Status::Removed { reason: *reason },
        Some(RemoteMod::Listed(listed)) => listed,
    };
    let downloads = current_downloads(m);
    // 172: a file kept without a replacement comes back as replaced once a listed file of its name is new to us.
    let returned: Vec<Download<'_>> = retired_downloads(m)
        .into_iter()
        .filter(|d| candidate(m, listed, d.archive_name).is_some_and(|c| c.in_library.is_empty()))
        .collect();
    if downloads.is_empty() && returned.is_empty() {
        return Status::Untracked;
    }
    if listed.files.is_empty() {
        return Status::Removed {
            reason: RemovedReason::NoFiles,
        };
    }
    let is_listed = |id: u64| listed.files.iter().any(|f| f.id == id);
    let (kept, mut gone): (Vec<_>, Vec<_>) = downloads.into_iter().partition(|d| is_listed(d.id));
    gone.extend(returned);
    if gone.is_empty() {
        return new_files(m, listed, &kept, dismissed);
    }

    let replaced: Vec<Replaced> = gone
        .iter()
        .map(|d| Replaced {
            remote_file_id: d.id,
            archive_name: d.archive_name.to_owned(),
            files: d.files.clone(),
            candidate: candidate(m, listed, d.archive_name),
        })
        .collect();
    let proposed: Vec<u64> = replaced
        .iter()
        .filter_map(|r| r.candidate.as_ref().map(|c| c.file.id))
        .collect();
    let kept: Vec<u64> = kept
        .iter()
        .map(|d| d.id)
        .filter(|id| !proposed.contains(id))
        .collect();
    let others = listed
        .files
        .iter()
        .filter(|f| !kept.contains(&f.id) && !proposed.contains(&f.id))
        .cloned()
        .collect();
    let gone_ids: Vec<u64> = gone.iter().map(|d| d.id).collect();
    let posts = posts_since(m, listed, &gone_ids);
    let version = listed
        .version
        .clone()
        .or_else(|| listed.posts.first().and_then(|p| p.version.clone()));
    Status::UpdateAvailable(Update {
        version,
        date_updated: listed.date_updated,
        replaced,
        kept,
        others,
        posts,
    })
}

/// The number N3 reports: mods with replaced files. New files alone don't count (decision 164).
pub fn updates_available(checks: &[ModCheck]) -> usize {
    checks
        .iter()
        .filter(|c| matches!(c.status, Status::UpdateAvailable(_)))
        .count()
}

/// One GameBanana file of ours and the library files extracted from it.
#[derive(Debug)]
struct Download<'a> {
    id: u64,
    archive_name: &'a str,
    files: Vec<String>,
}

/// The mod's library files that are current (no successor) and know their GameBanana file, per GameBanana file,
/// in library order.
fn current_downloads(m: &Mod) -> Vec<Download<'_>> {
    downloads_where(m, |f| f.successor.is_none())
}

/// The mod's GameBanana files that were removed and kept without a replacement (`Successor::None`).
fn retired_downloads(m: &Mod) -> Vec<Download<'_>> {
    downloads_where(m, |f| f.successor == Some(Successor::None))
}

fn downloads_where(m: &Mod, keep: impl Fn(&ModFile) -> bool) -> Vec<Download<'_>> {
    let mut downloads: Vec<Download<'_>> = Vec::new();
    for f in m.files.iter().filter(|f| keep(f)) {
        let Some(id) = f.remote_file_id else { continue };
        match downloads.iter_mut().find(|d| d.id == id) {
            Some(d) => d.files.push(f.id.clone()),
            None => downloads.push(Download {
                id,
                archive_name: &f.archive_name,
                files: vec![f.id.clone()],
            }),
        }
    }
    downloads
}

fn in_library(m: &Mod, remote_id: u64) -> Vec<String> {
    m.files
        .iter()
        .filter(|f| f.remote_file_id == Some(remote_id))
        .map(|f| f.id.clone())
        .collect()
}

/// Listed files that are not in the library and went up clearly after our newest listed file. Without upload dates
/// nothing is called new: a missed notice costs less than a false one.
fn new_files(m: &Mod, listed: &ListedMod, kept: &[Download<'_>], dismissed: &[u64]) -> Status {
    let newest_ours = listed
        .files
        .iter()
        .filter(|f| kept.iter().any(|d| d.id == f.id))
        .filter_map(|f| f.date_added)
        .max();
    let Some(newest_ours) = newest_ours else {
        return Status::UpToDate;
    };
    let files: Vec<RemoteFile> = listed
        .files
        .iter()
        .filter(|f| in_library(m, f.id).is_empty() && !dismissed.contains(&f.id))
        .filter(|f| {
            f.date_added
                .is_some_and(|d| d > newest_ours + SAME_UPLOAD_SECONDS)
        })
        .cloned()
        .collect();
    if files.is_empty() {
        Status::UpToDate
    } else {
        Status::NewFiles { files }
    }
}

/// The one listed file with our archive name: exactly, else ignoring GameBanana's `_xxxxx` suffix. Several equally
/// good names are left to the user. A listed file of ours can only match when it is the replacement downloaded
/// already, since it carries the same name.
fn candidate(m: &Mod, listed: &ListedMod, name: &str) -> Option<Candidate> {
    let same = |a: &str, b: &str| a.to_lowercase() == b.to_lowercase();
    let unique = |hits: Vec<&RemoteFile>| match hits.as_slice() {
        [one] => Some((*one).clone()),
        _ => None,
    };
    let exact: Vec<&RemoteFile> = listed
        .files
        .iter()
        .filter(|f| same(&f.file_name, name))
        .collect();
    let (file, matched_by) = if exact.is_empty() {
        let bare = without_suffix(name);
        let hits = listed
            .files
            .iter()
            .filter(|f| same(&without_suffix(&f.file_name), &bare))
            .collect();
        (unique(hits)?, NameMatch::WithoutSuffix)
    } else {
        (unique(exact)?, NameMatch::Same)
    };
    Some(Candidate {
        in_library: in_library(m, file.id),
        file,
        matched_by,
    })
}

/// `cafevo_ea17d.zip` → `cafevo.zip`: drops a final `_` plus five hex digits before the extension.
fn without_suffix(name: &str) -> String {
    let (stem, ext) = match name.rfind('.') {
        Some(dot) => name.split_at(dot),
        None => (name, ""),
    };
    match stem.rsplit_once('_') {
        Some((base, tail))
            if !base.is_empty()
                && tail.len() == 5
                && tail.chars().all(|c| c.is_ascii_hexdigit()) =>
        {
            format!("{base}{ext}")
        }
        _ => name.to_owned(),
    }
}

/// The posts newer than the newest one that names a file of ours that is gone. Without such a post: the posts that
/// brought listed files we don't have.
fn posts_since(m: &Mod, listed: &ListedMod, gone: &[u64]) -> Vec<UpdatePost> {
    match listed
        .posts
        .iter()
        .position(|p| p.file_ids.iter().any(|id| gone.contains(id)))
    {
        Some(ours) => listed.posts[..ours].to_vec(),
        None => listed
            .posts
            .iter()
            .filter(|p| {
                p.file_ids.iter().any(|&id| {
                    listed.files.iter().any(|f| f.id == id) && in_library(m, id).is_empty()
                })
            })
            .cloned()
            .collect(),
    }
}

/// A proposed pair of library files: `new` holds the newer version of `old`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePair {
    pub old: String,
    pub new: String,
}

/// Library files of a replaced download matched with those of its replacement by their place in the archive.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArchivePairing {
    pub pairs: Vec<FilePair>,
    /// Old files without a partner here; match them by overlap or let the user map them.
    pub old_left: Vec<String>,
    pub new_left: Vec<String>,
}

/// Pairs the files of an old download with those of its same-named replacement: one file each is a pair, otherwise
/// files at the same path inside the archive (ignoring letter case and slash direction) are.
pub fn pair_within_archive(old: &[&ModFile], new: &[&ModFile]) -> ArchivePairing {
    if let ([o], [n]) = (old, new) {
        return ArchivePairing {
            pairs: vec![FilePair {
                old: o.id.clone(),
                new: n.id.clone(),
            }],
            ..ArchivePairing::default()
        };
    }
    let key = |f: &ModFile| f.inner_path.replace('\\', "/").to_lowercase();
    let unique_in = |files: &[&ModFile], k: &str| files.iter().filter(|f| key(f) == k).count() == 1;
    let mut pairing = ArchivePairing::default();
    let mut paired_new: Vec<&str> = Vec::new();
    for o in old {
        let k = key(o);
        let partner = new
            .iter()
            .find(|n| key(n) == k)
            .filter(|_| unique_in(old, &k) && unique_in(new, &k));
        match partner {
            Some(n) => {
                paired_new.push(&n.id);
                pairing.pairs.push(FilePair {
                    old: o.id.clone(),
                    new: n.id.clone(),
                });
            }
            None => pairing.old_left.push(o.id.clone()),
        }
    }
    pairing.new_left = new
        .iter()
        .filter(|n| !paired_new.contains(&n.id.as_str()))
        .map(|n| n.id.clone())
        .collect();
    pairing
}

/// Step 2 of decision 164 for one old library file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlapMatch {
    pub old: String,
    pub outcome: OverlapOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlapOutcome {
    /// One new file changes more of the same game files than any other.
    Match {
        new: String,
        shared_paths: usize,
        /// Of those, files with the very same content (CRC32).
        identical: usize,
    },
    /// Several new files score the same; the user picks (D5).
    Tie {
        candidates: Vec<String>,
        shared_paths: usize,
        identical: usize,
    },
    /// No new file changes any game file the old one does.
    NoOverlap,
}

/// Proposes, for every old file, the downloaded new file that changes the same game files (paths compared as in
/// [`crate::overlap`]): most shared paths first, then most identical files, which tells colour options apart when a
/// texture stayed the same. Several old files may point to one new file.
pub fn match_by_overlap(old: &[FileIndex], new: &[FileIndex]) -> Vec<OverlapMatch> {
    let game_files = |index: &FileIndex| -> HashMap<String, u32> {
        index
            .entries
            .iter()
            .filter_map(|(path, crc)| Some((game_path(path)?, *crc)))
            .collect()
    };
    let new: Vec<(&str, HashMap<String, u32>)> =
        new.iter().map(|n| (n.id.as_str(), game_files(n))).collect();
    old.iter()
        .map(|o| {
            let ours = game_files(o);
            let scores: Vec<(&str, (usize, usize))> = new
                .iter()
                .map(|(id, theirs)| {
                    let shared = ours.keys().filter(|p| theirs.contains_key(*p)).count();
                    let identical = ours
                        .iter()
                        .filter(|(p, crc)| theirs.get(*p) == Some(crc))
                        .count();
                    (*id, (shared, identical))
                })
                .collect();
            let best = scores.iter().map(|(_, s)| *s).max().unwrap_or((0, 0));
            let top: Vec<&str> = scores
                .iter()
                .filter(|(_, s)| *s == best)
                .map(|(id, _)| *id)
                .collect();
            let (shared_paths, identical) = best;
            let outcome = match top.as_slice() {
                _ if shared_paths == 0 => OverlapOutcome::NoOverlap,
                [one] => OverlapOutcome::Match {
                    new: (*one).to_owned(),
                    shared_paths,
                    identical,
                },
                several => OverlapOutcome::Tie {
                    candidates: several.iter().map(|s| (*s).to_owned()).collect(),
                    shared_paths,
                    identical,
                },
            };
            OverlapMatch {
                old: o.id.clone(),
                outcome,
            }
        })
        .collect()
}

/// The user's confirmed choice for one old library file in D5.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    pub old: String,
    /// The library file with the newer version, or `None` to keep the old file as it is and stop reporting it.
    pub new: Option<String>,
}

/// An update the user confirmed (decision 39).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmedUpdate {
    pub mod_id: String,
    /// New library files to add to the mod; empty when they are in the library already.
    pub added: Vec<ModFile>,
    pub mappings: Vec<Mapping>,
    /// The mod's version, given to added files that have none of their own.
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateError {
    UnknownMod(String),
    /// A mapped file is not a file of the mod.
    NotInMod(String),
    MappedTwice(String),
    /// A file mapped to itself.
    SameFile(String),
    Manifest(ModelError),
}

impl std::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownMod(id) => write!(f, "the library has no mod \"{id}\""),
            Self::NotInMod(id) => write!(f, "\"{id}\" is not a file of the updated mod"),
            Self::MappedTwice(id) => write!(f, "\"{id}\" is mapped more than once"),
            Self::SameFile(id) => write!(f, "\"{id}\" is mapped to itself"),
            Self::Manifest(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for UpdateError {}

/// Adds the new files (old ones stay, decision 22), records each old file's successor and makes the new file active
/// wherever the old one was (137). Returns the indexes of the changed profiles (the caller saves them and the
/// manifest). Nothing changes on error.
pub fn apply_update(
    manifest: &mut Manifest,
    profiles: &mut [Profile],
    update: &ConfirmedUpdate,
) -> Result<Vec<usize>, UpdateError> {
    let mut next = manifest.clone();
    let m = next
        .mods
        .iter_mut()
        .find(|m| m.id == update.mod_id)
        .ok_or_else(|| UpdateError::UnknownMod(update.mod_id.clone()))?;
    m.files.extend(update.added.iter().cloned().map(|mut f| {
        if f.version.is_none() {
            f.version = update.version.clone();
        }
        f
    }));
    let mut seen: Vec<&str> = Vec::new();
    for mapping in &update.mappings {
        if seen.contains(&mapping.old.as_str()) {
            return Err(UpdateError::MappedTwice(mapping.old.clone()));
        }
        seen.push(&mapping.old);
        if let Some(new) = &mapping.new {
            if !m.files.iter().any(|f| &f.id == new) {
                return Err(UpdateError::NotInMod(new.clone()));
            }
            if new == &mapping.old {
                return Err(UpdateError::SameFile(new.clone()));
            }
        }
        let old = m
            .files
            .iter_mut()
            .find(|f| f.id == mapping.old)
            .ok_or_else(|| UpdateError::NotInMod(mapping.old.clone()))?;
        old.successor = Some(match &mapping.new {
            Some(new) => Successor::File(new.clone()),
            None => Successor::None,
        });
    }
    next.validate().map_err(UpdateError::Manifest)?;

    let mut changed: Vec<usize> = Vec::new();
    for mapping in &update.mappings {
        if let Some(new) = &mapping.new {
            changed.extend(selection::replace_file(
                profiles,
                &update.mod_id,
                &mapping.old,
                new,
            ));
        }
    }
    changed.sort_unstable();
    changed.dedup();
    for &i in &changed {
        for entry in profiles[i]
            .entries
            .iter_mut()
            .filter(|e| e.mod_id == update.mod_id)
        {
            drop_repeats(entry);
        }
    }
    *manifest = next;
    Ok(changed)
}

/// Two old files replaced by one new file leave it once: in the first place it appears.
fn drop_repeats(entry: &mut ProfileEntry) {
    let mut seen: Vec<String> = Vec::new();
    for list in [&mut entry.variants, &mut entry.add_ons] {
        list.retain(|f| {
            if seen.contains(f) {
                false
            } else {
                seen.push(f.clone());
                true
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{Mod, ModFile, Section, Source};

    // Ids, names and dates from the recorded GameBanana answers for Manhattan Cafe (638388) and Sabrina (633086),
    // unless a comment says otherwise.

    /// The shape the UI reads (`ui/src/lib/api.ts`).
    #[test]
    fn results_are_tagged_for_the_ui() {
        let removed = ModCheck {
            mod_id: "gb-1".to_owned(),
            status: Status::Removed {
                reason: RemovedReason::Withheld,
            },
        };
        assert_eq!(
            serde_json::to_value(&removed).expect("json"),
            serde_json::json!({"mod_id": "gb-1", "status": {"kind": "removed", "reason": "withheld"}})
        );
        let update = Status::UpdateAvailable(Update {
            version: Some("v5.3".to_owned()),
            date_updated: None,
            replaced: Vec::new(),
            kept: vec![7],
            others: Vec::new(),
            posts: Vec::new(),
        });
        let json = serde_json::to_value(&update).expect("json");
        assert_eq!(json["kind"], "update_available");
        assert_eq!(json["version"], "v5.3");
        assert_eq!(json["kept"], serde_json::json!([7]));
    }

    fn lib_file(id: &str, remote: Option<u64>, archive: &str) -> ModFile {
        ModFile {
            id: id.to_owned(),
            remote_file_id: remote,
            archive_name: archive.to_owned(),
            inner_path: "pak01_dir.vpk".to_owned(),
            library_path: format!("library/m/{id}.vpk"),
            len: 1,
            sha256: [0; 32],
            version: None,
            successor: None,
        }
    }

    fn gb_mod(mod_id: u64, files: Vec<ModFile>) -> Mod {
        Mod {
            id: format!("gb-{mod_id}"),
            name: "A mod".to_owned(),
            author: None,
            source: Source::GameBanana {
                section: Section::Mod,
                mod_id,
            },
            files,
        }
    }

    fn remote(id: u64, name: &str, date: i64) -> RemoteFile {
        RemoteFile {
            id,
            file_name: name.to_owned(),
            size_bytes: 1000,
            date_added: Some(date),
            version: None,
            description: None,
        }
    }

    fn post(id: u64, version: &str, date: i64, file_ids: &[u64]) -> UpdatePost {
        UpdatePost {
            id,
            title: format!("post {version}"),
            version: Some(version.to_owned()),
            date_added: Some(date),
            changelog: Vec::new(),
            file_ids: file_ids.to_vec(),
            text_html: String::new(),
        }
    }

    fn listed(version: &str, files: Vec<RemoteFile>, posts: Vec<UpdatePost>) -> RemoteMod {
        RemoteMod::Listed(ListedMod {
            version: Some(version.to_owned()),
            date_updated: Some(1_792_007_098),
            files,
            posts,
        })
    }

    #[test]
    fn hidden_new_files_stay_hidden_until_another_one_appears() {
        let ours = 1_000;
        let m = gb_mod(9, vec![lib_file("gb-9-1-0", Some(11), "base.zip")]);
        let answer = listed(
            "1.0",
            vec![
                remote(11, "base.zip", ours),
                remote(12, "pink.zip", ours + 7_200),
                remote(13, "blue.zip", ours + 7_200),
            ],
            Vec::new(),
        );
        // 168: the user hid the pink one.
        let status = check_mod_with(&m, Some(&answer), &[12]);
        let Status::NewFiles { files } = status else {
            panic!("{status:?}");
        };
        assert_eq!(files.iter().map(|f| f.id).collect::<Vec<_>>(), [13]);
        assert_eq!(
            check_mod_with(&m, Some(&answer), &[12, 13]),
            Status::UpToDate
        );
    }

    #[test]
    fn the_manifest_remembers_what_was_hidden() {
        let mut manifest = Manifest {
            mods: vec![gb_mod(9, vec![lib_file("gb-9-1-0", Some(11), "base.zip")])],
            ..Manifest::default()
        };
        manifest
            .dismissed_new_files
            .insert("gb-9".to_owned(), vec![12]);
        let answer = listed(
            "1.0",
            vec![remote(11, "base.zip", 1_000), remote(12, "pink.zip", 9_000)],
            Vec::new(),
        );
        let checks = check(&manifest, |_, _| Some(&answer));
        assert_eq!(checks[0].status, Status::UpToDate);
    }

    #[test]
    fn a_file_kept_without_replacement_is_offered_again_when_its_name_returns() {
        // 172: the author removed b.zip, the user kept the old one (no successor), later a new b.zip appears.
        let mut retired = lib_file("gb-9-1-0", Some(20), "b.zip");
        retired.successor = Some(Successor::None);
        let m = gb_mod(9, vec![retired]);
        let back = listed("2.0", vec![remote(21, "b.zip", 5_000)], Vec::new());
        let Status::UpdateAvailable(update) = check_mod(&m, Some(&back)) else {
            panic!("an update");
        };
        assert_eq!(update.replaced[0].remote_file_id, 20);
        assert_eq!(
            update.replaced[0].candidate.as_ref().map(|c| c.file.id),
            Some(21)
        );
        // Still gone: nothing to report.
        let other = listed("2.0", vec![remote(22, "c.zip", 5_000)], Vec::new());
        assert_eq!(check_mod(&m, Some(&other)), Status::Untracked);
    }

    /// Manhattan Cafe after v5.3: every file re-uploaded, four "new suitcase" variants added.
    fn manhattan_v5_3() -> Vec<RemoteFile> {
        vec![
            remote(1_848_526, "cafevo_ea17d.zip", 1_792_006_837),
            remote(1_848_520, "variant1.zip", 1_792_006_690),
            remote(1_848_515, "variant1newsuitcase.zip", 1_792_006_663),
            remote(1_848_521, "variant2.zip", 1_792_006_693),
            remote(1_848_516, "variant2newsuitcase.zip", 1_792_006_664),
            remote(1_848_523, "variant3.zip", 1_792_006_693),
            remote(1_848_517, "variant3newsuitcase.zip", 1_792_006_664),
            remote(1_848_522, "variant4.zip", 1_792_006_693),
            remote(1_848_518, "variant4newsuitcase.zip", 1_792_006_666),
        ]
    }

    /// The posts' file ids are recorded; which old id carried which name is assumed (names looked stable).
    fn manhattan_posts() -> Vec<UpdatePost> {
        vec![
            post(
                464_201,
                "v5.3",
                1_792_007_098,
                &[
                    1_848_526, 1_848_520, 1_848_515, 1_848_521, 1_848_516, 1_848_523, 1_848_517,
                    1_848_522, 1_848_518,
                ],
            ),
            post(
                461_463,
                "v5.2",
                1_791_209_471,
                &[
                    1_838_412, 1_838_410, 1_838_413, 1_838_411, 1_838_417, 1_838_415, 1_838_418,
                    1_838_416,
                ],
            ),
            post(
                459_775,
                "v5.1",
                1_790_764_088,
                &[
                    1_831_897, 1_831_901, 1_831_896, 1_831_898, 1_831_894, 1_831_900, 1_831_895,
                    1_831_899,
                ],
            ),
        ]
    }

    fn update(status: Status) -> Update {
        match status {
            Status::UpdateAvailable(u) => u,
            other => panic!("expected an update, got {other:?}"),
        }
    }

    fn candidate_id(r: &Replaced) -> Option<u64> {
        r.candidate.as_ref().map(|c| c.file.id)
    }

    #[test]
    fn files_still_listed_are_up_to_date() {
        let m = gb_mod(
            638_388,
            vec![lib_file("v1", Some(1_848_520), "variant1.zip")],
        );
        let answer = listed("v5.3", manhattan_v5_3(), manhattan_posts());
        assert_eq!(check_mod(&m, Some(&answer)), Status::UpToDate);
    }

    #[test]
    fn a_re_uploaded_file_is_replaced_by_the_one_with_its_name() {
        // Installed at v5.2: variant1.zip was 1838410. v5.3 re-uploaded everything.
        let m = gb_mod(
            638_388,
            vec![lib_file("v1", Some(1_838_410), "variant1.zip")],
        );
        let answer = listed("v5.3", manhattan_v5_3(), manhattan_posts());
        let u = update(check_mod(&m, Some(&answer)));
        assert_eq!(u.version.as_deref(), Some("v5.3"));
        assert_eq!(u.date_updated, Some(1_792_007_098));
        assert_eq!(u.replaced.len(), 1);
        let r = &u.replaced[0];
        assert_eq!(r.remote_file_id, 1_838_410);
        assert_eq!(r.archive_name, "variant1.zip");
        assert_eq!(r.files, ["v1"]);
        let c = r.candidate.as_ref().expect("candidate");
        assert_eq!(c.file.id, 1_848_520);
        assert_eq!(c.matched_by, NameMatch::Same);
        assert!(c.in_library.is_empty());
        assert!(u.kept.is_empty());
        // Everything else is on offer for mapping, the added suitcase variants included.
        let others: Vec<u64> = u.others.iter().map(|f| f.id).collect();
        assert_eq!(
            others,
            [
                1_848_526, 1_848_515, 1_848_521, 1_848_516, 1_848_523, 1_848_517, 1_848_522,
                1_848_518
            ]
        );
        // The changelog covers the posts after the one our file came with.
        let posts: Vec<u64> = u.posts.iter().map(|p| p.id).collect();
        assert_eq!(posts, [464_201]);
    }

    #[test]
    fn an_update_skipped_over_shows_every_post_since() {
        // Installed at v5.1 (1831897 assumed to be variant1.zip); v5.2 and v5.3 came since.
        let m = gb_mod(
            638_388,
            vec![lib_file("v1", Some(1_831_897), "variant1.zip")],
        );
        let answer = listed("v5.3", manhattan_v5_3(), manhattan_posts());
        let u = update(check_mod(&m, Some(&answer)));
        let posts: Vec<u64> = u.posts.iter().map(|p| p.id).collect();
        assert_eq!(posts, [464_201, 461_463]);
    }

    #[test]
    fn without_a_post_naming_our_file_the_posts_of_the_current_files_are_shown() {
        // Our file predates the fetched page of posts.
        let m = gb_mod(638_388, vec![lib_file("v1", Some(1_000), "variant1.zip")]);
        let answer = listed("v5.3", manhattan_v5_3(), manhattan_posts());
        let u = update(check_mod(&m, Some(&answer)));
        let posts: Vec<u64> = u.posts.iter().map(|p| p.id).collect();
        assert_eq!(posts, [464_201]);
        // And without any posts there is no changelog, only the version.
        let answer = listed("v5.3", manhattan_v5_3(), Vec::new());
        let u = update(check_mod(&m, Some(&answer)));
        assert!(u.posts.is_empty());
        assert_eq!(u.version.as_deref(), Some("v5.3"));
    }

    #[test]
    fn the_version_falls_back_to_the_newest_post() {
        let m = gb_mod(
            638_388,
            vec![lib_file("v1", Some(1_838_410), "variant1.zip")],
        );
        let answer = RemoteMod::Listed(ListedMod {
            version: None,
            date_updated: None,
            files: manhattan_v5_3(),
            posts: manhattan_posts(),
        });
        let u = update(check_mod(&m, Some(&answer)));
        assert_eq!(u.version.as_deref(), Some("v5.3"));
    }

    #[test]
    fn many_colour_files_re_uploaded_under_the_same_names() {
        // Sabrina 2.0 lists 20 colour/pose files; three are installed. A later re-upload (new ids made up here)
        // keeps every name.
        let names = [
            (1_848_720, "sabrina_-_green_-_house_tour.zip"),
            (1_848_725, "sabrina_-_pink_-_house_tour.zip"),
            (1_848_739, "sabrina_-_pink_-_busy_woman.zip"),
            (1_848_721, "sabrina_-_purple_-_house_tour.zip"),
            (1_848_731, "sabrina_-_purple_-_juno.zip"),
        ];
        let m = gb_mod(
            633_086,
            names[..3]
                .iter()
                .map(|&(id, name)| lib_file(&format!("f{id}"), Some(id), name))
                .collect(),
        );
        let reuploaded: Vec<RemoteFile> = names
            .iter()
            .map(|&(id, name)| remote(id + 100_000, name, 1_800_000_000))
            .collect();
        let u = update(check_mod(&m, Some(&listed("2.1", reuploaded, Vec::new()))));
        let pairs: Vec<(u64, Option<u64>)> = u
            .replaced
            .iter()
            .map(|r| (r.remote_file_id, candidate_id(r)))
            .collect();
        assert_eq!(
            pairs,
            [
                (1_848_720, Some(1_948_720)),
                (1_848_725, Some(1_948_725)),
                (1_848_739, Some(1_948_739)),
            ]
        );
        let others: Vec<u64> = u.others.iter().map(|f| f.id).collect();
        assert_eq!(others, [1_948_721, 1_948_731]);
    }

    #[test]
    fn renamed_files_have_no_candidate() {
        // Sabrina 2.0 replaced the three v1.03 files (1559864–1559866) by twenty under new names. The old name is
        // made up; it matches none of the new ones.
        let m = gb_mod(
            633_086,
            vec![lib_file("old", Some(1_559_865), "sabrina_green.zip")],
        );
        let files: Vec<RemoteFile> = (0..20)
            .map(|i| remote(1_848_720 + i, &format!("sabrina_-_{i}.zip"), 1_792_019_690))
            .collect();
        let posts = vec![
            post(
                464_258,
                "2.0",
                1_792_020_022,
                &(1_848_720..1_848_740).collect::<Vec<_>>(),
            ),
            post(
                377_656,
                "v1.03",
                1_762_970_889,
                &[1_559_864, 1_559_865, 1_559_866],
            ),
            post(377_644, "v1.02", 1_762_965_377, &[1_559_370]),
        ];
        let u = update(check_mod(&m, Some(&listed("2.0", files, posts))));
        assert_eq!(u.replaced.len(), 1);
        assert_eq!(u.replaced[0].candidate, None);
        assert_eq!(u.others.len(), 20, "all of them can be mapped to");
        let posts: Vec<u64> = u.posts.iter().map(|p| p.id).collect();
        assert_eq!(posts, [464_258]);
    }

    #[test]
    fn a_variant_added_in_a_post_of_its_own_is_new_files_not_an_update() {
        // Manhattan Cafe v4.1 "New Variant" added two files (1720560, 1720561) days after v4; the v4 files stayed.
        let m = gb_mod(
            638_388,
            vec![
                lib_file("v1", Some(1_712_621), "variant1.zip"),
                lib_file("v2", Some(1_712_622), "variant2.zip"),
            ],
        );
        let v4 = 1_779_888_000;
        let v4_1 = 1_780_659_000;
        let files = vec![
            remote(1_712_621, "variant1.zip", v4),
            remote(1_712_622, "variant2.zip", v4 + 30),
            remote(1_712_623, "variant3.zip", v4 + 31),
            remote(1_720_560, "cafevo.zip", v4_1),
            remote(1_720_561, "variant4.zip", v4_1 + 2),
        ];
        let status = check_mod(&m, Some(&listed("v4.1", files, Vec::new())));
        let Status::NewFiles { files } = status else {
            panic!("expected new files, got {status:?}");
        };
        let ids: Vec<u64> = files.iter().map(|f| f.id).collect();
        assert_eq!(ids, [1_720_560, 1_720_561], "variant3 came with ours");
    }

    #[test]
    fn files_uploaded_with_ours_are_not_new() {
        // Sabrina 2.0's twenty files went up within seconds; one is installed.
        let m = gb_mod(633_086, vec![lib_file("g", Some(1_848_725), "pink.zip")]);
        let files: Vec<RemoteFile> = (0..20)
            .map(|i| {
                remote(
                    1_848_720 + i,
                    &format!("f{i}.zip"),
                    1_792_019_689 + (i % 4) as i64,
                )
            })
            .collect();
        assert_eq!(
            check_mod(&m, Some(&listed("2.0", files, Vec::new()))),
            Status::UpToDate
        );
    }

    #[test]
    fn without_upload_dates_nothing_is_called_new() {
        let m = gb_mod(1, vec![lib_file("a", Some(10), "a.zip")]);
        let mut ours = remote(10, "a.zip", 0);
        ours.date_added = None;
        let files = vec![ours, remote(11, "b.zip", 1_800_000_000)];
        assert_eq!(
            check_mod(&m, Some(&listed("1", files, Vec::new()))),
            Status::UpToDate
        );
    }

    #[test]
    fn a_file_deleted_without_replacement() {
        let m = gb_mod(
            638_388,
            vec![
                lib_file("v1", Some(1_848_520), "variant1.zip"),
                lib_file("old", Some(1_700_000), "variant_old.zip"),
            ],
        );
        let u = update(check_mod(
            &m,
            Some(&listed("v5.3", manhattan_v5_3(), Vec::new())),
        ));
        assert_eq!(u.replaced.len(), 1);
        assert_eq!(u.replaced[0].remote_file_id, 1_700_000);
        assert_eq!(u.replaced[0].candidate, None);
        assert_eq!(u.kept, [1_848_520]);
        assert_eq!(u.others.len(), 8, "every listed file but ours");
    }

    #[test]
    fn several_installed_variants_are_replaced_one_by_one() {
        let m = gb_mod(
            638_388,
            vec![
                lib_file("v1", Some(1_838_410), "variant1.zip"),
                lib_file("v2", Some(1_838_411), "Variant2.ZIP"),
                lib_file("v4", Some(1_838_416), "variant4.zip"),
            ],
        );
        let u = update(check_mod(
            &m,
            Some(&listed("v5.3", manhattan_v5_3(), Vec::new())),
        ));
        let pairs: Vec<(u64, Option<u64>)> = u
            .replaced
            .iter()
            .map(|r| (r.remote_file_id, candidate_id(r)))
            .collect();
        assert_eq!(
            pairs,
            [
                (1_838_410, Some(1_848_520)),
                (1_838_411, Some(1_848_521)),
                (1_838_416, Some(1_848_522)),
            ],
            "names compare without letter case"
        );
    }

    #[test]
    fn several_library_files_from_one_download_are_replaced_together() {
        let m = gb_mod(
            638_388,
            vec![
                lib_file("a", Some(1_838_410), "variant1.zip"),
                lib_file("b", Some(1_838_410), "variant1.zip"),
            ],
        );
        let u = update(check_mod(
            &m,
            Some(&listed("v5.3", manhattan_v5_3(), Vec::new())),
        ));
        assert_eq!(u.replaced.len(), 1);
        assert_eq!(u.replaced[0].files, ["a", "b"]);
    }

    #[test]
    fn gamebanana_s_name_suffix_is_ignored_when_nothing_matches_exactly() {
        // v5.3 lists the voice pack as cafevo_ea17d.zip.
        let m = gb_mod(638_388, vec![lib_file("vo", Some(1_838_412), "cafevo.zip")]);
        let u = update(check_mod(
            &m,
            Some(&listed("v5.3", manhattan_v5_3(), Vec::new())),
        ));
        let c = u.replaced[0].candidate.as_ref().expect("candidate");
        assert_eq!(c.file.id, 1_848_526);
        assert_eq!(c.matched_by, NameMatch::WithoutSuffix);

        // Our own name may carry the suffix too.
        let m = gb_mod(
            638_388,
            vec![lib_file("vo", Some(1_838_412), "cafevo_0a1b2.zip")],
        );
        let u = update(check_mod(
            &m,
            Some(&listed("v5.3", manhattan_v5_3(), Vec::new())),
        ));
        assert_eq!(candidate_id(&u.replaced[0]), Some(1_848_526));
    }

    #[test]
    fn an_exact_name_beats_a_suffixed_one() {
        let m = gb_mod(1, vec![lib_file("a", Some(1), "skin.zip")]);
        let files = vec![remote(2, "skin_abcde.zip", 5), remote(3, "skin.zip", 5)];
        let u = update(check_mod(&m, Some(&listed("2", files, Vec::new()))));
        let c = u.replaced[0].candidate.as_ref().expect("candidate");
        assert_eq!((c.file.id, c.matched_by), (3, NameMatch::Same));
    }

    #[test]
    fn two_listed_files_with_our_name_leave_the_choice_to_the_user() {
        let m = gb_mod(1, vec![lib_file("a", Some(1), "skin.zip")]);
        let files = vec![remote(2, "skin.zip", 5), remote(3, "SKIN.zip", 5)];
        let u = update(check_mod(&m, Some(&listed("2", files, Vec::new()))));
        assert_eq!(u.replaced[0].candidate, None);
        assert_eq!(u.others.len(), 2);
    }

    #[test]
    fn a_candidate_already_downloaded_is_marked() {
        // "Download updates automatically" fetched the new file; the switch still waits for confirmation (39).
        let m = gb_mod(
            638_388,
            vec![
                lib_file("old", Some(1_838_410), "variant1.zip"),
                lib_file("new", Some(1_848_520), "variant1.zip"),
            ],
        );
        let u = update(check_mod(
            &m,
            Some(&listed("v5.3", manhattan_v5_3(), Vec::new())),
        ));
        let c = u.replaced[0].candidate.as_ref().expect("candidate");
        assert_eq!(c.in_library, ["new"]);
        assert!(
            u.kept.is_empty(),
            "the downloaded candidate is not one of our kept files"
        );
        assert!(!u.others.iter().any(|f| f.id == 1_848_520));
    }

    #[test]
    fn replaced_files_with_a_successor_are_not_checked_again() {
        let mut old = lib_file("old", Some(1_838_410), "variant1.zip");
        old.successor = Some(Successor::File("new".to_owned()));
        let m = gb_mod(
            638_388,
            vec![old, lib_file("new", Some(1_848_520), "variant1.zip")],
        );
        let answer = listed("v5.3", manhattan_v5_3(), Vec::new());
        assert_eq!(check_mod(&m, Some(&answer)), Status::UpToDate);

        // Nor files the user kept although GameBanana dropped them.
        let mut kept = lib_file("kept", Some(1_700_000), "variant_old.zip");
        kept.successor = Some(Successor::None);
        let m = gb_mod(
            638_388,
            vec![kept, lib_file("new", Some(1_848_520), "variant1.zip")],
        );
        assert_eq!(check_mod(&m, Some(&answer)), Status::UpToDate);
    }

    #[test]
    fn a_mod_gone_from_gamebanana() {
        let m = gb_mod(1, vec![lib_file("a", Some(1), "a.zip")]);
        for reason in [
            RemovedReason::Deleted,
            RemovedReason::Trashed,
            RemovedReason::Withheld,
            RemovedReason::Private,
        ] {
            assert_eq!(
                check_mod(&m, Some(&RemoteMod::Removed(reason))),
                Status::Removed { reason }
            );
        }
        // A page that lists no files can't be updated from either.
        assert_eq!(
            check_mod(&m, Some(&listed("2", Vec::new(), Vec::new()))),
            Status::Removed {
                reason: RemovedReason::NoFiles
            }
        );
    }

    #[test]
    fn a_local_mod_is_never_checked() {
        let mut m = gb_mod(1, vec![lib_file("a", None, "a.zip")]);
        m.source = Source::Local {
            imported_from: Some(r"D:\mods\a.zip".to_owned()),
        };
        let answer = listed("1", vec![remote(1, "a.zip", 1)], Vec::new());
        assert_eq!(check_mod(&m, Some(&answer)), Status::Local);
        assert_eq!(check_mod(&m, None), Status::Local);
    }

    #[test]
    fn a_mod_without_an_answer_is_not_checked() {
        let m = gb_mod(1, vec![lib_file("a", Some(1), "a.zip")]);
        assert_eq!(check_mod(&m, None), Status::NotChecked);
    }

    #[test]
    fn files_without_gamebanana_ids_are_untracked() {
        // E.g. imported from disk, then linked to its GameBanana page.
        let m = gb_mod(1, vec![lib_file("a", None, "a.zip")]);
        let answer = listed("1", vec![remote(5, "a.zip", 1)], Vec::new());
        assert_eq!(check_mod(&m, Some(&answer)), Status::Untracked);
        // Untracked files next to tracked ones are left out of the comparison.
        let m = gb_mod(
            1,
            vec![
                lib_file("a", None, "a.zip"),
                lib_file("b", Some(5), "a.zip"),
            ],
        );
        assert_eq!(check_mod(&m, Some(&answer)), Status::UpToDate);
    }

    fn index(id: &str, entries: &[(&str, u32)]) -> FileIndex {
        FileIndex {
            id: id.to_owned(),
            entries: entries.iter().map(|&(p, c)| (p.to_owned(), c)).collect(),
        }
    }

    const BODY: &str = "models/heroes/pocket/pocket.vmdl_c";
    const COLOUR: &str = "materials/heroes/pocket/coat_color.vtex_c";
    const CASE: &str = "models/heroes/pocket/suitcase.vmdl_c";

    #[test]
    fn an_unchanged_texture_tells_colour_options_apart() {
        // Every new colour changes the same paths; the green texture stayed as it was.
        let old = [index("green", &[(BODY, 1), (COLOUR, 10)])];
        let new = [
            index("red2", &[(BODY, 2), (COLOUR, 20)]),
            index("green2", &[(BODY, 2), (COLOUR, 10)]),
        ];
        assert_eq!(
            match_by_overlap(&old, &new),
            [OverlapMatch {
                old: "green".into(),
                outcome: OverlapOutcome::Match {
                    new: "green2".into(),
                    shared_paths: 2,
                    identical: 1
                },
            }]
        );
    }

    #[test]
    fn colour_options_that_all_changed_are_a_tie() {
        let old = [index("green", &[(BODY, 1), (COLOUR, 10)])];
        let new = [
            index("red2", &[(BODY, 2), (COLOUR, 20)]),
            index("green2", &[(BODY, 2), (COLOUR, 11)]),
            index("vo", &[("sounds/vo/pocket/hello.vsnd_c", 5)]),
        ];
        assert_eq!(
            match_by_overlap(&old, &new)[0].outcome,
            OverlapOutcome::Tie {
                candidates: vec!["red2".into(), "green2".into()],
                shared_paths: 2,
                identical: 0
            }
        );
    }

    #[test]
    fn more_shared_game_files_win() {
        // Manhattan Cafe v5.3: variant1 and variant1newsuitcase share the body; only variant1 keeps the old case.
        let old = [index("variant1", &[(BODY, 1), (CASE, 3), (COLOUR, 4)])];
        let new = [
            index(
                "variant1newsuitcase",
                &[(BODY, 2), ("models/heroes/pocket/suitcase_new.vmdl_c", 6)],
            ),
            index("variant1", &[(BODY, 2), (CASE, 7), (COLOUR, 8)]),
        ];
        assert_eq!(
            match_by_overlap(&old, &new)[0].outcome,
            OverlapOutcome::Match {
                new: "variant1".into(),
                shared_paths: 3,
                identical: 0
            }
        );
    }

    #[test]
    fn no_shared_game_file_is_no_overlap() {
        // Files in the package root (README.txt) don't count, as in conflicts and variant groups.
        let old = [index("vo", &[("README.txt", 1), ("sounds/vo/a.vsnd_c", 2)])];
        let new = [index("skin", &[("README.txt", 1), (BODY, 3)])];
        assert_eq!(
            match_by_overlap(&old, &new),
            [OverlapMatch {
                old: "vo".into(),
                outcome: OverlapOutcome::NoOverlap
            }]
        );
        assert_eq!(
            match_by_overlap(&old, &[])[0].outcome,
            OverlapOutcome::NoOverlap
        );
    }

    #[test]
    fn overlap_paths_compare_without_case_and_several_old_files_may_share_a_match() {
        let old = [
            index("a", &[("Models/Heroes/Haze/Body.vmdl_c", 1)]),
            index("b", &[("models/heroes/haze/body.vmdl_c", 1)]),
        ];
        let new = [index("n", &[("models/heroes/haze/body.vmdl_c", 2)])];
        let matches = match_by_overlap(&old, &new);
        let olds: Vec<(&str, bool)> = matches
            .iter()
            .map(|m| {
                let hit = matches!(&m.outcome, OverlapOutcome::Match { new, .. } if new == "n");
                (m.old.as_str(), hit)
            })
            .collect();
        assert_eq!(olds, [("a", true), ("b", true)]);
    }

    fn inner(id: &str, path: &str) -> ModFile {
        let mut f = lib_file(id, Some(1), "variant1.zip");
        f.inner_path = path.to_owned();
        f
    }

    fn refs(files: &[ModFile]) -> Vec<&ModFile> {
        files.iter().collect()
    }

    #[test]
    fn one_file_on_each_side_is_a_pair() {
        let old = [inner("o", "pak01_dir.vpk")];
        let new = [inner("n", "other/pak01_dir.vpk")];
        assert_eq!(
            pair_within_archive(&refs(&old), &refs(&new)),
            ArchivePairing {
                pairs: vec![FilePair {
                    old: "o".into(),
                    new: "n".into()
                }],
                old_left: vec![],
                new_left: vec![],
            }
        );
    }

    #[test]
    fn several_files_pair_by_their_path_in_the_archive() {
        let old = [
            inner("o1", "Variant 1/pak01_dir.vpk"),
            inner("o2", r"Variant 2\pak01_dir.vpk"),
            inner("o3", "Old/pak01_dir.vpk"),
        ];
        let new = [
            inner("n2", "variant 2/pak01_dir.vpk"),
            inner("n1", "Variant 1/pak01_dir.vpk"),
            inner("n3", "Variant 3/pak01_dir.vpk"),
        ];
        let p = pair_within_archive(&refs(&old), &refs(&new));
        assert_eq!(
            p.pairs,
            [
                FilePair {
                    old: "o1".into(),
                    new: "n1".into()
                },
                FilePair {
                    old: "o2".into(),
                    new: "n2".into()
                },
            ]
        );
        assert_eq!(p.old_left, ["o3"]);
        assert_eq!(p.new_left, ["n3"]);
    }

    #[test]
    fn a_path_used_twice_on_one_side_pairs_nothing_there() {
        let old = [
            inner("o1", "a/pak01_dir.vpk"),
            inner("o2", "b/pak01_dir.vpk"),
        ];
        let new = [
            inner("n1", "a/pak01_dir.vpk"),
            inner("n2", "A/pak01_dir.vpk"),
        ];
        let p = pair_within_archive(&refs(&old), &refs(&new));
        assert!(p.pairs.is_empty());
        assert_eq!(p.old_left, ["o1", "o2"]);
        assert_eq!(p.new_left, ["n1", "n2"]);
    }

    fn entry(mod_id: &str, variants: &[&str], add_ons: &[&str]) -> ProfileEntry {
        ProfileEntry {
            mod_id: mod_id.into(),
            variants: variants.iter().map(|s| s.to_string()).collect(),
            add_ons: add_ons.iter().map(|s| s.to_string()).collect(),
            enabled: true,
        }
    }

    fn profile(entries: Vec<ProfileEntry>) -> Profile {
        Profile {
            name: "Main".into(),
            game_root: None,
            entries,
        }
    }

    fn library() -> Manifest {
        Manifest {
            mods: vec![
                gb_mod(
                    638_388,
                    vec![
                        lib_file("v1", Some(1_838_410), "variant1.zip"),
                        lib_file("v2", Some(1_838_411), "variant2.zip"),
                        lib_file("vo", Some(1_838_412), "cafevo.zip"),
                    ],
                ),
                gb_mod(1, vec![lib_file("x", Some(5), "x.zip")]),
            ],
            ..Manifest::default()
        }
    }

    fn map(old: &str, new: Option<&str>) -> Mapping {
        Mapping {
            old: old.into(),
            new: new.map(str::to_owned),
        }
    }

    fn confirmed(added: Vec<ModFile>, mappings: Vec<Mapping>) -> ConfirmedUpdate {
        ConfirmedUpdate {
            mod_id: "gb-638388".into(),
            added,
            mappings,
            version: Some("v5.3".into()),
        }
    }

    fn file_of<'a>(m: &'a Manifest, id: &str) -> &'a ModFile {
        m.file(id).expect("in the library").1
    }

    #[test]
    fn applying_an_update_adds_the_new_file_and_moves_every_profile_to_it() {
        let mut manifest = library();
        let mut profiles = [
            profile(vec![
                entry("gb-1", &[], &["x"]),
                entry("gb-638388", &["v1"], &["vo"]),
            ]),
            profile(vec![entry("gb-1", &[], &["x"])]),
            profile(vec![entry("gb-638388", &[], &["v1"])]),
        ];
        let mut own = lib_file("n4", Some(1_848_522), "variant4.zip");
        own.version = Some("v2.3".into());
        let update = confirmed(
            vec![lib_file("n1", Some(1_848_520), "variant1.zip"), own],
            vec![map("v1", Some("n1"))],
        );
        let changed = apply_update(&mut manifest, &mut profiles, &update).expect("applies");
        assert_eq!(changed, [0, 2]);
        assert_eq!(profiles[0].entries[1], entry("gb-638388", &["n1"], &["vo"]));
        assert_eq!(profiles[1].entries[0], entry("gb-1", &[], &["x"]));
        assert_eq!(profiles[2].entries[0], entry("gb-638388", &[], &["n1"]));

        let ids: Vec<&str> = manifest.mods[0]
            .files
            .iter()
            .map(|f| f.id.as_str())
            .collect();
        assert_eq!(
            ids,
            ["v1", "v2", "vo", "n1", "n4"],
            "the old version stays (22)"
        );
        assert_eq!(
            file_of(&manifest, "v1").successor,
            Some(Successor::File("n1".into()))
        );
        assert_eq!(file_of(&manifest, "v2").successor, None);
        assert_eq!(file_of(&manifest, "n1").version.as_deref(), Some("v5.3"));
        assert_eq!(
            file_of(&manifest, "n4").version.as_deref(),
            Some("v2.3"),
            "its own version"
        );
    }

    #[test]
    fn after_applying_the_mod_is_up_to_date() {
        let mut manifest = library();
        let update = confirmed(
            vec![
                lib_file("n1", Some(1_848_520), "variant1.zip"),
                lib_file("n2", Some(1_848_521), "variant2.zip"),
                lib_file("nvo", Some(1_848_526), "cafevo_ea17d.zip"),
            ],
            vec![
                map("v1", Some("n1")),
                map("v2", Some("n2")),
                map("vo", Some("nvo")),
            ],
        );
        apply_update(&mut manifest, &mut [], &update).expect("applies");
        let answer = listed("v5.3", manhattan_v5_3(), Vec::new());
        assert_eq!(
            check_mod(&manifest.mods[0], Some(&answer)),
            Status::UpToDate
        );
    }

    #[test]
    fn a_file_kept_without_successor_stays_active() {
        let mut manifest = library();
        let mut profiles = [profile(vec![entry("gb-638388", &["v1"], &[])])];
        let update = confirmed(vec![], vec![map("v1", None)]);
        let changed = apply_update(&mut manifest, &mut profiles, &update).expect("applies");
        assert!(changed.is_empty());
        assert_eq!(profiles[0].entries[0], entry("gb-638388", &["v1"], &[]));
        assert_eq!(file_of(&manifest, "v1").successor, Some(Successor::None));
    }

    #[test]
    fn a_new_file_already_in_the_library_is_used_as_it_is() {
        let mut manifest = library();
        manifest.mods[0]
            .files
            .push(lib_file("n1", Some(1_848_520), "variant1.zip"));
        let mut profiles = [profile(vec![entry("gb-638388", &["v1"], &[])])];
        let update = confirmed(vec![], vec![map("v1", Some("n1"))]);
        let changed = apply_update(&mut manifest, &mut profiles, &update).expect("applies");
        assert_eq!(changed, [0]);
        assert_eq!(file_of(&manifest, "n1").version, None, "not an added file");
        assert_eq!(manifest.mods[0].files.len(), 4);
    }

    #[test]
    fn two_old_files_merged_into_one_leave_it_listed_once() {
        let mut manifest = library();
        let mut profiles = [profile(vec![entry("gb-638388", &["v1"], &["v2"])])];
        let update = confirmed(
            vec![lib_file("all", Some(1_900_000), "all.zip")],
            vec![map("v1", Some("all")), map("v2", Some("all"))],
        );
        apply_update(&mut manifest, &mut profiles, &update).expect("applies");
        assert_eq!(profiles[0].entries[0], entry("gb-638388", &["all"], &[]));
    }

    #[test]
    fn a_wrong_mapping_changes_nothing() {
        let cases = [
            (
                ConfirmedUpdate {
                    mod_id: "missing".into(),
                    ..confirmed(vec![], vec![])
                },
                UpdateError::UnknownMod("missing".into()),
            ),
            (
                confirmed(vec![], vec![map("x", Some("v2"))]),
                UpdateError::NotInMod("x".into()),
            ),
            (
                confirmed(vec![], vec![map("v1", Some("x"))]),
                UpdateError::NotInMod("x".into()),
            ),
            (
                confirmed(vec![], vec![map("v1", Some("v1"))]),
                UpdateError::SameFile("v1".into()),
            ),
            (
                confirmed(vec![], vec![map("v1", Some("v2")), map("v1", None)]),
                UpdateError::MappedTwice("v1".into()),
            ),
            (
                confirmed(vec![lib_file("x", Some(7), "x.zip")], vec![]),
                UpdateError::Manifest(ModelError::DuplicateId("x".into())),
            ),
            (
                confirmed(vec![], vec![map("v1", Some("v2")), map("v2", Some("v1"))]),
                UpdateError::Manifest(ModelError::SuccessorLoop("v1".into())),
            ),
        ];
        for (update, expected) in cases {
            let mut manifest = library();
            let mut profiles = [profile(vec![entry("gb-638388", &["v1"], &[])])];
            let before = (manifest.clone(), profiles.clone());
            assert_eq!(
                apply_update(&mut manifest, &mut profiles, &update),
                Err(expected)
            );
            assert_eq!((manifest, profiles), before);
        }
    }

    #[test]
    fn the_whole_library_is_checked_and_n3_counts_only_updates() {
        let mut local = gb_mod(9, vec![lib_file("l", None, "l.zip")]);
        local.id = "local-1".to_owned();
        local.source = Source::Local {
            imported_from: None,
        };
        let manifest = Manifest {
            mods: vec![
                gb_mod(
                    638_388,
                    vec![lib_file("v1", Some(1_838_410), "variant1.zip")],
                ),
                gb_mod(1, vec![lib_file("a", Some(10), "a.zip")]),
                gb_mod(2, vec![lib_file("b", Some(20), "b.zip")]),
                local,
                gb_mod(3, vec![lib_file("c", Some(30), "c.zip")]),
            ],
            ..Manifest::default()
        };
        let answers = [
            (638_388, listed("v5.3", manhattan_v5_3(), Vec::new())),
            (
                1,
                listed(
                    "1",
                    vec![remote(10, "a.zip", 1), remote(11, "a2.zip", 900_000)],
                    Vec::new(),
                ),
            ),
            (2, RemoteMod::Removed(RemovedReason::Deleted)),
        ];
        let checks = check(&manifest, |section, id| {
            assert_eq!(section, Section::Mod);
            answers.iter().find(|(m, _)| *m == id).map(|(_, a)| a)
        });
        let summary: Vec<(&str, &str)> = checks
            .iter()
            .map(|c| {
                let kind = match &c.status {
                    Status::UpdateAvailable(_) => "update",
                    Status::NewFiles { .. } => "new",
                    Status::Removed { .. } => "removed",
                    Status::Local => "local",
                    Status::NotChecked => "not checked",
                    other => panic!("{other:?}"),
                };
                (c.mod_id.as_str(), kind)
            })
            .collect();
        assert_eq!(
            summary,
            [
                ("gb-638388", "update"),
                ("gb-1", "new"),
                ("gb-2", "removed"),
                ("local-1", "local"),
                ("gb-3", "not checked"),
            ]
        );
        assert_eq!(updates_available(&checks), 1);
    }
}
