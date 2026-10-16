//! The gameinfo.gi guard: recognising whether `game\citadel\gameinfo.gi` loads mods, adding and removing our lines.
//!
//! The file is never parsed into a tree and written back: only the lines of the `SearchPaths` block are read, and an
//! edit inserts or removes our lines in front of `Game citadel`, copying its style (indentation, separators, quotes,
//! line ending). Every other byte stays as it was.
//!
//! Our lines are `Game citadel/addons` and, unless the block has these keys already, `Mod citadel` and `Write citadel`.
//! The engine makes the *first* `Game` path the MOD and the default write path when there is no `Mod` / `Write` key;
//! with only the addons path in front, Deadlock looked for its own config in `addons` and stopped at start with
//! "FATAL ERROR: Unable to read default keybinding configuration" (0.1.0). The two keys keep both on `citadel`.
//!
//! "Vanilla" is recognised by what must *not* be there (any `addons` path, another manager's markers) rather than by
//! comparing with a reference copy, so harmless changes Valve makes in future builds don't lock the user out.

const OUR_VALUE: &str = "citadel/addons";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// As Valve ships it: no addons path; ours can be added.
    Vanilla,
    /// Vanilla plus exactly our lines in front of `Game citadel` (or 0.1.0's addons line alone: see [`upgrade`]).
    Ours,
    /// Changed by something else; we don't touch it.
    Foreign(Vec<Foreign>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Foreign {
    NoSearchPaths,
    MultipleSearchPaths,
    Unclosed,
    NoGameCitadel,
    /// An addons path that isn't exactly our single line in our place.
    AddonsElsewhere,
    /// A marker comment left by another mod manager.
    OtherManager,
}

pub fn inspect(text: &str) -> State {
    match analyse(text) {
        Ok(a) => a.state,
        Err(reason) => State::Foreign(vec![reason]),
    }
}

/// Inserts our lines (`Game citadel/addons`, then `Mod citadel` and `Write citadel` unless the block has those keys)
/// in front of the first `Game citadel` entry. Only for vanilla files.
pub fn add_addons_path(text: &str) -> Result<String, State> {
    let a = analyse(text).map_err(|r| State::Foreign(vec![r]))?;
    if a.state != State::Vanilla {
        return Err(a.state);
    }
    let game = &a.lines[a
        .game_citadel
        .ok_or(State::Foreign(vec![Foreign::NoGameCitadel]))?];
    let new_lines = our_lines(game, !a.has_mod, !a.has_write)
        .iter()
        .map(|l| format!("{l}{}", game.terminator(text)))
        .collect::<String>();
    let mut out = String::with_capacity(text.len() + new_lines.len());
    out.push_str(&text[..game.start]);
    out.push_str(&new_lines);
    out.push_str(&text[game.start..]);
    Ok(out)
}

/// Removes exactly the lines [`add_addons_path`] inserted (or 0.1.0's single line). Only for [`State::Ours`] files.
pub fn remove_addons_path(text: &str) -> Result<String, State> {
    let a = analyse(text).map_err(|r| State::Foreign(vec![r]))?;
    if a.state != State::Ours {
        return Err(a.state);
    }
    let first = &a.lines[a.addons[0]];
    let last = &a.lines[a.addons[0] + a.ours_keys];
    Ok(format!("{}{}", &text[..first.start], &text[last.end..]))
}

/// The file with our current lines when it has 0.1.0's edit (the addons line without the `Mod` / `Write` keys the
/// game needs); `None` for anything else. Applying a profile runs it, so a 0.1.0 install repairs itself.
pub fn upgrade(text: &str) -> Option<String> {
    let a = analyse(text).ok()?;
    if a.state != State::Ours || a.ours_keys > 0 || (a.has_mod && a.has_write) {
        return None;
    }
    add_addons_path(&remove_addons_path(text).ok()?).ok()
}

/// Our lines in the style of the `Game citadel` line `game`, without line endings.
fn our_lines(game: &Line<'_>, with_mod: bool, with_write: bool) -> Vec<String> {
    let (key, value) = (game.tokens[0], game.tokens[1]);
    let content = game.content;
    let with_value = |v: &str| {
        format!(
            "{}{}{}",
            &content[..value.start],
            content[value.start..value.end].replacen("citadel", v, 1),
            &content[value.end..]
        )
    };
    let with_key = |k: &str| {
        format!(
            "{}{}{}",
            &content[..key.start],
            content[key.start..key.end].replacen(key.text(content), k, 1),
            &content[key.end..]
        )
    };
    let mut lines = vec![with_value(OUR_VALUE)];
    if with_mod {
        lines.push(with_key("Mod"));
    }
    if with_write {
        lines.push(with_key("Write"));
    }
    lines
}

/// One line of [`Diff`], without its line ending.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct DiffLine {
    pub text: String,
    /// Expected: the line deadloader adds. Found: a line another program added (an addons path or its marker).
    pub marked: bool,
}

/// D10: the `SearchPaths` block as it should be (the original plus our line) next to what the file has now.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub struct Diff {
    /// Empty when the file has no single, closed `SearchPaths` block to compare with.
    pub expected: Vec<DiffLine>,
    pub found: Vec<DiffLine>,
}

/// At most this many lines of a file without a usable block are shown.
const DIFF_MAX_LINES: usize = 400;

/// What D10 shows: the expected block is the found one without other addons paths and manager markers, with our line
/// in front of `Game citadel`. A guess at the original, never written anywhere: only Steam restores the real file.
pub fn diff(text: &str) -> Diff {
    let lines = split_lines(text);
    let is_marker = |l: &Line<'_>| l.comment().to_ascii_lowercase().contains("mod manager");
    let line = |l: &Line<'_>, marked: bool| DiffLine {
        text: l.content.to_owned(),
        marked,
    };
    let Some((header, end)) = block(&lines) else {
        return Diff {
            expected: Vec::new(),
            found: lines
                .iter()
                .take(DIFF_MAX_LINES)
                .map(|l| line(l, is_marker(l)))
                .collect(),
        };
    };
    let is_addons = |i: usize| {
        let l = &lines[i];
        i > header
            && i < end
            && l.tokens.len() >= 2
            && l.tokens[1]
                .text(l.content)
                .replace('\\', "/")
                .split('/')
                .any(|seg| seg.eq_ignore_ascii_case("addons"))
    };
    // `Mod` / `Write` keys the block has besides other programs' addons paths.
    let has_key = |key: &str| {
        (header + 1..end).any(|i| {
            let l = &lines[i];
            l.tokens.len() >= 2
                && l.tokens[0].text(l.content).eq_ignore_ascii_case(key)
                && !is_addons(i)
        })
    };
    let mut found = Vec::new();
    let mut expected = Vec::new();
    let mut ours_added = false;
    for (i, l) in lines.iter().enumerate() {
        let inside = (header..=end).contains(&i);
        let marker = is_marker(l) && l.tokens.is_empty();
        if !inside && !marker {
            continue;
        }
        let foreign = marker || is_addons(i);
        found.push(line(l, foreign));
        if foreign {
            continue;
        }
        let game_citadel = inside
            && i > header
            && l.tokens.len() >= 2
            && l.tokens[0].text(l.content).eq_ignore_ascii_case("Game")
            && l.tokens[1].text(l.content).eq_ignore_ascii_case("citadel");
        if game_citadel && !ours_added {
            expected.extend(
                our_lines(l, !has_key("Mod"), !has_key("Write"))
                    .into_iter()
                    .map(|text| DiffLine { text, marked: true }),
            );
            ours_added = true;
        }
        expected.push(line(l, false));
    }
    Diff { expected, found }
}

/// The lines of the one `SearchPaths` block, header to closing brace; `None` when there isn't exactly one closed block.
fn block(lines: &[Line<'_>]) -> Option<(usize, usize)> {
    let mut headers = lines.iter().enumerate().filter(|(_, l)| {
        l.tokens
            .first()
            .is_some_and(|t| t.text(l.content).eq_ignore_ascii_case("SearchPaths"))
    });
    let (header, _) = headers.next()?;
    if headers.next().is_some() {
        return None;
    }
    let mut depth = 0usize;
    let mut opened = false;
    for (i, line) in lines.iter().enumerate().skip(header) {
        for tok in &line.tokens {
            match tok.text(line.content) {
                "{" => {
                    depth += 1;
                    opened = true;
                }
                "}" => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        if opened && depth == 0 {
            return Some((header, i));
        }
    }
    None
}

struct Analysis<'a> {
    lines: Vec<Line<'a>>,
    state: State,
    game_citadel: Option<usize>,
    /// Indices into `lines` of entries whose value is an addons path.
    addons: Vec<usize>,
    /// How many of our `Mod citadel` / `Write citadel` lines follow our addons line (0 for 0.1.0's edit).
    ours_keys: usize,
    has_mod: bool,
    has_write: bool,
}

fn analyse(text: &str) -> Result<Analysis<'_>, Foreign> {
    let lines = split_lines(text);
    let other_manager = lines
        .iter()
        .any(|l| l.comment().to_ascii_lowercase().contains("mod manager"));

    let mut headers = lines.iter().enumerate().filter(|(_, l)| {
        l.tokens
            .first()
            .is_some_and(|t| t.text(l.content).eq_ignore_ascii_case("SearchPaths"))
    });
    let (header, _) = headers.next().ok_or(Foreign::NoSearchPaths)?;
    if headers.next().is_some() {
        return Err(Foreign::MultipleSearchPaths);
    }

    let mut depth = 0usize;
    let mut opened = false;
    let mut entries = Vec::new();
    let mut closed = false;
    for (i, line) in lines.iter().enumerate().skip(header) {
        for (n, tok) in line.tokens.iter().enumerate() {
            match tok.text(line.content) {
                "{" => {
                    depth += 1;
                    opened = true;
                }
                "}" => depth = depth.saturating_sub(1),
                _ if i > header && n == 0 && opened && depth == 1 && line.tokens.len() >= 2 => {
                    entries.push(i)
                }
                _ => {}
            }
        }
        if opened && depth == 0 {
            closed = true;
            break;
        }
    }
    if !closed {
        return Err(Foreign::Unclosed);
    }

    let entry = |i: usize| {
        let l = &lines[i];
        (l.tokens[0].text(l.content), l.tokens[1].text(l.content))
    };
    let game_citadel = entries.iter().copied().find(|&i| {
        let (k, v) = entry(i);
        k.eq_ignore_ascii_case("Game") && v.eq_ignore_ascii_case("citadel")
    });
    let addons: Vec<usize> = entries
        .iter()
        .copied()
        .filter(|&i| {
            entry(i)
                .1
                .replace('\\', "/")
                .split('/')
                .any(|seg| seg.eq_ignore_ascii_case("addons"))
        })
        .collect();

    let has_key = |key: &str| {
        entries
            .iter()
            .any(|&i| entry(i).0.eq_ignore_ascii_case(key))
    };
    let (has_mod, has_write) = (has_key("Mod"), has_key("Write"));
    // Ours: the addons line, then only our `Mod citadel` / `Write citadel` lines, then `Game citadel`.
    let ours_keys = match (addons.as_slice(), game_citadel) {
        ([only], Some(g)) if *only < g => {
            let (k, v) = entry(*only);
            let keys = (only + 1..g).all(|i| {
                lines[i].tokens.len() >= 2 && {
                    let (k, v) = entry(i);
                    (k.eq_ignore_ascii_case("Mod") || k.eq_ignore_ascii_case("Write"))
                        && v.eq_ignore_ascii_case("citadel")
                }
            });
            (k.eq_ignore_ascii_case("Game")
                && v.eq_ignore_ascii_case(OUR_VALUE)
                && keys
                && g - only <= 3)
                .then_some(g - only - 1)
        }
        _ => None,
    };
    let ours = ours_keys.is_some();

    let mut reasons = Vec::new();
    if other_manager {
        reasons.push(Foreign::OtherManager);
    }
    if !addons.is_empty() && !ours {
        reasons.push(Foreign::AddonsElsewhere);
    }
    if game_citadel.is_none() {
        reasons.push(Foreign::NoGameCitadel);
    }
    let state = if !reasons.is_empty() {
        State::Foreign(reasons)
    } else if ours {
        State::Ours
    } else {
        State::Vanilla
    };
    Ok(Analysis {
        lines,
        state,
        game_citadel,
        addons,
        ours_keys: ours_keys.unwrap_or(0),
        has_mod,
        has_write,
    })
}

#[derive(Debug, Clone, Copy)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// The token without surrounding quotes.
    fn text<'a>(&self, content: &'a str) -> &'a str {
        content[self.start..self.end].trim_matches('"')
    }
}

#[derive(Debug)]
struct Line<'a> {
    /// Byte offset of the line in the whole text, and of the next line.
    start: usize,
    end: usize,
    /// The line without its terminator.
    content: &'a str,
    /// Tokens of the part before any `//` comment, as spans into `content`.
    tokens: Vec<Span>,
    code_len: usize,
}

impl<'a> Line<'a> {
    fn comment(&self) -> &'a str {
        &self.content[self.code_len..]
    }

    fn terminator<'t>(&self, text: &'t str) -> &'t str {
        let t = &text[self.start + self.content.len()..self.end];
        if t.is_empty() { "\r\n" } else { t }
    }
}

fn split_lines(text: &str) -> Vec<Line<'_>> {
    let mut lines = Vec::new();
    let mut start = 0;
    for raw in text.split_inclusive('\n') {
        let content = raw.trim_end_matches(['\n', '\r']);
        let code_len = content.find("//").unwrap_or(content.len());
        lines.push(Line {
            start,
            end: start + raw.len(),
            content,
            tokens: tokens(&content[..code_len]),
            code_len,
        });
        start += raw.len();
    }
    lines
}

/// Whitespace-separated tokens; a quoted token may contain spaces; braces are tokens of their own.
fn tokens(code: &str) -> Vec<Span> {
    let bytes = code.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b if b.is_ascii_whitespace() => i += 1,
            b'{' | b'}' => {
                out.push(Span {
                    start: i,
                    end: i + 1,
                });
                i += 1;
            }
            b'"' => {
                let end = code[i + 1..]
                    .find('"')
                    .map_or(bytes.len(), |p| i + 1 + p + 1);
                out.push(Span { start: i, end });
                i = end;
            }
            _ => {
                let start = i;
                while i < bytes.len()
                    && !bytes[i].is_ascii_whitespace()
                    && !matches!(bytes[i], b'{' | b'}' | b'"')
                {
                    i += 1;
                }
                out.push(Span { start, end: i });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Same structure as the file Valve ships (CRLF, tabs, comments, a lowercase `game` key in another block).
    const VANILLA: &str = "\"GameInfo\"\r\n\
{\r\n\
\tgame \t\t\"Deadlock\"\r\n\
\tFileSystem\r\n\
\t{\r\n\
\t\t// Search paths are relative to the exe directory\\..\\\r\n\
\t\tSearchPaths\r\n\
\t\t{\r\n\
\t\t\t// These are optional language paths.\r\n\
\t\t\tGame_UILanguage\t\tcitadel_*LANGUAGE*\r\n\
\r\n\
\t\t\tGame_LowViolence\tcitadel_lv\r\n\
\r\n\
\t\t\tGame\t\t\t\tcitadel\r\n\
\t\t\tGame\t\t\t\tcore\r\n\
\t\t}\r\n\
\t\t\"UserSettingsPathID\"\t\t\"USRLOCAL\"\r\n\
\t}\r\n\
\tMaterialSystem2\r\n\
\t{\r\n\
\t\tRenderModes\r\n\
\t\t{\r\n\
\t\t\tgame Default\r\n\
\t\t}\r\n\
\t}\r\n\
}\r\n";

    /// What we insert in front of `Game citadel`: the addons path first, and the `Mod` and `Write` paths the engine
    /// would otherwise take from the first `Game` line — ours.
    const OUR_LINES: &str = "\t\t\tGame\t\t\t\tcitadel/addons\r\n\t\t\tMod\t\t\t\tcitadel\r\n\t\t\tWrite\t\t\t\tcitadel\r\n";
    /// What 0.1.0 inserted: the addons path alone, which made it the game's MOD and write path (FATAL ERROR at start).
    const OLD_LINE: &str = "\t\t\tGame\t\t\t\tcitadel/addons\r\n";

    fn ours() -> String {
        VANILLA.replacen(
            "\t\t\tGame\t\t\t\tcitadel\r\n",
            &format!("{OUR_LINES}\t\t\tGame\t\t\t\tcitadel\r\n"),
            1,
        )
    }

    fn old_ours() -> String {
        VANILLA.replacen(
            "\t\t\tGame\t\t\t\tcitadel\r\n",
            &format!("{OLD_LINE}\t\t\tGame\t\t\t\tcitadel\r\n"),
            1,
        )
    }

    fn foreign_reasons(text: &str) -> Vec<Foreign> {
        match inspect(text) {
            State::Foreign(reasons) => reasons,
            other => panic!("expected foreign, got {other:?}"),
        }
    }

    #[test]
    fn the_shipped_layout_is_vanilla() {
        assert_eq!(inspect(VANILLA), State::Vanilla);
    }

    #[test]
    fn adding_inserts_our_lines_in_the_same_style_before_game_citadel() {
        let patched = add_addons_path(VANILLA).expect("vanilla");
        assert_eq!(patched, ours());
    }

    #[test]
    fn the_game_keeps_citadel_as_its_mod_and_write_path() {
        // The engine makes the first `Game` path the MOD and write path unless `Mod` / `Write` keys exist; with only
        // `Game citadel/addons` in front, Deadlock looked for its own config in addons and stopped with a FATAL ERROR.
        let patched = add_addons_path(VANILLA).expect("vanilla");
        let block: Vec<&str> = patched
            .lines()
            .map(str::trim)
            .filter(|l| l.starts_with("Game\t") || l.starts_with("Mod") || l.starts_with("Write"))
            .collect();
        assert_eq!(
            block,
            [
                "Game\t\t\t\tcitadel/addons",
                "Mod\t\t\t\tcitadel",
                "Write\t\t\t\tcitadel",
                "Game\t\t\t\tcitadel",
                "Game\t\t\t\tcore"
            ]
        );
    }

    #[test]
    fn mod_and_write_keys_the_file_already_has_are_not_added_again() {
        let with_keys = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\tMod\t\t\t\tcitadel\r\n\t\t\tWrite\t\t\t\tcitadel\r\n",
        );
        assert_eq!(inspect(&with_keys), State::Vanilla);
        let patched = add_addons_path(&with_keys).expect("vanilla");
        assert_eq!(
            patched,
            with_keys.replacen(
                "\t\t\tGame\t\t\t\tcitadel\r\n",
                &format!("{OLD_LINE}\t\t\tGame\t\t\t\tcitadel\r\n"),
                1
            )
        );
        assert_eq!(inspect(&patched), State::Ours);
        assert_eq!(upgrade(&patched), None, "nothing is missing");
        assert_eq!(remove_addons_path(&patched).expect("ours"), with_keys);
    }

    #[test]
    fn the_line_0_1_0_added_is_ours_and_is_upgraded() {
        assert_eq!(inspect(&old_ours()), State::Ours);
        assert_eq!(upgrade(&old_ours()), Some(ours()));
        assert_eq!(remove_addons_path(&old_ours()).expect("ours"), VANILLA);
    }

    #[test]
    fn only_an_old_edit_is_upgraded() {
        assert_eq!(upgrade(&ours()), None);
        assert_eq!(upgrade(VANILLA), None);
        assert_eq!(upgrade("x"), None);
    }

    #[test]
    fn after_adding_the_file_is_ours() {
        let patched = add_addons_path(VANILLA).expect("vanilla");
        assert_eq!(inspect(&patched), State::Ours);
    }

    #[test]
    fn removing_restores_the_exact_original_bytes() {
        let patched = add_addons_path(VANILLA).expect("vanilla");
        assert_eq!(remove_addons_path(&patched).expect("ours"), VANILLA);
    }

    #[test]
    fn lf_files_stay_lf() {
        let lf = VANILLA.replace("\r\n", "\n");
        let patched = add_addons_path(&lf).expect("vanilla");
        assert!(!patched.contains('\r'));
        assert_eq!(patched, ours().replace("\r\n", "\n"));
    }

    #[test]
    fn quoted_entries_keep_their_quotes() {
        let quoted = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcitadel\r\n",
            "\t\t\t\"Game\"\t\"citadel\"\r\n",
        );
        let patched = add_addons_path(&quoted).expect("vanilla");
        assert!(
            patched
                .contains("\t\t\t\"Game\"\t\"citadel/addons\"\r\n\t\t\t\"Mod\"\t\"citadel\"\r\n\t\t\t\"Write\"\t\"citadel\"\r\n\t\t\t\"Game\"\t\"citadel\"\r\n")
        );
        assert_eq!(inspect(&patched), State::Ours);
    }

    #[test]
    fn a_commented_out_addons_line_is_ignored() {
        let text = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\t// Game citadel/addons\r\n",
        );
        assert_eq!(inspect(&text), State::Vanilla);
    }

    #[test]
    fn keys_are_case_insensitive_inside_search_paths() {
        let text = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcitadel\r\n",
            "\t\t\tgame\t\t\t\tcitadel\r\n",
        );
        assert_eq!(inspect(&text), State::Vanilla);
    }

    #[test]
    fn game_entries_outside_search_paths_are_not_used() {
        // Only the lowercase "game Default" outside the block mentions `game`; without `Game citadel` in the block
        // there is nowhere to insert.
        let text = VANILLA.replace("\t\t\tGame\t\t\t\tcitadel\r\n", "");
        assert_eq!(foreign_reasons(&text), [Foreign::NoGameCitadel]);
    }

    #[test]
    fn a_deadlock_mod_manager_block_is_foreign() {
        let dmm = "\"GameInfo\"\n{\n\tFileSystem\n\t{\n// Deadlock Mod Manager - Start\n\t\tSearchPaths\n         {\n             Game_Language       citadel_*LANGUAGE*\n             Game                citadel/addons\n             Mod                 citadel\n             Game                citadel\n             Game                core\n         }\n// Deadlock Mod Manager - End\n\t}\n}\n";
        let reasons = foreign_reasons(dmm);
        assert!(reasons.contains(&Foreign::OtherManager), "{reasons:?}");
    }

    #[test]
    fn an_addons_path_in_another_place_is_foreign() {
        let text = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\tGame\t\t\t\tcitadel/addons\r\n",
        );
        assert_eq!(foreign_reasons(&text), [Foreign::AddonsElsewhere]);
    }

    #[test]
    fn two_addons_paths_are_foreign() {
        let text = ours().replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\tMod\t\t\t\tcitadel/addons\r\n",
        );
        assert_eq!(foreign_reasons(&text), [Foreign::AddonsElsewhere]);
    }

    #[test]
    fn a_file_without_search_paths_is_foreign() {
        assert_eq!(
            foreign_reasons("\"GameInfo\"\r\n{\r\n}\r\n"),
            [Foreign::NoSearchPaths]
        );
    }

    #[test]
    fn two_search_paths_blocks_are_foreign() {
        let block = "\t\tSearchPaths\r\n\t\t{\r\n\t\t\tGame\tcore\r\n\t\t}\r\n";
        let text = VANILLA.replace(
            "\t\t\"UserSettingsPathID\"",
            &format!("{block}\t\t\"UserSettingsPathID\""),
        );
        assert_eq!(foreign_reasons(&text), [Foreign::MultipleSearchPaths]);
    }

    #[test]
    fn an_unclosed_search_paths_block_is_foreign() {
        let text = "\"GameInfo\"\r\n{\r\n\tSearchPaths\r\n\t{\r\n\t\tGame\tcitadel\r\n";
        assert_eq!(foreign_reasons(text), [Foreign::Unclosed]);
    }

    fn texts(lines: &[DiffLine]) -> Vec<(&str, bool)> {
        lines.iter().map(|l| (l.text.as_str(), l.marked)).collect()
    }

    #[test]
    fn the_diff_of_another_managers_block_marks_its_lines_and_ours() {
        let text = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcitadel\r\n",
            "\t\t\tGame\t\t\t\tcitadel/addons/dmm\r\n\t\t\tGame\t\t\t\tcitadel/addons\r\n\t\t\tGame\t\t\t\tcitadel\r\n",
        );
        let diff = diff(&text);
        assert_eq!(
            texts(&diff.found)[7..10],
            [
                ("\t\t\tGame\t\t\t\tcitadel/addons/dmm", true),
                ("\t\t\tGame\t\t\t\tcitadel/addons", true),
                ("\t\t\tGame\t\t\t\tcitadel", false),
            ]
        );
        assert_eq!(texts(&diff.found)[0], ("\t\tSearchPaths", false));
        assert_eq!(texts(&diff.found).last(), Some(&("\t\t}", false)));
        // Expected: the other lines gone, ours in front of `Game citadel` in the same style.
        assert_eq!(
            texts(&diff.expected)[7..11],
            [
                ("\t\t\tGame\t\t\t\tcitadel/addons", true),
                ("\t\t\tMod\t\t\t\tcitadel", true),
                ("\t\t\tWrite\t\t\t\tcitadel", true),
                ("\t\t\tGame\t\t\t\tcitadel", false),
            ]
        );
        assert_eq!(diff.expected.len(), diff.found.len() + 1);
    }

    #[test]
    fn the_diff_marks_another_managers_comments() {
        let dmm = "\"GameInfo\"\n{\n// Deadlock Mod Manager - Start\n\tSearchPaths\n\t{\n\t\tGame\tcitadel/addons\n\t\tGame\tcitadel\n\t}\n// Deadlock Mod Manager - End\n}\n";
        let diff = diff(dmm);
        assert_eq!(
            texts(&diff.found),
            [
                ("// Deadlock Mod Manager - Start", true),
                ("\tSearchPaths", false),
                ("\t{", false),
                ("\t\tGame\tcitadel/addons", true),
                ("\t\tGame\tcitadel", false),
                ("\t}", false),
                ("// Deadlock Mod Manager - End", true),
            ]
        );
        assert_eq!(
            texts(&diff.expected),
            [
                ("\tSearchPaths", false),
                ("\t{", false),
                ("\t\tGame\tcitadel/addons", true),
                ("\t\tMod\tcitadel", true),
                ("\t\tWrite\tcitadel", true),
                ("\t\tGame\tcitadel", false),
                ("\t}", false),
            ]
        );
    }

    #[test]
    fn a_file_without_a_block_shows_what_it_has_and_expects_nothing() {
        let diff = diff("\"GameInfo\"\r\n{\r\n}\r\n");
        assert_eq!(
            texts(&diff.found),
            [("\"GameInfo\"", false), ("{", false), ("}", false)]
        );
        assert!(diff.expected.is_empty());
    }

    #[test]
    fn the_diff_of_a_vanilla_file_only_adds_our_lines() {
        let diff = diff(VANILLA);
        assert!(diff.found.iter().all(|l| !l.marked));
        assert_eq!(diff.expected.iter().filter(|l| l.marked).count(), 3);
    }

    #[test]
    fn adding_refuses_anything_but_vanilla() {
        assert_eq!(add_addons_path(&ours()), Err(State::Ours));
        assert!(matches!(add_addons_path("x"), Err(State::Foreign(_))));
    }

    #[test]
    fn removing_refuses_anything_but_ours() {
        assert_eq!(remove_addons_path(VANILLA), Err(State::Vanilla));
        let elsewhere = VANILLA.replace(
            "\t\t\tGame\t\t\t\tcore\r\n",
            "\t\t\tGame\t\t\t\tcore\r\n\t\t\tGame\t\tcitadel/addons\r\n",
        );
        assert!(matches!(
            remove_addons_path(&elsewhere),
            Err(State::Foreign(_))
        ));
    }
}
