//! Local favourites (decision 34): catalog mods the user marked with the heart, kept in `favourites.json` next to the
//! library. No account is involved; "favourites only" in the catalog lists them from here.
//!
//! Each entry keeps what its card showed when it was marked (name, author, categories), so the list can be shown
//! and filtered without asking GameBanana. The file is pretty-printed JSON with a schema number, like the manifest:
//! users may read it, and a newer format is refused rather than misread.

use crate::library::{ModelError, Section};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

pub const SCHEMA: u32 = 1;

/// Its name in `deadloader-data\`.
pub const FILE_NAME: &str = "favourites.json";

/// A marked mod and what its card showed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Favourite {
    pub section: Section,
    /// GameBanana's id of the mod.
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub root_category: Option<CategoryName>,
    /// A hero under Skins, or a subcategory of another root category.
    #[serde(default)]
    pub subcategory: Option<CategoryName>,
    /// The card said it has content ratings (hidden or blurred like any other such card).
    #[serde(default)]
    pub nsfw: bool,
    /// Unix seconds; lets the catalog serve a saved mod page as current.
    #[serde(default)]
    pub date_updated: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryName {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct FavouritesFile {
    schema: u32,
    items: Vec<Favourite>,
}

#[derive(Deserialize)]
struct Head {
    schema: u32,
}

/// The saved favourites, newest first; a missing file is an empty list.
pub fn load(path: &Path) -> Result<Vec<Favourite>, ModelError> {
    match std::fs::read_to_string(path) {
        Ok(text) => from_json(&text),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e.into()),
    }
}

pub fn from_json(text: &str) -> Result<Vec<Favourite>, ModelError> {
    let head: Head = serde_json::from_str(text)?;
    if head.schema > SCHEMA {
        return Err(ModelError::NewerSchema {
            found: head.schema,
            supported: SCHEMA,
        });
    }
    let file: FavouritesFile = serde_json::from_str(text)?;
    let mut items: Vec<Favourite> = Vec::with_capacity(file.items.len());
    // A hand-edited file may list a mod twice: the first entry wins.
    for item in file.items {
        if !items.iter().any(|f| same(f, &item)) {
            items.push(item);
        }
    }
    Ok(items)
}

pub fn to_json(items: &[Favourite]) -> Result<String, ModelError> {
    Ok(serde_json::to_string_pretty(&FavouritesFile {
        schema: SCHEMA,
        items: items.to_vec(),
    })?)
}

/// Writes the list so a crash leaves the old or the new file, never half of one.
pub fn save(path: &Path, items: &[Favourite]) -> Result<(), ModelError> {
    crate::fs::write_atomic(path, &to_json(items)?)?;
    Ok(())
}

/// Marks (`on`) or unmarks a mod. Marking an already marked mod refreshes what its card showed and keeps its place;
/// a new one goes first. Returns whether the list changed.
pub fn set(items: &mut Vec<Favourite>, favourite: Favourite, on: bool) -> bool {
    let at = items.iter().position(|f| same(f, &favourite));
    match (at, on) {
        (Some(i), true) => {
            let changed = items[i] != favourite;
            items[i] = favourite;
            changed
        }
        (None, true) => {
            items.insert(0, favourite);
            true
        }
        (Some(i), false) => {
            items.remove(i);
            true
        }
        (None, false) => false,
    }
}

fn same(a: &Favourite, b: &Favourite) -> bool {
    a.section == b.section && a.id == b.id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{remove_all, temp_dir};

    fn fav(id: u64, name: &str) -> Favourite {
        Favourite {
            section: Section::Mod,
            id,
            name: name.to_owned(),
            author: Some("basilisken".to_owned()),
            root_category: Some(CategoryName {
                id: 31_701,
                name: "Skins".to_owned(),
            }),
            subcategory: Some(CategoryName {
                id: 31_719,
                name: "Celeste".to_owned(),
            }),
            nsfw: false,
            date_updated: Some(1_783_120_000),
        }
    }

    #[test]
    fn marking_puts_the_newest_first_and_unmarking_removes() {
        let mut items = Vec::new();
        assert!(set(&mut items, fav(1, "One"), true));
        assert!(set(&mut items, fav(2, "Two"), true));
        assert_eq!(items.iter().map(|f| f.id).collect::<Vec<_>>(), [2, 1]);
        assert!(set(&mut items, fav(1, "One"), false));
        assert_eq!(items.iter().map(|f| f.id).collect::<Vec<_>>(), [2]);
        assert!(
            !set(&mut items, fav(1, "One"), false),
            "not marked: nothing changes"
        );
    }

    #[test]
    fn marking_again_refreshes_the_card_in_place() {
        let mut items = vec![fav(2, "Two"), fav(1, "One")];
        assert!(!set(&mut items, fav(1, "One"), true));
        assert!(set(&mut items, fav(1, "One (v2)"), true));
        assert_eq!(items[1].name, "One (v2)");
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn the_same_id_in_another_section_is_another_mod() {
        let mut items = vec![fav(7, "A skin")];
        let sound = Favourite {
            section: Section::Sound,
            ..fav(7, "A sound")
        };
        assert!(set(&mut items, sound, true));
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn the_file_round_trips_and_a_missing_one_is_empty() {
        let dir = temp_dir("favourites", "round-trip");
        let path = dir.join(FILE_NAME);
        assert_eq!(load(&path), Ok(Vec::new()));
        let items = vec![fav(2, "Two"), fav(1, "One")];
        save(&path, &items).expect("saved");
        assert_eq!(load(&path), Ok(items));
        let text = std::fs::read_to_string(&path).expect("text");
        assert!(text.starts_with("{\n  \"schema\": 1,"), "{text}");
        assert!(text.contains("\"section\": \"mod\""), "{text}");
        remove_all(&dir);
    }

    #[test]
    fn a_newer_or_broken_file_is_refused_and_duplicates_collapse() {
        assert_eq!(
            from_json(r#"{"schema": 2, "items": []}"#),
            Err(ModelError::NewerSchema {
                found: 2,
                supported: SCHEMA
            })
        );
        assert!(matches!(from_json("{"), Err(ModelError::Json { .. })));
        let twice = r#"{"schema": 1, "items": [
            {"section": "mod", "id": 1, "name": "First"},
            {"section": "mod", "id": 1, "name": "Again"}
        ]}"#;
        let items = from_json(twice).expect("read");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "First");
        assert_eq!(items[0].author, None, "older entries may lack fields");
    }
}
