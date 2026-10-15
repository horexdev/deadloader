//! Reading back what WebView2 actually uses, to warn when something outside deadloader overrides our settings
//! (an environment variable or a policy in the registry can move the profile folder, ADR 0001).
//!
//! WebView2 is a COM API, so this needs `unsafe`; every call states why it is sound.

#![allow(unsafe_code)]

use std::path::PathBuf;
use webview2_com::Microsoft::Web::WebView2::Win32::{
    COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW, COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL,
    ICoreWebView2_2, ICoreWebView2_19, ICoreWebView2Controller, ICoreWebView2Environment7,
};
use webview2_com::take_pwstr;
use windows_core::{Interface, PWSTR};

/// The profile ("user data") folder the webview behind `controller` was created with.
pub fn user_data_folder(controller: &ICoreWebView2Controller) -> windows_core::Result<PathBuf> {
    // SAFETY: `controller` is a live COM interface owned by the caller; the getter has no preconditions and returns
    // an owned reference.
    let webview = unsafe { controller.CoreWebView2() }?;
    let webview: ICoreWebView2_2 = webview.cast()?;
    // SAFETY: as above, a plain getter on a live interface.
    let environment = unsafe { webview.Environment() }?;
    let environment: ICoreWebView2Environment7 = environment.cast()?;
    let mut folder = PWSTR::null();
    // SAFETY: `folder` is a valid out-pointer; on success it receives a string allocated with CoTaskMemAlloc,
    // which `take_pwstr` copies and frees exactly once.
    unsafe { environment.UserDataFolder(&mut folder) }?;
    Ok(PathBuf::from(take_pwstr(folder)))
}

/// Asks the webview behind `controller` to keep little memory (`low`: it drops caches it can rebuild, such as decoded
/// pictures) or to work normally again. Fails on a WebView2 runtime older than ICoreWebView2_19 (1.0.2210).
pub fn set_memory_low(controller: &ICoreWebView2Controller, low: bool) -> windows_core::Result<()> {
    // SAFETY: `controller` is a live COM interface owned by the caller; the getter has no preconditions and returns
    // an owned reference.
    let webview = unsafe { controller.CoreWebView2() }?;
    let webview: ICoreWebView2_19 = webview.cast()?;
    let level = if low {
        COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_LOW
    } else {
        COREWEBVIEW2_MEMORY_USAGE_TARGET_LEVEL_NORMAL
    };
    // SAFETY: a plain setter on a live interface; `level` is one of the two values the API defines.
    unsafe { webview.SetMemoryUsageTargetLevel(level) }
}
