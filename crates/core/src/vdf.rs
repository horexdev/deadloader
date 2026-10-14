//! Reader for Valve's text KeyValues format (`.vdf`, `.acf`) as used by Steam's library files.
//!
//! Only the subset Steam writes is accepted: quoted keys and values, `{ }` sections, `//` comments and the escapes
//! `\\`, `\"`, `\n`, `\t`. Keys are matched case-insensitively, like Steam does; entries keep their file order.

use std::fmt;

/// Sections nested deeper than this are rejected instead of risking a stack overflow on a damaged file.
pub const MAX_DEPTH: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Text(String),
    Section(Section),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Section {
    entries: Vec<(String, Value)>,
}

impl Section {
    pub fn entries(&self) -> &[(String, Value)] {
        &self.entries
    }

    /// The first text value stored under `key`.
    pub fn value(&self, key: &str) -> Option<&str> {
        self.entries.iter().find_map(|(k, v)| match v {
            Value::Text(t) if k.eq_ignore_ascii_case(key) => Some(t.as_str()),
            _ => None,
        })
    }

    /// The first section stored under `key`.
    pub fn section(&self, key: &str) -> Option<&Section> {
        self.entries.iter().find_map(|(k, v)| match v {
            Value::Section(s) if k.eq_ignore_ascii_case(key) => Some(s),
            _ => None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    UnexpectedToken,
    UnexpectedEnd,
    UnterminatedString,
    TooDeep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub kind: ErrorKind,
    /// 1-based line where the problem was found.
    pub line: usize,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let what = match self.kind {
            ErrorKind::UnexpectedToken => "unexpected token",
            ErrorKind::UnexpectedEnd => "unexpected end of file",
            ErrorKind::UnterminatedString => "unterminated string",
            ErrorKind::TooDeep => "sections nested too deeply",
        };
        write!(f, "{what} on line {}", self.line)
    }
}

impl std::error::Error for Error {}

/// Parses a whole document; its top level is an implicit section.
pub fn parse(text: &str) -> Result<Section, Error> {
    let mut lexer = Lexer::new(text);
    let root = parse_entries(&mut lexer, 0)?;
    match lexer.next()? {
        None => Ok(root),
        Some((_, line)) => Err(Error {
            kind: ErrorKind::UnexpectedToken,
            line,
        }),
    }
}

/// Reads entries until a closing brace (left for the caller) or the end of input.
fn parse_entries(lexer: &mut Lexer<'_>, depth: usize) -> Result<Section, Error> {
    let mut section = Section::default();
    loop {
        let Some(key) = lexer.next_string()? else {
            return Ok(section);
        };
        let value = match lexer.next()? {
            Some((Token::Str(v), _)) => Value::Text(v),
            Some((Token::Open, line)) => {
                if depth + 1 > MAX_DEPTH {
                    return Err(Error {
                        kind: ErrorKind::TooDeep,
                        line,
                    });
                }
                let inner = parse_entries(lexer, depth + 1)?;
                match lexer.next()? {
                    Some((Token::Close, _)) => Value::Section(inner),
                    Some((_, line)) => {
                        return Err(Error {
                            kind: ErrorKind::UnexpectedToken,
                            line,
                        });
                    }
                    None => {
                        return Err(Error {
                            kind: ErrorKind::UnexpectedEnd,
                            line: lexer.line,
                        });
                    }
                }
            }
            Some((Token::Close, line)) => {
                return Err(Error {
                    kind: ErrorKind::UnexpectedToken,
                    line,
                });
            }
            None => {
                return Err(Error {
                    kind: ErrorKind::UnexpectedEnd,
                    line: lexer.line,
                });
            }
        };
        section.entries.push((key, value));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Str(String),
    Open,
    Close,
}

#[derive(Debug)]
struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    peeked: Option<Option<(Token, usize)>>,
}

impl<'a> Lexer<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            chars: text.chars().peekable(),
            line: 1,
            peeked: None,
        }
    }

    /// Takes the next token only if it is a string; anything else stays for the next call.
    fn next_string(&mut self) -> Result<Option<String>, Error> {
        match self.next()? {
            Some((Token::Str(s), _)) => Ok(Some(s)),
            other => {
                self.peeked = Some(other);
                Ok(None)
            }
        }
    }

    fn next(&mut self) -> Result<Option<(Token, usize)>, Error> {
        match self.peeked.take() {
            Some(token) => Ok(token),
            None => self.read(),
        }
    }

    fn read(&mut self) -> Result<Option<(Token, usize)>, Error> {
        loop {
            let Some(c) = self.chars.next() else {
                return Ok(None);
            };
            match c {
                '\n' => self.line += 1,
                c if c.is_whitespace() => {}
                '/' if self.chars.peek() == Some(&'/') => {
                    while self.chars.next_if(|&c| c != '\n').is_some() {}
                }
                '{' => return Ok(Some((Token::Open, self.line))),
                '}' => return Ok(Some((Token::Close, self.line))),
                '"' => return self.read_string().map(|s| Some((Token::Str(s), self.line))),
                _ => {
                    return Err(Error {
                        kind: ErrorKind::UnexpectedToken,
                        line: self.line,
                    });
                }
            }
        }
    }

    fn read_string(&mut self) -> Result<String, Error> {
        let start = self.line;
        let unterminated = Error {
            kind: ErrorKind::UnterminatedString,
            line: start,
        };
        let mut out = String::new();
        loop {
            match self.chars.next() {
                None | Some('\n') => return Err(unterminated),
                Some('"') => return Ok(out),
                Some('\\') => match self.chars.next() {
                    Some('\\') => out.push('\\'),
                    Some('"') => out.push('"'),
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                    None => return Err(unterminated),
                },
                Some(c) => out.push(c),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_nested_sections_and_values() {
        let doc = parse(
            r#""libraryfolders"
{
	"0"
	{
		"path"		"C:\\Program Files (x86)\\Steam"
		"apps"
		{
			"228980"		"348323003"
		}
	}
}"#,
        )
        .expect("valid document");

        let folders = doc.section("libraryfolders").expect("root section");
        let first = folders.section("0").expect("first library");
        assert_eq!(first.value("path"), Some(r"C:\Program Files (x86)\Steam"));
        assert_eq!(
            first.section("apps").and_then(|a| a.value("228980")),
            Some("348323003")
        );
    }

    #[test]
    fn keeps_entries_in_file_order() {
        let doc = parse(r#""root" { "b" "2" "a" "1" "c" { } }"#).expect("valid document");
        let keys: Vec<&str> = doc
            .section("root")
            .expect("root")
            .entries()
            .iter()
            .map(|(k, _)| k.as_str())
            .collect();
        assert_eq!(keys, ["b", "a", "c"]);
    }

    #[test]
    fn looks_up_keys_case_insensitively() {
        let doc = parse(r#""AppState" { "InstallDir" "Deadlock" }"#).expect("valid document");
        assert_eq!(
            doc.section("appstate").and_then(|s| s.value("installdir")),
            Some("Deadlock")
        );
    }

    #[test]
    fn decodes_escape_sequences() {
        let doc = parse(r#""k" "a\\b \"q\" \n\t""#).expect("valid document");
        assert_eq!(doc.value("k"), Some("a\\b \"q\" \n\t"));
    }

    #[test]
    fn skips_line_comments_and_blank_lines() {
        let doc = parse("// header comment\n\n\"k\" \"v\" // trailing\n").expect("valid document");
        assert_eq!(doc.value("k"), Some("v"));
    }

    #[test]
    fn accepts_empty_values_and_empty_sections() {
        let doc = parse(r#""label" "" "apps" { }"#).expect("valid document");
        assert_eq!(doc.value("label"), Some(""));
        assert!(doc.section("apps").expect("apps").entries().is_empty());
    }

    #[test]
    fn accepts_crlf_line_endings() {
        let doc = parse("\"root\"\r\n{\r\n\t\"k\"\t\"v\"\r\n}\r\n").expect("valid document");
        assert_eq!(doc.section("root").and_then(|s| s.value("k")), Some("v"));
    }

    #[test]
    fn value_and_section_lookups_do_not_mix() {
        let doc = parse(r#""k" "v" "s" { }"#).expect("valid document");
        assert!(doc.section("k").is_none());
        assert!(doc.value("s").is_none());
    }

    #[test]
    fn reports_unclosed_section_with_line() {
        let err = parse("\"root\"\n{\n\t\"k\" \"v\"\n").expect_err("missing closing brace");
        assert_eq!(err.kind, ErrorKind::UnexpectedEnd);
        assert_eq!(err.line, 4);
    }

    #[test]
    fn reports_unterminated_string_with_line() {
        let err = parse("\"root\"\n{\n\t\"k\" \"v\n}").expect_err("unterminated string");
        assert_eq!(err.kind, ErrorKind::UnterminatedString);
        assert_eq!(err.line, 3);
    }

    #[test]
    fn reports_stray_closing_brace() {
        let err = parse("\"k\" \"v\"\n}").expect_err("stray brace");
        assert_eq!(err.kind, ErrorKind::UnexpectedToken);
        assert_eq!(err.line, 2);
    }

    #[test]
    fn reports_key_without_value() {
        let err = parse("\"root\" { \"k\" }").expect_err("key without value");
        assert_eq!(err.kind, ErrorKind::UnexpectedToken);
    }

    #[test]
    fn rejects_nesting_deeper_than_the_limit() {
        let deep = format!(
            "{}{}",
            "\"k\" {".repeat(MAX_DEPTH + 1),
            "}".repeat(MAX_DEPTH + 1)
        );
        let err = parse(&deep).expect_err("too deep");
        assert_eq!(err.kind, ErrorKind::TooDeep);

        let ok = format!("{}{}", "\"k\" {".repeat(MAX_DEPTH), "}".repeat(MAX_DEPTH));
        assert!(parse(&ok).is_ok());
    }

    #[test]
    fn rejects_unquoted_tokens() {
        let err = parse("root { }").expect_err("unquoted key");
        assert_eq!(err.kind, ErrorKind::UnexpectedToken);
        assert_eq!(err.line, 1);
    }
}
