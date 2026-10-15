//! A safe reader over libarchive's RAR and RAR5 readers, compiled from `vendor/libarchive` by `build.rs`.
//!
//! This module is the only place in the crate that needs `unsafe` (C FFI); every call states why it is sound. The
//! rules libarchive imposes, and how they are kept:
//! - one handle per [`Reader`], freed exactly once in `Drop`; the type is neither `Send` nor `Sync` (raw pointer);
//! - the input is a boxed [`Source`] whose address is handed to libarchive and stays fixed until the handle is
//!   freed; callbacks run only inside our own calls into libarchive, which hold `&mut Reader`;
//! - entry pointers and strings are valid only until the next call, so they are copied out at once;
//! - after a failed call only `archive_error_string`, `archive_errno` and `archive_read_free` are used: libarchive
//!   leaves other state undefined (a failed format bid, for one, leaves the current-format pointer past its table).

#![allow(unsafe_code)]

use std::ffi::{CStr, c_char, c_int, c_void};
use std::fmt;
use std::io::{self, Read, Seek, SeekFrom};
use std::ptr::NonNull;

#[allow(non_camel_case_types)]
mod ffi {
    use std::ffi::{c_char, c_int, c_void};

    #[repr(C)]
    pub struct archive {
        _opaque: [u8; 0],
    }

    #[repr(C)]
    pub struct archive_entry {
        _opaque: [u8; 0],
    }

    pub const ARCHIVE_EOF: c_int = 1;
    pub const ARCHIVE_OK: c_int = 0;
    pub const ARCHIVE_WARN: c_int = -20;
    pub const AE_IFMT: u32 = 0o170_000;
    pub const AE_IFDIR: u32 = 0o040_000;
    pub const AE_IFLNK: u32 = 0o120_000;

    pub type ReadCallback =
        unsafe extern "C" fn(*mut archive, *mut c_void, *mut *const c_void) -> isize;
    pub type SkipCallback = unsafe extern "C" fn(*mut archive, *mut c_void, i64) -> i64;
    pub type SeekCallback = unsafe extern "C" fn(*mut archive, *mut c_void, i64, c_int) -> i64;

    unsafe extern "C" {
        pub fn archive_read_new() -> *mut archive;
        pub fn archive_read_free(a: *mut archive) -> c_int;
        pub fn archive_read_support_format_rar(a: *mut archive) -> c_int;
        pub fn archive_read_support_format_rar5(a: *mut archive) -> c_int;
        pub fn archive_read_set_read_callback(a: *mut archive, cb: Option<ReadCallback>) -> c_int;
        pub fn archive_read_set_skip_callback(a: *mut archive, cb: Option<SkipCallback>) -> c_int;
        pub fn archive_read_set_seek_callback(a: *mut archive, cb: Option<SeekCallback>) -> c_int;
        pub fn archive_read_set_callback_data(a: *mut archive, data: *mut c_void) -> c_int;
        pub fn archive_read_open1(a: *mut archive) -> c_int;
        pub fn archive_read_next_header(a: *mut archive, entry: *mut *mut archive_entry) -> c_int;
        pub fn archive_read_header_position(a: *mut archive) -> i64;
        pub fn archive_read_data(a: *mut archive, buf: *mut c_void, len: usize) -> isize;
        pub fn archive_read_data_skip(a: *mut archive) -> c_int;
        pub fn archive_error_string(a: *mut archive) -> *const c_char;
        pub fn archive_entry_pathname_utf8(entry: *mut archive_entry) -> *const c_char;
        pub fn archive_entry_pathname_w(entry: *mut archive_entry) -> *const u16;
        pub fn archive_entry_hardlink_w(entry: *mut archive_entry) -> *const u16;
        pub fn archive_entry_size(entry: *mut archive_entry) -> i64;
        pub fn archive_entry_size_is_set(entry: *mut archive_entry) -> c_int;
        pub fn archive_entry_filetype(entry: *mut archive_entry) -> u32;
        pub fn archive_entry_is_encrypted(entry: *mut archive_entry) -> c_int;
    }
}

/// Why a call failed.
#[derive(Debug)]
pub(crate) enum Failure {
    /// Reading the archive file itself failed.
    Input(io::Error),
    /// The file list or the entry being read is password protected.
    Encrypted,
    /// libarchive refused the data: damaged, truncated, or a feature it does not implement.
    Library(String),
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(e) => write!(f, "{e}"),
            Self::Encrypted => f.write_str("password protected"),
            Self::Library(message) => f.write_str(message),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    File,
    Dir,
    /// A symbolic or hard link: no data of its own.
    Link,
}

/// One entry as its header describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Header {
    pub(crate) name: String,
    pub(crate) size: Option<u64>,
    pub(crate) kind: Kind,
    pub(crate) encrypted: bool,
    /// Offset of the entry's header in the archive file.
    pub(crate) position: u64,
}

const CHUNK: usize = 64 * 1024;

struct Source<R> {
    inner: R,
    buf: Box<[u8]>,
    /// The error of the last failed callback; libarchive itself only learns that the call failed.
    error: Option<io::Error>,
}

/// An open RAR archive. Entries are visited in archive order with [`Reader::next`]; after each, its data is either
/// read with [`Reader::read`] or skipped with [`Reader::skip`].
pub(crate) struct Reader<R: Read + Seek> {
    handle: NonNull<ffi::archive>,
    // Boxed: libarchive keeps this address as its callback data.
    source: Box<Source<R>>,
    failed: bool,
    current_encrypted: bool,
}

impl<R: Read + Seek> fmt::Debug for Reader<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Reader")
            .field("failed", &self.failed)
            .finish_non_exhaustive()
    }
}

/// `data` is the `Source<R>` registered in [`Reader::open`].
unsafe fn source<'a, R>(data: *mut c_void) -> &'a mut Source<R> {
    // SAFETY: (caller) `data` is the boxed source registered with this handle; it outlives the handle, and
    // callbacks run only inside calls that hold `&mut Reader`, so no other reference to it exists meanwhile.
    unsafe { &mut *data.cast::<Source<R>>() }
}

unsafe extern "C" fn read_callback<R: Read + Seek>(
    _: *mut ffi::archive,
    data: *mut c_void,
    out: *mut *const c_void,
) -> isize {
    // SAFETY: `data` is what `open` registered (see `source`).
    let src = unsafe { source::<R>(data) };
    loop {
        match src.inner.read(&mut src.buf) {
            Ok(n) => {
                // SAFETY: `out` is libarchive's valid out-pointer; the buffer stays untouched until the next
                // callback, which is as long as libarchive uses it.
                unsafe { *out = src.buf.as_ptr().cast() };
                // `n <= CHUNK`, far below isize::MAX.
                return isize::try_from(n).unwrap_or(-1);
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => {
                src.error = Some(e);
                return -1;
            }
        }
    }
}

unsafe extern "C" fn seek_callback<R: Read + Seek>(
    _: *mut ffi::archive,
    data: *mut c_void,
    offset: i64,
    whence: c_int,
) -> i64 {
    // SAFETY: `data` is what `open` registered (see `source`).
    let src = unsafe { source::<R>(data) };
    let to = match whence {
        0 => match u64::try_from(offset) {
            Ok(o) => SeekFrom::Start(o),
            Err(_) => return -1,
        },
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return -1,
    };
    match src.inner.seek(to) {
        Ok(at) => i64::try_from(at).unwrap_or(-1),
        Err(e) => {
            src.error = Some(e);
            -1
        }
    }
}

unsafe extern "C" fn skip_callback<R: Read + Seek>(
    _: *mut ffi::archive,
    data: *mut c_void,
    request: i64,
) -> i64 {
    // SAFETY: `data` is what `open` registered (see `source`).
    let src = unsafe { source::<R>(data) };
    if request <= 0 {
        return 0;
    }
    match src.inner.seek(SeekFrom::Current(request)) {
        Ok(_) => request,
        Err(e) => {
            src.error = Some(e);
            -1
        }
    }
}

impl<R: Read + Seek> Reader<R> {
    /// Opens `inner` (positioned at the archive's first byte) with the RAR and RAR5 readers only.
    pub(crate) fn open(inner: R) -> Result<Self, Failure> {
        // SAFETY: plain constructor; null means out of memory.
        let handle = NonNull::new(unsafe { ffi::archive_read_new() })
            .ok_or_else(|| Failure::Input(io::ErrorKind::OutOfMemory.into()))?;
        let mut reader = Self {
            handle,
            source: Box::new(Source {
                inner,
                buf: vec![0; CHUNK].into_boxed_slice(),
                error: None,
            }),
            failed: false,
            current_encrypted: false,
        };
        let a = reader.handle.as_ptr();
        let data: *mut Source<R> = &mut *reader.source;
        // SAFETY: `a` is a fresh handle in the "new" state these setters require; the callbacks have the C
        // signatures libarchive expects and `data` stays valid until `archive_read_free` in `Drop`.
        let rc = unsafe {
            ffi::archive_read_support_format_rar(a);
            ffi::archive_read_support_format_rar5(a);
            ffi::archive_read_set_read_callback(a, Some(read_callback::<R>));
            ffi::archive_read_set_seek_callback(a, Some(seek_callback::<R>));
            ffi::archive_read_set_skip_callback(a, Some(skip_callback::<R>));
            ffi::archive_read_set_callback_data(a, data.cast());
            ffi::archive_read_open1(a)
        };
        if rc != ffi::ARCHIVE_OK && rc != ffi::ARCHIVE_WARN {
            return Err(reader.failure(false));
        }
        Ok(reader)
    }

    /// The next entry; `None` after the last one. Data of the previous entry that was not read is skipped.
    pub(crate) fn next(&mut self) -> Result<Option<Header>, Failure> {
        self.usable()?;
        let a = self.handle.as_ptr();
        let mut entry: *mut ffi::archive_entry = std::ptr::null_mut();
        // SAFETY: valid handle in a reading state (`usable`); `entry` receives a pointer owned by the handle.
        let rc = unsafe { ffi::archive_read_next_header(a, &raw mut entry) };
        if rc == ffi::ARCHIVE_EOF {
            return Ok(None);
        }
        let Some(entry) = NonNull::new(entry) else {
            return Err(self.failure(false));
        };
        let e = entry.as_ptr();
        if rc != ffi::ARCHIVE_OK && rc != ffi::ARCHIVE_WARN {
            // SAFETY: libarchive always hands back its entry, and sets the encryption flags before it fails on
            // an encrypted file list.
            let encrypted = unsafe { ffi::archive_entry_is_encrypted(e) } != 0;
            return Err(self.failure(encrypted));
        }
        // SAFETY: `e` is valid until the next call on the handle; every string is copied before returning.
        let header = unsafe {
            let kind = match ffi::archive_entry_filetype(e) & ffi::AE_IFMT {
                ffi::AE_IFDIR => Kind::Dir,
                ffi::AE_IFLNK => Kind::Link,
                _ if !ffi::archive_entry_hardlink_w(e).is_null() => Kind::Link,
                _ => Kind::File,
            };
            Header {
                name: pathname(e),
                size: (ffi::archive_entry_size_is_set(e) != 0)
                    .then(|| u64::try_from(ffi::archive_entry_size(e)).ok())
                    .flatten(),
                kind,
                encrypted: ffi::archive_entry_is_encrypted(e) != 0,
                position: u64::try_from(ffi::archive_read_header_position(a)).unwrap_or(0),
            }
        };
        self.current_encrypted = header.encrypted;
        Ok(Some(header))
    }

    /// Reads the current entry's data into `buf`; `Ok(0)` at its end.
    pub(crate) fn read(&mut self, buf: &mut [u8]) -> Result<usize, Failure> {
        self.usable()?;
        if buf.is_empty() {
            return Ok(0);
        }
        // SAFETY: valid handle; `buf` is writable for `buf.len()` bytes.
        let n = unsafe {
            ffi::archive_read_data(self.handle.as_ptr(), buf.as_mut_ptr().cast(), buf.len())
        };
        usize::try_from(n).map_err(|_| self.failure(self.current_encrypted))
    }

    /// Moves past the current entry's data without returning it.
    pub(crate) fn skip(&mut self) -> Result<(), Failure> {
        self.usable()?;
        // SAFETY: valid handle.
        let rc = unsafe { ffi::archive_read_data_skip(self.handle.as_ptr()) };
        if rc != ffi::ARCHIVE_OK && rc != ffi::ARCHIVE_WARN {
            return Err(self.failure(self.current_encrypted));
        }
        Ok(())
    }

    fn usable(&self) -> Result<(), Failure> {
        if self.failed {
            return Err(Failure::Library(
                "the archive could not be read further".into(),
            ));
        }
        Ok(())
    }

    /// Describes the failure of the last call and marks the handle unusable.
    fn failure(&mut self, encrypted: bool) -> Failure {
        self.failed = true;
        if let Some(e) = self.source.error.take() {
            return Failure::Input(e);
        }
        if encrypted {
            return Failure::Encrypted;
        }
        // SAFETY: valid handle; the message is owned by the handle and copied at once.
        let message = unsafe {
            let p: *const c_char = ffi::archive_error_string(self.handle.as_ptr());
            if p.is_null() {
                String::new()
            } else {
                CStr::from_ptr(p).to_string_lossy().into_owned()
            }
        };
        Failure::Library(if message.is_empty() {
            "unknown error".into()
        } else {
            message
        })
    }
}

impl<R: Read + Seek> Drop for Reader<R> {
    fn drop(&mut self) {
        // SAFETY: the handle is valid and freed exactly once; the source box is dropped after this returns.
        unsafe { ffi::archive_read_free(self.handle.as_ptr()) };
    }
}

/// The entry's path as UTF-8, or from UTF-16 if libarchive cannot convert it; lossy, it is display text only.
///
/// # Safety
/// `e` must be a valid entry pointer.
unsafe fn pathname(e: *mut ffi::archive_entry) -> String {
    // SAFETY: (caller) `e` is valid; returned strings are NUL-terminated and owned by the entry.
    unsafe {
        let utf8 = ffi::archive_entry_pathname_utf8(e);
        if !utf8.is_null() {
            return CStr::from_ptr(utf8).to_string_lossy().into_owned();
        }
        let wide = ffi::archive_entry_pathname_w(e);
        if wide.is_null() {
            return String::new();
        }
        let mut len = 0;
        while *wide.add(len) != 0 {
            len += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(wide, len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn a_file_that_is_not_rar_is_refused_without_touching_freed_state() {
        let err =
            Reader::open(Cursor::new(b"not an archive at all".to_vec())).expect_err("not rar");
        match err {
            Failure::Library(message) => assert!(message.contains("Unrecognized"), "{message}"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn a_failing_input_is_reported_as_the_input_error() {
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("disk on fire"))
            }
        }
        impl Seek for Broken {
            fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
                Err(io::Error::other("disk on fire"))
            }
        }
        match Reader::open(Broken).expect_err("broken") {
            Failure::Input(e) => assert_eq!(e.to_string(), "disk on fire"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn calls_after_a_failure_are_refused_by_the_wrapper() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/rar/test_read_format_rar5_compressed.rar"
        );
        let mut bytes = std::fs::read(path).expect("fixture");
        let len = bytes.len();
        for b in &mut bytes[len - 200..len - 100] {
            *b ^= 0x5a;
        }
        let mut reader = Reader::open(Cursor::new(bytes)).expect("opened");
        reader.next().expect("header").expect("an entry");
        let mut buf = vec![0; 4096];
        let failed = loop {
            match reader.read(&mut buf) {
                Ok(0) => break false,
                Ok(_) => {}
                Err(_) => break true,
            }
        };
        assert!(failed, "corrupted data must fail");
        assert!(matches!(reader.next(), Err(Failure::Library(m)) if m.contains("further")));
        assert!(reader.read(&mut buf).is_err());
        assert!(reader.skip().is_err());
    }
}
