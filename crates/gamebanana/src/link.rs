//! Which submission a GameBanana page address names, for linking a mod imported from disk (F10).

use crate::types::Section;

/// The section and id of a mod or sound page address as people copy it from their browser:
/// `https://gamebanana.com/mods/655209`, with or without the scheme or `www.`, with a query, a fragment or a trailing
/// slash. Anything else (another site, a download link, a member page) is `None`.
pub fn submission_from_url(text: &str) -> Option<(Section, u64)> {
    let text = text.trim();
    let rest = text
        .strip_prefix("https://")
        .or_else(|| text.strip_prefix("http://"))
        .unwrap_or(text);
    let end = rest.find(['?', '#']).unwrap_or(rest.len());
    let rest = &rest[..end];
    let (host, path) = rest.split_once('/')?;
    if !matches!(
        host.to_ascii_lowercase().as_str(),
        "gamebanana.com" | "www.gamebanana.com"
    ) {
        return None;
    }
    let mut parts = path.split('/').filter(|p| !p.is_empty());
    let section = match parts.next()?.to_ascii_lowercase().as_str() {
        "mods" => Section::Mod,
        "sounds" => Section::Sound,
        _ => return None,
    };
    let id = parts.next()?;
    if parts.next().is_some() || id.is_empty() || !id.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    id.parse::<u64>()
        .ok()
        .filter(|&id| id > 0)
        .map(|id| (section, id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_addresses_as_people_copy_them() {
        for text in [
            "https://gamebanana.com/mods/655209",
            "http://gamebanana.com/mods/655209",
            "gamebanana.com/mods/655209",
            "https://www.gamebanana.com/mods/655209/",
            "https://gamebanana.com/mods/655209#FileInfo",
            "https://GameBanana.com/Mods/655209?ref=x",
            "  https://gamebanana.com/mods/655209  ",
        ] {
            assert_eq!(
                submission_from_url(text),
                Some((Section::Mod, 655_209)),
                "{text}"
            );
        }
        assert_eq!(
            submission_from_url("https://gamebanana.com/sounds/81234"),
            Some((Section::Sound, 81_234))
        );
    }

    #[test]
    fn other_addresses_are_not_a_submission() {
        for text in [
            "",
            "655209",
            "https://gamebanana.com/",
            "https://gamebanana.com/mods/",
            "https://gamebanana.com/mods/cats/1234",
            "https://gamebanana.com/dl/1480331",
            "https://gamebanana.com/members/1",
            "https://gamebanana.com/mods/0",
            "https://gamebanana.com/mods/+12",
            "https://gamebanana.com/mods/99999999999999999999999",
            "https://gamebanana.com.evil.example/mods/1",
            "https://evil.example/gamebanana.com/mods/1",
            "https://user@gamebanana.com/mods/1",
        ] {
            assert_eq!(submission_from_url(text), None, "{text}");
        }
    }
}
