//! Windows specifics: Steam discovery, hard links, the recycle bin, free disk space, process detection and WebView2
//! settings.

#[cfg(windows)]
pub mod disk;
#[cfg(windows)]
pub mod process;
#[cfg(windows)]
pub mod recycle;
#[cfg(windows)]
pub mod steam;
#[cfg(windows)]
pub mod webview2;
