//! The gameinfo.gi guard: recognising whether `game\citadel\gameinfo.gi` loads mods, adding and removing our one line.
//!
//! The file is never parsed into a tree and written back: only the lines of the `SearchPaths` block are read, and an
//! edit inserts or removes exactly one line, copying the style (indentation, separators, quotes, line ending) of the
//! `Game citadel` line it sits on. Every other byte stays as it was, so a diff of our change is one line.
//!
//! "Vanilla" is recognised by what must *not* be there (any `addons` path, another manager's markers) rather than by
//! comparing with a reference copy, so harmless changes Valve makes in future builds don't lock the user out.

const OUR_VALUE: &str = "citadel/addons";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// As Valve ships it: no addons path; ours can be added.
    Vanilla,
    /// Vanilla plus exactly our line in front of `Game citadel`.
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

/// Inserts `Game citadel/addons` in front of the first `Game citadel` entry. Only for vanilla files.
pub fn add_addons_path(text: &str) -> Result<String, State> {
    let a = analyse(text).map_err(|r| State::Foreign(vec![r]))?;
    if a.state != State::Vanilla {
        return Err(a.state);
    }
    let game = &a.lines[a
        .game_citadel
        .ok_or(State::Foreign(vec![Foreign::NoGameCitadel]))?];
    let value = game.tokens[1];
    let mut new_line = String::new();
    new_line.push_str(&game.content[..value.start]);
    new_line.push_str(&game.content[value.start..value.end].replacen("citadel", OUR_VALUE, 1));
    new_line.push_str(game.terminator(text));
    let mut out = String::with_capacity(text.len() + new_line.len());
    out.push_str(&text[..game.start]);
    out.push_str(&new_line);
    out.push_str(&text[game.start..]);
    Ok(out)
}

/// Removes exactly the line [`add_addons_path`] inserted. Only for files in the [`State::Ours`] state.
pub fn remove_addons_path(text: &str) -> Result<String, State> {
    let a = analyse(text).map_err(|r| State::Foreign(vec![r]))?;
    if a.state != State::Ours {
        return Err(a.state);
    }
    let ours = &a.lines[a.addons[0]];
    Ok(format!("{}{}", &text[..ours.start], &text[ours.end..]))
}

struct Analysis<'a> {
    lines: Vec<Line<'a>>,
    state: State,
    game_citadel: Option<usize>,
    /// Indices into `lines` of entries whose value is an addons path.
    addons: Vec<usize>,
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

    let ours = match (addons.as_slice(), game_citadel) {
        ([only], Some(g)) => {
            let (k, v) = entry(*only);
            k.eq_ignore_ascii_case("Game") && v.eq_ignore_ascii_case(OUR_VALUE) && only + 1 == g
        }
        _ => false,
    };

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

    const OUR_LINE: &str = "\t\t\tGame\t\t\t\tcitadel/addons\r\n";

    fn ours() -> String {
        VANILLA.replacen(
            "\t\t\tGame\t\t\t\tcitadel\r\n",
            &format!("{OUR_LINE}\t\t\tGame\t\t\t\tcitadel\r\n"),
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
    fn adding_inserts_one_line_in_the_same_style_before_game_citadel() {
        let patched = add_addons_path(VANILLA).expect("vanilla");
        assert_eq!(patched, ours());
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
                .contains("\t\t\t\"Game\"\t\"citadel/addons\"\r\n\t\t\t\"Game\"\t\"citadel\"\r\n")
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
