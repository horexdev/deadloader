//! Windows specifics: Steam discovery, hard links, the recycle bin, process detection and WebView2 settings.

#[cfg(windows)]
pub mod process;
#[cfg(windows)]
pub mod recycle;
#[cfg(windows)]
pub mod steam;
