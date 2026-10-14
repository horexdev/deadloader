//! deadloader's own view of GameBanana content. Nothing here mirrors GameBanana's field names, so the rest of the
//! app keeps compiling unchanged when the API does.

/// The GameBanana sections the catalog shows (decision 131).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Section {
    Mod,
    Sound,
}
