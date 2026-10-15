//! The user's Windows languages, for the UI language ("system", decision 179) and the regional format of times and
//! numbers (178).

#![allow(unsafe_code)]

use std::io;
use windows_sys::Win32::Globalization::{
    GetUserDefaultLocaleName, GetUserPreferredUILanguages, MUI_LANGUAGE_NAME,
};

/// `LOCALE_NAME_MAX_LENGTH`.
const LOCALE_NAME_MAX: usize = 85;

/// The display languages the user chose in Windows, best first, as BCP 47 tags (`ru-RU`, `en-US`).
pub fn preferred_ui_languages() -> io::Result<Vec<String>> {
    let mut count: u32 = 0;
    let mut len: u32 = 0;
    // SAFETY: a size query: no buffer, `count` and `len` are valid writable u32s.
    let ok = unsafe {
        GetUserPreferredUILanguages(
            MUI_LANGUAGE_NAME,
            &mut count,
            std::ptr::null_mut(),
            &mut len,
        )
    };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut buf = vec![0u16; len as usize];
    // SAFETY: `buf` holds `len` u16s as the size query asked for; the outputs are valid writable u32s.
    let ok = unsafe {
        GetUserPreferredUILanguages(MUI_LANGUAGE_NAME, &mut count, buf.as_mut_ptr(), &mut len)
    };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(split_multi_string(&buf))
}

/// The user's regional format ("Region" in Windows settings), e.g. `de-DE`.
pub fn regional_format() -> io::Result<String> {
    let mut buf = [0u16; LOCALE_NAME_MAX];
    // SAFETY: `buf` is a writable buffer of the length passed.
    let len = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), LOCALE_NAME_MAX as i32) };
    if len <= 0 {
        return Err(io::Error::last_os_error());
    }
    let text = &buf[..(len as usize).saturating_sub(1)];
    Ok(String::from_utf16_lossy(text))
}

/// A Windows multi-string: NUL-separated, ending with an empty string.
fn split_multi_string(buf: &[u16]) -> Vec<String> {
    buf.split(|&c| c == 0)
        .filter(|s| !s.is_empty())
        .map(String::from_utf16_lossy)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    #[test]
    fn multi_strings_split_into_tags() {
        assert_eq!(
            split_multi_string(&wide("ru-RU\0en-US\0\0")),
            ["ru-RU", "en-US"]
        );
        assert!(split_multi_string(&wide("\0")).is_empty());
    }

    /// The real settings of this PC: some language and a region always exist.
    #[test]
    fn this_pcs_languages_and_region_can_be_read() {
        let languages = preferred_ui_languages().expect("languages");
        assert!(!languages.is_empty());
        assert!(
            languages.iter().all(|l| l.contains('-') || l.len() == 2),
            "{languages:?}"
        );
        let region = regional_format().expect("region");
        assert!(region.len() >= 2 && !region.contains('\0'), "{region:?}");
    }
}
