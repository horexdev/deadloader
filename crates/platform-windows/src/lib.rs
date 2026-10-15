//! Windows specifics: Steam discovery, hard links, the recycle bin, free disk space, known folders, process detection, the system
//! proxy, the user's languages and regional format, and WebView2 settings.

#[cfg(windows)]
pub mod disk;
#[cfg(windows)]
pub mod folders;
#[cfg(windows)]
pub mod locale;
#[cfg(windows)]
pub mod process;
#[cfg(windows)]
pub mod proxy;
#[cfg(windows)]
pub mod recycle;
#[cfg(windows)]
pub mod shell;
#[cfg(windows)]
pub mod steam;
#[cfg(windows)]
pub mod webview2;
