//! Reading back what WebView2 actually uses, to warn when something outside deadloader overrides our settings
//! (an environment variable or a policy in the registry can move the profile folder, ADR 0001).
//!
//! WebView2 is a COM API, so this needs `unsafe`; every call states why it is sound.

#![allow(unsafe_code)]

use std::path::PathBuf;
use webview2_com::Microsoft::Web::WebView2::Win32::{
    ICoreWebView2_2, ICoreWebView2Controller, ICoreWebView2Environment7,
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
